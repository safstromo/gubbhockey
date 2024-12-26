use chrono::{DateTime, Utc};
use leptos::prelude::*;

#[component]
pub fn TimeCard(start: DateTime<Utc>, end: DateTime<Utc>) -> impl IntoView {
    let start = start.with_timezone(&Utc).format("%H:%M").to_string();
    let end = end.with_timezone(&Utc).format("%H:%M").to_string();
    view! {
        <div class="flex flex-row items-center justify-center m-2 w-full">
            <p class="text-center">{start}</p>
            <p class="text-center">-</p>
            <p class="text-center">{end}</p>
        </div>
    }
}
