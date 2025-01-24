use leptos::{prelude::*, task::spawn_local};
use leptos_router::components::A;

use crate::{
    auth::{user_from_session, validate_admin},
    components::{
        gameday_card::GamedayCard, join_button::get_gamedays_by_player, login_button::LoginButton,
        logout_button::LogoutButton,
    },
    models::Gameday,
};

#[component]
pub fn HomePage() -> impl IntoView {
    let (logged_in, set_loggedin) = signal(false);
    let (is_admin, set_is_admin) = signal(false);
    let (gamedays_joined, set_gamedays_joined) = signal(Vec::new());
    let player = Resource::new(|| (), |_| async move { user_from_session().await });
    let admin_check = Resource::new(|| (), |_| async move { validate_admin().await });

    let gamedays = Resource::new(
        move || gamedays_joined.get(),
        |_| async move { get_next_5_gamedays().await },
    );

    Effect::new(move |_| {
        if let Some(Ok(_player_data)) = player.get() {
            set_loggedin.set(true);

            spawn_local(async move {
                if let Ok(gamedays) = get_gamedays_by_player().await {
                    set_gamedays_joined.set(gamedays);
                }
            });
        }
    });

    Effect::new(move |_| {
        if let Some(Ok(admin)) = admin_check.get() {
            set_is_admin.set(admin);
        }
    });

    view! {
        <div class="flex flex-col min-h-screen w-full items-center relative">
            <div class="absolute top-4 right-4 flex flex-col justify-center">
                <Show when=move || { logged_in.get() } fallback=|| view! { <LoginButton /> }>
                    <Show when=move || { logged_in.get() }>
                        <Transition>
                            {move || Suspend::new(async move {
                                let player = player.await;
                                view! {
                                    <A href="/profile">
                                        <div class="avatar placeholder flex justify-center mb-2">
                                            <div class="bg-neutral text-neutral-content w-20 rounded-full">
                                                <span class="text-3xl">
                                                    {format!(
                                                        "{}{}",
                                                        player
                                                            .clone()
                                                            .unwrap()
                                                            .given_name
                                                            .chars()
                                                            .next()
                                                            .unwrap_or(' '),
                                                        player
                                                            .clone()
                                                            .unwrap()
                                                            .family_name
                                                            .chars()
                                                            .next()
                                                            .unwrap_or(' '),
                                                    )}
                                                </span>
                                            </div>
                                        </div>
                                    </A>
                                }
                            })}
                        </Transition>
                    </Show>
                </Show>
            </div>
            <div class="absolute top-4 left-4">
                <Show when=move || { is_admin.get() }>
                    <A href="/create">
                        <button class="btn btn-xs btn-success">Adminpanel</button>
                    </A>
                </Show>
            </div>
            <div class="flex justify-center">
                <A href="/">
                    <img src="Logo-nobg.png" alt="Logo" class="h-60 w-60" />
                </A>
            </div>
            <h3 class="text-xl">Speldagar</h3>
            <Transition fallback=move || view! { <p>"Loading..."</p> }>
                <ul class="flex flex-col items-center w-11/12">
                    {move || Suspend::new(async move {
                        let days = gamedays.await.expect("No gamedays found");
                        days.into_iter()
                            .map(|day| {
                                view! {
                                    <li class="my-2">
                                        <GamedayCard
                                            logged_in=logged_in
                                            gamedays_joined=gamedays_joined
                                            set_gamedays_joined=set_gamedays_joined
                                            gameday=day
                                        />
                                    </li>
                                }
                            })
                            .collect_view()
                    })}
                </ul>
            </Transition>
            <footer>
                <div class="text-center mt-6">
                    <A href="/terms">
                        <p class="underline m-2">"Terms and Conditions"</p>
                    </A>
                    <p>"2025 Falkenbergs Gubbhockey."</p>
                    <p>"All rights reserved."</p>
                    <p>
                        <a href="#" class="underline">
                            Back to top
                        </a>
                    </p>
                </div>
            </footer>

        </div>
    }
}

#[server]
async fn get_next_5_gamedays() -> Result<Vec<Gameday>, ServerFnError> {
    use crate::database::get_db;
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
            g.start_date >= NOW() 
        GROUP BY 
            g.gameday_id, g.start_date, g.end_date
        ORDER BY 
            g.start_date ASC
        LIMIT 5         
        "#
    )
    .fetch_all(pool)
    .await
    {
        Ok(results) => {
            info!("Successfully retrieved next 5 gamedays with player counts.");
            Ok(results)
        }
        Err(e) => {
            error!("Database error: {:?}", e);
            Err(ServerFnError::ServerError(
                "Failed to get the next 5 gamedays.".to_string(),
            ))
        }
    }
}
