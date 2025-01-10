use leptos::{prelude::*, task::spawn_local};

use crate::models::Gameday;

#[component]
pub fn LeaveButton(
    gameday_id: i32,
    gamedays_joined: ReadSignal<Vec<Gameday>>,
    set_gamedays_joined: WriteSignal<Vec<Gameday>>,
) -> impl IntoView {
    view! {
        <button
            class="btn btn-error h-20 m-2 flex-col"
            on:click=move |_| {
                spawn_local(async move {
                    if leave_gameday(gameday_id).await.is_ok() {
                        delete_joined(set_gamedays_joined, gamedays_joined, gameday_id);
                    }
                });
            }
        >
            <p class="font-bold">Kommer</p>
            <p class="font-bold">inte</p>
        </button>
    }
}

fn delete_joined(
    set_gamedays_joined: WriteSignal<Vec<Gameday>>,
    gamedays_joined: ReadSignal<Vec<Gameday>>,
    gameday_id: i32,
) {
    let updated_gamedays: Vec<Gameday> = gamedays_joined
        .get_untracked()
        .into_iter()
        .filter(|day| day.gameday_id != gameday_id)
        .collect();

    set_gamedays_joined.set(updated_gamedays);
}

#[server]
async fn leave_gameday(gameday_id: i32) -> Result<(), ServerFnError> {
    use crate::auth::user_from_session;
    use crate::database::get_db;
    use leptos::logging::log;

    match user_from_session().await {
        Ok(user) => {
            let pool = get_db();
            match sqlx::query!(
                r#"
        DELETE FROM player_gameday
        WHERE player_id = $1 AND gameday_id = $2
        "#,
                user.player_id,
                gameday_id
            )
            .execute(pool)
            .await
            {
                Ok(_) => {
                    log!(
                        "Player: {:?} left gameday: {:?}",
                        user.player_id,
                        gameday_id
                    );
                    Ok(())
                }
                Err(e) => {
                    log!("Database error: {:?}", e);
                    Err(ServerFnError::ServerError(
                        "Failed to remove player from gameday.".to_string(),
                    ))
                }
            }
        }
        Err(err) => Err(err),
    }
}
