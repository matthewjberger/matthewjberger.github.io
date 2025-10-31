use crate::components::about::About;
use crate::components::education::Education;
use crate::components::experience::Experience;
use crate::components::hero::Hero;
use crate::components::highlights::Highlights;
use crate::components::navigation::Navigation;
use crate::components::projects::Projects;
use crate::components::published_crates::PublishedCrates;
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="scroll-smooth">
            <Navigation />
            <Hero />
            <About />
            <Highlights />
            <Experience />
            <PublishedCrates />
            <Projects />
            <Education />
        </div>
    }
}
