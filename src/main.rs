mod components;
mod models;
use components::app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
