use leptos::{prelude::*, task::spawn_local};

use crate::{
    components::{date_card::DateCard, num_players::NumPlayers, time_card::TimeCard},
    models::{delete_gameday, Gameday},
};

#[component]
pub fn GamedayCreate(
    gameday: Gameday,
    set_invalidate_gamedays: WriteSignal<bool>,
) -> impl IntoView {
    view! {
        <div class="card flex-row items-center justify-around bg-base-100 shadow-xl border">
            <DateCard start=gameday.start_date show_month=true />
            <div class="flex flex-col items-center justify-evenly">
                <TimeCard start=gameday.start_date end=gameday.end_date />
                <NumPlayers num_players=gameday.player_count.unwrap_or(0) />
            </div>
            <button
                class="btn btn-error h-20 m-2 flex-col"
                on:click=move |_| {
                    spawn_local(async move {
                        let _ = delete_gameday(gameday.gameday_id).await;
                        set_invalidate_gamedays.set(true);
                    });
                }
            >
                <p class="font-bold">Delete</p>
            </button>

        </div>
    }
}
