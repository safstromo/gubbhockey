use http::StatusCode;
use leptos::prelude::*;
use leptos::{logging::log, prelude::ServerFnError, server};
use oauth2::{
    basic::BasicClient, url::Url, AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    RedirectUrl, Scope, TokenUrl,
};
use std::env;
use uuid::Uuid;

use crate::models::{delete_session, get_player_by_session, store_pkce_verifier, Player};

#[cfg(feature = "ssr")]
use leptos_axum::extract;
#[cfg(feature = "ssr")]
use tower_cookies::Cookies;

#[server]
pub async fn get_auth_url() -> Result<(), ServerFnError> {
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
pub async fn validate_session() -> Result<Player, ServerFnError> {
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

    return Err(ServerFnError::ServerError("Unauthorized".to_string()));
}

#[server]
pub async fn logout() -> Result<(), ServerFnError> {
    use tower_cookies::Cookie;
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
