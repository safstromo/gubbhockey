use leptos::prelude::*;

use crate::{
    components::{
        date_card::DateCard, join_button::JoinButton, num_players::NumPlayers, time_card::TimeCard,
    },
    models::{get_players_by_gameday, Gameday, Player},
};

#[component]
pub fn GamedayCard(
    gameday: Gameday,
    logged_in: ReadSignal<bool>,
    player: Option<Result<Player, ServerFnError>>,
) -> impl IntoView {
    let (count, set_count) = signal(0);
    let players = Resource::new(
        || (),
        move |_| async move { get_players_by_gameday(gameday.gameday_id).await },
    );
    Effect::new(move |_| {
        if let Some(players) = players.get() {
            set_count.set(players.unwrap_or_else(|_| Vec::new()).len());
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
