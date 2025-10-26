use crate::components::about::About;
use crate::components::education::Education;
use crate::components::experience::Experience;
use crate::components::hero::Hero;
use crate::components::navigation::Navigation;
use crate::components::projects::Projects;
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="scroll-smooth">
            <Navigation />
            <Hero />
            <About />
            <Projects />
            <Experience />
            <Education />
        </div>
    }
}
