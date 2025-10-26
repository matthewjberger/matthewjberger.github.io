use leptos::prelude::*;
use leptos_pdf::PdfRenderer;

#[component]
pub fn Resume() -> impl IntoView {
    let (scale, set_scale) = signal(1.8_f32);

    view! {
        <section id="resume" class="py-20 bg-gray-800">
            <div class="max-w-6xl mx-auto px-4">
                <div class="flex justify-between items-center mb-8">
                    <h2 class="text-4xl font-bold text-white">
                        "Resume"
                    </h2>
                    <div class="flex items-center gap-4">
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
                    </div>
                </div>
                <div class="bg-gray-700 rounded-lg p-4" style="width: 100%; min-height: 120vh;">
                    <PdfRenderer
                        url="/Resume.pdf"
                        scale=scale
                        text=true
                    />
                </div>
            </div>
        </section>
    }
}
