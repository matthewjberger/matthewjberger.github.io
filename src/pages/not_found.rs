use leptos::prelude::*;

/// 404 Not Found Page
#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center min-h-screen">
            <h1 class="text-6xl font-bold text-center mb-4">"404: Page not found"</h1>
            <h2 class="text-2xl text-center text-gray-600">"We couldn't find that page!"</h2>
        </div>
    }
}
