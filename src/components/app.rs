use reqwasm::http::Request;
use yew::prelude::*;

use crate::{models::recipe_model::RecipeList, components::recipe::RecipeComponent};

#[function_component(App)]
pub fn app_component() -> Html {
    let recipe_list = Box::new(use_state(|| None));
    let error = Box::new(use_state(|| None));
    let retry = {
        let recipe_list = recipe_list.clone();
        let error = error.clone();
        Callback::from(move |_| {
            let recipe_list = recipe_list.clone();
            let error = error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let recipe_endpoint = "http://localhost:8000/recipes/true";
                let fetched_recipe_list = Request::get(&recipe_endpoint).send().await;

                match fetched_recipe_list {
                    Ok(response) => {
                        let json: Result<RecipeList, _> = response.json().await;
                        match json {
                            Ok(r) => {
                                recipe_list.set(Some(r));
                            }
                            Err(e) => error.set(Some(e.to_string())),
                        }
                    }
                    Err(e) => error.set(Some(e.to_string())),
                }
            });
        })
    };

    match (*recipe_list).as_ref() {
        Some(recipes) => recipes
            .iter()
            .map(|recipe| {
                html! {
                    <RecipeComponent recipe={recipe.clone()}/>
                }
            })
            .collect(),
        None => match (*error).as_ref() {
            Some(e) => {
                html! {
                    <>
                        {"error"} {e}
                        <button onclick={retry}>{"retry"}</button>
                    </>
                }
            }
            None => {
                html! {
                    <>
                        {"No data yet"}
                        <button onclick={retry}>{"Call API"}</button>
                    </>
                }
            }
        },
    }
}
