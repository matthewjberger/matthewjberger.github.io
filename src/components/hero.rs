use leptos::prelude::*;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section id="hero" class="min-h-screen flex items-center justify-center bg-gradient-to-br from-gray-900 to-gray-800">
            <div class="text-center px-4">
                <h1 class="text-5xl md:text-7xl font-bold text-white mb-4">
                    "Hi, I'm " <span class="text-blue-400">"Matthew Berger"</span>
                </h1>
                <p class="text-xl md:text-2xl text-gray-300 mb-8">
                    "Robotics Software Engineer"
                </p>
                <div class="flex gap-4 justify-center">
                    <a
                        href="mailto:matthewjordanberger@gmail.com"
                        class="px-6 py-3 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors"
                    >
                        "Get In Touch"
                    </a>
                    <a
                        href="https://github.com/matthewjberger"
                        target="_blank"
                        class="px-6 py-3 border-2 border-blue-400 text-blue-400 rounded-lg hover:bg-gray-800 transition-colors"
                    >
                        "View GitHub"
                    </a>
                </div>
            </div>
        </section>
    }
}
