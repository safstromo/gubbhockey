use leptos::prelude::*;

#[component]
pub fn Loading() -> impl IntoView {
    view! {
        <div class="flex flex-col w-full items-center justify-center">
            <h3 class="text-xl text-center">"Loading"</h3>
            <span class="loading loading-dots loading-lg"></span>
        </div>
    }
}
