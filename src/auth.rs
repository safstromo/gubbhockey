#[cfg(feature = "ssr")]
use http::StatusCode;
use leptos::prelude::*;
use leptos::{prelude::ServerFnError, server};
use std::env;
#[cfg(feature = "ssr")]
use uuid::Uuid;

#[cfg(feature = "ssr")]
use crate::models::{delete_session, get_player_by_session, store_pkce_verifier};
use crate::models::{Player, UserInfo};
#[cfg(feature = "ssr")]
use leptos::logging::log;

#[cfg(feature = "ssr")]
use leptos_axum::extract;

#[server]
pub async fn get_auth_url() -> Result<(), ServerFnError> {
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
async fn insert_player(userinfo: UserInfo) -> Result<Player, ServerFnError> {
    use crate::database::get_db;

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
pub async fn user_from_session() -> Result<Player, ServerFnError> {
    use tower_cookies::Cookies;
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
    log!("Validate admin session");
    use tower_cookies::Cookies;
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
    use tower_cookies::Cookies;
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
