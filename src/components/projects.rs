use leptos::prelude::*;

#[derive(Clone)]
struct Project {
    title: &'static str,
    description: &'static str,
    technologies: Vec<&'static str>,
    link: &'static str,
}

#[component]
pub fn Projects() -> impl IntoView {
    let projects = vec![
        Project {
            title: "dragonglass",
            description: "A 3D graphics engine built with Rust and Vulkan, featuring PBR rendering, 3D object picking, and a visual editor. Demonstrates modern systems programming applied to interactive 3D visualization.",
            technologies: vec!["Rust", "Vulkan", "GLSL", "3D Graphics"],
            link: "https://github.com/matthewjberger/dragonglass",
        },
        Project {
            title: "freecs",
            description: "A high-performance Entity Component System library in ~1,350 lines of Rust. Features zero-cost abstractions, multi-threaded processing with Rayon, and no unsafe code.",
            technologies: vec!["Rust", "ECS", "Game Engine", "Performance"],
            link: "https://github.com/matthewjberger/freecs",
        },
        Project {
            title: "wgpu-example",
            description: "A minimal example of using Rust, wgpu, and egui without eframe. Cross-platform support for native and WebAssembly with WebGL and WebGPU backends.",
            technologies: vec!["Rust", "wgpu", "egui", "WebAssembly"],
            link: "https://github.com/matthewjberger/wgpu-example",
        },
        Project {
            title: "enum2egui",
            description: "A procedural macro for automatically generating egui UI code from Rust types. Supports structs, enums, nested types, and provides both read-only and mutable editing interfaces.",
            technologies: vec!["Rust", "Macros", "egui", "Code Generation"],
            link: "https://github.com/matthewjberger/enum2egui",
        },
        Project {
            title: "taps",
            description: "A Tokio async pub/sub message broker for in-process communication. Enables decoupled component communication with topic-based routing and scalable async architecture.",
            technologies: vec!["Rust", "Tokio", "Async", "Pub/Sub"],
            link: "https://github.com/matthewjberger/taps",
        },
        Project {
            title: "scoop-nerd-fonts",
            description: "PowerShell bucket for installing nerd fonts on Windows. Actively maintained with community engagement and 419 stars.",
            technologies: vec!["PowerShell", "Scoop", "Developer Tools"],
            link: "https://github.com/matthewjberger/scoop-nerd-fonts",
        },
    ];

    view! {
        <section id="projects" class="py-20 bg-gray-800">
            <div class="max-w-6xl mx-auto px-4">
                <h2 class="text-4xl font-bold text-center mb-12 text-white">
                    "Projects"
                </h2>
                <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
                    {projects.into_iter().map(|project| {
                        view! {
                            <div class="bg-gray-900 rounded-lg shadow-lg overflow-hidden hover:shadow-xl transition-shadow border border-gray-700">
                                <div class="p-6">
                                    <h3 class="text-xl font-bold mb-3 text-white">{project.title}</h3>
                                    <p class="text-gray-300 mb-4 leading-relaxed">{project.description}</p>
                                    <div class="flex flex-wrap gap-2 mb-4">
                                        {project.technologies.into_iter().map(|tech| {
                                            view! {
                                                <span class="px-3 py-1 bg-gray-800 text-gray-300 rounded text-sm border border-gray-700">
                                                    {tech}
                                                </span>
                                            }
                                        }).collect_view()}
                                    </div>
                                    <a
                                        href=project.link
                                        target="_blank"
                                        class="text-blue-400 hover:text-blue-300 font-medium"
                                    >
                                        "View Project →"
                                    </a>
                                </div>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>
    }
}
