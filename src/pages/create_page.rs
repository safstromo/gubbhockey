use leptos::prelude::*;
use leptos_router::components::Redirect;

use crate::{
    auth::validate_admin,
    components::{
        cup_card::CupCard, cup_form::CupForm, date_picker::DatePicker, event_tab::EventTab,
        gameday_create::GamedayCreate,
    },
    models::{Cup, Gameday},
};

#[component]
pub fn CreatePage() -> impl IntoView {
    let admin_check = Resource::new(
        || (),
        |_| async move { validate_admin().await.unwrap_or(false) },
    );

    let (invalidate_gamedays, set_invalidate_gamedays) = signal(false);
    let (tab_change, set_tab_change) = signal(true);
    let (show_create_day, set_show_create_day) = signal(false);
    let (show_create_cup, set_show_create_cup) = signal(false);
    let gamedays = Resource::new(|| (), |_| async move { get_all_gamedays().await });
    let cups = Resource::new(|| (), |_| async move { get_all_cups().await });

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
                        <div class="flex flex-col w-full items-center relative">
                            <div class="dropdown">
                                <div tabindex="0" role="button" class="btn m-1">
                                    Skapa event
                                </div>
                                <ul
                                    tabindex="0"
                                    class="dropdown-content menu bg-base-100 rounded-box z-[1] w-52 p-2 shadow"
                                >
                                    <li>
                                        <a on:click=move |_| {
                                            set_show_create_day.set(!show_create_day.get());
                                            set_show_create_cup.set(false);
                                        }>Speldag</a>
                                    </li>
                                    <li>
                                        <a on:click=move |_| {
                                            set_show_create_cup.set(!show_create_cup.get());
                                            set_show_create_day.set(false);
                                        }>Cup</a>
                                    </li>
                                </ul>
                            </div>
                            <EventTab tab_change set_tab_change />
                            <Show when=move || { show_create_day.get() }>
                                <DatePicker set_invalidate_gamedays />
                            </Show>
                            <Show when=move || { show_create_cup.get() }>
                                <CupForm />
                            </Show>
                            <Show
                                when=move || { tab_change.get() }
                                fallback=move || {
                                    view! {
                                        <Transition fallback=move || view! { <p>"Loading..."</p> }>
                                            <h3 class="text-center text-xl mt-2">Kommande cupper</h3>
                                            <ul class="flex flex-col items-center w-11/12">
                                                {move || Suspend::new(async move {
                                                    let cups = cups.await.expect("No cups found");
                                                    cups.into_iter()
                                                        .map(|cup| {
                                                            view! {
                                                                <li class="flex justify-center my-2">
                                                                    <CupCard cup edit_button=true />
                                                                </li>
                                                            }
                                                        })
                                                        .collect_view()
                                                })}
                                            </ul>
                                        </Transition>
                                    }
                                }
                            >
                                <Transition fallback=move || view! { <p>"Loading..."</p> }>
                                    <h3 class="text-center text-xl mt-2">Kommande Speldagar</h3>
                                    <ul class="flex flex-col items-center w-11/12">
                                        {move || Suspend::new(async move {
                                            let days = gamedays.await.expect("No gamedays found");
                                            days.into_iter()
                                                .map(|day| {
                                                    view! {
                                                        <li class="flex justify-center my-2">
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
                            </Show>
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

#[server]
async fn get_all_cups() -> Result<Vec<Cup>, ServerFnError> {
    use crate::database::get_db;
    use tracing::{error, info};

    if let Err(err) = validate_admin().await {
        return Err(err);
    }

    let pool = get_db();
    match sqlx::query_as!(
        Cup,
        r#"
        SELECT 
            c.cup_id, 
            c.start_date, 
            c.end_date,
            c.title,
            c.info,
            COUNT(pc.player_id) as player_count 
        FROM 
            cup c
        LEFT JOIN 
            player_gameday pc ON c.cup_id = pc.gameday_id
        WHERE 
            c.start_date >= NOW() 
        GROUP BY 
            c.cup_id, c.start_date, c.end_date, c.title, c.info
        ORDER BY 
            c.start_date ASC
        "#
    )
    .fetch_all(pool)
    .await
    {
        Ok(results) => {
            info!("Successfully retrieved all cups with player counts.");
            Ok(results)
        }
        Err(e) => {
            error!("Database error: {:?}", e);
            Err(ServerFnError::ServerError(
                "Failed to get all cups.".to_string(),
            ))
        }
    }
}
