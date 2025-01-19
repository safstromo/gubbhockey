use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn TermsPage() -> impl IntoView {
    view! {
        <div class="flex flex-col min-h-screen">
            <div class="flex justify-center">
                <A href="/">
                    <img src="Logo-nobg.png" alt="Logo" class="h-60 w-60" />
                </A>
            </div>
            <header class="pb-6">
                <div class="container mx-auto px-4">
                    <h1 class="text-3xl font-bold text-center">Terms and Conditions</h1>
                </div>
            </header>

            <main class="flex-grow container mx-auto px-4 py-8 space-y-8">
                <section id="privacy-policy" class="p-6 border rounded">
                    <h2 class="text-2xl font-semibold mb-4">Privacy Policy</h2>
                    <p class="mb-4">
                        <strong>"Effective Date: "</strong>
                        "2025-01-20"
                    </p>
                    <p class="mb-4">
                        <strong>"Falkenbergs Gubbhockey "</strong>
                        ("we," "our," or "us") values your privacy and is committed to protecting the personal information you share with us. This Privacy Policy outlines how we handle your data when you visit our website or book a practice spot.
                    </p>

                    <h3 class="text-xl font-semibold mt-6">1. Information We Collect</h3>
                    <p class="mb-4">
                        - <strong>"Personal Information: "</strong>
                        To facilitate practice bookings, we may collect your name and email address. This information is used solely for organizational purposes.
                    </p>
                    <p class="mb-4">
                        - <strong>"Usage Data: "</strong>
                        We may collect basic usage data, such as the pages you visit, to improve website performance. No personally identifiable information is linked to this data.
                    </p>

                    <h3 class="text-xl font-semibold mt-6">2. How We Use Your Information</h3>
                    <p class="mb-4">"We use your information strictly to: "</p>
                    <ul class="list-disc list-inside space-y-2">
                        <li>Manage practice bookings.</li>
                        <li>Notify you about schedule updates or cancellations.</li>
                    </ul>

                    <h3 class="text-xl font-semibold mt-6">3. Data Sharing</h3>
                    <p class="mb-4">
                        We do <strong>" not "</strong>
                        sell, share, or distribute your data to third parties under any circumstances.
                    </p>

                    <h3 class="text-xl font-semibold mt-6">4. Data Security</h3>
                    <p class="mb-4">
                        We take reasonable measures to protect your information. However, as with any online platform, we cannot guarantee absolute security.
                    </p>

                    <h3 class="text-xl font-semibold mt-6">5. Cookies</h3>
                    <p class="mb-4">
                        Our website may use cookies to improve user experience. You can disable cookies in your browser settings if preferred.
                    </p>

                    <h3 class="text-xl font-semibold mt-6">6. Contact Us</h3>
                    <p class="mb-4">
                        "For questions or concerns about this Privacy Policy, contact us at: "
                    </p>
                    <p>
                        "Email: "<a href="mailto:fbg.gubbhockey@gmail.com" class="underline">
                            "fbg.gubbhockey@gmail.com"
                        </a>
                    </p>
                </section>

                <section id="terms-and-conditions" class="p-6 border rounded">
                    <h2 class="text-2xl font-semibold mb-4">Terms and Conditions</h2>
                    <p class="mb-4">
                        <strong>"Effective Date: "</strong>
                        "2025-01-20"
                    </p>
                    <p class="mb-4">
                        Welcome to <strong>" Falkenbergs Gubbhockey"</strong>
                        ! These terms govern your use of our website. By using our site, you agree to these Terms and Conditions.
                    </p>

                    <h3 class="text-xl font-semibold mt-6">1. Website Use</h3>
                    <p class="mb-4">
                        - <strong>"Purpose: "</strong>
                        This website is intended to facilitate practice spot bookings for members of the veteran hockey team.
                    </p>
                    <p class="mb-4">
                        - <strong>"User Conduct: "</strong>
                        Users agree to respect the website intended use and refrain from any actions that may disrupt its functionality.
                    </p>

                    <h3 class="text-xl font-semibold mt-6">2. Booking Practices</h3>
                    <p class="mb-4">
                        - <strong>"Open Booking: "</strong>
                        Players are free to book or unbook practice sessions at their discretion. There are no restrictions on booking frequency or cancellations.
                    </p>
                    <p class="mb-4">
                        - <strong>"No Fees: "</strong>
                        Our website does not charge for bookings. All sessions are free for participants.
                    </p>

                    <h3 class="text-xl font-semibold mt-6">3. Liability</h3>
                    <p class="mb-4">
                        - Participation in hockey practices is voluntary and at your own risk.
                    </p>
                    <p class="mb-4">
                        - <strong>"Falkenbergs Gubbhockey "</strong>
                        is not responsible for injuries, accidents, or lost items during practice sessions.
                    </p>

                    <h3 class="text-xl font-semibold mt-6">4. Amendments</h3>
                    <p class="mb-4">
                        We may update these Terms and Conditions periodically. Continued use of the website signifies your acceptance of any updates.
                    </p>

                    <h3 class="text-xl font-semibold mt-6">5. Contact Us</h3>
                    <p class="mb-4">
                        "For inquiries about these Terms and Conditions, contact us at:"
                    </p>
                    <p>
                        "Email: " <a href="mailto:fbg.gubbhockey@gmail.com" class="underline">
                            "fbg.gubbhockey@gmail.com"
                        </a>
                    </p>
                </section>
            </main>

            <footer class="py-4">
                <div class="text-center">
                    <p>"2025 Falkenbergs Gubbhockey."</p>
                    <p>"All rights reserved."</p>
                    <p>
                        <a href="#" class="underline">
                            Back to top
                        </a>
                    </p>
                </div>
            </footer>
        </div>
    }
}
