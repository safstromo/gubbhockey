use leptos::prelude::*;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;

use crate::{components::not_found::NotFound, models::Cup};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[component]
pub fn EditCupPage() -> impl IntoView {
    let params = use_params::<CupParam>();
    let id = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.id)
            .unwrap_or(0)
    };

    let cup = Resource::new(
        move || id(),
        |id| async move { get_cup_by_id_admin(id).await },
    );
    let title = RwSignal::new("".to_string());
    let info = RwSignal::new("".to_string());
    let date = RwSignal::new("".to_string());
    let start = RwSignal::new("".to_string());
    let end = RwSignal::new("".to_string());

    Effect::new(move |_| {
        if let Some(Ok(cup)) = cup.get() {
            title.set(cup.title.unwrap_or("".to_string()));
            info.set(cup.info.unwrap_or("".to_string()));
            date.set(cup.start_date.date_naive().to_string());
            start.set(cup.start_date.time().format("%H:%M").to_string());
            end.set(cup.end_date.time().format("%H:%M").to_string());
        }
    });

    let submit = ServerAction::<UpdateCup>::new();
    view! {
        <div class="flex flex-col items-center justify-center m-2 w-full">
            <Transition fallback=move || {
                view! { <p>"Loading..."</p> }
            }>
                {move || Suspend::new(async move {
                    let cup = cup.await;
                    let cup_exist = cup.is_ok();

                    view! {
                        <Show
                            when=move || { cup_exist }
                            fallback=|| {
                                view! { <NotFound /> }
                            }
                        >
                            <ActionForm action=submit>
                                <div class="flex flex-col justify-center">
                                    <div class="flex flex-col m-2 max-w-44">
                                        <label for="input[date]">Datum</label>
                                        <input
                                            type="date"
                                            bind:value=date
                                            required
                                            name="input[date]"
                                            class="input input-bordered"
                                        />
                                    </div>
                                    <div class="flex gap-2 m-2">
                                        <div class="flex flex-col">
                                            <label for="input[start]">Start</label>
                                            <input
                                                type="time"
                                                bind:value=start
                                                required
                                                name="input[start]"
                                                class="input input-bordered"
                                            />
                                        </div>
                                        <div class="flex flex-col">
                                            <label for="input[end]">Slut</label>
                                            <input
                                                type="time"
                                                bind:value=end
                                                required
                                                name="input[end]"
                                                class="input input-bordered"
                                            />
                                        </div>
                                    </div>
                                </div>
                                <div class="flex flex-col m-2">
                                    <label for="input[title]" class="mt-2">
                                        Titel
                                    </label>
                                    <input
                                        type="text"
                                        bind:value=title
                                        required
                                        name="input[title]"
                                        class="input input-bordered"
                                    />
                                </div>
                                <div class="flex flex-col m-2">
                                    <label for="input[info]" class="mt-2">
                                        Info
                                    </label>
                                    <textarea
                                        bind:value=info
                                        required
                                        name="input[info]"
                                        class="textarea textarea-bordered min-h-56"
                                    />
                                </div>
                                <input
                                    class="hidden"
                                    type="number"
                                    value=move || { id() }
                                    name="input[cup_id]"
                                />
                                <div class="flex justify-center m-4">
                                    <button class="btn btn-success w-40" type="submit">
                                        Uppdatera cup
                                    </button>
                                </div>
                            </ActionForm>
                        </Show>
                    }
                })}
            </Transition>
        </div>
    }
}

#[derive(Params, PartialEq)]
struct CupParam {
    id: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct InputCup {
    cup_id: i32,
    date: String,
    start: String,
    end: String,
    title: String,
    info: String,
}

#[server]
async fn get_cup_by_id_admin(id: i32) -> Result<Cup, ServerFnError> {
    use crate::auth::validate_admin;
    use crate::database::get_db;
    use http::StatusCode;
    use tracing::{error, info};

    if let Err(err) = validate_admin().await {
        return Err(err);
    }

    let pool = get_db();
    match sqlx::query_as!(
        Cup,
        r#"
    SELECT 
        c.cup_id, 
        c.start_date, 
        c.end_date,
        c.title,
        c.info,
        COUNT(pc.player_id) AS player_count
    FROM 
        cup c
    LEFT JOIN 
        player_cup pc ON c.cup_id = pc.cup_id
    WHERE 
        c.cup_id = $1
    GROUP BY 
        c.cup_id, c.start_date, c.end_date, c.title, c.info
    "#,
        id
    )
    .fetch_one(pool)
    .await
    {
        Ok(results) => {
            info!("Successfully retrieved cup with player counts.");
            Ok(results)
        }
        Err(e) => {
            error!("Database error: {:?}", e);
            let opts = expect_context::<leptos_axum::ResponseOptions>();
            opts.set_status(StatusCode::NOT_FOUND);
            Err(ServerFnError::ServerError("No cup found.".to_string()))
        }
    }
}

#[server]
async fn update_cup(input: InputCup) -> Result<(), ServerFnError> {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
    use tracing::info;

    info!("Updating cup with values: {:?}", input);

    // Parse date and time
    let date = NaiveDate::parse_from_str(&input.date, "%Y-%m-%d")?;
    let start_time = NaiveTime::parse_from_str(&input.start, "%H:%M")?;
    let end_time = NaiveTime::parse_from_str(&input.end, "%H:%M")?;

    // Combine date and time into NaiveDateTime
    let start_datetime = NaiveDateTime::new(date, start_time);
    let end_datetime = NaiveDateTime::new(date, end_time);
    info!(
        "Parsed NaiveStart: {:?}, NaiveEnd: {:?}",
        start_datetime, end_datetime
    );

    update_cup_db(
        start_datetime.and_utc(),
        end_datetime.and_utc(),
        input.title,
        input.info,
        input.cup_id,
    )
    .await?;

    Ok(())
}

#[server]
async fn update_cup_db(
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    title: String,
    info: String,
    cup_id: i32,
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
        UPDATE cup
        SET start_date = $1, end_date = $2, title = $3, info = $4
        WHERE cup_id = $5
        "#,
        start_date,
        end_date,
        title,
        info,
        cup_id
    )
    .execute(pool)
    .await
    {
        Ok(_) => {
            info!("Cup updated: {cup_id}",);
            leptos_axum::redirect(format!("/cup/{}", cup_id).as_str());
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
