use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{hero::Hero, about::About, projects::Projects};

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/projects")]
    Projects,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Hero /> },
        Route::About => html! { <About /> },
        Route::Projects => html! { <Projects /> },
        Route::NotFound => html! {
            <div class="min-h-screen flex items-center justify-center bg-gray-50 dark:bg-gray-900 transition-colors duration-200">
                <div class="text-center">
                    <h1 class="text-6xl font-bold text-gray-800 dark:text-gray-200 mb-4">{"404"}</h1>
                    <p class="text-xl text-gray-600 dark:text-gray-400">{"Page not found"}</p>
                </div>
            </div>
        },
    }
}

