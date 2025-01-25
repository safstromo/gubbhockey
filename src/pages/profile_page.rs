use leptos::{logging::log, prelude::*, task::spawn_local};

use crate::{components::logout_button::LogoutButton, models::Player};

#[component]
pub fn ProfilePage() -> impl IntoView {
    let player =
        use_context::<Resource<Result<Player, ServerFnError>>>().expect("player context not found");

    let goalkeeper = RwSignal::new(false);

    Effect::new(move |_| {
        if let Some(Ok(player_data)) = player.get() {
            goalkeeper.set(player_data.is_goalkeeper);
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
                let player = player.await.expect("no player found");
                view! {
                    <div class="flex flex-col min-h-screen w-full items-center">
                        <div class="avatar placeholder mb-6">
                            <div class="bg-neutral text-neutral-content w-24 rounded-full">
                                <span class="text-3xl">
                                    {format!(
                                        "{}{}",
                                        player.given_name.chars().next().unwrap_or(' '),
                                        player.family_name.chars().next().unwrap_or(' '),
                                    )}

                                </span>
                            </div>
                        </div>
                        <p class="m-2 text-xl text-bold">{player.name}</p>
                        <p class="m-2">{player.email}</p>

                        <label class="label cursor-pointer mt-2">
                            <span class="label-text mx-2">Utespelare</span>
                            <input
                                type="checkbox"
                                class="toggle"
                                bind:checked=goalkeeper
                                on:change=move |_| {
                                    let change_made = goalkeeper.get();
                                    spawn_local(async move {
                                        if let Err(err) = update_player_position(change_made).await
                                        {
                                            log!("Failed to update position: {:?}", err);
                                        }
                                    });
                                }
                            />
                            <span class="label-text mx-2">"Målvakt"</span>
                        </label>
                        <div class="mt-20">
                            <LogoutButton />
                        </div>

                    </div>
                }
            })}
        </Suspense>
    }
}

#[server]
async fn update_player_position(is_goalkeeper: bool) -> Result<(), ServerFnError> {
    use crate::auth::user_from_session;
    use crate::database::get_db;
    use tracing::{error, info};

    match user_from_session().await {
        Ok(user) => {
            let pool = get_db();
            match sqlx::query!(
                r#"
        UPDATE player
        SET is_goalkeeper = $1
        WHERE player_id = $2
        "#,
                is_goalkeeper,
                user.player_id
            )
            .execute(pool)
            .await
            {
                Ok(_) => {
                    info!(
                        "Player: {:?} set goalkeeper: {:?}",
                        user.player_id, is_goalkeeper
                    );
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
