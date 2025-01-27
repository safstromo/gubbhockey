use leptos::prelude::*;
use leptos_router::components::A;

use crate::{
    components::{date_card::DateCard, num_players::NumPlayers, time_card::TimeCard},
    models::Cup,
};

#[component]
pub fn CupCard(cup: Cup) -> impl IntoView {
    view! {
        <div class="card flex-row items-center justify-around bg-base-100 shadow-xl border w-11/12">

            <A href=format!("/cup/{}", cup.cup_id)>
                <DateCard start=cup.start_date show_month=true />
            </A>
            <div class="flex flex-col items-center justify-evenly">
                <A href=format!("/cup/{}", cup.cup_id)>
                    <p class="text-center text-bold mt-2">{cup.title}</p>
                    <TimeCard start=cup.start_date end=cup.end_date />
                    <NumPlayers num_players=cup.player_count.unwrap_or(0) />
                </A>
            </div>
            <button class="btn btn-error h-20 m-2 flex-col">
                <p class="font-bold">Ändra</p>
            </button>

        </div>
    }
}
