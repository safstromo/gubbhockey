use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path, StaticSegment,
};

use crate::{
    components::not_found::NotFound,
    pages::{
        auth_page::AuthPage, create_page::CreatePage, day_page::DayPage, homepage::HomePage,
        profile_page::ProfilePage, terms_page::TermsPage,
    },
};

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

    let website = view! {
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
                <Routes fallback=|| view! { <NotFound /> }>
                    <Route path=StaticSegment("") view=HomePage />

                    <Route path=path!("/auth") view=AuthPage />
                    <Route path=path!("/create") view=CreatePage />
                    <Route path=path!("/day/:id") view=DayPage />
                    <Route path=path!("/terms") view=TermsPage />
                    <Route path=path!("/profile") view=ProfilePage />
                </Routes>
            </main>
        </Router>
    };
    website
}
