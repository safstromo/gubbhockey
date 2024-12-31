use leptos::prelude::*;

#[component]
pub fn NumPlayers(num_players: i64) -> impl IntoView {
    view! { <p class="text-center m-2">{num_players}" Spelare kommer"</p> }
}
