use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::components::{date_picker::DatePicker, gameday_card::GamedayCard};

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
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="flex flex-col min-h-screen w-full items-center">

            <h1 class="text-4xl">"Falkenbergs Gubbhockey"</h1>
            <Await future=get_next_5_gamedays() let:gamedays>
                <ul class="flex flex-col items-center w-11/12">
                    {gamedays
                        .clone()
                        .unwrap()
                        .into_iter()
                        .map(|day| {
                            view! {
                                <li class="my-2">
                                    <GamedayCard gameday=day />
                                </li>
                            }
                        })
                        .collect_view()}
                </ul>
            </Await>
            <DatePicker />
        </div>
    }
}
