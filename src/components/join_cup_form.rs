use leptos::{prelude::*, task::spawn_local};

use crate::{models::Cup, pages::cup_page::get_cups_by_player};

#[component]
pub fn JoinCupForm(
    cup_id: i32,
    set_refetch_players: WriteSignal<bool>,
    set_cups_joined: WriteSignal<Vec<Cup>>,
) -> impl IntoView {
    let submit = ServerAction::<JoinCup>::new();

    let value = submit.value();
    Effect::new(move |_| {
        if value.get().is_some() {
            set_refetch_players.set(true);

            spawn_local(async move {
                add_joined(set_cups_joined).await;
            });
        }
    });

    view! {
        <ActionForm action=submit>
            <div class="flex flex-col m-2 mb-6">
                <h3 class="text-center text-bold text-2xl underline m-2">Anmälan</h3>
                <select class="select select-bordered w-full max-w-xs" required name="position">
                    <option disabled selected>
                        "Spelarposition?"
                    </option>
                    <option value="goalkeeper">Målvakt</option>
                    <option value="forward">Forward</option>
                    <option value="defender">Back</option>
                </select>
                <input type="hidden" name="cup_id" value=cup_id />
                <button class="btn btn-success mt-4" type="submit">
                    Jag kommer.
                </button>
            </div>
        </ActionForm>
    }
}

async fn add_joined(set_cups_joined: WriteSignal<Vec<Cup>>) {
    if let Ok(cups) = get_cups_by_player().await {
        set_cups_joined.set(cups);
    }
}

#[server]
async fn join_cup(position: String, cup_id: i32) -> Result<(), ServerFnError> {
    use crate::auth::user_from_session;
    use crate::database::get_db;
    use tracing::{error, info};

    match user_from_session().await {
        Ok(user) => {
            let pool = get_db();
            match sqlx::query!(
                r#"
        INSERT INTO player_cup (player_id, cup_id, position)
        VALUES ($1, $2, $3)
        "#,
                user.player_id,
                cup_id,
                position
            )
            .execute(pool)
            .await
            {
                Ok(_) => {
                    info!("Player: {:?} joined cup: {:?}", user.player_id, cup_id);
                    Ok(())
                }
                Err(e) => {
                    error!("Database error: {:?}", e);
                    Err(ServerFnError::ServerError(
                        "Failed to add player to cup.".to_string(),
                    ))
                }
            }
        }
        Err(err) => Err(err),
    }
}
