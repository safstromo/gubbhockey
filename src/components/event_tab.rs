use leptos::prelude::*;

#[component]
pub fn EventTab(tab_change: ReadSignal<bool>, set_tab_change: WriteSignal<bool>) -> impl IntoView {
    view! {
        <div role="tablist" class="tabs tabs-boxed m-4">
            <a
                role="tab"
                class="tab"
                class:tab-active=move || tab_change.get()
                on:click=move |_| {
                    set_tab_change.set(!tab_change.get());
                }
            >
                Speldagar
            </a>
            <a
                role="tab"
                class="tab "
                class:tab-active=move || !tab_change.get()
                on:click=move |_| {
                    set_tab_change.set(!tab_change.get());
                }
            >
                Cupper
            </a>
        </div>
    }
}
