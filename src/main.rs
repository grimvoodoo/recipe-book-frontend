mod components;
mod models;
mod routers;
use routers::router::Router;

pub fn main() {
    yew::Renderer::<Router>::new().render();
}
