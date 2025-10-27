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
        ("Projects", "#projects"),
        ("Education", "#education"),
    ];

    let external_link = ("Articles", "https://matthewberger.dev/articles");

    view! {
        <nav class="fixed top-0 left-0 right-0 bg-white dark:bg-gray-900 shadow-md z-50 border-b border-gray-200 dark:border-gray-800">
            <div class="max-w-6xl mx-auto px-4">
                <div class="flex justify-between items-center h-16">
                    <a href="#hero" class="text-xl font-bold text-gray-900 dark:text-white">
                        "Matthew Berger - Portfolio"
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
                                href=external_link.1
                                target="_blank"
                                class="px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors inline-flex items-center gap-2 whitespace-nowrap"
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" class="flex-shrink-0" style="width: 14px; height: 14px;" viewBox="0 0 20 20" fill="currentColor">
                                    <path d="M11 3a1 1 0 100 2h2.586l-6.293 6.293a1 1 0 101.414 1.414L15 6.414V9a1 1 0 102 0V4a1 1 0 00-1-1h-5z" />
                                    <path d="M5 5a2 2 0 00-2 2v8a2 2 0 002 2h8a2 2 0 002-2v-3a1 1 0 10-2 0v3H5V7h3a1 1 0 000-2H5z" />
                                </svg>
                                {external_link.0}
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
                            href=external_link.1
                            target="_blank"
                            on:click=move |_| set_mobile_menu_open.set(false)
                            class="block py-2 text-blue-500 dark:text-blue-400 hover:text-blue-600 dark:hover:text-blue-300 transition-colors"
                        >
                            {external_link.0}
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
