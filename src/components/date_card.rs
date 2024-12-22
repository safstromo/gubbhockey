use leptos::prelude::*;

#[component]
pub fn DateCard() -> impl IntoView {
    view! {
        <div class="flex m-2 w-30 h-20">
            <div class="flex-col w-full items-center content-center ">
                <p class="text-center font-bold">Tis</p>
                <p class="text-center text-3xl font-bold">24</p>
            </div>
            <div class="divider divider-horizontal" />
        </div>
    }
}
