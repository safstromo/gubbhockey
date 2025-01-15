use leptos::prelude::*;
use leptos_router::{
    components::{Redirect, A},
    hooks::use_params,
    params::Params,
};

use crate::{
    auth::validate_admin,
    components::{gameday_create::GamedayCreate, not_found::NotFound},
    models::{get_players_by_gameday, Gameday},
};

#[component]
pub fn DayPage() -> impl IntoView {
    let params = use_params::<Day>();
    let id = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.id)
            .unwrap_or(0)
    };

    let admin_check = Resource::new(
        || (),
        |_| async move { validate_admin().await.unwrap_or(false) },
    );

    let players = Resource::new(
        move || id(),
        |id| async move { get_players_by_gameday(id).await },
    );
    let gameday = Resource::new(
        move || id(),
        |id| async move { get_gameday_by_id(id).await },
    );

    view! {
        <div class="flex flex-col min-h-screen w-full items-center relative">
            <A href="/">
                <h1 class="text-4xl text-center mt-14 mb-6">"Falkenbergs Gubbhockey"</h1>
            </A>
            <Suspense fallback=|| {
                view! { <NotFound /> }
            }>
                {move || Suspend::new(async move {
                    let is_admin = admin_check.await;
                    view! {
                        <Show when=move || { is_admin } fallback=|| view! { <Redirect path="/" /> }>
                            <div class="absolute top-4 left-4">
                                <A href="/create">
                                    <button class="btn btn-xs btn-success">Adminpanel</button>
                                </A>
                            </div>

                            <Transition fallback=move || {
                                view! { <p>"Loading..."</p> }
                            }>
                                {move || Suspend::new(async move {
                                    let day = gameday.await;
                                    let day_exist = day.is_ok();
                                    view! {
                                        <Show
                                            when=move || { day_exist }
                                            fallback=|| {
                                                view! { <NotFound /> }
                                            }
                                        >
                                            <GamedayCreate
                                                gameday=day.clone().expect("Day should be there")
                                                set_invalidate_gamedays=None
                                                redirect_on_delete=true
                                            />
                                        </Show>
                                    }
                                })}

                            </Transition>
                            <Transition fallback=move || view! { <p>"Loading..."</p> }>
                                <h3 class="text-center text-xl mt-6">Anmälda spelare</h3>
                                <ul class="flex flex-col items-center w-11/12">
                                    {move || Suspend::new(async move {
                                        let players_vec = players.await.expect("No players found");
                                        players_vec
                                            .clone()
                                            .into_iter()
                                            .map(|player| {
                                                view! {
                                                    <li class="my-2">
                                                        <p>{player.name}</p>
                                                    </li>
                                                }
                                            })
                                            .collect_view()
                                    })}
                                </ul>
                            </Transition>
                        </Show>
                    }
                })}
            </Suspense>
        </div>
    }
}

#[derive(Params, PartialEq)]
struct Day {
    id: Option<i32>,
}

#[server]
async fn get_gameday_by_id(id: i32) -> Result<Gameday, ServerFnError> {
    use crate::database::get_db;
    use http::StatusCode;
    use tracing::{error, info};

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
            g.gameday_id = $1 
        GROUP BY 
            g.gameday_id, g.start_date, g.end_date
        "#,
        id
    )
    .fetch_one(pool)
    .await
    {
        Ok(results) => {
            info!("Successfully retrieved gameday with player counts.");
            Ok(results)
        }
        Err(e) => {
            error!("Database error: {:?}", e);
            let opts = expect_context::<leptos_axum::ResponseOptions>();
            opts.set_status(StatusCode::NOT_FOUND);
            Err(ServerFnError::ServerError("No gameday found.".to_string()))
        }
    }
}
