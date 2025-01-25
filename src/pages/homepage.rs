use leptos::{prelude::*, task::spawn_local};
use leptos_router::components::A;

use crate::{
    components::{gameday_card::GamedayCard, join_button::get_gamedays_by_player},
    models::{Gameday, Player},
};

#[component]
pub fn HomePage() -> impl IntoView {
    let player =
        use_context::<Resource<Result<Player, ServerFnError>>>().expect("player context not found");
    let (gamedays_joined, set_gamedays_joined) = signal(Vec::new());

    let gamedays = Resource::new(
        move || gamedays_joined.get(),
        |_| async move { get_next_5_gamedays().await },
    );

    Effect::new(move |_| {
        if let Some(Ok(_player_data)) = player.get() {
            spawn_local(async move {
                if let Ok(gamedays) = get_gamedays_by_player().await {
                    set_gamedays_joined.set(gamedays);
                }
            });
        }
    });

    view! {
        <div class="flex flex-col min-h-screen w-full items-center relative">
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
