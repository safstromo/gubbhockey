use leptos::{prelude::*, svg::view};
use leptos_router::components::{Redirect, A};

use crate::{auth::user_from_session, components::logout_button::LogoutButton};

#[component]
pub fn ProfilePage() -> impl IntoView {
    let player = Resource::new(|| (), |_| async move { user_from_session().await });

    let goalkeeper = RwSignal::new(false);

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
                let player = player.await.expect("no player found");
                view! {
                    <div class="flex flex-col min-h-screen w-full items-center relative">
                        <div class="absolute top-4 right-4">
                            <LogoutButton />
                        </div>

                        <div class="flex justify-center mb-10">
                            <A href="/">
                                <img src="Logo-nobg.png" alt="Logo" class="h-60 w-60" />
                            </A>
                        </div>
                        <p class="m-2 text-xl text-bold">{player.name}</p>
                        <p class="m-2">{player.email}</p>

                        <label class="label cursor-pointer mt-2">
                            <span class="label-text mx-2">Utespelare</span>
                            <input type="checkbox" class="toggle" bind:checked=goalkeeper />
                            <span class="label-text mx-2">Målvakt</span>
                        </label>

                    </div>
                }
            })}
        </Suspense>
    }
}
