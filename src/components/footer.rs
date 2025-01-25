use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer>
            <div class="text-center mt-6 mb-4">
                <A href="/terms">
                    <p class="underline m-2">"Terms and Conditions"</p>
                </A>
                <p>"2025 Falkenbergs Gubbhockey."</p>
                <p>"All rights reserved."</p>
                <p>
                    <a href="#" class="underline">
                        Back to top
                    </a>
                </p>
            </div>
        </footer>
    }
}
