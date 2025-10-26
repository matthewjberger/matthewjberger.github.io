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
                            class="px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors inline-flex items-center gap-2 whitespace-nowrap"
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" class="flex-shrink-0" style="width: 11px; height: 11px;" viewBox="0 0 20 20" fill="currentColor">
                                <path d="M11 3a1 1 0 100 2h2.586l-6.293 6.293a1 1 0 101.414 1.414L15 6.414V9a1 1 0 102 0V4a1 1 0 00-1-1h-5z" />
                                <path d="M5 5a2 2 0 00-2 2v8a2 2 0 002 2h8a2 2 0 002-2v-3a1 1 0 10-2 0v3H5V7h3a1 1 0 000-2H5z" />
                            </svg>
                            {external_link.0}
                        </a>
                    </div>
                </div>
            </div>
        </nav>
    }
}
