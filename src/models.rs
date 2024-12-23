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
    pub access_group: Option<String>,
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

pub async fn leave_gameday(
    pool: &PgPool,
    player: Player,
    gameday: Gameday,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM player_gameday
        WHERE player_id = $1 AND gameday_id = $2
        "#,
        player.player_id,
        gameday.gameday_id
    )
    .execute(pool)
    .await?;

    log!("Player: {:?} left gameday: {:?}", player, gameday);
    Ok(())
}

pub async fn get_next_5_gamedays(pool: &PgPool) -> Result<Vec<Gameday>, sqlx::Error> {
    let results = sqlx::query_as!(
        Gameday,
        r#"
        SELECT gameday_id, date
        FROM gameday
        WHERE date >= NOW()
        ORDER BY date ASC
        LIMIT 5
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(results)
}

pub async fn get_players_by_gameday(
    pool: &PgPool,
    gameday_id: i32,
) -> Result<Vec<Player>, sqlx::Error> {
    let players = sqlx::query_as!(
        Player,
        r#"
        SELECT 
            p.player_id,
            p.name,
            p.surname,
            p.email,
            p.access_group
        FROM 
            Player p
        JOIN 
            Player_Gameday pg ON p.player_id = pg.player_id
        JOIN 
            Gameday g ON pg.gameday_id = g.gameday_id
        WHERE 
            g.gameday_id = $1
        "#,
        gameday_id
    )
    .fetch_all(pool)
    .await?;

    Ok(players)
}
