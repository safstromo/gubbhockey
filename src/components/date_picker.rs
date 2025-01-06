#[cfg(feature = "ssr")]
use crate::models::insert_gameday;
use crate::models::Gameday;
#[cfg(feature = "ssr")]
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
// #[cfg(feature = "ssr")]
use leptos::logging::log;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[component]
pub fn DatePicker(set_invalidate_gamedays: WriteSignal<bool>) -> impl IntoView {
    let submit = ServerAction::<AddDate>::new();
    Effect::new(move || {
        if submit.value().get().is_some_and(|result| result.is_ok()) {
            set_invalidate_gamedays.set(true);
        }
    });

    view! {
        <ActionForm action=submit>
            <div class="flex flex-col m-2">
                <label for="input_date[date]" class="">
                    Datum
                </label>
                <input type="date" required name="input_date[date]" />
                <label for="input_date[start]" class="">
                    Start
                </label>
                <input type="time" required name="input_date[start]" />
                <label for="input_date[end]" class="">
                    Slut
                </label>
                <input type="time" required name="input_date[end]" />
            </div>
            <button class="btn btn-success mt-2" type="submit">
                Lägg till dag
            </button>
        </ActionForm>
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct InputDate {
    date: String,
    start: String,
    end: String,
}

#[server]
async fn add_date(input_date: InputDate) -> Result<(), ServerFnError> {
    log!("Date submit: {:?}", input_date);

    // Parse date and time
    let date = NaiveDate::parse_from_str(&input_date.date, "%Y-%m-%d")?;
    let start_time = NaiveTime::parse_from_str(&input_date.start, "%H:%M")?;
    let end_time = NaiveTime::parse_from_str(&input_date.end, "%H:%M")?;

    // Combine date and time into NaiveDateTime
    let start_datetime = NaiveDateTime::new(date, start_time);
    let end_datetime = NaiveDateTime::new(date, end_time);
    log!(
        "Parsed NaiveStart: {:?}, NaiveEnd: {:?}",
        start_datetime,
        end_datetime
    );

    insert_gameday(start_datetime.and_utc(), end_datetime.and_utc()).await?;

    // leptos_axum::redirect("/create");
    Ok(())
}
