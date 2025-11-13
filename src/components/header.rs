use yew::prelude::*;
use crate::contexts::theme::{use_theme, Theme};

#[function_component(Header)]
pub fn header() -> Html {
    let (theme, toggle_theme) = use_theme();
    
    let nav_link_classes = || -> Classes {
        classes!("px-4", "py-2", "rounded-lg", "transition-colors", "duration-200", "font-medium", "text-gray-700", "dark:text-gray-300", "hover:bg-blue-50", "dark:hover:bg-gray-700", "hover:text-blue-600", "dark:hover:text-blue-400")
    };

    html! {
        <nav class="sticky top-0 z-50 bg-white/95 dark:bg-gray-900/95 backdrop-blur-sm shadow-md border-b border-gray-200 dark:border-gray-700">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex justify-between items-center h-16">
                    <a href="#home" class="signature-text text-2xl md:text-3xl font-signature bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent hover:from-blue-700 hover:to-purple-700 transition-all scroll-smooth">
                        {"Jan Piekarz"}
                    </a>
                    <div class="flex items-center space-x-4">
                        <button 
                            onclick={move |_| toggle_theme.emit(Callback::noop())}
                            class="p-2 rounded-lg bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors"
                            aria-label="Toggle theme"
                        >
                            {if *theme == Theme::Dark {
                                html! {
                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"></path>
                                    </svg>
                                }
                            } else {
                                html! {
                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"></path>
                                    </svg>
                                }
                            }}
                        </button>
                        <div class="hidden md:flex items-center space-x-2">
                            <a href="#about" class={nav_link_classes()}>
                                {"About"}
                            </a>
                            <a href="#projects" class={nav_link_classes()}>
                                {"Projects"}
                            </a>
                        </div>
                        
                    </div>
                </div>
            </div>
        </nav>
    }
}

