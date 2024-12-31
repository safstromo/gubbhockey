use leptos::prelude::*;

use crate::{
    components::{
        date_card::DateCard, join_button::JoinButton, num_players::NumPlayers, time_card::TimeCard,
    },
    models::{Gameday, Player},
};

#[component]
pub fn GamedayCard(
    gameday: Gameday,
    logged_in: ReadSignal<bool>,
    player: Option<Result<Player, ServerFnError>>,
    player_id: ReadSignal<i32>,
) -> impl IntoView {
    view! {
        <div class="card flex-row items-center justify-around bg-base-100 shadow-xl border">
            <DateCard start=gameday.start_date />
            <div class="flex flex-col items-center justify-evenly">
                <TimeCard start=gameday.start_date end=gameday.end_date />
                <NumPlayers num_players=gameday.player_count.unwrap_or(0) />
            </div>
            <JoinButton logged_in=logged_in gameday_id=gameday.gameday_id player=player />
        </div>
    }
}
