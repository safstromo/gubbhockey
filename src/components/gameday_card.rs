use leptos::prelude::*;

use crate::{
    components::{
        date_card::DateCard, join_button::JoinButton, num_players::NumPlayers, time_card::TimeCard,
    },
    models::{count_players_by_gameday, Gameday, Player},
};

#[component]
pub fn GamedayCard(
    gameday: Gameday,
    logged_in: ReadSignal<bool>,
    player: Option<Result<Player, ServerFnError>>,
) -> impl IntoView {
    let (count, set_count) = signal(0);
    let players_count = Resource::new(
        || (),
        move |_| async move { count_players_by_gameday(gameday.gameday_id).await },
    );
    Effect::new(move |_| {
        if let Some(data) = players_count.get() {
            if let Ok(count) = data {
                set_count.set(count);
            }
        }
    });

    view! {
        <div class="card flex-row items-center justify-around bg-base-100 shadow-xl border">
            <DateCard start=gameday.start_date />
            <div class="flex flex-col items-center justify-evenly">
                <TimeCard start=gameday.start_date end=gameday.end_date />
                <NumPlayers num_players=count />
            </div>
            <JoinButton logged_in=logged_in gameday_id=gameday.gameday_id player=player />
        </div>
    }
}
