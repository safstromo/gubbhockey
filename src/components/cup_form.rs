use chrono::{DateTime, Utc};
use leptos::{logging::log, prelude::*};
use serde::{Deserialize, Serialize};

#[component]
pub fn CupForm() -> impl IntoView {
    let submit = ServerAction::<AddCup>::new();

    let value = submit.value();
    Effect::new(move |_| {
        if value.get().is_some() {
            log!("refetch cups")
        }
    });

    view! {
        <ActionForm action=submit>
            <div class="flex flex-col justify-center m-2">
                <div class="max-w-40">
                    <label for="input_cup[date]">Datum</label>
                    <input
                        type="date"
                        required
                        name="input_cup[date]"
                        class="input input-bordered"
                    />
                    <div class="flex gap-2 mt-2">
                        <div class="flex-col">
                            <label for="input_cup[start]" class="">
                                Start
                            </label>
                            <input
                                type="time"
                                required
                                name="input_cup[start]"
                                class="input input-bordered"
                            />
                        </div>
                        <div class="flex-col">
                            <label for="input_cup[end]" class="">
                                Slut
                            </label>
                            <input
                                type="time"
                                required
                                name="input_cup[end]"
                                class="input input-bordered"
                            />
                        </div>
                    </div>
                </div>
                <label for="input_cup[title]" class="mt-2">
                    Titel
                </label>
                <input
                    type="text"
                    placeholder="Falcon cup"
                    required
                    name="input_cup[title]"
                    class="input input-bordered"
                />
                <label for="input_cup[info]" class="mt-2">
                    Info
                </label>
                <textarea
                    placeholder="Info om cuppen"
                    required
                    name="input_cup[info]"
                    class="textarea textarea-bordered min-h-56"
                />
                <button class="btn btn-success mt-4 w-40" type="submit">
                    Skapa cup
                </button>
            </div>
        </ActionForm>
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct InputCup {
    date: String,
    start: String,
    end: String,
    title: String,
    info: String,
}

#[server]
async fn add_cup(input_cup: InputCup) -> Result<(), ServerFnError> {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
    use tracing::info;

    info!("Date submit: {:?}", input_cup);
    info!("txt submit: {:?}", input_cup.info);

    // Parse date and time
    let date = NaiveDate::parse_from_str(&input_cup.date, "%Y-%m-%d")?;
    let start_time = NaiveTime::parse_from_str(&input_cup.start, "%H:%M")?;
    let end_time = NaiveTime::parse_from_str(&input_cup.end, "%H:%M")?;

    // Combine date and time into NaiveDateTime
    let start_datetime = NaiveDateTime::new(date, start_time);
    let end_datetime = NaiveDateTime::new(date, end_time);
    info!(
        "Parsed NaiveStart: {:?}, NaiveEnd: {:?}",
        start_datetime, end_datetime
    );

    insert_cup(
        start_datetime.and_utc(),
        end_datetime.and_utc(),
        input_cup.title,
        input_cup.info,
    )
    .await?;

    Ok(())
}

#[server]
async fn insert_cup(
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    title: String,
    info: String,
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
        INSERT INTO cup (start_date, end_date, title, info)
        VALUES ($1, $2, $3, $4)
        "#,
        start_date,
        end_date,
        title,
        info
    )
    .execute(pool)
    .await
    {
        Ok(_) => {
            info!(
                "Cup inserted successfully! Start:{:?}, End:{:?}",
                start_date, end_date
            );
            Ok(())
        }
        Err(e) => {
            error!("Database error: {:?}", e);
            Err(ServerFnError::ServerError(
                "Failed to create cup.".to_string(),
            ))
        }
    }
}
