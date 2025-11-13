use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    let skills = vec![
        ("Rust", "bg-orange-500"),
        ("WebAssembly", "bg-yellow-500"),
        ("Yew", "bg-green-500"),
        ("TypeScript", "bg-blue-500"),
        ("React", "bg-cyan-500"),
        ("Node.js", "bg-emerald-500"),
    ];

    html! {
        <section class="py-20 px-4 bg-white dark:bg-gray-800 transition-colors duration-200">
            <div class="max-w-6xl mx-auto">
                <div class="text-center mb-16">
                    <h2 class="text-4xl md:text-5xl font-bold mb-4 bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
                        {"About Me"}
                    </h2>
                    <div class="w-24 h-1 bg-gradient-to-r from-blue-600 to-purple-600 mx-auto"></div>
                </div>
                
                <div class="grid md:grid-cols-2 gap-12 items-center mb-16">
                    <div class="space-y-6">
                        <p class="text-lg text-gray-700 dark:text-gray-300 leading-relaxed">
                            {"I'm a passionate full-stack developer specializing in Rust, WebAssembly, and modern web technologies. I love building fast, reliable, and beautiful web experiences that push the boundaries of what's possible in the browser."}
                        </p>
                        <p class="text-lg text-gray-700 dark:text-gray-300 leading-relaxed">
                            {"With a strong foundation in systems programming and a keen eye for design, I create applications that are both performant and delightful to use. When I'm not coding, you can find me contributing to open-source projects or exploring the latest web technologies."}
                        </p>
                        <div class="pt-6">
                            <h3 class="text-2xl font-semibold mb-4 text-gray-800 dark:text-gray-200">{"What I Do"}</h3>
                            <ul class="space-y-3 text-gray-700 dark:text-gray-300">
                                <li class="flex items-start">
                                    <span class="text-blue-600 dark:text-blue-400 mr-2">{"▹"}</span>
                                    <span>{"Build high-performance web applications with Rust and WebAssembly"}</span>
                                </li>
                                <li class="flex items-start">
                                    <span class="text-blue-600 dark:text-blue-400 mr-2">{"▹"}</span>
                                    <span>{"Create beautiful, responsive user interfaces with modern CSS frameworks"}</span>
                                </li>
                                <li class="flex items-start">
                                    <span class="text-blue-600 dark:text-blue-400 mr-2">{"▹"}</span>
                                    <span>{"Design and implement scalable backend systems"}</span>
                                </li>
                                <li class="flex items-start">
                                    <span class="text-blue-600 dark:text-blue-400 mr-2">{"▹"}</span>
                                    <span>{"Contribute to open-source projects and share knowledge"}</span>
                                </li>
                            </ul>
                        </div>
                    </div>
                    <div class="bg-gradient-to-br from-blue-50 to-purple-50 dark:from-gray-700 dark:to-gray-600 p-8 rounded-2xl shadow-lg">
                        <h3 class="text-2xl font-semibold mb-6 text-gray-800 dark:text-gray-200">{"Skills & Technologies"}</h3>
                        <div class="flex flex-wrap gap-3">
                            {for skills.iter().map(|(skill, color)| html! {
                                <span class={format!("{} text-white px-4 py-2 rounded-full text-sm font-medium shadow-md", color)}>
                                    {*skill}
                                </span>
                            })}
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

