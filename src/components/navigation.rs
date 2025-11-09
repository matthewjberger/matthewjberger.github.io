use crate::ThemeContext;
use leptos::prelude::*;

#[component]
pub fn Navigation() -> impl IntoView {
    let (mobile_menu_open, set_mobile_menu_open) = signal(false);
    let theme_context = expect_context::<ThemeContext>();

    let nav_items = vec![
        ("About", "#about"),
        ("Highlights", "#highlights"),
        ("Experience", "#experience"),
        ("Crates", "#published-crates"),
        ("Projects", "#projects"),
        ("Education", "#education"),
    ];

    let articles_link = ("Articles", "https://matthewberger.dev/articles");
    let github_link = "https://github.com/matthewjberger/matthewjberger.github.io";

    view! {
        <nav class="fixed top-0 left-0 right-0 bg-white dark:bg-gray-900 shadow-md z-50 border-b border-gray-200 dark:border-gray-800">
            <div class="max-w-6xl mx-auto px-4">
                <div class="flex justify-between items-center h-16">
                    <a href="#hero" class="text-xl font-bold text-gray-900 dark:text-white">
                        "Matthew Berger"
                    </a>
                    <div class="flex items-center gap-4">
                        <div class="hidden md:flex space-x-8 items-center">
                            {nav_items.clone().into_iter().map(|(label, href)| {
                                view! {
                                    <a
                                        href=href
                                        class="text-gray-700 dark:text-gray-300 hover:text-blue-500 dark:hover:text-blue-400 transition-colors"
                                    >
                                        {label}
                                    </a>
                                }
                            }).collect_view()}
                            <a
                                href=articles_link.1
                                target="_blank"
                                class="px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors inline-flex items-center gap-2 whitespace-nowrap"
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" class="flex-shrink-0" style="width: 14px; height: 14px;" viewBox="0 0 20 20" fill="currentColor">
                                    <path d="M11 3a1 1 0 100 2h2.586l-6.293 6.293a1 1 0 101.414 1.414L15 6.414V9a1 1 0 102 0V4a1 1 0 00-1-1h-5z" />
                                    <path d="M5 5a2 2 0 00-2 2v8a2 2 0 002 2h8a2 2 0 002-2v-3a1 1 0 10-2 0v3H5V7h3a1 1 0 000-2H5z" />
                                </svg>
                                {articles_link.0}
                            </a>
                            <a
                                href="https://github.com/sponsors/matthewjberger"
                                target="_blank"
                                class="px-4 py-2 bg-pink-500 text-white rounded-lg hover:bg-pink-600 transition-colors inline-flex items-center gap-2 whitespace-nowrap"
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" class="flex-shrink-0" style="width: 14px; height: 14px;" viewBox="0 0 20 20" fill="currentColor">
                                    <path fill-rule="evenodd" d="M3.172 5.172a4 4 0 015.656 0L10 6.343l1.172-1.171a4 4 0 115.656 5.656L10 17.657l-6.828-6.829a4 4 0 010-5.656z" clip-rule="evenodd"/>
                                </svg>
                                "Sponsor"
                            </a>
                            <a
                                href=github_link
                                target="_blank"
                                class="text-gray-700 dark:text-gray-300 hover:text-blue-500 dark:hover:text-blue-400 transition-colors p-2"
                                aria-label="View this website on GitHub"
                                title="View this website on GitHub"
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" viewBox="0 0 24 24" fill="currentColor">
                                    <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                                </svg>
                            </a>
                        </div>
                        <button
                            on:click=move |_| theme_context.toggle_theme.update(|dark| *dark = !*dark)
                            class="text-gray-700 dark:text-gray-300 hover:text-blue-500 dark:hover:text-blue-400 transition-colors p-2"
                            aria-label="Toggle theme"
                        >
                            <Show
                                when=move || theme_context.is_dark.get()
                                fallback=move || view! {
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
                                    </svg>
                                }
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
                                </svg>
                            </Show>
                        </button>
                        <button
                            on:click=move |_| set_mobile_menu_open.update(|open| *open = !*open)
                            class="md:hidden text-gray-900 dark:text-white"
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
            <Show when=move || mobile_menu_open.get()>
                <div class="md:hidden bg-gray-100 dark:bg-gray-800 border-t border-gray-300 dark:border-gray-700">
                    <div class="px-4 py-2 space-y-2">
                        {nav_items.clone().into_iter().map(|(label, href)| {
                            view! {
                                <a
                                    href=href
                                    on:click=move |_| set_mobile_menu_open.set(false)
                                    class="block py-2 text-gray-700 dark:text-gray-300 hover:text-blue-500 dark:hover:text-blue-400 transition-colors"
                                >
                                    {label}
                                </a>
                            }
                        }).collect_view()}
                        <a
                            href=articles_link.1
                            target="_blank"
                            on:click=move |_| set_mobile_menu_open.set(false)
                            class="block py-2 text-blue-500 dark:text-blue-400 hover:text-blue-600 dark:hover:text-blue-300 transition-colors"
                        >
                            {articles_link.0}
                        </a>
                        <a
                            href="https://github.com/sponsors/matthewjberger"
                            target="_blank"
                            on:click=move |_| set_mobile_menu_open.set(false)
                            class="flex items-center gap-2 py-2 text-pink-500 dark:text-pink-400 hover:text-pink-600 dark:hover:text-pink-300 transition-colors"
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                                <path fill-rule="evenodd" d="M3.172 5.172a4 4 0 015.656 0L10 6.343l1.172-1.171a4 4 0 115.656 5.656L10 17.657l-6.828-6.829a4 4 0 010-5.656z" clip-rule="evenodd"/>
                            </svg>
                            <span>"Sponsor"</span>
                        </a>
                        <a
                            href=github_link
                            target="_blank"
                            on:click=move |_| set_mobile_menu_open.set(false)
                            class="flex items-center gap-2 py-2 text-gray-700 dark:text-gray-300 hover:text-blue-500 dark:hover:text-blue-400 transition-colors"
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 24 24" fill="currentColor">
                                <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                            </svg>
                            <span>"View on GitHub"</span>
                        </a>
                        <button
                            on:click=move |_| {
                                theme_context.toggle_theme.update(|dark| *dark = !*dark);
                                set_mobile_menu_open.set(false);
                            }
                            class="flex items-center gap-2 py-2 text-gray-700 dark:text-gray-300 hover:text-blue-500 dark:hover:text-blue-400 transition-colors w-full"
                        >
                            <Show
                                when=move || theme_context.is_dark.get()
                                fallback=move || view! {
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
                                    </svg>
                                    <span>"Dark Mode"</span>
                                }
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
                                </svg>
                                <span>"Light Mode"</span>
                            </Show>
                        </button>
                    </div>
                </div>
            </Show>
        </nav>
    }
}
