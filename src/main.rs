#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use gubbhockey::app::*;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let database_url = std::env::var("DATABASE_URL").expect("no database url specify");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(4)
        .connect(database_url.as_str())
        .await
        .expect("could not connect to database_url");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("migrations failed");

    // join_gameday(&pool).await.expect("nope");

    // let now = Utc::now();
    // insert_gameday(&pool, now).await.expect("nope");
    //
    //     let test = sqlx::query!(
    //         r#"
    // INSERT INTO player ( name, surname, email,  access_group )
    // VALUES ( $1, $2, $3, $4 )
    // RETURNING player_id
    //         "#,
    //         "test",
    //         "testsson",
    //         "testar@test.com",
    //         "admin"
    //     )
    //     .fetch_one(&pool)
    //     .await
    //     .expect("Could not insert");

    //     let test = sqlx::query!(
    //         r#"
    // INSERT INTO player ( name, surname, email,  access_group )
    // VALUES ( $1, $2, $3, $4 )
    // RETURNING player_id
    //         "#,
    //         "test",
    //         "testsson",
    //         "testar@test.com",
    //         "admin"
    //     )
    //     .fetch_one(&pool)
    //     .await
    //     .expect("Could not insert");
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
