use leptos::prelude::*;
use leptos_router::components::A;
use reactive_stores::Store;

use crate::{
    components::{
        date_card::DateCard, join_button::JoinButton, leave_button::LeaveButton,
        num_players::NumPlayers, time_card::TimeCard,
    },
    models::{Gameday, GlobalState, GlobalStateStoreFields},
};

#[component]
pub fn GamedayCard(
    gameday: Gameday,
    gamedays_joined: ReadSignal<Vec<Gameday>>,
    set_gamedays_joined: WriteSignal<Vec<Gameday>>,
) -> impl IntoView {
    let state = expect_context::<Store<GlobalState>>();
    let logged_in = state.logged_in();

    view! {
        <div class="card flex-row items-center justify-around bg-base-100 shadow-xl border">
            <Show
                when=move || logged_in.get()
                fallback=move || {
                    view! {
                        <DateCard start=gameday.start_date show_month=true />
                        <div class="flex flex-col items-center justify-evenly mr-2">
                            <TimeCard start=gameday.start_date end=gameday.end_date />
                            <NumPlayers num_players=gameday.player_count.unwrap_or(0) />
                        </div>
                    }
                }
            >
                <A href=format!("/day/{}", gameday.gameday_id)>
                    <DateCard start=gameday.start_date show_month=true />
                </A>
                <div class="flex flex-col items-center justify-evenly mr-2">
                    <A href=format!("/day/{}", gameday.gameday_id)>
                        <TimeCard start=gameday.start_date end=gameday.end_date />
                        <NumPlayers num_players=gameday.player_count.unwrap_or(0) />
                    </A>
                </div>
            </Show>
            <Show
                when=move || { is_player_joined(gamedays_joined.get(), gameday.gameday_id) }
                fallback=move || {
                    view! {
                        <JoinButton
                            gameday_id=gameday.gameday_id
                            set_gamedays_joined=set_gamedays_joined
                        />
                    }
                }
            >
                <LeaveButton
                    gameday_id=gameday.gameday_id
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
