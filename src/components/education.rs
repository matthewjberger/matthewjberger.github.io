use leptos::prelude::*;

#[derive(Clone)]
struct Degree {
    degree: &'static str,
    institution: &'static str,
    period: &'static str,
    description: &'static str,
}

#[component]
pub fn Education() -> impl IntoView {
    let degrees = vec![
        Degree {
            degree: "Bachelor of Science in Computer Science & Engineering",
            institution: "University of Nevada, Reno",
            period: "2017",
            description: "Minor in Mathematics. Comprehensive program covering software engineering, embedded systems, algorithms, and computer architecture.",
        },
    ];

    view! {
        <section id="education" class="py-20 bg-gray-800">
            <div class="max-w-4xl mx-auto px-4">
                <h2 class="text-4xl font-bold text-center mb-12 text-white">
                    "Education"
                </h2>
                <div class="space-y-8">
                    {degrees.into_iter().map(|degree| {
                        view! {
                            <div class="bg-gray-900 p-6 rounded-lg shadow-md border border-gray-700">
                                <h3 class="text-2xl font-bold text-white mb-2">{degree.degree}</h3>
                                <p class="text-lg text-blue-400 mb-1">{degree.institution}</p>
                                <p class="text-sm text-gray-400 mb-3">{degree.period}</p>
                                <p class="text-gray-300">{degree.description}</p>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>
    }
}
