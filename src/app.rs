use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::{Route, switch};
use crate::components::header::Header;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="min-h-screen flex flex-col bg-white dark:bg-gray-900 transition-colors duration-200">
                <Header />
                <main class="flex-grow">
                    <Switch<Route> render={switch} />
                </main>
            </div>
        </BrowserRouter>
    }
}
