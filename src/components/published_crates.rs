use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum SortOrder {
    Alphabetical,
    ReverseAlphabetical,
}

#[derive(Clone)]
struct Crate {
    title: &'static str,
    description: &'static str,
    technologies: Vec<&'static str>,
    link: &'static str,
}

#[component]
pub fn PublishedCrates() -> impl IntoView {
    let (sort_order, set_sort_order) = signal(SortOrder::Alphabetical);

    let crates = vec![
        Crate {
            title: "enum2contract 🦀",
            description: "A no_std compatible Rust derive macro for defining pub/sub messaging contracts using strongly typed enums. Generates topic strings, payload structs, and supports JSON and binary formats.",
            technologies: vec!["Rust", "Macros", "no_std"],
            link: "https://github.com/matthewjberger/enum2contract",
        },
        Crate {
            title: "enum2egui 🦀",
            description: "A procedural macro for automatically generating egui UI code from Rust types. Supports structs, enums, nested types, and provides both read-only and mutable editing interfaces.",
            technologies: vec!["Rust", "Macros", "egui"],
            link: "https://github.com/matthewjberger/enum2egui",
        },
        Crate {
            title: "enum2pos 🦀",
            description: "A Rust derive macro library that maps enum variants to their declaration position, enabling bidirectional transformation between variants and numeric indices.",
            technologies: vec!["Rust", "Macros"],
            link: "https://github.com/matthewjberger/enum2pos",
        },
        Crate {
            title: "enum2repr 🦀",
            description: "A Rust derive macro that automates bidirectional conversion between enum variants and their numeric representations, eliminating boilerplate for manual type conversions.",
            technologies: vec!["Rust", "Macros"],
            link: "https://github.com/matthewjberger/enum2repr",
        },
        Crate {
            title: "enum2str 🦀",
            description: "A Rust derive macro that automatically implements the Display trait for enums with support for custom formatting, templates, and no_std compatibility.",
            technologies: vec!["Rust", "Macros", "no_std"],
            link: "https://github.com/matthewjberger/enum2str",
        },
        Crate {
            title: "freecs 🦀",
            description: "A high-performance Entity Component System library in ~1,350 lines of Rust. Features zero-cost abstractions, multi-threaded processing with Rayon, and no unsafe code.",
            technologies: vec!["Rust", "ECS"],
            link: "https://github.com/matthewjberger/freecs",
        },
        Crate {
            title: "stateless 🦀",
            description: "A lightweight, zero-cost state machine library that separates structure from behavior. Guards and actions live in wrapper code, not the DSL, enabling zero coupling and complete flexibility.",
            technologies: vec!["Rust", "Macros", "State Machine", "no_std"],
            link: "https://github.com/matthewjberger/stateless",
        },
        Crate {
            title: "taps 🦀",
            description: "A Tokio async pub/sub message broker for in-process communication. Enables decoupled component communication with topic-based routing and scalable async architecture.",
            technologies: vec!["Rust", "Tokio", "Async", "Pub/Sub"],
            link: "https://github.com/matthewjberger/taps",
        },
    ];

    let sorted_crates = move || {
        let mut crates_clone = crates.clone();
        match sort_order.get() {
            SortOrder::Alphabetical => {
                crates_clone.sort_by(|a, b| a.title.cmp(b.title));
            }
            SortOrder::ReverseAlphabetical => {
                crates_clone.sort_by(|a, b| b.title.cmp(a.title));
            }
        }
        crates_clone
    };

    view! {
        <section id="published-crates" class="py-20 bg-gray-100 dark:bg-gray-800">
            <div class="max-w-6xl mx-auto px-4">
                <div class="flex justify-between items-center mb-12">
                    <h2 class="text-4xl font-bold text-gray-900 dark:text-white">
                        "Published Crates"
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
                    {move || sorted_crates().into_iter().map(|crate_item| {
                        view! {
                            <a
                                href=crate_item.link
                                target="_blank"
                                class="block bg-white dark:bg-gray-900 rounded-lg shadow-lg overflow-hidden hover:shadow-xl transition-all border border-gray-300 dark:border-gray-700 hover:scale-105 cursor-pointer"
                            >
                                <div class="p-6">
                                    <h3 class="text-xl font-bold mb-3 text-gray-900 dark:text-white">{crate_item.title}</h3>
                                    <p class="text-gray-700 dark:text-gray-300 mb-4 leading-relaxed">{crate_item.description}</p>
                                    <div class="flex flex-wrap gap-2">
                                        {crate_item.technologies.into_iter().map(|tech| {
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
