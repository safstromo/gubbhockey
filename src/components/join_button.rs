use leptos::{logging::log, prelude::*, task::spawn_local};

use crate::models::{join_gameday, Player};

#[component]
pub fn JoinButton(
    logged_in: ReadSignal<bool>,
    gameday_id: i32,
    player: Option<Result<Player, ServerFnError>>,
) -> impl IntoView {
    view! {
        <button
            class="btn btn-success h-20 m-2 flex-col"
            on:click=move |_| {
                if let Some(player) = &player {
                    if let Ok(player) = player {
                        let player_id = player.player_id;
                        spawn_local(async move {
                            let _ = join_gameday(player_id, gameday_id).await;
                        });
                    }
                }
            }
            disabled=move || !logged_in.get()
        >
            <p class="font-bold">Jag</p>
            <p class="font-bold">kommer</p>
        </button>
    }
}
