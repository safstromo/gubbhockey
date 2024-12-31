use leptos::{logging::log, prelude::*, task::spawn_local};

use crate::models::{leave_gameday, Gameday};

#[component]
pub fn LeaveButton(
    gameday_id: i32,
    player_id: ReadSignal<i32>,
    gamedays_joined: ReadSignal<Vec<Gameday>>,
    set_gamedays_joined: WriteSignal<Vec<Gameday>>,
) -> impl IntoView {
    view! {
        <button
            class="btn btn-error h-20 m-2 flex-col"
            on:click=move |_| {
                spawn_local(async move {
                    if leave_gameday(player_id.get_untracked(), gameday_id).await.is_ok() {
                        delete_joined(set_gamedays_joined, gamedays_joined, gameday_id);
                    }
                });
            }
        >
            <p class="font-bold">Kommer</p>
            <p class="font-bold">inte</p>
        </button>
    }
}

fn delete_joined(
    set_gamedays_joined: WriteSignal<Vec<Gameday>>,
    gamedays_joined: ReadSignal<Vec<Gameday>>,
    gameday_id: i32,
) {
    let updated_gamedays: Vec<Gameday> = gamedays_joined
        .get_untracked()
        .into_iter()
        .filter(|day| day.gameday_id != gameday_id)
        .collect();

    set_gamedays_joined.set(updated_gamedays);
}
