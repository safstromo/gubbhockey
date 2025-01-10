use chrono::{DateTime, Utc};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Player {
    pub player_id: i32,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub email: String,
    pub access_group: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Gameday {
    pub gameday_id: i32,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub player_count: Option<i64>,
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
pub async fn get_players_by_gameday(gameday_id: i32) -> Result<Vec<Player>, ServerFnError> {
    use crate::auth::validate_admin;
    use leptos::logging::log;

    use crate::database::get_db;
    if let Err(err) = validate_admin().await {
        return Err(err);
    }

    let pool = get_db();
    match sqlx::query_as!(
        Player,
        r#"
        SELECT 
            p.player_id,
            p.name,
            p.given_name,
            p.family_name,
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
