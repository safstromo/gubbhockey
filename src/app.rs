use crate::{
    auth::{get_auth_url, validate_session},
    components::{auth_page::Auth, date_picker::DatePicker, gameday_card::GamedayCard},
    models::Player,
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
    let player = Resource::new(|| (), |_| async move { validate_session().await });
    Effect::new(move |_| {
        if let Some(play) = player.get() {
            set_loggedin.set(play.is_ok());
        }
    });
    view! {
        <div class="flex flex-col min-h-screen w-full items-center">
            <h1 class="text-4xl text-center m-6">"Falkenbergs Gubbhockey"</h1>
            <h3 class="text-center text-xl">Speldagar</h3>
            <Await future=get_next_5_gamedays() let:gamedays>
                <ul class="flex flex-col items-center w-11/12">
                    {gamedays
                        .clone()
                        .unwrap()
                        .into_iter()
                        .map(|day| {
                            view! {
                                <li class="my-2">
                                    <GamedayCard gameday=day logged_in=logged_in />
                                </li>
                            }
                        })
                        .collect_view()}
                </ul>
            </Await>
            <button
                on:click=move |_| {
                    spawn_local(async {
                        let _ = get_auth_url().await;
                    });
                }
                class="btn btn-info w-30 h-10"
            >
                "Logga in"
            </button>
            <DatePicker />
        </div>
    }
}
