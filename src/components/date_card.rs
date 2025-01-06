use chrono::{DateTime, Datelike, Utc};
use leptos::prelude::*;

#[component]
pub fn DateCard(start: DateTime<Utc>, show_month: bool) -> impl IntoView {
    let date = start.date_naive().day();
    let month = match start.date_naive().month0() {
        0 => "Jan",
        1 => "Feb",
        2 => "Mar",
        3 => "Apr",
        4 => "Maj",
        5 => "Jun",
        6 => "Jul",
        7 => "Aug",
        8 => "Sep",
        9 => "Oct",
        10 => "Nov",
        11 => "Dec",
        _ => "",
    };

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
        <div class="flex m-2">
            <div class="flex-col w-full items-center content-center ">
                <p class="text-center font-bold">{day}</p>
                <p class="text-center text-3xl font-bold">{date}</p>
                <Show when=move || { show_month }>
                    <p class="text-center font-bold">{month}</p>
                </Show>
            </div>
            <div class="divider divider-horizontal" />
        </div>
    }
}
