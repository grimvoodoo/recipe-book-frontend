use reqwasm::http::Request;
use yew::prelude::*;

use crate::{
    components::{recipe::RecipeComponent, recipe_list::RecipeListComponent},
    models::recipe_model::RecipeList,
};

#[function_component(App)]
pub fn app_component() -> Html {
    let recipe_list = Box::new(use_state(|| None));
    let error = Box::new(use_state(|| None));
    let selected_recipe = Box::new(use_state(|| None));
    let retry = {
        let recipe_list = recipe_list.clone();
        let error = error.clone();
        Callback::from(move |_| {
            let recipe_list = recipe_list.clone();
            let error = error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let recipe_list_endpoint = "http://localhost:8000/recipes/true";
                let fetched_recipe_list = Request::get(&recipe_list_endpoint).send().await;

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

    html! {
        <>
            <div class="recipe-list">
                {match (*recipe_list).as_ref() {
                    Some(recipes) => recipes
                        .iter()
                        .map(|recipe| {
                            let on_click = {
                                let recipe = recipe.clone();
                                let selected_recipe = selected_recipe.clone();
                                Callback::from(move |_| {
                                    selected_recipe.set(Some(recipe.clone()));
                                })
                            };
                            html! {
                                <RecipeListComponent recipe={recipe.clone()} on_click={on_click} />
                            }
                        })
                        .collect::<Html>(),
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
                }}
            </div>
            <div class="selected-recipe">
                {match (*selected_recipe).as_ref() {
                    Some(recipe) => html! { <RecipeComponent recipe={recipe.clone()} /> },
                    None => html! {},
                }}
            </div>
        </>
    }
}
