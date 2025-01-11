use leptos::prelude::*;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="flex flex-col min-h-screen w-full items-center justify-center">
            <h3 class="text-xl text-center">"Nothing here move along..."</h3>
            <span class="loading loading-dots loading-lg"></span>
        </div>
    }
}
