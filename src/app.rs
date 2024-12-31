use crate::{
    auth::{get_auth_url, validate_session},
    components::{
        auth_page::Auth, date_picker::DatePicker, gameday_card::GamedayCard,
        login_button::LoginButton, logout_button::LogoutButton,
    },
    models::{get_gamedays_by_player, Gameday, Player},
};
use leptos::{logging::log, prelude::*, task::spawn_local};
use leptos_meta::{provide_meta_context, Link, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    hooks::use_query_map,
    path, StaticSegment,
};

use crate::models::get_next_5_gamedays;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/gubbhockey.css" />
        <Link
            href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:ital,wght@0,100..800;1,100..800&display=swap"
            rel="stylesheet"
        />
        // sets the document title
        <Title text="Gubbhockey" />

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage />

                    <Route path=path!("/auth") view=Auth />
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let (logged_in, set_loggedin) = signal(false);
    let (player_id, set_player_id) = signal(0);
    let (gamedays_joined, set_gamedays_joined) = signal(Vec::new());
    let player = Resource::new(|| (), |_| async move { validate_session().await });

    let gamedays = Resource::new(
        move || gamedays_joined.get(),
        |_| async move { get_next_5_gamedays().await },
    );

    Effect::new(move |_| {
        if let Some(player_data) = player.get() {
            if let Ok(data) = player_data {
                set_loggedin.set(true);
                set_player_id.set(data.player_id);

                spawn_local(async move {
                    if let Ok(gamedays) = get_gamedays_by_player(data.player_id).await {
                        set_gamedays_joined.set(gamedays);
                    }
                });
            }
        }
    });
    view! {
        <div class="flex flex-col min-h-screen w-full items-center relative">
            <div class="absolute top-4 right-4">
                <Show when=move || { logged_in.get() } fallback=|| view! { <LoginButton /> }>
                    <LogoutButton />
                </Show>
            </div>
            <h1 class="text-4xl text-center mt-14 mb-6">"Falkenbergs Gubbhockey"</h1>
            <h3 class="text-center text-xl">Speldagar</h3>
            <Transition fallback=move || view! { <p>"Loading..."</p> }>
                <ul class="flex flex-col items-center w-11/12">
                    {move || Suspend::new(async move {
                        let days = gamedays.await.expect("No gamedays found");
                        days.into_iter()
                            .map(|day| {
                                view! {
                                    <li class="my-2">
                                        <GamedayCard
                                            logged_in=logged_in
                                            gamedays_joined=gamedays_joined
                                            set_gamedays_joined=set_gamedays_joined
                                            gameday=day
                                            player_id=player_id
                                        />
                                    </li>
                                }
                            })
                            .collect_view()
                    })}
                </ul>
            // <DatePicker />
            </Transition>
        </div>
    }
}
