use leptos::{prelude::*, task::spawn_local};

use crate::models::Gameday;

#[component]
pub fn JoinButton(
    logged_in: ReadSignal<bool>,
    gameday_id: i32,
    set_gamedays_joined: WriteSignal<Vec<Gameday>>,
) -> impl IntoView {
    view! {
        <button
            class="btn btn-success h-20 m-2 flex-col"
            on:click=move |_| {
                spawn_local(async move {
                    if join_gameday(gameday_id).await.is_ok() {
                        add_joined(set_gamedays_joined).await;
                    }
                });
            }
            disabled=move || !logged_in.get()
        >
            <p class="font-bold">Jag</p>
            <p class="font-bold">kommer</p>
        </button>
    }
}

async fn add_joined(set_gamedays_joined: WriteSignal<Vec<Gameday>>) {
    if let Ok(gamedays) = get_gamedays_by_player().await {
        set_gamedays_joined.set(gamedays);
    }
}

#[server]
async fn join_gameday(gameday_id: i32) -> Result<(), ServerFnError> {
    use crate::auth::user_from_session;
    use crate::database::get_db;
    use tracing::{error, info};

    match user_from_session().await {
        Ok(user) => {
            let pool = get_db();
            match sqlx::query!(
                r#"
        INSERT INTO player_gameday (player_id, gameday_id)
        VALUES ($1, $2)
        "#,
                user.player_id,
                gameday_id
            )
            .execute(pool)
            .await
            {
                Ok(_) => {
                    info!("Player: {:?} joined: {:?}", user.player_id, gameday_id);
                    Ok(())
                }
                Err(e) => {
                    error!("Database error: {:?}", e);
                    Err(ServerFnError::ServerError(
                        "Failed to add player to gameday.".to_string(),
                    ))
                }
            }
        }
        Err(err) => Err(err),
    }
}

#[server]
pub async fn get_gamedays_by_player() -> Result<Vec<Gameday>, ServerFnError> {
    use crate::auth::user_from_session;
    use crate::database::get_db;
    use tracing::{error, info};

    match user_from_session().await {
        Ok(user) => {
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
            Gameday g
        LEFT JOIN 
            player_gameday pg ON g.gameday_id = pg.gameday_id
        WHERE
            pg.player_id = $1
        GROUP BY 
            g.gameday_id, g.start_date, g.end_date
        ORDER BY
            g.start_date DESC        
        "#,
                user.player_id
            )
            .fetch_all(pool)
            .await
            {
                Ok(gamedays) => {
                    info!(
                        "Successfully retrieved {} gamedays for player {}",
                        gamedays.len(),
                        user.player_id
                    );
                    Ok(gamedays)
                }
                Err(e) => {
                    error!("Database error while fetching gamedays for player: {:?}", e);
                    Err(ServerFnError::from(e))
                }
            }
        }
        Err(err) => Err(err),
    }
}
