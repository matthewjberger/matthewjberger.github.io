use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    let skills = vec![
        "Rust",
        "C++",
        "TypeScript",
        "Python",
        "C#",
        "Distributed Systems",
        "Real-time Control",
        "Embedded Linux",
        "Message Brokers",
        "AWS",
        "Docker",
        "Kubernetes",
        "Yocto Linux",
        "GitHub Actions",
    ];

    view! {
        <section id="about" class="py-20 bg-gray-900">
            <div class="max-w-6xl mx-auto px-4">
                <h2 class="text-4xl font-bold text-center mb-12 text-white">
                    "About Me"
                </h2>
                <div class="grid md:grid-cols-2 gap-12">
                    <div>
                        <h3 class="text-2xl font-semibold mb-4 text-white">"Who I am"</h3>
                        <p class="text-gray-300 leading-relaxed mb-4">
                            "Founding engineer who architected and scaled robotics platform from concept to enterprise deployment.
                            5+ years production Rust expertise across embedded systems and distributed infrastructure."
                        </p>
                        <p class="text-gray-300 leading-relaxed">
                            "Transformed company technical direction from PLCs to modern architecture,
                            securing major enterprise investments from Chipotle and Cava."
                        </p>
                    </div>
                    <div>
                        <h3 class="text-2xl font-semibold mb-4 text-white">"Skills"</h3>
                        <div class="flex flex-wrap gap-2">
                            {skills.into_iter().map(|skill| {
                                view! {
                                    <span class="px-4 py-2 bg-blue-900 text-blue-200 rounded-full text-sm font-medium">
                                        {skill}
                                    </span>
                                }
                            }).collect_view()}
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
