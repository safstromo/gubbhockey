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
use uuid::Uuid;

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
pub async fn insert_player(userinfo: UserInfo) -> Result<Player, ServerFnError> {
    let pool = get_db();

    match sqlx::query_as!(
        Player,
        r#"
        INSERT INTO player (name, given_name, family_name, email, access_group)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING player_id, name, given_name, family_name, email, access_group
        "#,
        userinfo.name,
        userinfo.given_name,
        userinfo.family_name,
        userinfo.email,
        "user"
    )
    .fetch_one(pool)
    .await
    {
        Ok(player) => {
            log!("User inserted successfully! {:?}", player.name);
            Ok(player)
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
pub async fn join_gameday(player_id: i32, gameday_id: i32) -> Result<(), ServerFnError> {
    let pool = get_db();
    match sqlx::query!(
        r#"
        INSERT INTO player_gameday (player_id, gameday_id)
        VALUES ($1, $2)
        "#,
        player_id,
        gameday_id
    )
    .execute(pool)
    .await
    {
        Ok(_) => {
            log!("Player: {:?} joined: {:?}", player_id, gameday_id);
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
pub async fn leave_gameday(player_id: i32, gameday_id: i32) -> Result<(), ServerFnError> {
    let pool = get_db();
    match sqlx::query!(
        r#"
        DELETE FROM player_gameday
        WHERE player_id = $1 AND gameday_id = $2
        "#,
        player_id,
        gameday_id
    )
    .execute(pool)
    .await
    {
        Ok(_) => {
            log!("Player: {:?} left gameday: {:?}", player_id, gameday_id);
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
        SELECT 
            g.gameday_id, 
            g.start_date, 
            g.end_date,
            COUNT(pg.player_id) as player_count 
        FROM 
            gameday g
        LEFT JOIN 
            player_gameday pg ON g.gameday_id = pg.gameday_id
        WHERE 
            g.start_date >= NOW() 
        GROUP BY 
            g.gameday_id, g.start_date, g.end_date
        ORDER BY 
            g.start_date ASC
        LIMIT 5         
        "#
    )
    .fetch_all(pool)
    .await
    {
        Ok(results) => {
            log!("Successfully retrieved next 5 gamedays with player counts.");
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
pub async fn get_all_gamedays() -> Result<Vec<Gameday>, ServerFnError> {
    let pool = get_db();
    match sqlx::query_as!(
        Gameday,
        r#"
        SELECT 
            g.gameday_id, 
            g.start_date, 
            g.end_date,
            COUNT(pg.player_id) as player_count 
        FROM 
            gameday g
        LEFT JOIN 
            player_gameday pg ON g.gameday_id = pg.gameday_id
        WHERE 
            g.start_date >= NOW() 
        GROUP BY 
            g.gameday_id, g.start_date, g.end_date
        ORDER BY 
            g.start_date ASC
        "#
    )
    .fetch_all(pool)
    .await
    {
        Ok(results) => {
            log!("Successfully retrieved all gamedays with player counts.");
            Ok(results)
        }
        Err(e) => {
            log!("Database error: {:?}", e);
            Err(ServerFnError::ServerError(
                "Failed to get all gamedays.".to_string(),
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

#[server]
pub async fn count_players_by_gameday(gameday_id: i32) -> Result<i64, ServerFnError> {
    let pool = get_db();

    let count = sqlx::query_scalar!(
        r#"
        SELECT 
            COUNT(*)
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
    .fetch_one(pool)
    .await
    .unwrap_or(Some(0));

    log!(
        "Counted {} players for gameday {}",
        count.unwrap(),
        gameday_id
    );
    Ok(count.unwrap())
}

#[server]
pub async fn get_gamedays_by_player(player_id: i32) -> Result<Vec<Gameday>, ServerFnError> {
    let pool = get_db();

    match sqlx::query_as!(
        Gameday,
        r#"
        SELECT
            g.gameday_id,
            g.start_date,
            g.end_date,
            COUNT(pg.player_id) as player_count
        FROM
            Gameday g
        LEFT JOIN 
            player_gameday pg ON g.gameday_id = pg.gameday_id
        WHERE
            pg.player_id = $1
        GROUP BY 
            g.gameday_id, g.start_date, g.end_date
        ORDER BY
            g.start_date DESC        
        "#,
        player_id
    )
    .fetch_all(pool)
    .await
    {
        Ok(gamedays) => {
            log!(
                "Successfully retrieved {} gamedays for player {}",
                gamedays.len(),
                player_id
            );
            Ok(gamedays)
        }
        Err(e) => {
            log!(
                "Database error while fetching gamedays for player {}: {:?}",
                player_id,
                e
            );
            Err(ServerFnError::from(e))
        }
    }
}

#[server]
pub async fn get_gamedays_with_player_count() -> Result<Vec<Gameday>, ServerFnError> {
    let pool = get_db();
    match sqlx::query_as!(
        Gameday,
        r#"
        SELECT 
            g.gameday_id,
            g.start_date,
            g.end_date,
            COUNT(pg.player_id) as player_count
        FROM 
            gameday g
        LEFT JOIN 
            player_gameday pg ON g.gameday_id = pg.gameday_id
        GROUP BY 
            g.gameday_id, g.start_date, g.end_date
        ORDER BY 
            g.start_date ASC
        "#
    )
    .fetch_all(pool)
    .await
    {
        Ok(gamedays) => {
            log!("Successfully retrieved gamedays with player counts.");
            Ok(gamedays)
        }
        Err(e) => {
            log!("Database error: {:?}", e);
            Err(ServerFnError::ServerError(
                "Failed to fetch gamedays with player counts.".to_string(),
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

#[server]
pub async fn insert_session(player_id: i32) -> Result<Uuid, ServerFnError> {
    let pool = get_db();
    let session_id = uuid::Uuid::new_v4();
    let expires_at = Utc::now() + chrono::Duration::minutes(15); // Set expiration to 15 minutes from now

    match sqlx::query!(
        r#"
        INSERT INTO session (session_id, player_id, created_at, expires_at)
        VALUES ($1, $2, NOW(), $3)
        "#,
        session_id,
        player_id,
        expires_at
    )
    .execute(pool)
    .await
    {
        Ok(_) => {
            log!(
                "Session inserted successfully! SessionID: {:?}, PlayerID: {:?}, Expires: {:?}",
                session_id,
                player_id,
                expires_at
            );
            Ok(session_id)
        }
        Err(e) => {
            log!("Database error: {:?}", e);
            Err(ServerFnError::ServerError(
                "Failed to create session.".to_string(),
            ))
        }
    }
}

#[server]
pub async fn get_player_by_session(
    session_id: uuid::Uuid,
) -> Result<Option<Player>, ServerFnError> {
    let pool = get_db();

    match sqlx::query_as!(
        Player,
        r#"
        SELECT p.player_id, p.name, p.given_name, p.family_name, p.email, p.access_group
        FROM session s
        JOIN player p ON s.player_id = p.player_id
        WHERE s.session_id = $1
          AND s.expires_at > NOW()
        "#,
        session_id
    )
    .fetch_optional(pool)
    .await
    {
        Ok(player) => Ok(player),
        Err(e) => {
            log!("Database error: {:?}", e);
            Err(ServerFnError::ServerError(
                "Failed to fetch player.".to_string(),
            ))
        }
    }
}

#[server]
pub async fn delete_session(session_id: uuid::Uuid) -> Result<(), ServerFnError> {
    let pool = get_db();

    match sqlx::query!(
        r#"
        DELETE FROM session
        WHERE session_id = $1
        "#,
        session_id
    )
    .execute(pool)
    .await
    {
        Ok(_) => {
            log!("Session {:?} deleted successfully.", session_id);
            Ok(())
        }
        Err(e) => {
            log!("Database error: {:?}", e);
            Err(ServerFnError::ServerError(
                "Failed to delete session.".to_string(),
            ))
        }
    }
}

#[server]
pub async fn delete_gameday(gameday_id: i32) -> Result<(), ServerFnError> {
    let pool = get_db();

    match sqlx::query!(
        r#"
        DELETE FROM gameday
        WHERE gameday_id = $1
        "#,
        gameday_id
    )
    .execute(pool)
    .await
    {
        Ok(_) => {
            log!("Gameday {:?} deleted successfully.", gameday_id);
            Ok(())
        }
        Err(e) => {
            log!("Database error: {:?}", e);
            Err(ServerFnError::ServerError(
                "Failed to delete gameday.".to_string(),
            ))
        }
    }
}

#[server]
pub async fn get_player_by_email(email: String) -> Result<Option<Player>, ServerFnError> {
    let pool = get_db();

    match sqlx::query_as!(
        Player,
        r#"
        SELECT p.player_id, p.name, p.given_name, p.family_name, p.email, p.access_group
        FROM player p
        WHERE p.email = $1
        "#,
        email
    )
    .fetch_optional(pool)
    .await
    {
        Ok(player) => Ok(player),
        Err(e) => {
            log!("Database error: {:?}", e);
            Err(ServerFnError::ServerError(
                "Failed to fetch player.".to_string(),
            ))
        }
    }
}
