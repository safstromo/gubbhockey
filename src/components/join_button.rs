use leptos::{prelude::*, task::spawn_local};

use crate::models::{get_gamedays_by_player, join_gameday, Gameday};

#[component]
pub fn JoinButton(
    logged_in: ReadSignal<bool>,
    gameday_id: i32,
    player_id: ReadSignal<i32>,
    set_gamedays_joined: WriteSignal<Vec<Gameday>>,
) -> impl IntoView {
    view! {
        <button
            class="btn btn-success h-20 m-2 flex-col"
            on:click=move |_| {
                spawn_local(async move {
                    if join_gameday(player_id.get_untracked(), gameday_id).await.is_ok() {
                        add_joined(set_gamedays_joined, player_id.get_untracked()).await;
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

async fn add_joined(set_gamedays_joined: WriteSignal<Vec<Gameday>>, player_id: i32) {
    if let Ok(gamedays) = get_gamedays_by_player(player_id).await {
        set_gamedays_joined.set(gamedays);
    }
}
