use leptos::prelude::*;

#[component]
pub fn JoinButton() -> impl IntoView {
    view! {
        <button
            class="btn btn-success h-20 m-2 flex-col"
            on:click=move |_| println!("joinbutton pressed")
        >
            <p class="font-bold">Jag</p>
            <p class="font-bold">kommer</p>
        </button>
    }
}
