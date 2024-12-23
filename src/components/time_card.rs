use leptos::prelude::*;

#[component]
pub fn TimeCard() -> impl IntoView {
    view! {
        <div class="flex flex-row items-center justify-center m-2 w-full">
            <p class="text-center">"22:00"</p>
            <p class="text-center">-</p>
            <p class="text-center">"23:00"</p>
        </div>
    }
}
