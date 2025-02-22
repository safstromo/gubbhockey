#[cfg(feature = "ssr")]
use leptos::prelude::*;
#[cfg(feature = "ssr")]
use tracing::{error, info};

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use gubbhockey::app::*;
    use gubbhockey::database::init_db;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    extern crate dotenv;
    use dotenv::dotenv;
    use tokio_cron_scheduler::Job;
    use tokio_cron_scheduler::JobScheduler;
    use tower_cookies::CookieManagerLayer;
    use tracing_subscriber::{EnvFilter, FmtSubscriber};

    dotenv().ok();

    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    init_db().await.expect("Unable to init db");

    let sched = JobScheduler::new()
        .await
        .expect("could not create scheduler");
    // Add async job to clear db
    sched
        .add(
            //3am every day
            Job::new_async("0 0 3 * * *", |uuid, mut l| {
                Box::pin(async move {
                    info!("Running scheduled job for cleaning up db");
                    if let Err(err) = delete_old_sessions().await {
                        info!("Failed to delete old sessions: {}", err);
                    }

                    if let Err(err) = delete_old_pkce().await {
                        info!("Failed to delete old pkce: {}", err);
                    }

                    // Query the next execution time for this job
                    let next_tick = l.next_tick_for_job(uuid).await;
                    match next_tick {
                        Ok(Some(ts)) => info!("Next time for job is {:?}", ts),
                        _ => error!("Could not get next tick for job"),
                    }
                })
            })
            .expect("unable to create async job"),
        )
        .await
        .expect("unable to create scheduled job");

    sched.start().await.expect("unable to start scheduler");

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .layer(CookieManagerLayer::new())
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    info!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(feature = "ssr")]
async fn delete_old_sessions() -> Result<(), ServerFnError> {
    use gubbhockey::database::get_db;

    let pool = get_db();

    info!("Deleting old sessions");
    match sqlx::query!(
        r#"
        DELETE FROM session
        WHERE expires_at <= NOW()
        "#,
    )
    .execute(pool)
    .await
    {
        Ok(_) => {
            info!("Sessions deleted successfully.");
            Ok(())
        }
        Err(e) => {
            error!("Database error: {:?}", e);
            Err(ServerFnError::ServerError(
                "Failed to delete old sessions.".to_string(),
            ))
        }
    }
}

#[cfg(feature = "ssr")]
async fn delete_old_pkce() -> Result<(), ServerFnError> {
    use gubbhockey::database::get_db;

    let pool = get_db();

    info!("Deleting old pkce");
    match sqlx::query!(
        r#"
        DELETE FROM pkce_store
        WHERE expires_at <= NOW()
        "#,
    )
    .execute(pool)
    .await
    {
        Ok(_) => {
            info!("PKCE store deleted successfully.");
            Ok(())
        }
        Err(e) => {
            error!("Database error: {:?}", e);
            Err(ServerFnError::ServerError(
                "Failed to delete old pkce.".to_string(),
            ))
        }
    }
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
