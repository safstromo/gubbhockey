use leptos::{prelude::*, task::spawn_local};

use crate::auth::logout;

#[component]
pub fn LogoutButton() -> impl IntoView {
    view! {
        <button
            on:click=move |_| {
                spawn_local(async {
                    let _ = logout().await;
                });
            }
            class="btn btn-sm btn-error"
        >
            "Logga ut"
        </button>
    }
}
