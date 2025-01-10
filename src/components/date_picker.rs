use chrono::{DateTime, Utc};
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
}

#[server]
async fn add_date(input_date: InputDate) -> Result<(), ServerFnError> {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
    use leptos::logging::log;

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

#[server]
async fn insert_gameday(
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
) -> Result<(), ServerFnError> {
    use crate::auth::validate_admin;
    use crate::database::get_db;
    use leptos::logging::log;

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
            log!(
                "Date inserted successfully! Start:{:?}, End:{:?}",
                start_date,
                end_date
            );
            Ok(())
        }
        Err(e) => {
            log!("Database error: {:?}", e);
            Err(ServerFnError::ServerError(
                "Failed to create gameday.".to_string(),
            ))
        }
    }
}
