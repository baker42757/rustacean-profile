mod app;
mod router;
mod components;
mod data;
mod contexts;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}