#[cfg(feature = "ssr")]
use crate::database::get_db;
use chrono::{DateTime, Utc};
#[cfg(feature = "ssr")]
use leptos::logging::log;
use leptos::prelude::ServerFnError;
use leptos::server;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Player {
    pub player_id: i32,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub access_group: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Gameday {
    pub gameday_id: i32,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct PkceStore {
    pub id: i32,                   // Unique identifier for the entry
    pub csrf_token: String,        // CSRF token for validation
    pub pkce_verifier: String,     // PKCE verifier
    pub created_at: DateTime<Utc>, // Timestamp when the entry was created
    pub expires_at: DateTime<Utc>, // Expiration timestamp
}
#[server]
pub async fn insert_player(
    name: String,
    surname: String,
    email: String,
) -> Result<(), ServerFnError> {
    let pool = get_db();
    match sqlx::query!(
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
    .await
    {
        Ok(_) => {
            log!("User inserted successfully! {:?}", name);
            Ok(())
        }
        Err(e) => {
            log!("Database error: {:?}", e);
            Err(ServerFnError::ServerError(
                "Failed to create player.".to_string(),
            ))
        }
    }
}

#[server]
pub async fn insert_gameday(
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
) -> Result<(), ServerFnError> {
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
                "Date inserted successfully! Start{:?}, End:{:?}",
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

#[server]
pub async fn join_gameday(player: Player, gameday: Gameday) -> Result<(), ServerFnError> {
    let pool = get_db();
    match sqlx::query!(
        r#"
        INSERT INTO player_gameday (player_id, gameday_id)
        VALUES ($1, $2)
        "#,
        player.player_id,
        gameday.gameday_id
    )
    .execute(pool)
    .await
    {
        Ok(_) => {
            log!("Player: {:?} joined: {:?}", player, gameday);
            Ok(())
        }
        Err(e) => {
            log!("Database error: {:?}", e);
            Err(ServerFnError::ServerError(
                "Failed to add player to gameday.".to_string(),
            ))
        }
    }
}

#[server]
pub async fn leave_gameday(player: Player, gameday: Gameday) -> Result<(), ServerFnError> {
    let pool = get_db();
    match sqlx::query!(
        r#"
        DELETE FROM player_gameday
        WHERE player_id = $1 AND gameday_id = $2
        "#,
        player.player_id,
        gameday.gameday_id
    )
    .execute(pool)
    .await
    {
        Ok(_) => {
            log!("Player: {:?} left gameday: {:?}", player, gameday);
            Ok(())
        }
        Err(e) => {
            log!("Database error: {:?}", e);
            Err(ServerFnError::ServerError(
                "Failed to remove player from gameday.".to_string(),
            ))
        }
    }
}

#[server]
pub async fn get_next_5_gamedays() -> Result<Vec<Gameday>, ServerFnError> {
    let pool = get_db();
    match sqlx::query_as!(
        Gameday,
        r#"
        SELECT gameday_id, start_date, end_date
        FROM gameday
        WHERE start_date >= NOW()
        ORDER BY start_date ASC
        LIMIT 5
        "#
    )
    .fetch_all(pool)
    .await
    {
        Ok(results) => {
            log!("Successfully got next 5 gamedays");
            Ok(results)
        }
        Err(e) => {
            log!("Database error: {:?}", e);
            Err(ServerFnError::ServerError(
                "Failed to get the next 5 gamedays.".to_string(),
            ))
        }
    }
}

#[server]
pub async fn get_players_by_gameday(gameday_id: i32) -> Result<Vec<Player>, ServerFnError> {
    let pool = get_db();
    match sqlx::query_as!(
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
    .await
    {
        Ok(players) => {
            log!("Successfully got players connected to {:?}", gameday_id);
            Ok(players)
        }
        Err(e) => {
            log!("Database error: {:?}", e);
            Err(ServerFnError::ServerError(
                "Failed to get players connected to gameday.".to_string(),
            ))
        }
    }
}

#[server]
pub async fn store_pkce_verifier(
    csrf_token: String,
    pkce_verifier: String,
) -> Result<(), ServerFnError> {
    let pool = get_db();

    // Calculate expiration (15 minutes)
    let expires_at = Utc::now() + chrono::Duration::minutes(15);

    match sqlx::query!(
        "INSERT INTO pkce_store (csrf_token, pkce_verifier, created_at, expires_at)
         VALUES ($1, $2, NOW(), $3)
         ON CONFLICT (csrf_token) DO NOTHING",
        csrf_token,
        pkce_verifier,
        expires_at
    )
    .execute(pool)
    .await
    {
        Ok(_) => {
            log!("Successfully stored PKCE verifier and CSRF token.");
            Ok(())
        }
        Err(e) => {
            // Log detailed error in development
            log!("Database error: {:?}", e);
            Err(ServerFnError::ServerError(format!(
                "Failed to insert tokens: {:?}",
                e
            )))
        }
    }
}

#[server]
pub async fn get_pkce_verifier(csrf_token: String) -> Result<Option<PkceStore>, ServerFnError> {
    let pool = get_db();
    match sqlx::query_as::<_, PkceStore>(
        "SELECT * FROM pkce_store WHERE csrf_token = $1 AND expires_at > NOW()",
    )
    .bind(csrf_token)
    .fetch_optional(pool)
    .await
    {
        Ok(pkce) => {
            log!("Successfully got Pkcestore.");
            Ok(pkce)
        }
        Err(e) => {
            log!("Database error: {:?}", e);
            Err(ServerFnError::ServerError(format!(
                "Failed to get tokens: {:?}",
                e
            )))
        }
    }
}
