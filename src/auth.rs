use crate::models::{PkceStore, Player};
use leptos::prelude::*;
use leptos::{prelude::ServerFnError, server};
use std::env;

#[server]
pub async fn get_auth_url() -> Result<(), ServerFnError> {
    use leptos::logging::log;
    use oauth2::{
        basic::BasicClient, AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
        RedirectUrl, Scope, TokenUrl,
    };

    // Create an OAuth2 client by specifying the client ID, client secret, authorization URL and
    // token URL.
    let client = BasicClient::new(
        ClientId::new(env::var("OAUTH_CLIENT_ID")?),
        Some(ClientSecret::new(env::var("OAUTH_CLIENT_SECRET")?)),
        AuthUrl::new(env::var("OAUTH_AUTH_URL")?)?,
        Some(TokenUrl::new(env::var("OAUTH_TOKEN_URL")?)?),
    )
    // Set the URL the user will be redirected to after the authorization process.
    .set_redirect_uri(RedirectUrl::new(env::var("OAUTH_REDIRECT_URL")?)?);

    // Generate a PKCE challenge.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new("email".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();

    store_pkce_verifier(
        csrf_token.secret().to_string(),
        pkce_verifier.secret().to_string(),
    )
    .await?;

    log!("Redirecting to auth provider.");
    leptos_axum::redirect(auth_url.as_str());
    Ok(())
}

#[server]
pub async fn user_from_session() -> Result<Player, ServerFnError> {
    use http::StatusCode;
    use leptos::logging::log;
    use leptos_axum::extract;
    use tower_cookies::Cookies;
    use uuid::Uuid;

    log!("Getting user from session");
    if let Some(cookies) = extract::<Cookies>().await.ok() {
        if let Some(session_id) = cookies.get("session_id") {
            match Uuid::parse_str(session_id.value()) {
                Ok(uuid) => match get_player_by_session(uuid).await? {
                    Some(player) => return Ok(player),
                    None => {
                        log!("No player with session {:?} found", uuid);

                        let opts = expect_context::<leptos_axum::ResponseOptions>();
                        opts.set_status(StatusCode::UNAUTHORIZED);
                        return Err(ServerFnError::ServerError("Unauthorized".to_string()));
                    }
                },
                Err(_) => {
                    log!("Invalid session_id: {:?}", session_id.value());
                    let opts = expect_context::<leptos_axum::ResponseOptions>();
                    opts.set_status(StatusCode::UNAUTHORIZED);
                    return Err(ServerFnError::ServerError("Unauthorized".to_string()));
                }
            };
        }
    }

    log!("No session cookie found");
    let opts = expect_context::<leptos_axum::ResponseOptions>();
    opts.set_status(StatusCode::NOT_FOUND);
    return Err(ServerFnError::ServerError("No user found".to_string()));
}

#[server]
pub async fn validate_admin() -> Result<bool, ServerFnError> {
    use http::StatusCode;
    use leptos::logging::log;
    use leptos_axum::extract;
    use tower_cookies::Cookies;
    use uuid::Uuid;

    log!("Validate admin session");
    if let Some(cookies) = extract::<Cookies>().await.ok() {
        if let Some(session_id) = cookies.get("session_id") {
            match Uuid::parse_str(session_id.value()) {
                Ok(uuid) => match get_player_by_session(uuid).await? {
                    Some(player) => {
                        if let Some(access_group) = player.access_group {
                            if access_group == "admin" {
                                return Ok(true);
                            }
                        }
                    }
                    None => {
                        log!("No player with session {:?} found", uuid);

                        let opts = expect_context::<leptos_axum::ResponseOptions>();
                        opts.set_status(StatusCode::UNAUTHORIZED);
                        leptos_axum::redirect("/");
                        return Err(ServerFnError::ServerError("Unauthorized".to_string()));
                    }
                },
                Err(_) => {
                    log!("Invalid session_id: {:?}", session_id.value());
                    let opts = expect_context::<leptos_axum::ResponseOptions>();
                    opts.set_status(StatusCode::UNAUTHORIZED);
                    leptos_axum::redirect("/");
                    return Err(ServerFnError::ServerError("Unauthorized".to_string()));
                }
            };
        }
    }

    log!("No session cookie found");
    let opts = expect_context::<leptos_axum::ResponseOptions>();
    opts.set_status(StatusCode::UNAUTHORIZED);
    return Err(ServerFnError::ServerError("Unauthorized".to_string()));
}

#[server]
pub async fn logout() -> Result<(), ServerFnError> {
    use leptos_axum::extract;
    use tower_cookies::Cookies;
    use uuid::Uuid;

    if let Some(cookies) = extract::<Cookies>().await.ok() {
        if let Some(session_id) = cookies.get("session_id") {
            match Uuid::parse_str(session_id.value()) {
                Ok(uuid) => {
                    let _ = delete_session(uuid).await;
                }
                Err(_) => {
                    return Err(ServerFnError::ServerError("No session_id".to_string()));
                }
            };
        }
    }
    let logout_url = env::var("OAUTH_LOGOUT_URL")?;
    leptos_axum::redirect(&logout_url);
    Ok(())
}

#[server]
async fn store_pkce_verifier(
    csrf_token: String,
    pkce_verifier: String,
) -> Result<(), ServerFnError> {
    use crate::database::get_db;
    use chrono::Utc;
    use leptos::logging::log;

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
    use crate::database::get_db;
    use leptos::logging::log;

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
async fn delete_session(session_id: uuid::Uuid) -> Result<(), ServerFnError> {
    use crate::database::get_db;
    use leptos::logging::log;

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
async fn get_player_by_session(session_id: uuid::Uuid) -> Result<Option<Player>, ServerFnError> {
    use crate::database::get_db;
    use leptos::logging::log;

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
