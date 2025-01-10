use leptos::{prelude::*, task::spawn_local};

use crate::models::{get_gamedays_by_player, join_gameday, Gameday};

#[component]
pub fn JoinButton(
    logged_in: ReadSignal<bool>,
    gameday_id: i32,
    set_gamedays_joined: WriteSignal<Vec<Gameday>>,
) -> impl IntoView {
    view! {
        <button
            class="btn btn-success h-20 m-2 flex-col"
            on:click=move |_| {
                spawn_local(async move {
                    if join_gameday(gameday_id).await.is_ok() {
                        add_joined(set_gamedays_joined).await;
                    }
                });
            }
            disabled=move || !logged_in.get()
        >
            <p class="font-bold">Jag</p>
            <p class="font-bold">kommer</p>
        </button>
    }
}

async fn add_joined(set_gamedays_joined: WriteSignal<Vec<Gameday>>) {
    if let Ok(gamedays) = get_gamedays_by_player().await {
        set_gamedays_joined.set(gamedays);
    }
}
