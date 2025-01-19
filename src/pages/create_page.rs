use leptos::prelude::*;
use leptos_router::components::{Redirect, A};

use crate::{
    auth::validate_admin,
    components::{date_picker::DatePicker, gameday_create::GamedayCreate},
    models::Gameday,
};

#[component]
pub fn CreatePage() -> impl IntoView {
    let admin_check = Resource::new(
        || (),
        |_| async move { validate_admin().await.unwrap_or(false) },
    );

    let (invalidate_gamedays, set_invalidate_gamedays) = signal(false);
    let gamedays = Resource::new(|| (), |_| async move { get_all_gamedays().await });

    Effect::new(move || {
        if invalidate_gamedays.get() {
            gamedays.refetch();
            set_invalidate_gamedays.set(false);
        }
    });

    view! {
        <Suspense fallback=move || {
            view! {
                <div class="flex flex-col min-h-screen w-full items-center justify-center">
                    <h3 class="text-xl text-center">"Logging in..."</h3>
                    <span class="loading loading-dots loading-lg"></span>
                </div>
            }
        }>
            {move || Suspend::new(async move {
                let is_admin = admin_check.await;
                view! {
                    <Show when=move || { is_admin } fallback=|| view! { <Redirect path="/" /> }>
                        <div class="flex flex-col min-h-screen w-full items-center relative">
                            <div class="flex justify-center">
                                <A href="/">
                                    <img src="Logo-nobg.png" alt="Logo" class="h-60 w-60" />
                                </A>
                            </div>
                            <DatePicker set_invalidate_gamedays />
                            <Transition fallback=move || view! { <p>"Loading..."</p> }>
                                <h3 class="text-center text-xl mt-6">Alla Speldagar</h3>
                                <ul class="flex flex-col items-center w-11/12">
                                    {move || Suspend::new(async move {
                                        let days = gamedays.await.expect("No gamedays found");
                                        days.into_iter()
                                            .map(|day| {
                                                view! {
                                                    <li class="my-2">
                                                        <GamedayCreate
                                                            gameday=day
                                                            set_invalidate_gamedays=Some(set_invalidate_gamedays)
                                                            redirect_on_delete=false
                                                        />
                                                    </li>
                                                }
                                            })
                                            .collect_view()
                                    })}
                                </ul>
                            </Transition>
                        </div>
                    </Show>
                }
            })}
        </Suspense>
    }
}

#[server]
async fn get_all_gamedays() -> Result<Vec<Gameday>, ServerFnError> {
    use crate::database::get_db;
    use tracing::{error, info};

    if let Err(err) = validate_admin().await {
        return Err(err);
    }

    let pool = get_db();
    match sqlx::query_as!(
        Gameday,
        r#"
        SELECT 
            g.gameday_id, 
            g.start_date, 
            g.end_date,
            COUNT(pg.player_id) as player_count 
        FROM 
            gameday g
        LEFT JOIN 
            player_gameday pg ON g.gameday_id = pg.gameday_id
        WHERE 
            g.start_date >= NOW() 
        GROUP BY 
            g.gameday_id, g.start_date, g.end_date
        ORDER BY 
            g.start_date ASC
        "#
    )
    .fetch_all(pool)
    .await
    {
        Ok(results) => {
            info!("Successfully retrieved all gamedays with player counts.");
            Ok(results)
        }
        Err(e) => {
            error!("Database error: {:?}", e);
            Err(ServerFnError::ServerError(
                "Failed to get all gamedays.".to_string(),
            ))
        }
    }
}
