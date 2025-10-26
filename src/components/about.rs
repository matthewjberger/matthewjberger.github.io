use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <section id="about" class="py-20 bg-gray-900">
            <div class="max-w-6xl mx-auto px-4">
                <h2 class="text-4xl font-bold text-center mb-12 text-white">
                    "About Me"
                </h2>
                <div class="max-w-3xl mx-auto space-y-8">
                    <p class="text-gray-300 leading-relaxed text-lg text-center">
                        "A passionate software architect automating the food industry with Rust 🦀🤖🥗"
                    </p>
                    <p class="text-gray-300 leading-relaxed text-lg text-center">
                        "Founding engineer with 11 years Rust expertise (pre-1.0 early adopter), 5+ years in production systems who built food assembly robotics controls suite from zero in 22 months, securing $10M+ investment from Chipotle and Cava."
                    </p>
                </div>
            </div>
        </section>
    }
}
