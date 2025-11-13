use yew::prelude::*;
use crate::data::projects::Project;
use crate::components::project_card::ProjectCard;

#[function_component(Projects)]
pub fn projects() -> Html {
    let projects = Project::all();
    
    html! {
        <section class="py-20 px-4 bg-gradient-to-b from-gray-50 to-white dark:from-gray-800 dark:to-gray-900 transition-colors duration-200">
            <div class="max-w-7xl mx-auto">
                <div class="text-center mb-16">
                    <h2 class="text-4xl md:text-5xl font-bold mb-4 bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
                        {"My Projects"}
                    </h2>
                    <div class="w-24 h-1 bg-gradient-to-r from-blue-600 to-purple-600 mx-auto mb-4"></div>
                    <p class="text-lg text-gray-600 dark:text-gray-400 max-w-2xl mx-auto">
                        {"A collection of projects I've built using Rust, WebAssembly, and modern web technologies"}
                    </p>
                </div>
                
                <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
                    {for projects.iter().map(|project| html! {
                        <ProjectCard project={project.clone()} />
                    })}
                </div>
            </div>
        </section>
    }
}

