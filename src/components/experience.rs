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
            company: "Hyphen Robotics",
            period: "Sep 2022 - Present",
            achievements: vec![
                "Founded food assembly robotics controls suite as first engineer, driving technical pivot from PLC to Rust architecture",
                "Achieved complete product delivery in 22 months, securing $10M+ investment from Chipotle and Cava",
                "Engineered full technology stack from bare-metal firmware (RP2040/Embassy) to distributed robotics control system and cloud infrastructure, including custom Yocto Linux distribution",
                "Built core async message broker coordinating distributed processes on industrial PC with direct networking to embedded boards, capable of handling 1M+ msgs/sec with <1ms latency, zero message loss over 3 years of constant use",
                "Transformed team of web developers into proficient Rust systems programmers through hands-on mentorship, establishing architectural patterns, and pair programming",
                "Designed deterministic layered configuration system enabling unlimited fleet scaling",
                "Created Hyphen Explorer tool achieving 100% adoption across all software engineers, ops technicians, and culinary operators for real-time IPC visualization, debugging, and system control",
                "Published open-source Rust crates (enum2contract, enum2egui, enum2str, enum2pos, enum2repr) extending language capabilities for robotics",
            ],
        },
        Job {
            title: "Senior Software Engineer",
            company: "Hyphen Robotics",
            period: "Jul 2021 - Sep 2022",
            achievements: vec![
                "Hired to extend initial PLC-based system, identified scaling limitations that would prevent growth beyond prototype stage",
                "Led complete architectural pivot to embedded Rust, convincing leadership to abandon PLC approach for scalable alternative",
                "Pioneered RP2040 firmware with Embassy-rs, creating first Rust TMC5160 motor and AS5048A encoder drivers proving Rust could meet real-time control loop requirements",
                "Built initial AWS cloud infrastructure with Pulumi infrastructure and greengrass IoT",
                "Established foundation for Gen2 platform that would become company's core product and competitive advantage",
            ],
        },
        Job {
            title: "Software Engineer III",
            company: "Sierra Nevada Corporation",
            period: "May 2020 - Jul 2021",
            achievements: vec![
                "Developed robust aerospace imaging software capable of collecting and orthorectifying gigabytes of pixel data per second during a flight",
                "Built asynchronous Rust simulator for unavailable flight hardware, saving months on project timeline",
            ],
        },
        Job {
            title: "Software Engineer",
            company: "Scientific Games",
            period: "Jul 2019 - May 2020",
            achievements: vec![
                "Improved game engine stability by resolving critical performance and rendering defects in Unity-based casino-gaming platform",
            ],
        },
        Job {
            title: "Software Engineer",
            company: "Hamilton Company",
            period: "Jan 2018 - Jul 2019",
            achievements: vec![
                "Designed and built safety-critical software in a cross-disciplinary environment that calibrates and operates liquid-handling medical robots",
                "Reduced development time for new calibration routines from two months to less than two weeks by consolidating multiple applications into a single GUI and reusable-plugin framework",
                "Decreased development environment setup from a day to a few clicks by creating a bootstrapper that installs dependencies and configures settings on a Windows virtual machine",
                "Worked directly with Hamilton's largest OEM customer to architect an application that commands the firmware in their robots for rapidly diagnosing and resolving mechanical issues",
                "Mentored new hires and interns on team processes and coding practices",
            ],
        },
        Job {
            title: "Software Engineering Intern",
            company: "Hamilton Company",
            period: "Oct 2014 - Dec 2017",
            achievements: vec![
                "Decreased time spent quality testing robots by more than a week per robot by automating the process of gravimetric analysis",
                "Saved developers hours of time per day on common tasks by creating a suite of in-house tools",
                "Improved instrument sales by creating software adapters for both SiLA and non-SiLA compliant devices through collaboration with third-party device manufacturers",
            ],
        },
    ];

    let total_jobs = jobs.len();
    let (current_index, set_current_index) = signal(total_jobs - 1);
    let (show_all, set_show_all) = signal(true);
    let jobs_stored = StoredValue::new(jobs);

    view! {
        <section id="experience" class="py-20 bg-white dark:bg-gray-900">
            <div class="max-w-4xl mx-auto px-4">
                <div class="flex justify-between items-center mb-12">
                    <h2 class="text-4xl font-bold text-gray-900 dark:text-white">
                        "Experience"
                    </h2>
                    <button
                        on:click=move |_| set_show_all.update(|v| *v = !*v)
                        class="px-4 py-2 bg-gray-200 dark:bg-gray-700 text-gray-900 dark:text-white rounded-lg hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors"
                    >
                        {move || if show_all.get() { "Show Roles" } else { "Show Timeline" }}
                    </button>
                </div>
                <Show
                    when=move || show_all.get()
                    fallback=move || view! {
                        <div class="space-y-8">
                            <div class="flex justify-between items-center mb-8">
                                <button
                                    on:click=move |_| set_current_index.update(|i| *i = i.saturating_sub(1))
                                    class="px-4 py-2 bg-blue-500 dark:bg-blue-500 text-white rounded-lg hover:bg-blue-600 dark:hover:bg-blue-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                                    disabled=move || current_index.get() == 0
                                >
                                    "← Older"
                                </button>
                                <span class="text-gray-700 dark:text-gray-300">
                                    {move || format!("{} of {}", current_index.get() + 1, total_jobs)}
                                </span>
                                <button
                                    on:click=move |_| set_current_index.update(|i| *i = (*i + 1).min(total_jobs - 1))
                                    class="px-4 py-2 bg-blue-500 dark:bg-blue-500 text-white rounded-lg hover:bg-blue-600 dark:hover:bg-blue-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                                    disabled={move || current_index.get() >= total_jobs - 1}
                                >
                                    "Newer →"
                                </button>
                            </div>
                            <div style="min-height: 500px;">
                                {move || {
                                    let index = current_index.get();
                                    let reversed_index = total_jobs - 1 - index;
                                    jobs_stored.with_value(|jobs| {
                                        let job = &jobs[reversed_index];
                                        view! {
                                        <div class="border-l-4 border-blue-500 dark:border-blue-500 pl-6">
                                            <h3 class="text-2xl font-bold text-gray-900 dark:text-white">{job.title}</h3>
                                            <p class="text-lg text-gray-700 dark:text-gray-300 mb-2">{job.company}</p>
                                            <p class="text-sm text-gray-600 dark:text-gray-400 mb-4">{job.period}</p>
                                            <ul class="space-y-2">
                                                {job.achievements.iter().map(|achievement| {
                                                    view! {
                                                        <li class="flex items-start">
                                                            <span class="text-blue-500 dark:text-blue-400 mr-2">"•"</span>
                                                            <span class="text-gray-700 dark:text-gray-300">{*achievement}</span>
                                                        </li>
                                                    }
                                                }).collect_view()}
                                            </ul>
                                        </div>
                                    }
                                    })
                                }}
                            </div>
                        </div>
                    }
                >
                    <div class="space-y-8">
                        {jobs_stored.with_value(|jobs| {
                            jobs.iter().map(|job| {
                                view! {
                                    <div class="border-l-4 border-blue-500 dark:border-blue-500 pl-6">
                                        <h3 class="text-2xl font-bold text-gray-900 dark:text-white">{job.title}</h3>
                                        <p class="text-lg text-gray-700 dark:text-gray-300 mb-2">{job.company}</p>
                                        <p class="text-sm text-gray-600 dark:text-gray-400 mb-4">{job.period}</p>
                                        <ul class="space-y-2">
                                            {job.achievements.iter().map(|achievement| {
                                                view! {
                                                    <li class="flex items-start">
                                                        <span class="text-blue-500 dark:text-blue-400 mr-2">"•"</span>
                                                        <span class="text-gray-700 dark:text-gray-300">{*achievement}</span>
                                                    </li>
                                                }
                                            }).collect_view()}
                                        </ul>
                                    </div>
                                }
                            }).collect_view()
                        })}
                    </div>
                </Show>
            </div>
        </section>
    }
}
