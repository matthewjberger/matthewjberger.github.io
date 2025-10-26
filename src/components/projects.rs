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
            title: "Portfolio Website",
            description: "A modern portfolio website built with Rust and WebAssembly, featuring client-side rendering with Leptos and styled with Tailwind CSS.",
            technologies: vec!["Rust", "Leptos", "WASM", "Tailwind CSS"],
            link: "#",
        },
        Project {
            title: "Project Two",
            description: "A full-stack web application that demonstrates modern development practices and clean architecture principles.",
            technologies: vec!["TypeScript", "React", "Node.js"],
            link: "#",
        },
        Project {
            title: "Project Three",
            description: "An open-source tool that helps developers streamline their workflow and increase productivity.",
            technologies: vec!["Python", "Docker", "AWS"],
            link: "#",
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
