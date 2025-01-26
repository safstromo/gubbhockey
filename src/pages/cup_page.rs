use leptos::prelude::*;
use leptos_router::{components::Redirect, hooks::use_params, params::Params};

use crate::{
    components::not_found::NotFound,
    models::{Cup, CupPlayer, Player},
};

#[component]
pub fn CupPage() -> impl IntoView {
    let params = use_params::<CupParam>();
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
        |id| async move { get_players_by_cup_id(id).await },
    );
    let cup = Resource::new(move || id(), |id| async move { get_cup_by_id(id).await });

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
                                view! { <p>"Loading..."</p> }
                            }>
                                {move || Suspend::new(async move {
                                    let cup = cup.await;
                                    let cup_exist = cup.is_ok();
                                    view! {
                                        <Show
                                            when=move || { cup_exist }
                                            fallback=|| {
                                                view! { <NotFound /> }
                                            }
                                        >
                                            <p>{cup.clone().expect("cup should be there").title}</p>
                                            <p>{cup.clone().expect("cup should be there").info}</p>
                                        </Show>
                                    }
                                })}

                            </Transition>
                            <h2 class="text-center text-bold text-2xl mt-6">"Anmälda spelare"</h2>
                            <Transition fallback=move || view! { <p>"Loading..."</p> }>
                                <h3 class="text-center text-bold underline text-xl mt-6 mb-2">
                                    "Målvakter"
                                </h3>
                                <ul class="flex flex-col items-center w-11/12">
                                    {move || Suspend::new(async move {
                                        let players_vec = players.await.expect("No players found");
                                        players_vec
                                            .clone()
                                            .into_iter()
                                            .filter(|player| player.position == *"goalkeeper")
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
                            <Transition fallback=move || view! { <p>"Loading..."</p> }>
                                <h3 class="text-center text-bold underline text-xl mt-6 mb-2">
                                    "Forward"
                                </h3>
                                <ul class="flex flex-col items-center w-11/12">
                                    {move || Suspend::new(async move {
                                        let players_vec = players.await.expect("No players found");
                                        players_vec
                                            .clone()
                                            .into_iter()
                                            .filter(|player| player.position == *"forward")
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
struct CupParam {
    id: Option<i32>,
}

#[server]
async fn get_cup_by_id(id: i32) -> Result<Cup, ServerFnError> {
    use crate::database::get_db;
    use http::StatusCode;
    use tracing::{error, info};

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
        COUNT(pc.player_id) AS player_count
    FROM 
        cup c
    LEFT JOIN 
        player_cup pc ON c.cup_id = pc.cup_id
    WHERE 
        c.cup_id = $1
    GROUP BY 
        c.cup_id, c.start_date, c.end_date, c.title, c.info
    "#,
        id
    )
    .fetch_one(pool)
    .await
    {
        Ok(results) => {
            info!("Successfully retrieved cup with player counts.");
            Ok(results)
        }
        Err(e) => {
            error!("Database error: {:?}", e);
            let opts = expect_context::<leptos_axum::ResponseOptions>();
            opts.set_status(StatusCode::NOT_FOUND);
            Err(ServerFnError::ServerError("No cup found.".to_string()))
        }
    }
}

#[server]
async fn get_players_by_cup_id(id: i32) -> Result<Vec<CupPlayer>, ServerFnError> {
    use crate::database::get_db;
    use http::StatusCode;
    use tracing::{error, info};

    let pool = get_db();
    match sqlx::query_as!(
        CupPlayer,
        r#"
    SELECT 
        p.name, 
        pc.position
    FROM 
        player p
    JOIN 
        player_cup pc ON p.player_id = pc.player_id
    JOIN 
        cup c ON pc.cup_id = c.cup_id
    WHERE 
        c.cup_id = $1
    "#,
        id
    )
    .fetch_all(pool)
    .await
    {
        Ok(results) => {
            info!("Successfully retrieved players on cup: {}.", id);
            Ok(results)
        }
        Err(e) => {
            error!("Database error: {:?}", e);
            let opts = expect_context::<leptos_axum::ResponseOptions>();
            opts.set_status(StatusCode::NOT_FOUND);
            Err(ServerFnError::ServerError("No players found.".to_string()))
        }
    }
}
