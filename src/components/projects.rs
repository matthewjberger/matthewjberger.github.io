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
            title: "dragonglass 🦀",
            description: "A 3D graphics engine built with Rust and Vulkan, featuring PBR rendering, 3D object picking, and a visual editor. Demonstrates modern systems programming applied to interactive 3D visualization.",
            technologies: vec!["Rust", "Vulkan", "GLSL", "3D Graphics"],
            link: "https://github.com/matthewjberger/dragonglass",
        },
        Project {
            title: "freecs 🦀",
            description: "A high-performance Entity Component System library in ~1,350 lines of Rust. Features zero-cost abstractions, multi-threaded processing with Rayon, and no unsafe code.",
            technologies: vec!["Rust", "ECS", "Game Engine", "Performance"],
            link: "https://github.com/matthewjberger/freecs",
        },
        Project {
            title: "wgpu-example 🦀",
            description: "A minimal example of using Rust, wgpu, and egui without eframe. Cross-platform support for native and WebAssembly with WebGL and WebGPU backends.",
            technologies: vec!["Rust", "wgpu", "egui", "WebAssembly"],
            link: "https://github.com/matthewjberger/wgpu-example",
        },
        Project {
            title: "nightshade-viewer 🦀",
            description: "A portable graphics engine written in Rust with cross-platform support for desktop, web, and server environments. Features ECS architecture, global transform management, and WebAssembly compilation.",
            technologies: vec!["Rust", "wgpu", "WGSL", "WebAssembly"],
            link: "https://matthewjberger.github.io/nightshade-viewer/",
        },
        Project {
            title: "frost 🦀",
            description: "A small interpreted programming language built in Rust with full unit tests. Features a REPL interface, recursive functions, conditional logic, and arithmetic operations.",
            technologies: vec!["Rust", "Interpreter", "Programming Language"],
            link: "https://github.com/matthewjberger/frost",
        },
        Project {
            title: "enum2egui 🦀",
            description: "A procedural macro for automatically generating egui UI code from Rust types. Supports structs, enums, nested types, and provides both read-only and mutable editing interfaces.",
            technologies: vec!["Rust", "Macros", "egui", "Code Generation"],
            link: "https://github.com/matthewjberger/enum2egui",
        },
        Project {
            title: "taps 🦀",
            description: "A Tokio async pub/sub message broker for in-process communication. Enables decoupled component communication with topic-based routing and scalable async architecture.",
            technologies: vec!["Rust", "Tokio", "Async", "Pub/Sub"],
            link: "https://github.com/matthewjberger/taps",
        },
        Project {
            title: "scoop-nerd-fonts",
            description: "PowerShell bucket for installing nerd fonts on Windows. Actively maintained with community engagement.",
            technologies: vec!["PowerShell", "Scoop", "Developer Tools"],
            link: "https://github.com/matthewjberger/scoop-nerd-fonts",
        },
        Project {
            title: "enum2contract 🦀",
            description: "A no_std compatible Rust derive macro for defining pub/sub messaging contracts using strongly typed enums. Generates topic strings, payload structs, and supports JSON and binary formats.",
            technologies: vec!["Rust", "Macros", "Pub/Sub", "Embedded"],
            link: "https://github.com/matthewjberger/enum2contract",
        },
        Project {
            title: "enum2repr 🦀",
            description: "A Rust derive macro that automates bidirectional conversion between enum variants and their numeric representations, eliminating boilerplate for manual type conversions.",
            technologies: vec!["Rust", "Macros", "Code Generation"],
            link: "https://github.com/matthewjberger/enum2repr",
        },
        Project {
            title: "enum2pos 🦀",
            description: "A Rust derive macro library that maps enum variants to their declaration position, enabling bidirectional transformation between variants and numeric indices.",
            technologies: vec!["Rust", "Macros", "Code Generation"],
            link: "https://github.com/matthewjberger/enum2pos",
        },
        Project {
            title: "enum2str 🦀",
            description: "A Rust derive macro that automatically implements the Display trait for enums with support for custom formatting, templates, and no_std compatibility.",
            technologies: vec!["Rust", "Macros", "Code Generation"],
            link: "https://github.com/matthewjberger/enum2str",
        },
        Project {
            title: "eager",
            description: "An Ogre3D 1.9 and C++ project template using CMake for quick setup. Features multi-platform support, optional Bullet Physics integration, and Vagrant development environment.",
            technologies: vec!["C++", "CMake", "Ogre3D", "3D Graphics"],
            link: "https://github.com/matthewjberger/eager",
        },
        Project {
            title: "superbible 🦀",
            description: "Rust implementations of OpenGL examples from the OpenGL Superbible 7th edition. Features advanced shader techniques, texture mapping, 3D transformations, and lighting models.",
            technologies: vec!["Rust", "OpenGL", "GLSL", "3D Graphics"],
            link: "https://github.com/matthewjberger/superbible",
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
                                        class="text-blue-400 hover:text-blue-300 font-medium inline-flex items-center gap-1 whitespace-nowrap"
                                    >
                                        "View Project"
                                        <svg xmlns="http://www.w3.org/2000/svg" class="flex-shrink-0" style="width: 14px; height: 14px;" viewBox="0 0 20 20" fill="currentColor">
                                            <path d="M11 3a1 1 0 100 2h2.586l-6.293 6.293a1 1 0 101.414 1.414L15 6.414V9a1 1 0 102 0V4a1 1 0 00-1-1h-5z" />
                                            <path d="M5 5a2 2 0 00-2 2v8a2 2 0 002 2h8a2 2 0 002-2v-3a1 1 0 10-2 0v3H5V7h3a1 1 0 000-2H5z" />
                                        </svg>
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
