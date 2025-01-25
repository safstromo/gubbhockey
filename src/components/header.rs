use leptos::prelude::*;
use leptos_router::components::A;
use reactive_stores::Store;

use crate::{
    components::login_button::LoginButton,
    models::{GlobalState, GlobalStateStoreFields, Player},
};

#[component]
pub fn Header(player: Resource<Result<Player, ServerFnError>>) -> impl IntoView {
    let state = expect_context::<Store<GlobalState>>();
    let logged_in = state.logged_in();
    let is_admin = state.is_admin();
    view! {
        <div class="absolute top-4 right-4 flex flex-col justify-center">
            <Show when=move || { logged_in.get() } fallback=|| view! { <LoginButton /> }>
                <Show when=move || { logged_in.get() }>
                    <Transition>
                        {move || Suspend::new(async move {
                            let player = player.await;
                            view! {
                                <A href="/profile">
                                    <div class="avatar placeholder flex justify-center mb-2">
                                        <div class="bg-neutral text-neutral-content w-20 rounded-full">
                                            <span class="text-3xl">
                                                {format!(
                                                    "{}{}",
                                                    player
                                                        .clone()
                                                        .unwrap()
                                                        .given_name
                                                        .chars()
                                                        .next()
                                                        .unwrap_or(' '),
                                                    player
                                                        .clone()
                                                        .unwrap()
                                                        .family_name
                                                        .chars()
                                                        .next()
                                                        .unwrap_or(' '),
                                                )}
                                            </span>
                                        </div>
                                    </div>
                                </A>
                            }
                        })}
                    </Transition>
                </Show>
            </Show>
        </div>
        <div class="absolute top-4 left-4">
            <Show when=move || { is_admin.get() }>
                <A href="/create">
                    <button class="btn btn-xs btn-success">Adminpanel</button>
                </A>
            </Show>
        </div>
        <div class="flex justify-center">
            <A href="/">
                <img src="Logo-nobg.png" alt="Logo" class="h-60 w-60" />
            </A>
        </div>
    }
}
