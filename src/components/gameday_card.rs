use leptos::prelude::*;

use crate::{
    components::{
        date_card::DateCard, join_button::JoinButton, leave_button::LeaveButton,
        num_players::NumPlayers, time_card::TimeCard,
    },
    models::Gameday,
};

#[component]
pub fn GamedayCard(
    gameday: Gameday,
    logged_in: ReadSignal<bool>,
    gamedays_joined: ReadSignal<Vec<Gameday>>,
    set_gamedays_joined: WriteSignal<Vec<Gameday>>,
    player_id: ReadSignal<i32>,
) -> impl IntoView {
    view! {
        <div class="card flex-row items-center justify-around bg-base-100 shadow-xl border">
            <DateCard start=gameday.start_date show_month=true />
            <div class="flex flex-col items-center justify-evenly">
                <TimeCard start=gameday.start_date end=gameday.end_date />
                <NumPlayers num_players=gameday.player_count.unwrap_or(0) />
            </div>
            <Show
                when=move || { is_player_joined(gamedays_joined.get(), gameday.gameday_id) }
                fallback=move || {
                    view! {
                        <JoinButton
                            logged_in=logged_in
                            gameday_id=gameday.gameday_id
                            player_id=player_id
                            set_gamedays_joined=set_gamedays_joined
                        />
                    }
                }
            >
                <LeaveButton
                    gameday_id=gameday.gameday_id
                    player_id=player_id
                    gamedays_joined=gamedays_joined
                    set_gamedays_joined=set_gamedays_joined
                />
            </Show>
        </div>
    }
}
fn is_player_joined(gamedays_joined: Vec<Gameday>, gameday_id: i32) -> bool {
    gamedays_joined
        .iter()
        .any(|day| day.gameday_id == gameday_id)
}
