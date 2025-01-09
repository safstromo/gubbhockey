use crate::{
    auth::{user_from_session, validate_admin},
    components::{
        auth_page::Auth, date_picker::DatePicker, gameday_card::GamedayCard,
        gameday_create::GamedayCreate, login_button::LoginButton, logout_button::LogoutButton,
    },
    models::{get_all_gamedays, get_gamedays_by_player},
};
use leptos::{prelude::*, task::spawn_local};
use leptos_meta::{provide_meta_context, Link, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Redirect, Route, Router, Routes, A},
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
                    <Route path=path!("/create") view=CreatePage />
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let (logged_in, set_loggedin) = signal(false);
    let (is_admin, set_is_admin) = signal(false);
    let (player_id, set_player_id) = signal(0);
    let (gamedays_joined, set_gamedays_joined) = signal(Vec::new());
    let player = Resource::new(|| (), |_| async move { user_from_session().await });
    let admin_check = Resource::new(|| (), |_| async move { validate_admin().await });

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

    Effect::new(move |_| {
        if let Some(admin) = admin_check.get() {
            if let Ok(check) = admin {
                set_is_admin.set(check);
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
            <div class="absolute top-4 left-4">
                <Show when=move || { is_admin.get() }>
                    <A href="/create">
                        <button class="btn btn-xs btn-success">Adminpanel</button>
                    </A>
                </Show>
            </div>
            <h1 class="text-4xl text-center mt-14 mb-6">"Falkenbergs Gubbhockey"</h1>
            <h3 class="text-xl">Speldagar</h3>
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
            </Transition>
        </div>
    }
}

#[component]
fn CreatePage() -> impl IntoView {
    let admin_check = Resource::new(
        || (),
        |_| async move { validate_admin().await.unwrap_or(false) },
    );

    let (invalidate_gamedays, set_invalidate_gamedays) = signal(false);
    let gamedays = Resource::new(|| (), |_| async move { get_all_gamedays().await });

    Effect::new(move || {
        if invalidate_gamedays.get() {
            gamedays.refetch();
            set_invalidate_gamedays.set(false);
        }
    });

    view! {
        <Suspense fallback=move || {
            view! {
                <div class="flex flex-col min-h-screen w-full items-center justify-center">
                    <h3 class="text-xl text-center">"Logging in..."</h3>
                    <span class="loading loading-dots loading-lg"></span>
                </div>
            }
        }>
            {move || Suspend::new(async move {
                let is_admin = admin_check.await;
                view! {
                    <Show when=move || { is_admin } fallback=|| view! { <Redirect path="/" /> }>
                        <div class="flex flex-col min-h-screen w-full items-center relative">
                            <A href="/">
                                <h1 class="text-4xl text-center mt-14 mb-6">
                                    "Falkenbergs Gubbhockey"
                                </h1>
                            </A>
                            <DatePicker set_invalidate_gamedays />
                            <Transition fallback=move || view! { <p>"Loading..."</p> }>
                                <h3 class="text-center text-xl mt-6">Alla Speldagar</h3>
                                <ul class="flex flex-col items-center w-11/12">
                                    {move || Suspend::new(async move {
                                        let days = gamedays.await.expect("No gamedays found");
                                        days.into_iter()
                                            .map(|day| {
                                                view! {
                                                    <li class="my-2">
                                                        <GamedayCreate gameday=day set_invalidate_gamedays />
                                                    </li>
                                                }
                                            })
                                            .collect_view()
                                    })}
                                </ul>
                            </Transition>
                        </div>
                    </Show>
                }
            })}
        </Suspense>
    }
}
