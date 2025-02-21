use leptos::{prelude::*, task::spawn_local};
use leptos_router::{components::Redirect, hooks::use_params, params::Params};

use crate::{
    components::{
        gameday_card::GamedayCard, join_button::get_gamedays_by_player, not_found::NotFound,
    },
    models::{get_players_by_gameday, Gameday, Player},
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

    let player =
        use_context::<Resource<Result<Player, ServerFnError>>>().expect("player context not found");

    let players = Resource::new(
        move || id(),
        |id| async move { get_players_by_gameday(id).await },
    );
    let gameday = Resource::new(
        move || id(),
        |id| async move { get_gameday_by_id(id).await },
    );
    let (gamedays_joined, set_gamedays_joined) = signal(Vec::new());

    Effect::new(move |_| {
        if let Some(Ok(_player_data)) = player.get() {
            // set_loggedin.set(true);

            spawn_local(async move {
                if let Ok(gamedays) = get_gamedays_by_player().await {
                    set_gamedays_joined.set(gamedays);
                }
            });
        }
    });

    Effect::new(move |_| {
        if !gamedays_joined.get().is_empty() {
            players.refetch();
        }
    });

    view! {
        <div class="flex flex-col w-full items-center relative">
            <Suspense fallback=|| {
                view! { <NotFound /> }
            }>
                {move || Suspend::new(async move {
                    let player_loggedin = player.await;
                    view! {
                        <Show
                            when=move || { player_loggedin.is_ok() }
                            fallback=|| view! { <Redirect path="/" /> }
                        >
                            <Transition fallback=move || {
                                view! { <Loading /> }
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
                                            <GamedayCard
                                                gamedays_joined=gamedays_joined
                                                set_gamedays_joined=set_gamedays_joined
                                                gameday=day.clone().expect("Day should be there")
                                            />
                                        </Show>
                                    }
                                })}

                            </Transition>
                            <h2 class="text-center text-bold text-2xl mt-6">"Anmälda spelare"</h2>
                            <Transition fallback=move || view! { <Loading /> }>
                                <h3 class="text-center text-bold underline text-xl mt-6 mb-2">
                                    "Målvakter"
                                </h3>
                                <ul class="flex flex-col items-center w-11/12">
                                    {move || Suspend::new(async move {
                                        let players_vec = players.await.expect("No players found");
                                        players_vec
                                            .clone()
                                            .into_iter()
                                            .filter(|player| player.is_goalkeeper)
                                            .map(|player| {
                                                view! {
                                                    <li class="my-1">
                                                        <p>{player.name}</p>
                                                    </li>
                                                }
                                            })
                                            .collect_view()
                                    })}
                                </ul>
                            </Transition>
                            <Transition fallback=move || view! { <Loading /> }>
                                <h3 class="text-center text-bold underline text-xl mt-6 mb-2">
                                    "Utespelare"
                                </h3>
                                <ul class="flex flex-col items-center w-11/12">
                                    {move || Suspend::new(async move {
                                        let players_vec = players.await.expect("No players found");
                                        players_vec
                                            .clone()
                                            .into_iter()
                                            .filter(|player| !player.is_goalkeeper)
                                            .map(|player| {
                                                view! {
                                                    <li class="my-1">
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
