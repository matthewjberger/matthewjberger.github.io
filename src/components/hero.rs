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
                <div class="flex gap-4 justify-center flex-wrap">
                    <a
                        href="mailto:matthewjordanberger@gmail.com"
                        class="px-6 py-3 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors inline-flex items-center gap-2 whitespace-nowrap"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" class="flex-shrink-0" style="width: 11px; height: 11px;" viewBox="0 0 20 20" fill="currentColor">
                            <path d="M2.003 5.884L10 9.882l7.997-3.998A2 2 0 0016 4H4a2 2 0 00-1.997 1.884z" />
                            <path d="M18 8.118l-8 4-8-4V14a2 2 0 002 2h12a2 2 0 002-2V8.118z" />
                        </svg>
                        "Get In Touch"
                    </a>
                    <a
                        href="/Resume.pdf"
                        download="Berger_Matthew_Resume.pdf"
                        class="px-6 py-3 bg-green-500 text-white rounded-lg hover:bg-green-600 transition-colors inline-flex items-center gap-2 whitespace-nowrap"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" class="flex-shrink-0" style="width: 11px; height: 11px;" viewBox="0 0 20 20" fill="currentColor">
                            <path fill-rule="evenodd" d="M3 17a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm3.293-7.707a1 1 0 011.414 0L9 10.586V3a1 1 0 112 0v7.586l1.293-1.293a1 1 0 111.414 1.414l-3 3a1 1 0 01-1.414 0l-3-3a1 1 0 010-1.414z" clip-rule="evenodd" />
                        </svg>
                        "Download Resume"
                    </a>
                    <a
                        href="https://github.com/matthewjberger"
                        target="_blank"
                        class="px-6 py-3 border-2 border-blue-400 text-blue-400 rounded-lg hover:bg-gray-800 transition-colors inline-flex items-center gap-2 whitespace-nowrap"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" class="flex-shrink-0" style="width: 11px; height: 11px;" viewBox="0 0 24 24" fill="currentColor">
                            <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                        </svg>
                        "View GitHub"
                    </a>
                </div>
            </div>
        </section>
    }
}
