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
            <div class="absolute top-4 right-4">
                <Show when=move || { logged_in.get() } fallback=|| view! { <LoginButton /> }>
                    <LogoutButton />
                </Show>
            </div>
            <div class="absolute top-4 left-4">
                <Show when=move || { is_admin.get() }>
                    <A href="/create">
                        <button class="btn btn-xs btn-success">Adminpanel</button>
                    </A>
                </Show>
            </div>
            <h1 class="text-4xl text-center mt-14 mb-6">"Falkenbergs Gubbhockey"</h1>
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
