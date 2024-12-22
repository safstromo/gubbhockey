use leptos::prelude::*;

#[component]
pub fn TimeCard() -> impl IntoView {
    view! {
        <div class="flex flex-row items-center justify-around m-2 w-2/5">
            <p class="text-center">"22:00"</p>
            <p class="text-center">-</p>
            <p class="text-center">"23:00"</p>
        </div>
    }
}
