use leptos::{prelude::*, task::spawn_local};

use crate::auth::get_auth_url;

#[component]
pub fn LoginButton() -> impl IntoView {
    view! {
        <button
            on:click=move |_| {
                spawn_local(async {
                    let _ = get_auth_url().await;
                });
            }
            class="btn btn-sm btn-info"
        >
            "Logga in"
        </button>
    }
}
