use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum SortOrder {
    Alphabetical,
    ReverseAlphabetical,
}

#[derive(Clone)]
struct Project {
    title: &'static str,
    description: &'static str,
    technologies: Vec<&'static str>,
    link: &'static str,
}

#[component]
pub fn Projects() -> impl IntoView {
    let (sort_order, set_sort_order) = signal(SortOrder::Alphabetical);

    let projects = vec![
        Project {
            title: "eager",
            description: "An Ogre3D 1.9 and C++ project template using CMake for quick setup. Features multi-platform support, optional Bullet Physics integration, and Vagrant development environment.",
            technologies: vec!["C++", "Ogre3D", "CMake"],
            link: "https://github.com/matthewjberger/eager",
        },
        Project {
            title: "enum2contract 🦀",
            description: "A no_std compatible Rust derive macro for defining pub/sub messaging contracts using strongly typed enums. Generates topic strings, payload structs, and supports JSON and binary formats.",
            technologies: vec!["Rust", "Macros", "no_std"],
            link: "https://github.com/matthewjberger/enum2contract",
        },
        Project {
            title: "enum2egui 🦀",
            description: "A procedural macro for automatically generating egui UI code from Rust types. Supports structs, enums, nested types, and provides both read-only and mutable editing interfaces.",
            technologies: vec!["Rust", "Macros", "egui"],
            link: "https://github.com/matthewjberger/enum2egui",
        },
        Project {
            title: "enum2pos 🦀",
            description: "A Rust derive macro library that maps enum variants to their declaration position, enabling bidirectional transformation between variants and numeric indices.",
            technologies: vec!["Rust", "Macros"],
            link: "https://github.com/matthewjberger/enum2pos",
        },
        Project {
            title: "enum2repr 🦀",
            description: "A Rust derive macro that automates bidirectional conversion between enum variants and their numeric representations, eliminating boilerplate for manual type conversions.",
            technologies: vec!["Rust", "Macros"],
            link: "https://github.com/matthewjberger/enum2repr",
        },
        Project {
            title: "enum2str 🦀",
            description: "A Rust derive macro that automatically implements the Display trait for enums with support for custom formatting, templates, and no_std compatibility.",
            technologies: vec!["Rust", "Macros", "no_std"],
            link: "https://github.com/matthewjberger/enum2str",
        },
        Project {
            title: "frost 🦀",
            description: "A small interpreted programming language built in Rust with full unit tests. Features a REPL interface, recursive functions, conditional logic, and arithmetic operations.",
            technologies: vec!["Rust", "Interpreter", "REPL"],
            link: "https://github.com/matthewjberger/frost",
        },
        Project {
            title: "scoop-nerd-fonts",
            description: "PowerShell bucket for installing nerd fonts on Windows. Actively maintained with community engagement.",
            technologies: vec!["PowerShell", "Scoop", "Fonts"],
            link: "https://github.com/matthewjberger/scoop-nerd-fonts",
        },
        Project {
            title: "taps 🦀",
            description: "A Tokio async pub/sub message broker for in-process communication. Enables decoupled component communication with topic-based routing and scalable async architecture.",
            technologies: vec!["Rust", "Tokio", "Async", "Pub/Sub"],
            link: "https://github.com/matthewjberger/taps",
        },
        Project {
            title: "superbible 🦀",
            description: "Rust implementations of OpenGL examples from the OpenGL Superbible 7th edition. Features advanced shader techniques, texture mapping, 3D transformations, and lighting models.",
            technologies: vec!["Rust", "OpenGL", "GLSL"],
            link: "https://github.com/matthewjberger/superbible",
        },
        Project {
            title: "obsidian 🦀",
            description: "A modern game engine written in Rust with modular architecture and high-performance rendering capabilities.",
            technologies: vec!["Rust", "Game Engine", "ECS"],
            link: "https://github.com/matthewjberger/obsidian",
        },
        Project {
            title: "vulkan-example 🦀",
            description: "A minimal example of using Rust, Vulkan, and egui together without eframe. Features Vulkan 1.3 with dynamic rendering and GLSL shader compilation to SPIR-V.",
            technologies: vec!["Rust", "Vulkan", "egui", "GLSL"],
            link: "https://github.com/matthewjberger/vulkan-example",
        },
    ];

    let sorted_projects = move || {
        let mut projects_clone = projects.clone();
        match sort_order.get() {
            SortOrder::Alphabetical => {
                projects_clone.sort_by(|a, b| a.title.cmp(b.title));
            }
            SortOrder::ReverseAlphabetical => {
                projects_clone.sort_by(|a, b| b.title.cmp(a.title));
            }
        }
        projects_clone
    };

    view! {
        <section id="projects" class="py-20 bg-gray-100 dark:bg-gray-800">
            <div class="max-w-6xl mx-auto px-4">
                <div class="flex justify-between items-center mb-12">
                    <h2 class="text-4xl font-bold text-gray-900 dark:text-white">
                        "Projects"
                    </h2>
                    <div class="flex gap-2">
                        <button
                            on:click=move |_| set_sort_order.set(SortOrder::Alphabetical)
                            class=move || {
                                let base = "px-4 py-2 rounded-lg transition-colors";
                                if sort_order.get() == SortOrder::Alphabetical {
                                    format!("{} bg-blue-500 text-white", base)
                                } else {
                                    format!("{} bg-gray-200 dark:bg-gray-700 text-gray-900 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600", base)
                                }
                            }
                        >
                            "A-Z"
                        </button>
                        <button
                            on:click=move |_| set_sort_order.set(SortOrder::ReverseAlphabetical)
                            class=move || {
                                let base = "px-4 py-2 rounded-lg transition-colors";
                                if sort_order.get() == SortOrder::ReverseAlphabetical {
                                    format!("{} bg-blue-500 text-white", base)
                                } else {
                                    format!("{} bg-gray-200 dark:bg-gray-700 text-gray-900 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600", base)
                                }
                            }
                        >
                            "Z-A"
                        </button>
                    </div>
                </div>
                <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
                    {move || sorted_projects().into_iter().map(|project| {
                        view! {
                            <a
                                href=project.link
                                target="_blank"
                                class="block bg-white dark:bg-gray-900 rounded-lg shadow-lg overflow-hidden hover:shadow-xl transition-all border border-gray-300 dark:border-gray-700 hover:scale-105 cursor-pointer"
                            >
                                <div class="p-6">
                                    <h3 class="text-xl font-bold mb-3 text-gray-900 dark:text-white">{project.title}</h3>
                                    <p class="text-gray-700 dark:text-gray-300 mb-4 leading-relaxed">{project.description}</p>
                                    <div class="flex flex-wrap gap-2">
                                        {project.technologies.into_iter().map(|tech| {
                                            view! {
                                                <span class="px-3 py-1 bg-gray-100 dark:bg-gray-800 text-gray-900 dark:text-gray-300 rounded text-sm border border-gray-300 dark:border-gray-700">
                                                    {tech}
                                                </span>
                                            }
                                        }).collect_view()}
                                    </div>
                                </div>
                            </a>
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>
    }
}
