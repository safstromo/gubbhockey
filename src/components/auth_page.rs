use std::env;

use leptos::{logging::log, prelude::*, task::spawn_local};
use leptos_router::hooks::use_query_map;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, PkceCodeVerifier, RedirectUrl, TokenUrl,
};
use serde::{Deserialize, Serialize};

use crate::models::{get_pkce_verifier, get_player_by_email, insert_player, UserInfo};
use oauth2::TokenResponse;

#[component]
pub fn Auth() -> impl IntoView {
    let query = use_query_map();

    create_effect(move |_| {
        let id_token = query.read().get("code");
        let csrf_token = query.read().get("state");
        if id_token.is_some() && csrf_token.is_some() {
            spawn_local(async {
                let _ = set_loggin_session(csrf_token.unwrap(), id_token.unwrap()).await;
            });
        }
    });

    view! {
        <div class="flex flex-col min-h-screen w-full items-center justify-center">
            <h3 class="text-xl text-center">"Logging in..."</h3>
            <span class="loading loading-dots loading-lg"></span>
        </div>
    }
}

#[server]
async fn set_loggin_session(csrf_token: String, id_token: String) -> Result<(), ServerFnError> {
    log!("Getting pkce verifier");
    if let Some(pkcestore) = get_pkce_verifier(csrf_token).await? {
        log!("Creating Oath client");
        //TODO: Make this global resource
        let client = BasicClient::new(
            ClientId::new(env::var("OAUTH_CLIENT_ID")?),
            Some(ClientSecret::new(env::var("OAUTH_CLIENT_SECRET")?)),
            AuthUrl::new(env::var("OAUTH_AUTH_URL")?)?,
            Some(TokenUrl::new(env::var("OAUTH_TOKEN_URL")?)?),
        )
        // Set the URL the user will be redirected to after the authorization process.
        .set_redirect_uri(RedirectUrl::new(env::var("OAUTH_REDIRECT_URL")?)?);

        log!("Getting access token");
        let token_result = client
            .exchange_code(AuthorizationCode::new(id_token))
            // Set the PKCE code verifier.
            .set_pkce_verifier(PkceCodeVerifier::new(pkcestore.pkce_verifier))
            .request_async(async_http_client)
            .await?;

        log!("Getting userinfo");
        let client = reqwest::Client::new();
        let userinfo = client
            .get("https://dev-6368dhsgrpcts8kr.eu.auth0.com/userinfo")
            .bearer_auth(token_result.access_token().secret())
            .send()
            .await?
            .json::<UserInfo>()
            .await?;

        log!("userinfo{:?}", userinfo);

        if let Some(player) = get_player_by_email(userinfo.email.clone()).await? {
            log!("player exist{:?}", player);
        } else {
            let player = insert_player(userinfo).await?;
            log!("player inserted: {:?}", player);
        }

        //TODO: Create session for user and set sessionid in cookie
    }
    Ok(())
}
