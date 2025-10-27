use leptos::prelude::*;

#[derive(Clone)]
struct Highlight {
    title: &'static str,
    description: &'static str,
    technologies: Vec<&'static str>,
    link: &'static str,
    demo_link: Option<&'static str>,
}

#[component]
pub fn Highlights() -> impl IntoView {
    let mut highlights = vec![
        Highlight {
            title: "dragonglass 🦀",
            description: "A 3D graphics engine built with Rust and Vulkan, featuring PBR rendering, 3D object picking, and a visual editor.",
            technologies: vec!["Rust", "Vulkan", "GLSL", "3D Graphics"],
            link: "https://github.com/matthewjberger/dragonglass",
            demo_link: None,
        },
        Highlight {
            title: "nightshade 🦀",
            description: "Cross-platform GPU-driven graphics framework built with Rust and wgpu for modern rendering pipelines.",
            technologies: vec!["Rust", "wgpu", "WGSL", "Graphics"],
            link: "https://github.com/matthewjberger/nightshade",
            demo_link: None,
        },
        Highlight {
            title: "nightshade-viewer 🦀",
            description: "A portable graphics engine written in Rust with cross-platform support for desktop, web, and server environments.",
            technologies: vec!["Rust", "wgpu", "WGSL", "WebAssembly"],
            link: "https://github.com/matthewjberger/nightshade-viewer",
            demo_link: Some("https://matthewberger.dev/nightshade-viewer"),
        },
        Highlight {
            title: "wgpu-example 🦀",
            description: "A minimal vertical slice of wgpu boilerplate. Cross-platform support for native and WebAssembly with WebGL and WebGPU backends.",
            technologies: vec!["Rust", "wgpu", "egui", "WebAssembly"],
            link: "https://github.com/matthewjberger/wgpu-example",
            demo_link: Some("https://matthewberger.dev/wgpu-example/"),
        },
        Highlight {
            title: "wgpu-rendergraph 🦀",
            description: "Modern render graph implementation using wgpu with optimized resource management and parallel execution.",
            technologies: vec!["Rust", "wgpu", "Render Graph", "Graphics"],
            link: "https://github.com/matthewjberger/wgpu-rendergraph",
            demo_link: Some("https://matthewberger.dev/wgpu-rendergraph/"),
        },
        Highlight {
            title: "vulkan-example 🦀",
            description: "A minimal vertical slice of Vulkan boilerplate with Rust, featuring advanced techniques like multidraw indirect and mesh/task shaders.",
            technologies: vec!["Rust", "Vulkan", "egui", "GLSL"],
            link: "https://github.com/matthewjberger/vulkan-example",
            demo_link: None,
        },
        Highlight {
            title: "freecs 🦀",
            description: "A high-performance Entity Component System library in ~1,350 lines of Rust. Features zero-cost abstractions, multi-threaded processing with Rayon, and no unsafe code.",
            technologies: vec!["Rust", "ECS", "Game Engine", "Performance"],
            link: "https://github.com/matthewjberger/freecs",
            demo_link: None,
        },
        Highlight {
            title: "superbible 🦀",
            description: "Rust implementations of OpenGL examples from the OpenGL Superbible 7th edition. Features advanced shader techniques, texture mapping, 3D transformations, and lighting models.",
            technologies: vec!["Rust", "OpenGL", "GLSL", "3D Graphics"],
            link: "https://github.com/matthewjberger/superbible",
            demo_link: None,
        },
        Highlight {
            title: "obsidian 🦀",
            description: "A modern game engine written in Rust with modular architecture and high-performance rendering capabilities.",
            technologies: vec!["Rust", "Game Engine", "Graphics", "ECS"],
            link: "https://github.com/matthewjberger/obsidian",
            demo_link: None,
        },
    ];

    highlights.sort_by(|a, b| a.title.cmp(b.title));

    view! {
        <section id="highlights" class="py-20 bg-gradient-to-br from-blue-50 to-indigo-50 dark:from-gray-900 dark:to-gray-800">
            <div class="max-w-6xl mx-auto px-4">
                <div class="text-center mb-12">
                    <h2 class="text-4xl font-bold text-gray-900 dark:text-white mb-4">
                        "Highlights"
                    </h2>
                    <p class="text-lg text-gray-700 dark:text-gray-300 max-w-3xl mx-auto">
                        "Featured projects in graphics programming and game engine development, showcasing cross-platform GPU-driven work with Vulkan and wgpu"
                    </p>
                </div>
                <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
                    {highlights.into_iter().map(|highlight| {
                        view! {
                            <div class="bg-white dark:bg-gray-900 rounded-lg shadow-lg overflow-hidden border border-gray-300 dark:border-gray-700 flex flex-col">
                                <div class="p-6 flex-1">
                                    <h3 class="text-xl font-bold mb-3 text-gray-900 dark:text-white">{highlight.title}</h3>
                                    <p class="text-gray-700 dark:text-gray-300 mb-4 leading-relaxed">{highlight.description}</p>
                                    <div class="flex flex-wrap gap-2 mb-4">
                                        {highlight.technologies.into_iter().map(|tech| {
                                            view! {
                                                <span class="px-3 py-1 bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 rounded text-sm border border-blue-200 dark:border-blue-800">
                                                    {tech}
                                                </span>
                                            }
                                        }).collect_view()}
                                    </div>
                                </div>
                                <div class="p-6 pt-0 flex gap-3">
                                    <a
                                        href=highlight.link
                                        target="_blank"
                                        class="flex-1 px-4 py-2 bg-gray-200 dark:bg-gray-700 text-gray-900 dark:text-white rounded-lg hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors text-center font-medium"
                                    >
                                        "View Code"
                                    </a>
                                    {highlight.demo_link.map(|demo| {
                                        view! {
                                            <a
                                                href=demo
                                                target="_blank"
                                                class="flex-1 px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors text-center font-medium"
                                            >
                                                "Live Demo"
                                            </a>
                                        }
                                    })}
                                </div>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>
    }
}
