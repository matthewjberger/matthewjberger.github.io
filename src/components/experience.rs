use leptos::prelude::*;

#[derive(Clone)]
struct Job {
    title: &'static str,
    company: &'static str,
    period: &'static str,
    achievements: Vec<&'static str>,
}

#[component]
pub fn Experience() -> impl IntoView {
    let jobs = vec![
        Job {
            title: "Staff Software Engineer (Founding Engineer)",
            company: "Hyphen",
            period: "Sep 2022 - Present",
            achievements: vec![
                "Founded robotics platform as first engineer, achieving complete product delivery in 22 months securing $10M+ investment from Chipotle and Cava",
                "Built core async message broker handling 10K+ msgs/sec with zero message loss over 3 years and <1ms latency",
                "Transformed 8-person web team into proficient Rust systems programmers through mentorship and documentation",
                "Designed deterministic config system enabling scaling from 1 to 1000+ robots across enterprise deployments",
                "Engineered full stack from bare-metal firmware (RP2040/Embassy) to cloud infrastructure on custom Yocto Linux",
                "Created Hyphen Explorer used daily by all engineers and operators with 100% team adoption",
                "Prevented 10+ critical failures through predictive architecture (connection pooling, event-driven telemetry, config diffing)",
                "Published open-source Rust crates (enum2contract, enum2egui, enum2str) with 1000+ downloads",
                "Established CI/CD pipeline reducing deployment time from hours to <20 minutes",
            ],
        },
        Job {
            title: "Senior Software Engineer",
            company: "Hyphen",
            period: "Jul 2021 - Sep 2022",
            achievements: vec![
                "Led architectural pivot to embedded Rust, convincing C-suite to abandon $500K PLC investment",
                "Prototyped RP2040 firmware with Embassy-rs, proving <10μs control loop feasibility",
                "Built AWS infrastructure with Pulumi while transitioning company to new technical direction",
            ],
        },
        Job {
            title: "Software Engineer III",
            company: "Sierra Nevada Corporation",
            period: "May 2020 - Jul 2021",
            achievements: vec![
                "Developed aerospace imaging software processing 5GB/sec pixel data during flight operations",
                "Built Rust simulator for unavailable hardware, saving 3 months on project timeline",
            ],
        },
        Job {
            title: "Software Engineer",
            company: "Scientific Games",
            period: "Jul 2019 - May 2020",
            achievements: vec![
                "Resolved performance defects improving frame rates by 40% in Unity-based casino platform",
            ],
        },
        Job {
            title: "Software Engineer",
            company: "Hamilton Company",
            period: "Jan 2018 - Jul 2019",
            achievements: vec![
                "Built safety-critical software for liquid-handling medical robots achieving FDA compliance",
                "Reduced calibration development from 2 months to 2 weeks through reusable plugin framework",
                "Decreased environment setup from 8 hours to 5 minutes with automated bootstrapper",
                "Architected diagnostic application for largest OEM customer ($2M contract)",
            ],
        },
    ];

    view! {
        <section id="experience" class="py-20 bg-gray-900">
            <div class="max-w-4xl mx-auto px-4">
                <h2 class="text-4xl font-bold text-center mb-12 text-white">
                    "Experience"
                </h2>
                <div class="space-y-8">
                    {jobs.into_iter().map(|job| {
                        view! {
                            <div class="border-l-4 border-blue-500 pl-6">
                                <h3 class="text-2xl font-bold text-white">{job.title}</h3>
                                <p class="text-lg text-gray-300 mb-2">{job.company}</p>
                                <p class="text-sm text-gray-400 mb-4">{job.period}</p>
                                <ul class="space-y-2">
                                    {job.achievements.into_iter().map(|achievement| {
                                        view! {
                                            <li class="flex items-start">
                                                <span class="text-blue-400 mr-2">"•"</span>
                                                <span class="text-gray-300">{achievement}</span>
                                            </li>
                                        }
                                    }).collect_view()}
                                </ul>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>
    }
}
