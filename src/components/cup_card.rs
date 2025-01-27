use leptos::prelude::*;
use leptos_router::components::A;

use crate::{
    components::{date_card::DateCard, num_players::NumPlayers, time_card::TimeCard},
    models::Cup,
};

#[component]
pub fn CupCard(cup: Cup, edit_button: bool) -> impl IntoView {
    view! {
        <div class="card flex-row items-center justify-around bg-base-100 shadow-xl border">

            <A href=format!("/cup/{}", cup.cup_id)>
                <DateCard start=cup.start_date show_month=true />
            </A>
            <div class="flex flex-col items-center justify-evenly mr-2">
                <A href=format!("/cup/{}", cup.cup_id)>
                    <p class="text-center text-bold mt-2">{cup.title}</p>
                    <TimeCard start=cup.start_date end=cup.end_date />
                    <NumPlayers num_players=cup.player_count.unwrap_or(0) />
                </A>
            </div>
            <Show
                when=move || { edit_button }
                fallback=move || {
                    view! {
                        <A href=format!("/cup/{}", cup.cup_id)>
                            <button class="btn btn-primary h-20 w-20 m-2 flex-col">
                                <p class="font-bold">Info</p>
                            </button>
                        </A>
                    }
                }
            >
                <button class="btn btn-error h-20 m-2 flex-col">
                    <p class="font-bold">Ändra</p>
                </button>
            </Show>
        </div>
    }
}
