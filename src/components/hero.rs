use leptos::prelude::*;
use leptos_pdf::PdfRenderer;

#[component]
pub fn Hero() -> impl IntoView {
    let (show_pdf, set_show_pdf) = signal(false);
    let (scale, set_scale) = signal(1.8_f32);

    view! {
        <section id="hero" class="min-h-screen flex items-center justify-center bg-gradient-to-br from-gray-900 to-gray-800">
            <div class="text-center px-4">
                <img
                    src="/bio-photo.jpg"
                    alt="Matthew Berger"
                    class="rounded-lg mx-auto mb-6 border-2 border-blue-400 object-cover"
                    style="width: 180px; height: 180px;"
                />
                <h1 class="text-5xl md:text-7xl font-bold text-white mb-4">
                    "Hi, I'm " <span class="text-blue-400">"Matthew Berger"</span> " 🦀"
                </h1>
                <p class="text-xl md:text-2xl text-gray-300 mb-8">
                    "Founding Staff Software Engineer at Hyphen Robotics"
                </p>
                <div class="flex gap-4 justify-center flex-wrap">
                    <a
                        href="https://www.linkedin.com/in/matthewjberger/"
                        target="_blank"
                        class="px-6 py-3 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors inline-flex items-center gap-2 whitespace-nowrap"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" class="flex-shrink-0" style="width: 14px; height: 14px;" viewBox="0 0 24 24" fill="currentColor">
                            <path d="M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433c-1.144 0-2.063-.926-2.063-2.065 0-1.138.92-2.063 2.063-2.063 1.14 0 2.064.925 2.064 2.063 0 1.139-.925 2.065-2.064 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z"/>
                        </svg>
                        "View LinkedIn"
                    </a>
                    <button
                        on:click=move |_| set_show_pdf.set(true)
                        class="px-6 py-3 bg-green-500 text-white rounded-lg hover:bg-green-600 transition-colors inline-flex items-center gap-2 whitespace-nowrap"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" class="flex-shrink-0" style="width: 14px; height: 14px;" viewBox="0 0 20 20" fill="currentColor">
                            <path fill-rule="evenodd" d="M4 4a2 2 0 012-2h4.586A2 2 0 0112 2.586L15.414 6A2 2 0 0116 7.414V16a2 2 0 01-2 2H6a2 2 0 01-2-2V4z" clip-rule="evenodd"/>
                        </svg>
                        "View Resume"
                    </button>
                    <a
                        href="https://github.com/matthewjberger"
                        target="_blank"
                        class="px-6 py-3 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors inline-flex items-center gap-2 whitespace-nowrap"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" class="flex-shrink-0" style="width: 14px; height: 14px;" viewBox="0 0 24 24" fill="currentColor">
                            <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                        </svg>
                        "View GitHub"
                    </a>
                </div>
            </div>

            <Show when=move || show_pdf.get()>
                <div
                    class="fixed inset-0 bg-black bg-opacity-75 z-50 flex items-center justify-center p-4"
                    on:click=move |_| set_show_pdf.set(false)
                >
                    <div
                        class="bg-gray-800 rounded-lg w-full max-w-6xl flex flex-col"
                        style:height="90vh"
                        on:click=move |e| e.stop_propagation()
                    >
                        <div class="flex justify-between items-center p-4 border-b border-gray-700">
                            <a
                                href="/Resume.pdf"
                                download="Berger_Matthew_Resume.pdf"
                                class="px-4 py-2 bg-green-500 text-white rounded-lg hover:bg-green-600 transition-colors inline-flex items-center gap-2"
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" class="flex-shrink-0" style="width: 14px; height: 14px;" viewBox="0 0 20 20" fill="currentColor">
                                    <path fill-rule="evenodd" d="M3 17a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm3.293-7.707a1 1 0 011.414 0L9 10.586V3a1 1 0 112 0v7.586l1.293-1.293a1 1 0 111.414 1.414l-3 3a1 1 0 01-1.414 0l-3-3a1 1 0 010-1.414z" clip-rule="evenodd"/>
                                </svg>
                                "Download"
                            </a>
                            <div class="flex items-center gap-4">
                                <div class="flex items-center gap-2">
                                    <button
                                        on:click=move |_| set_scale.update(|s| *s = (*s - 0.1_f32).max(0.5))
                                        class="px-3 py-1 bg-gray-700 hover:bg-gray-600 rounded text-white"
                                    >
                                        "−"
                                    </button>
                                    <span class="text-gray-300">{move || format!("{:.0}%", scale.get() * 100.0)}</span>
                                    <button
                                        on:click=move |_| set_scale.update(|s| *s = (*s + 0.1_f32).min(3.0))
                                        class="px-3 py-1 bg-gray-700 hover:bg-gray-600 rounded text-white"
                                    >
                                        "+"
                                    </button>
                                </div>
                                <button
                                    on:click=move |_| set_show_pdf.set(false)
                                    class="text-gray-400 hover:text-gray-200 text-2xl font-bold"
                                >
                                    "×"
                                </button>
                            </div>
                        </div>
                        <div class="bg-gray-700 p-4 text-center" style:flex="1" style:width="100%" style:height="100%" style:overflow="auto">
                            <PdfRenderer
                                url="/Resume.pdf"
                                scale=scale
                                text=true
                            />
                        </div>
                    </div>
                </div>
            </Show>
        </section>
    }
}
