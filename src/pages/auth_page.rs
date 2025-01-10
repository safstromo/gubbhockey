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
        <div class="flex flex-col min-h-screen w-full items-center justify-center">
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
    use leptos::logging::log;
    use oauth2::{
        basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
        ClientSecret, PkceCodeVerifier, RedirectUrl, TokenResponse, TokenUrl,
    };

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
        let response = expect_context::<leptos_axum::ResponseOptions>();

        if let Some(player) = get_player_by_email(userinfo.email.clone()).await? {
            log!("player exist{:?}", player);
            let session = insert_session(player.player_id).await?;
            let cookie = create_cookie(session);
            if let Ok(cookie) = HeaderValue::from_str(&cookie.to_string()) {
                response.insert_header(header::SET_COOKIE, cookie);
            }
        } else {
            let player = insert_player(userinfo).await?;
            log!("player inserted: {:?}", player);
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
    use leptos::logging::log;

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
    use leptos::logging::log;

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
async fn get_player_by_email(email: String) -> Result<Option<Player>, ServerFnError> {
    use crate::database::get_db;
    use leptos::logging::log;

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
