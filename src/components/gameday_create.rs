use leptos::{prelude::*, task::spawn_local};
use leptos_router::components::A;

use crate::{
    components::{date_card::DateCard, num_players::NumPlayers, time_card::TimeCard},
    models::Gameday,
};

#[component]
pub fn GamedayCreate(
    gameday: Gameday,
    set_invalidate_gamedays: Option<WriteSignal<bool>>,
    redirect_on_delete: bool,
) -> impl IntoView {
    view! {
        <div class="card flex-row items-center justify-around bg-base-100 shadow-xl border">
            <A href=format!("/day/{}", gameday.gameday_id)>
                <DateCard start=gameday.start_date show_month=true />
            </A>
            <div class="flex flex-col items-center justify-evenly">
                <A href=format!("/day/{}", gameday.gameday_id)>
                    <TimeCard start=gameday.start_date end=gameday.end_date />
                    <NumPlayers num_players=gameday.player_count.unwrap_or(0) />
                </A>
            </div>
            <button
                class="btn btn-error h-20 m-2 flex-col"
                on:click=move |_| {
                    spawn_local(async move {
                        let _ = delete_gameday(gameday.gameday_id, redirect_on_delete).await;
                        if let Some(set_invalidate_gamedays) = set_invalidate_gamedays {
                            set_invalidate_gamedays.set(true);
                        }
                    });
                }
            >
                <p class="font-bold">Delete</p>
            </button>

        </div>
    }
}

#[server]
async fn delete_gameday(gameday_id: i32, redirect_on_delete: bool) -> Result<(), ServerFnError> {
    use crate::auth::validate_admin;
    use crate::database::get_db;
    use tracing::{error, info};

    if let Err(err) = validate_admin().await {
        return Err(err);
    }

    let pool = get_db();

    match sqlx::query!(
        r#"
        DELETE FROM gameday
        WHERE gameday_id = $1
        "#,
        gameday_id
    )
    .execute(pool)
    .await
    {
        Ok(_) => {
            info!("Gameday {:?} deleted successfully.", gameday_id);
            if redirect_on_delete {
                leptos_axum::redirect("/");
            }
            Ok(())
        }
        Err(e) => {
            error!("Database error: {:?}", e);
            Err(ServerFnError::ServerError(
                "Failed to delete gameday.".to_string(),
            ))
        }
    }
}
