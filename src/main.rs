use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Properties)]
pub struct Ingredient {
    pub name: String,
    pub quantity: f32,
    pub unit: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Properties)]
pub struct Recipe {
    pub name: String,
    pub ingredients: Vec<Ingredient>,
    pub instructions: Vec<String>,
}

type RecipeList = Vec<Recipe>;

#[derive(PartialEq, Properties)]
struct RecipeComponentProps {
    pub recipe: Recipe,
}

#[function_component(RecipeComponent)]
fn recipe_component(props: &RecipeComponentProps) -> Html {
    let RecipeComponentProps { recipe } = props;
    let nothing = "".to_string();
    html! {
        <div class="recipe">
            <div class="name">{recipe.name.to_owned()}</div>
            <div class="ingredients">
                {
                    recipe.ingredients.iter().map(|ingredient| {
                        let unit = ingredient.unit.as_ref().unwrap_or(&nothing);
                        html! {<div>{format!("{} {}{}", ingredient.quantity, ingredient.name, unit)}</div>}
                    }).collect::<Html>()
                }
            </div>
            <div class="instructions">
                {
                    recipe.instructions.iter().map(|instruction| {
                        html! {<div>{instruction.to_owned()}</div>}
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}

#[function_component(App)]
fn app_component() -> Html {
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

fn main() {
    yew::Renderer::<App>::new().render();
}
