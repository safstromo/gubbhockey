use chrono::{DateTime, Utc};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[component]
pub fn DatePicker(set_invalidate_gamedays: WriteSignal<bool>) -> impl IntoView {
    let submit = ServerAction::<AddDate>::new();
    let show_repeat = RwSignal::new(false);
    Effect::new(move || {
        if submit.value().get().is_some_and(|result| result.is_ok()) {
            set_invalidate_gamedays.set(true);
        }
    });
    view! {
        <ActionForm action=submit>
            <div class="flex flex-col m-2">
                <label for="input_date[date]">Datum</label>
                <input type="date" required name="input_date[date]" class="input input-bordered" />
                <label for="input_date[start]" class="mt-2">
                    Start
                </label>
                <input type="time" required name="input_date[start]" class="input input-bordered" />
                <label for="input_date[end]" class="mt-2">
                    Slut
                </label>
                <input type="time" required name="input_date[end]" class="input input-bordered" />
                <label class="label cursor-pointer mt-2">
                    <span class="label-text">Återkommande</span>
                    <input type="checkbox" class="toggle" bind:checked=show_repeat />
                </label>
                <Show when=move || { show_repeat.get() }>
                    <label for="input_date[repeat]" class="max-w-40 mx-auto text-center">
                        "varje vecka i (1-12) veckor"
                    </label>
                    <input
                        type="number"
                        required
                        name="input_date[repeat]"
                        class="input input-bordered"
                        min="1"
                        max="12"
                    />
                </Show>
                <button class="btn btn-success mt-4" type="submit">
                    Lägg till dag
                </button>
            </div>
        </ActionForm>
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct InputDate {
    date: String,
    start: String,
    end: String,
    repeat: Option<i64>,
}

#[server]
async fn add_date(input_date: InputDate) -> Result<(), ServerFnError> {
    use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};
    use tracing::info;

    info!("Date submit: {:?}", input_date);

    // Parse date and time
    let date = NaiveDate::parse_from_str(&input_date.date, "%Y-%m-%d")?;
    let start_time = NaiveTime::parse_from_str(&input_date.start, "%H:%M")?;
    let end_time = NaiveTime::parse_from_str(&input_date.end, "%H:%M")?;

    // Combine date and time into NaiveDateTime
    let start_datetime = NaiveDateTime::new(date, start_time);
    let end_datetime = NaiveDateTime::new(date, end_time);
    info!(
        "Parsed NaiveStart: {:?}, NaiveEnd: {:?}",
        start_datetime, end_datetime
    );

    insert_gameday(start_datetime.and_utc(), end_datetime.and_utc()).await?;

    if let Some(repeat) = input_date.repeat {
        info!("Adding repeating date for {} weeks", repeat);

        for n in 1..=repeat {
            let new_start_date = start_datetime + Duration::weeks(n);
            let new_end_date = end_datetime + Duration::weeks(n);

            insert_gameday(new_start_date.and_utc(), new_end_date.and_utc()).await?;
        }
    }

    Ok(())
}

#[server]
async fn insert_gameday(
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
) -> Result<(), ServerFnError> {
    use crate::auth::validate_admin;
    use crate::database::get_db;
    use tracing::{error, info};

    if let Err(err) = validate_admin().await {
        return Err(err);
    }
    let pool = get_db();
    match sqlx::query!(
        r#"
        INSERT INTO gameday (start_date, end_date)
        VALUES ($1, $2)
        "#,
        start_date,
        end_date
    )
    .execute(pool)
    .await
    {
        Ok(_) => {
            info!(
                "Date inserted successfully! Start:{:?}, End:{:?}",
                start_date, end_date
            );
            Ok(())
        }
        Err(e) => {
            error!("Database error: {:?}", e);
            Err(ServerFnError::ServerError(
                "Failed to create gameday.".to_string(),
            ))
        }
    }
}
