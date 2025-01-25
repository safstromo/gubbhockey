use leptos::{prelude::*, task::spawn_local};
use leptos_meta::{provide_meta_context, Link, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path, StaticSegment,
};
use reactive_stores::Store;

use crate::{
    auth::{user_from_session, validate_admin},
    components::{footer::Footer, header::Header, not_found::NotFound},
    models::{GlobalState, GlobalStateStoreFields},
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
    let store = Store::new(GlobalState {
        logged_in: false,
        is_admin: false,
    });
    provide_context(store);

    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let player = Resource::new(|| (), |_| async move { user_from_session().await });

    Effect::new(move |_| {
        if let Some(Ok(player_data)) = player.get() {
            // set_loggedin.set(true);
            store.logged_in().set(true);
            if let Some(access_group) = player_data.access_group {
                if access_group == *"admin" {
                    spawn_local(async move {
                        if let Ok(admin) = validate_admin().await {
                            store.is_admin().set(admin);
                        }
                    });
                }
            }
        }
    });
    provide_context(player);

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
        <div class="flex flex-col h-screen justify-between">
            <Router>
                <div>
                    <Header player />
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
                </div>
                <Footer />
            </Router>
        </div>
    };
    website
}
