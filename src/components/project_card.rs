use yew::prelude::*;
use crate::data::projects::Project;

#[derive(Properties, PartialEq)]
pub struct ProjectCardProps {
    pub project: Project,
}

#[function_component(ProjectCard)]
pub fn project_card(props: &ProjectCardProps) -> Html {
    let project = &props.project;
    
    html! {
        <div class="bg-white dark:bg-gray-800 rounded-xl shadow-lg hover:shadow-2xl transition-all duration-300 overflow-hidden group transform hover:-translate-y-2">
            {if let Some(ref image_url) = project.image_url {
                html! {
                    <div class="h-48 bg-gradient-to-br from-blue-400 to-purple-500 overflow-hidden">
                        <img src={image_url.clone()} alt={project.title.clone()} class="w-full h-full object-cover group-hover:scale-110 transition-transform duration-300" />
                    </div>
                }
            } else {
                html! {
                    <div class="h-48 bg-gradient-to-br from-blue-400 via-purple-500 to-pink-500 flex items-center justify-center">
                        <span class="text-6xl">{"🚀"}</span>
                    </div>
                }
            }}
            <div class="p-6">
                <h3 class="text-2xl font-bold mb-2 text-gray-800 dark:text-gray-200 group-hover:text-blue-600 dark:group-hover:text-blue-400 transition-colors">
                    {&project.title}
                </h3>
                <p class="text-gray-600 dark:text-gray-400 mb-4 line-clamp-2">
                    {&project.description}
                </p>
                <div class="flex flex-wrap gap-2 mb-4">
                    {for project.technologies.iter().map(|tech| html! {
                        <span class="px-3 py-1 bg-blue-100 dark:bg-blue-900 text-blue-700 dark:text-blue-300 text-xs rounded-full font-medium">
                            {tech}
                        </span>
                    })}
                </div>
                <div class="flex gap-3">
                    {if let Some(ref github_url) = project.github_url {
                        html! {
                            <a href={github_url.clone()} target="_blank" rel="noopener noreferrer"
                               class="flex-1 px-4 py-2 bg-gray-800 dark:bg-gray-700 text-white rounded-lg text-center hover:bg-gray-900 dark:hover:bg-gray-600 transition-colors font-medium">
                                {"GitHub"}
                            </a>
                        }
                    } else {
                        html! {}
                    }}
                    {if let Some(ref live_url) = project.live_url {
                        html! {
                            <a href={live_url.clone()} target="_blank" rel="noopener noreferrer"
                               class="flex-1 px-4 py-2 bg-gradient-to-r from-blue-600 to-purple-600 text-white rounded-lg text-center hover:from-blue-700 hover:to-purple-700 transition-colors font-medium">
                                {"Live Demo"}
                            </a>
                        }
                    } else {
                        html! {}
                    }}
                </div>
            </div>
        </div>
    }
}

