use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="bg-gray-900 dark:bg-black text-gray-300 dark:text-gray-500 py-8 mt-auto transition-colors duration-200">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex flex-col md:flex-row justify-between items-center">
                    <div class="mb-4 md:mb-0">
                        <p class="text-sm">{"© 2025 Jan Piekarz. Built with Rust, Yew & WebAssembly"}</p>
                    </div>
                    <div class="flex space-x-6">
                        <a href="https://github.com" target="_blank" rel="noopener noreferrer" 
                           class="hover:text-blue-400 dark:hover:text-blue-500 transition-colors">
                            <span class="sr-only">{"GitHub"}</span>
                            {"GitHub"}
                        </a>
                        <a href="https://linkedin.com" target="_blank" rel="noopener noreferrer"
                           class="hover:text-blue-400 dark:hover:text-blue-500 transition-colors">
                            <span class="sr-only">{"LinkedIn"}</span>
                            {"LinkedIn"}
                        </a>
                        <a href="mailto:you@example.com" 
                           class="hover:text-blue-400 dark:hover:text-blue-500 transition-colors">
                            {"Email"}
                        </a>
                    </div>
                </div>
            </div>
        </footer>
    }
}

