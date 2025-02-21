use leptos::{prelude::*, task::spawn_local};
use leptos_router::hooks::use_params;
use leptos_router::params::Params;

use crate::{
    auth::get_auth_url,
    components::{
        join_cup_form::JoinCupForm, leave_cup_button::LeaveCupButton, loading::Loading,
        not_found::NotFound,
    },
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

    let (cups_joined, set_cups_joined) = signal(Vec::new());
    let (refetch_players, set_refetch_players) = signal(false);

    Effect::new(move |_| {
        if let Some(Ok(_player_data)) = player.get() {
            spawn_local(async move {
                if let Ok(gamedays) = get_cups_by_player().await {
                    set_cups_joined.set(gamedays);
                }
            });
        }
    });

    Effect::new(move |_| {
        if refetch_players.get() {
            players.refetch();
            set_refetch_players.set(false);
        }
    });

    view! {
        <div class="flex flex-col w-full items-center justify-center relative">
            <Transition fallback=move || {
                view! { <Loading /> }
            }>
                {move || Suspend::new(async move {
                    let cup = cup.await;
                    let cup_exist = cup.is_ok();
                    let mut info = Vec::new();
                    if cup_exist {
                        if let Some(cup_info) = cup.clone().unwrap().info {
                            info = cup_info.split("\n").map(String::from).collect::<Vec<_>>();
                        }
                    }
                    view! {
                        <Show
                            when=move || { cup_exist }
                            fallback=|| {
                                view! { <NotFound /> }
                            }
                        >
                            <h2 class="text-center text-bold text-3xl m-4">
                                {cup.clone().expect("cup should be there").title}
                            </h2>
                            <ul class="mb-10">
                                {info
                                    .clone()
                                    .into_iter()
                                    .map(|line| {
                                        view! {
                                            <li>
                                                <p class="text-center m-2">{line}</p>
                                            </li>
                                        }
                                    })
                                    .collect_view()}
                            </ul>
                            <Suspense>
                                {move || Suspend::new(async move {
                                    let player_loggedin = player.await;
                                    view! {
                                        <Show
                                            when=move || { player_loggedin.is_ok() }
                                            fallback=|| {
                                                view! {
                                                    <button
                                                        class="underline"
                                                        on:click=move |_| {
                                                            spawn_local(async {
                                                                let _ = get_auth_url().await;
                                                            });
                                                        }
                                                    >
                                                        Logga in för anmälan
                                                    </button>
                                                }
                                            }
                                        >
                                            <Show
                                                when=move || { is_player_joined(cups_joined.get(), id()) }
                                                fallback=move || {
                                                    view! {
                                                        <JoinCupForm
                                                            cup_id=id()
                                                            set_refetch_players
                                                            set_cups_joined
                                                        />
                                                    }
                                                }
                                            >
                                                <LeaveCupButton
                                                    cup_id=id()
                                                    cups_joined
                                                    set_cups_joined
                                                    set_refetch_players
                                                />
                                            </Show>
                                        </Show>
                                    }
                                })}
                            </Suspense>
                        </Show>
                    }
                })}
            </Transition>
            <h2 class="text-center text-bold text-2xl mt-6">"Anmälda spelare"</h2>
            <div class="flex md:flex-col justify-around w-full">
                <Transition fallback=move || view! { <Loading /> }>

                    <div class="flex flex-col justify-center">
                        <h3 class="text-center text-bold underline text-xl mt-6 mb-2">"Backar"</h3>
                        <ul class="flex flex-col items-center">
                            {move || Suspend::new(async move {
                                let players_vec = players.await.expect("No players found");
                                players_vec
                                    .clone()
                                    .into_iter()
                                    .filter(|player| player.position == *"defender")
                                    .map(|player| {
                                        view! {
                                            <li class="my-1">
                                                <p class="text-center text-xs">{player.name}</p>
                                            </li>
                                        }
                                    })
                                    .collect_view()
                            })}
                        </ul>
                    </div>
                </Transition>
                <Transition fallback=move || view! { <Loading /> }>
                    <div class="flex flex-col justify-center">
                        <h3 class="text-center text-bold underline text-xl mt-6 mb-2">
                            "Forwards"
                        </h3>
                        <ul class="flex flex-col justify-center">
                            {move || Suspend::new(async move {
                                let players_vec = players.await.expect("No players found");
                                players_vec
                                    .clone()
                                    .into_iter()
                                    .filter(|player| player.position == *"forward")
                                    .map(|player| {
                                        view! {
                                            <li class="my-1">
                                                <p class="text-center text-xs">{player.name}</p>
                                            </li>
                                        }
                                    })
                                    .collect_view()
                            })}
                        </ul>
                    </div>
                </Transition>
            </div>
            <Transition fallback=move || view! { <Loading /> }>
                <div class="flex flex-col justify-center">
                    <h3 class="text-center text-bold underline text-xl mt-6 mb-2">"Målvakter"</h3>
                    <ul class="flex flex-col justify-center ">
                        {move || Suspend::new(async move {
                            let players_vec = players.await.expect("No players found");
                            players_vec
                                .clone()
                                .into_iter()
                                .filter(|player| player.position == *"goalkeeper")
                                .map(|player| {
                                    view! {
                                        <li class="my-1">
                                            <p class="text-center text-xs">{player.name}</p>
                                        </li>
                                    }
                                })
                                .collect_view()
                        })}
                    </ul>
                </div>
            </Transition>
        </div>
    }
}

#[derive(Params, PartialEq)]
struct CupParam {
    id: Option<i32>,
}

fn is_player_joined(cups_joined: Vec<Cup>, cup_id: i32) -> bool {
    cups_joined.iter().any(|cup| cup.cup_id == cup_id)
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

#[server]
pub async fn get_cups_by_player() -> Result<Vec<Cup>, ServerFnError> {
    use crate::auth::user_from_session;
    use crate::database::get_db;
    use tracing::{error, info};

    match user_from_session().await {
        Ok(user) => {
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
            Cup c
        LEFT JOIN 
            player_cup pc ON c.cup_id = pc.cup_id
        WHERE
            pc.player_id = $1
        GROUP BY 
            c.cup_id, c.start_date, c.end_date, c.title, c.info
        ORDER BY
            c.start_date DESC        
        "#,
                user.player_id
            )
            .fetch_all(pool)
            .await
            {
                Ok(cups) => {
                    info!(
                        "Successfully retrieved {} cups for player {}",
                        cups.len(),
                        user.player_id
                    );
                    Ok(cups)
                }
                Err(e) => {
                    error!("Database error while fetching cups for player: {:?}", e);
                    Err(ServerFnError::from(e))
                }
            }
        }
        Err(err) => Err(err),
    }
}
