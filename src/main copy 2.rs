use js_sys::JsString;
use reqwasm::http::Request;
use web_sys::console;
use yew::prelude::*;

#[function_component(App)]
fn app_component() -> Html {
    wasm_bindgen_futures::spawn_local(async move {
        let recipes_endpoint = format!("http://localhost:8000/recipes/true");
        let fetched_recipes = Request::get(&recipes_endpoint)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        console::log_1(&JsString::from(fetched_recipes))
    });

    html!({ "hi!" })
}

fn main() {
    yew::start_app::<App>();
}
