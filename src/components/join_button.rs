use leptos::{logging::log, prelude::*};

#[component]
pub fn JoinButton(logged_in: ReadSignal<bool>) -> impl IntoView {
    view! {
        <button
            class="btn btn-success h-20 m-2 flex-col"
            on:click=move |_| log!("joinbutton pressed")
            disabled=move || !logged_in.get()
        >
            <p class="font-bold">Jag</p>
            <p class="font-bold">kommer</p>
        </button>
    }
}
