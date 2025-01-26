use leptos::{prelude::*, task::spawn_local};

use crate::models::Cup;

#[component]
pub fn LeaveCupButton(
    cup_id: i32,
    cups_joined: ReadSignal<Vec<Cup>>,
    set_cups_joined: WriteSignal<Vec<Cup>>,
    set_refetch_players: WriteSignal<bool>,
) -> impl IntoView {
    view! {
        <h3 class="text-center underline m-2">
            "Du är anmäld till denna cuppen, vill du avanmäla dig?"
        </h3>
        <button
            class="btn btn-error h-16 m-2 flex-col"
            on:click=move |_| {
                spawn_local(async move {
                    if leave_cup(cup_id).await.is_ok() {
                        delete_joined(set_cups_joined, cups_joined, cup_id);
                        set_refetch_players.set(true);
                    }
                });
            }
        >
            <p class="font-bold">"Jag kommer inte."</p>
        </button>
    }
}

fn delete_joined(
    set_cups_joined: WriteSignal<Vec<Cup>>,
    cups_joined: ReadSignal<Vec<Cup>>,
    cup_id: i32,
) {
    let updated_cups: Vec<Cup> = cups_joined
        .get_untracked()
        .into_iter()
        .filter(|cup| cup.cup_id != cup_id)
        .collect();

    set_cups_joined.set(updated_cups);
}

#[server]
async fn leave_cup(cup_id: i32) -> Result<(), ServerFnError> {
    use crate::auth::user_from_session;
    use crate::database::get_db;
    use tracing::{error, info};

    match user_from_session().await {
        Ok(user) => {
            let pool = get_db();
            match sqlx::query!(
                r#"
        DELETE FROM player_cup
        WHERE player_id = $1 AND cup_id = $2
        "#,
                user.player_id,
                cup_id
            )
            .execute(pool)
            .await
            {
                Ok(_) => {
                    info!("Player: {:?} left cup: {:?}", user.player_id, cup_id);
                    Ok(())
                }
                Err(e) => {
                    error!("Database error: {:?}", e);
                    Err(ServerFnError::ServerError(
                        "Failed to remove player from cup.".to_string(),
                    ))
                }
            }
        }
        Err(err) => Err(err),
    }
}
