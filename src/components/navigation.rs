use leptos::prelude::*;

#[component]
pub fn Navigation() -> impl IntoView {
    let nav_items = vec![
        ("About", "#about"),
        ("Experience", "#experience"),
        ("Projects", "#projects"),
        ("Education", "#education"),
    ];

    let external_link = ("Articles", "https://matthewberger.dev/articles");

    view! {
        <nav class="fixed top-0 left-0 right-0 bg-gray-900 shadow-md z-50 border-b border-gray-800">
            <div class="max-w-6xl mx-auto px-4">
                <div class="flex justify-between items-center h-16">
                    <a href="#hero" class="text-xl font-bold text-white">
                        "Portfolio"
                    </a>
                    <div class="hidden md:flex space-x-8 items-center">
                        {nav_items.into_iter().map(|(label, href)| {
                            view! {
                                <a
                                    href=href
                                    class="text-gray-300 hover:text-blue-400 transition-colors"
                                >
                                    {label}
                                </a>
                            }
                        }).collect_view()}
                        <a
                            href=external_link.1
                            target="_blank"
                            class="px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors"
                        >
                            {external_link.0}
                        </a>
                    </div>
                </div>
            </div>
        </nav>
    }
}
