use chrono::{DateTime, Datelike, Utc};
use leptos::prelude::*;

#[component]
pub fn DateCard(start: DateTime<Utc>) -> impl IntoView {
    let date = start.date_naive().day();

    let day = match start.weekday() {
        chrono::Weekday::Mon => "Mån",
        chrono::Weekday::Tue => "Tis",
        chrono::Weekday::Wed => "Ons",
        chrono::Weekday::Thu => "Tor",
        chrono::Weekday::Fri => "Fre",
        chrono::Weekday::Sat => "Lör",
        chrono::Weekday::Sun => "Sön",
    };

    view! {
        <div class="flex m-2 w-30 h-20">
            <div class="flex-col w-full items-center content-center ">
                <p class="text-center font-bold">{day}</p>
                <p class="text-center text-3xl font-bold">{date}</p>
            </div>
            <div class="divider divider-horizontal" />
        </div>
    }
}
