use leptos::prelude::*;
use leptos_router::components::Redirect;

use crate::{auth::validate_admin, components::loading::Loading, models::Player};

#[component]
pub fn AdminPage() -> impl IntoView {
    let admin_check = Resource::new(
        || (),
        |_| async move { validate_admin().await.unwrap_or(false) },
    );
    let add_admin = ServerAction::<AddAdmin>::new();
    let remove_admin = ServerAction::<RemoveAdmin>::new();

    let (invalidate_gamedays, set_invalidate_gamedays) = signal(false);
    let players = Resource::new(|| (), |_| async move { get_all_players().await });

    Effect::new(move || {
        if invalidate_gamedays.get() {
            players.refetch();
            set_invalidate_gamedays.set(false);
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
                let is_admin = admin_check.await;
                view! {
                    <Show when=move || { is_admin } fallback=|| view! { <Redirect path="/" /> }>
                        <Transition fallback=move || view! { <Loading /> }>
                            <ActionForm action=add_admin>
                                <div class="flex flex-col items-center w-full">
                                    <h3 class="text-center text-xl mt-2">
                                        "Gör användare till admin"
                                    </h3>
                                    <select
                                        class="select w-full max-w-xs select-bordered m-2"
                                        name="player_id"
                                    >
                                        <option disabled selected>
                                            "Välj användare"
                                        </option>
                                        {move || Suspend::new(async move {
                                            let users = players.await.expect("No users found");
                                            users
                                                .into_iter()
                                                .filter(|user| {
                                                    user.access_group.as_deref() == Some("user")
                                                })
                                                .map(|user| {
                                                    view! {
                                                        <option value=user
                                                            .player_id>{user.name}" ("{user.email}")"</option>
                                                    }
                                                })
                                                .collect_view()
                                        })}
                                    </select>
                                    <button type="submit" class="btn btn-success m-4">
                                        "Lägg till admin"
                                    </button>
                                    <div class="divider" />
                                </div>
                            </ActionForm>
                            <div class="flex w-full items-center justify-around">
                                <div class="flex flex-col w-full items-center">
                                    <h3 class="text-center text-xl mt-2">Admins</h3>
                                    <ul class="flex flex-col items-center w-11/12">
                                        {move || Suspend::new(async move {
                                            let users = players.await.expect("No users found");
                                            users
                                                .into_iter()
                                                .filter(|user| {
                                                    user.access_group.as_deref() == Some("admin")
                                                        || user.access_group.as_deref() == Some("super-admin")
                                                })
                                                .map(|user| {
                                                    view! {
                                                        <li class="flex justify-center my-2">
                                                            <ActionForm action=remove_admin>
                                                                <p>{user.name}</p>
                                                                <button
                                                                    type="submit"
                                                                    class="btn btn-square btn-outline ml-2"
                                                                    value=user.player_id
                                                                    name="player_id"
                                                                >
                                                                    <svg
                                                                        xmlns="http://www.w3.org/2000/svg"
                                                                        class="h-4 w-4"
                                                                        fill="none"
                                                                        viewBox="0 0 24 24"
                                                                        stroke="currentColor"
                                                                    >
                                                                        <path
                                                                            stroke-linecap="round"
                                                                            stroke-linejoin="round"
                                                                            stroke-width="2"
                                                                            d="M6 18L18 6M6 6l12 12"
                                                                        />
                                                                    </svg>
                                                                </button>
                                                            </ActionForm>
                                                        </li>
                                                    }
                                                })
                                                .collect_view()
                                        })}
                                    </ul>
                                </div>
                                <div class="flex flex-col w-full items-center">
                                    <h3 class="text-center text-xl mt-2">Users</h3>
                                    <ul class="flex flex-col items-center w-11/12">
                                        {move || Suspend::new(async move {
                                            let users = players.await.expect("No users found");
                                            users
                                                .into_iter()
                                                .filter(|user| {
                                                    user.access_group.as_deref() == Some("user")
                                                })
                                                .map(|user| {
                                                    view! {
                                                        <li class="flex justify-center my-2">
                                                            <p>{user.name}</p>
                                                        </li>
                                                    }
                                                })
                                                .collect_view()
                                        })}
                                    </ul>
                                </div>
                            </div>
                        </Transition>
                    </Show>
                }
            })}
        </Suspense>
    }
}

#[server]
async fn get_all_players() -> Result<Vec<Player>, ServerFnError> {
    use crate::database::get_db;
    use tracing::{error, info};

    if let Err(err) = validate_admin().await {
        return Err(err);
    }

    let pool = get_db();
    match sqlx::query_as!(
        Player,
        r#"
        SELECT 
            *
        FROM 
            player
        "#
    )
    .fetch_all(pool)
    .await
    {
        Ok(results) => {
            info!("Successfully retrieved all players.");
            Ok(results)
        }
        Err(e) => {
            error!("Database error: {:?}", e);
            Err(ServerFnError::ServerError(
                "Failed to get all players.".to_string(),
            ))
        }
    }
}

#[server]
async fn add_admin(player_id: i32) -> Result<(), ServerFnError> {
    use crate::database::get_db;
    use tracing::{error, info};

    if let Err(err) = validate_admin().await {
        return Err(err);
    }

    let pool = get_db();
    match sqlx::query!(
        r#"
        UPDATE player
        SET access_group = 'admin'
        WHERE player_id = $1
        "#,
        player_id
    )
    .execute(pool)
    .await
    {
        Ok(_) => {
            info!("Successfully added admin.");
            Ok(())
        }
        Err(e) => {
            error!("Database error: {:?}", e);
            Err(ServerFnError::ServerError(
                "Failed to add admin.".to_string(),
            ))
        }
    }
}

#[server]
async fn remove_admin(player_id: i32) -> Result<(), ServerFnError> {
    use crate::database::get_db;
    use tracing::{error, info};

    if let Err(err) = validate_admin().await {
        return Err(err);
    }

    let pool = get_db();
    match sqlx::query!(
        r#"
        UPDATE player
        SET access_group = 'user'
        WHERE player_id = $1
        "#,
        player_id
    )
    .execute(pool)
    .await
    {
        Ok(_) => {
            info!("Successfully removed admin.");
            Ok(())
        }
        Err(e) => {
            error!("Database error: {:?}", e);
            Err(ServerFnError::ServerError(
                "Failed to remove admin.".to_string(),
            ))
        }
    }
}
