use yew::prelude::*;

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <section id="contact" class="py-20 px-4 bg-gradient-to-br from-blue-600 via-purple-600 to-pink-600 dark:from-blue-800 dark:via-purple-800 dark:to-pink-800 transition-colors duration-200">
            <div class="max-w-4xl mx-auto text-center">
                <h2 class="text-4xl md:text-5xl font-bold mb-6 text-white">
                    {"Let's Work Together"}
                </h2>
                <p class="text-xl text-blue-100 dark:text-blue-200 mb-12 max-w-2xl mx-auto">
                    {"I'm always interested in new opportunities and exciting projects. Whether you have a question or just want to say hi, feel free to reach out!"}
                </p>
                
                <div class="bg-white/10 dark:bg-white/5 backdrop-blur-lg rounded-2xl p-8 md:p-12 shadow-2xl">
                    <div class="space-y-6">
                        <div class="flex flex-col md:flex-row gap-6">
                            <a href="mailto:you@example.com" 
                               class="flex-1 bg-white text-blue-600 px-8 py-4 rounded-lg font-semibold hover:bg-blue-50 transition-all duration-200 transform hover:scale-105 shadow-lg">
                                <div class="flex items-center justify-center gap-2">
                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"></path>
                                    </svg>
                                    {"Send Email"}
                                </div>
                            </a>
                            <a href="https://github.com" target="_blank" rel="noopener noreferrer"
                               class="flex-1 bg-white/20 text-white border-2 border-white/30 px-8 py-4 rounded-lg font-semibold hover:bg-white/30 transition-all duration-200 transform hover:scale-105">
                                <div class="flex items-center justify-center gap-2">
                                    <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                                        <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                                    </svg>
                                    {"GitHub"}
                                </div>
                            </a>
                        </div>
                        
                        <div class="pt-8 border-t border-white/20 dark:border-white/10">
                            <p class="text-blue-100 dark:text-blue-200 mb-4">{"Or connect with me on:"}</p>
                            <div class="flex justify-center gap-6">
                                <a href="https://linkedin.com" target="_blank" rel="noopener noreferrer"
                                   class="text-white hover:text-blue-200 dark:hover:text-blue-300 transition-colors">
                                    <span class="text-lg font-medium">{"LinkedIn"}</span>
                                </a>
                                <a href="https://twitter.com" target="_blank" rel="noopener noreferrer"
                                   class="text-white hover:text-blue-200 dark:hover:text-blue-300 transition-colors">
                                    <span class="text-lg font-medium">{"Twitter"}</span>
                                </a>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

