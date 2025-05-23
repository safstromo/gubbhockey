use leptos::{prelude::*, task::spawn_local};
use leptos_router::hooks::use_query_map;
use std::env;
use uuid::Uuid;

#[cfg(feature = "ssr")]
use tower_cookies::Cookie;

use crate::models::{Player, UserInfo};

#[component]
pub fn AuthPage() -> impl IntoView {
    let query = use_query_map();

    Effect::new(move |_| {
        let id_token = query.read().get("code");
        let csrf_token = query.read().get("state");
        if id_token.is_some() && csrf_token.is_some() {
            spawn_local(async {
                let _ = set_loggin_session(csrf_token.unwrap(), id_token.unwrap()).await;
            });
        }
    });

    view! {
        <div class="flex flex-col w-full items-center justify-center">
            <h3 class="text-xl text-center">"Logging in..."</h3>
            <span class="loading loading-dots loading-lg"></span>
        </div>
    }
}

//TODO: encrypt cookie
#[server]
async fn set_loggin_session(csrf_token: String, id_token: String) -> Result<(), ServerFnError> {
    use crate::{auth::get_pkce_verifier, models::UserInfo};
    use http::{header, HeaderValue};
    use oauth2::{
        basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
        ClientSecret, PkceCodeVerifier, RedirectUrl, TokenResponse, TokenUrl,
    };
    use tracing::info;

    info!("Getting pkce verifier");
    if let Some(pkcestore) = get_pkce_verifier(csrf_token).await? {
        info!("Creating Oath client");
        //TODO: Make this global resource
        let client = BasicClient::new(
            ClientId::new(env::var("OAUTH_CLIENT_ID")?),
            Some(ClientSecret::new(env::var("OAUTH_CLIENT_SECRET")?)),
            AuthUrl::new(env::var("OAUTH_AUTH_URL")?)?,
            Some(TokenUrl::new(env::var("OAUTH_TOKEN_URL")?)?),
        )
        // Set the URL the user will be redirected to after the authorization process.
        .set_redirect_uri(RedirectUrl::new(env::var("OAUTH_REDIRECT_URL")?)?);

        info!("Getting access token");
        let token_result = client
            .exchange_code(AuthorizationCode::new(id_token))
            // Set the PKCE code verifier.
            .set_pkce_verifier(PkceCodeVerifier::new(pkcestore.pkce_verifier))
            .request_async(async_http_client)
            .await?;

        info!("Getting userinfo");
        let client = reqwest::Client::new();
        let userinfo = client
            .get("https://dev-6368dhsgrpcts8kr.eu.auth0.com/userinfo")
            .bearer_auth(token_result.access_token().secret())
            .send()
            .await?
            .json::<UserInfo>()
            .await?;

        info!("userinfo{:?}", userinfo);
        let response = expect_context::<leptos_axum::ResponseOptions>();

        if let Some(player) = get_player_by_email(userinfo.email.clone()).await? {
            info!("player exist{:?}", player);
            let session = insert_session(player.player_id).await?;
            let cookie = create_cookie(session);
            if let Ok(cookie) = HeaderValue::from_str(&cookie.to_string()) {
                response.insert_header(header::SET_COOKIE, cookie);
            }
        } else {
            let player = insert_player(userinfo).await?;
            info!("player inserted: {:?}", player);
            let session = insert_session(player.player_id).await?;
            let cookie = create_cookie(session);
            if let Ok(cookie) = HeaderValue::from_str(&cookie.to_string()) {
                response.insert_header(header::SET_COOKIE, cookie);
            }
        }

        leptos_axum::redirect("/");
    }
    Ok(())
}

#[server]
async fn insert_session(player_id: i32) -> Result<Uuid, ServerFnError> {
    use crate::database::get_db;
    use chrono::Utc;
    use tracing::{error, info};

    let pool = get_db();
    let session_id = uuid::Uuid::new_v4();
    let expires_at = Utc::now() + chrono::Duration::days(1); // Set expiration to 1 day from now

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
            info!(
                "Session inserted successfully! SessionID: {:?}, PlayerID: {:?}, Expires: {:?}",
                session_id, player_id, expires_at
            );
            Ok(session_id)
        }
        Err(e) => {
            error!("Database error: {:?}", e);
            Err(ServerFnError::ServerError(
                "Failed to create session.".to_string(),
            ))
        }
    }
}

#[cfg(feature = "ssr")]
fn create_cookie(uuid: Uuid) -> Cookie<'static> {
    use tower_cookies::cookie::{time::Duration, Cookie, SameSite};
    let cookie: Cookie = Cookie::build(("session_id", uuid.to_string()))
        .path("/")
        .secure(true)
        .http_only(true)
        .max_age(Duration::days(1))
        .same_site(SameSite::None)
        .build();
    cookie
}

#[server]
async fn insert_player(userinfo: UserInfo) -> Result<Player, ServerFnError> {
    use crate::database::get_db;
    use tracing::{error, info};

    let pool = get_db();

    match sqlx::query_as!(
        Player,
        r#"
        INSERT INTO player (name, given_name, family_name, email, access_group)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING player_id, name, given_name, family_name, email, access_group, is_goalkeeper
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
            info!("User inserted successfully! {:?}", player.name);
            Ok(player)
        }
        Err(e) => {
            error!("Database error: {:?}", e);
            Err(ServerFnError::ServerError(
                "Failed to create player.".to_string(),
            ))
        }
    }
}

#[server]
async fn get_player_by_email(email: String) -> Result<Option<Player>, ServerFnError> {
    use crate::database::get_db;
    use tracing::error;

    let pool = get_db();

    match sqlx::query_as!(
        Player,
        r#"
        SELECT p.player_id, p.name, p.given_name, p.family_name, p.email, p.access_group, p.is_goalkeeper
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
            error!("Database error: {:?}", e);
            Err(ServerFnError::ServerError(
                "Failed to fetch player.".to_string(),
            ))
        }
    }
}
