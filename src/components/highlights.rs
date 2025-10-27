use leptos::prelude::*;

#[derive(Clone)]
struct Highlight {
    title: &'static str,
    description: &'static str,
    link: &'static str,
    demo_link: Option<&'static str>,
    image: Option<&'static str>,
}

#[component]
pub fn Highlights() -> impl IntoView {
    let mut highlights = vec![
        Highlight {
            title: "dragonglass 🦀",
            description: "A 3D graphics engine built with Rust and Vulkan, featuring PBR rendering, 3D object picking, and a visual editor.",
            link: "https://github.com/matthewjberger/dragonglass",
            demo_link: None,
            image: Some("/dragonglass.png"),
        },
        Highlight {
            title: "nightshade 🦀",
            description: "Cross-platform GPU-driven graphics framework built with Rust and wgpu for modern rendering pipelines.",
            link: "https://github.com/matthewjberger/nightshade",
            demo_link: None,
            image: Some("/nightshade.png"),
        },
        Highlight {
            title: "nightshade-viewer 🦀",
            description: "A portable graphics engine written in Rust with cross-platform support for desktop, web, and server environments.",
            link: "https://github.com/matthewjberger/nightshade-viewer",
            demo_link: Some("https://matthewberger.dev/nightshade-viewer"),
            image: Some("/nightshade-viewer.png"),
        },
        Highlight {
            title: "wgpu-rendergraph 🦀",
            description: "Modern render graph implementation using wgpu with optimized resource management.",
            link: "https://github.com/matthewjberger/wgpu-rendergraph",
            demo_link: Some("https://matthewberger.dev/wgpu-rendergraph/"),
            image: Some("/rendergraph.png"),
        },
        Highlight {
            title: "freecs 🦀",
            description: "A high-performance Entity Component System library in ~1,350 lines of Rust. Features zero-cost abstractions, multi-threaded processing with Rayon, and no unsafe code.",
            link: "https://github.com/matthewjberger/freecs",
            demo_link: None,
            image: Some("/freecs.png"),
        },
        Highlight {
            title: "wgpu-example 🦀",
            description: "A minimal example of using Rust, wgpu, and egui without eframe. Cross-platform support for native and WebAssembly with WebGL and WebGPU backends.",
            link: "https://github.com/matthewjberger/wgpu-example",
            demo_link: Some("https://matthewberger.dev/wgpu-example/"),
            image: Some("/wgpu-example.png"),
        },
    ];

    highlights.sort_by(|a, b| a.title.cmp(b.title));

    view! {
        <section
            id="highlights"
            class="py-20 bg-gradient-to-br from-blue-50 to-indigo-50 dark:from-gray-900 dark:to-gray-800"
        >
            <div class="max-w-6xl mx-auto px-4">
                <div class="text-center mb-12">
                    <h2 class="text-4xl font-bold text-gray-900 dark:text-white">"Highlights"</h2>
                </div>
                <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
                    {highlights
                        .into_iter()
                        .map(|highlight| {
                            view! {
                                <a
                                    href=highlight.link
                                    target="_blank"
                                    class="block bg-white dark:bg-gray-900 rounded-lg shadow-lg overflow-hidden border border-gray-300 dark:border-gray-700 hover:shadow-xl transition-all hover:scale-105 cursor-pointer flex flex-col"
                                >
                                    {highlight
                                        .image
                                        .map(|img| {
                                            view! {
                                                <img
                                                    src=img
                                                    alt=highlight.title
                                                    class="w-full h-48 object-cover"
                                                />
                                            }
                                        })}
                                    <div class="p-6 flex-1">
                                        <h3 class="text-xl font-bold mb-3 text-gray-900 dark:text-white">
                                            {highlight.title}
                                        </h3>
                                        <p class="text-gray-700 dark:text-gray-300 leading-relaxed">
                                            {highlight.description}
                                        </p>
                                    </div>
                                    {highlight
                                        .demo_link
                                        .map(|demo| {
                                            view! {
                                                <div class="p-6 pt-0">
                                                    <a
                                                        href=demo
                                                        target="_blank"
                                                        on:click=move |e| e.stop_propagation()
                                                        class="block px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors text-center font-medium"
                                                    >
                                                        "Live Demo (Desktop Browser)"
                                                    </a>
                                                </div>
                                            }
                                        })}
                                </a>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </section>
    }
}
