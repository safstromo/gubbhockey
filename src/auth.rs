use std::env;

use leptos::{logging::log, prelude::ServerFnError, server};
use oauth2::{
    basic::BasicClient, url::Url, AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    RedirectUrl, Scope, TokenUrl,
};

use crate::models::store_pkce_verifier;

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
