use leptos::prelude::*;

use crate::{
    components::{
        date_card::DateCard, join_button::JoinButton, num_players::NumPlayers, time_card::TimeCard,
    },
    models::Gameday,
};

#[component]
pub fn GamedayCard(gameday: Gameday, logged_in: ReadSignal<bool>) -> impl IntoView {
    view! {
        <div class="card flex-row items-center justify-around bg-base-100 shadow-xl border">
            <DateCard start=gameday.start_date />
            <div class="flex flex-col items-center justify-evenly">
                <TimeCard start=gameday.start_date end=gameday.end_date />
                <NumPlayers num_players=14 />
            </div>
            <JoinButton logged_in=logged_in />
        </div>
    }
}
