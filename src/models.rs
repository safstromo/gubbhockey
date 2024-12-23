#![cfg(feature = "ssr")]
use leptos::logging::log;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::Utc;
use sqlx::FromRow;
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Player {
    pub player_id: i32,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub access_group: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Gameday {
    pub gameday_id: i32,
    pub date: sqlx::types::chrono::DateTime<Utc>,
}

pub async fn insert_player(
    pool: &PgPool,
    name: &str,
    surname: &str,
    email: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO player (name, surname, email, access_group)
        VALUES ($1, $2, $3, $4)
        "#,
        name,
        surname,
        email,
        "user"
    )
    .execute(pool)
    .await?;

    log!("User inserted successfully! {:?}", name);
    Ok(())
}

pub async fn insert_gameday(
    pool: &PgPool,
    date: sqlx::types::chrono::DateTime<Utc>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO gameday (date)
        VALUES ($1)
        "#,
        date,
    )
    .execute(pool)
    .await?;

    log!("Date inserted successfully! {:?}", date);
    Ok(())
}

pub async fn join_gameday(
    pool: &PgPool,
    player: Player,
    gameday: Gameday,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO player_gameday (player_id, gameday_id)
        VALUES ($1, $2)
        "#,
        player.player_id,
        gameday.gameday_id
    )
    .execute(pool)
    .await?;

    log!("Player: {:?} joined: {:?}", player, gameday);
    Ok(())
}
