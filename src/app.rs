use yew::prelude::*;
use crate::components::{header::Header, hero::Hero, about::About, projects::Projects};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="min-h-screen flex flex-col bg-white dark:bg-gray-900 transition-colors duration-200">
            <Header />
            <main class="flex-grow">
                <Hero />
                <About />
                <Projects />
            </main>
        </div>
    }
}
