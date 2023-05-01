use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use std::fmt::format;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

async fn fetch_recipes() -> Result<Vec<Recipe>, anyhow::Error> {
    let request = Request::get("https://api.example.com/recipes")
        .header("Content-Type", "application/json")
        .build()?;
    let response = request.send().await?;
    let recipes: Vec<Recipe> = response.json().await?;
    Ok(recipes)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Ingredient {
    pub name: String,
    pub quantity: f32,
    pub unit: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Recipe {
    name: String,
    ingredients: Vec<Ingredient>,
    instructions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct RecipeList {
    link: ComponentLink<Self>,
    recipes: Vec<Recipe>,
    fetch_task: Option<FetchTask>,
    selected_recipe: Option<Recipe>,
}

struct RecipeDetails {
    recipe: Recipe,
}

enum Msg {
    FetchRecipes,
    FetchComplete(Result<Vec<Recipe>, anyhow::Error>),
    ShowRecipe(Recipe),
}

impl Component for RecipeList {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::FetchRecipes);
        Self {
            link,
            recipes: vec![],
            fetch_task: None,
            selected_recipe: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchRecipes => {
                let request = Request::get("/recipes.json")
                    .body(Nothing)
                    .expect("Failed to build request.");
                let callback = self.link.callback(
                    |response: Response<Json<Result<Cev<Recipe>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::FetchComplete(data)
                    },
                );
                let task = FetchService::fetch(request, callback).expect("Failed to start request");
                self.fetch_task = Some(task)
            }
            Msg::FetchComplete(Ok(recipes)) => {
                self.recipes = recipes;
                self.fetch_task = None;
            }
            Msg::FetchComplete(Err(_)) => {
                self.fetch_task = None;
            }
            Msg::ShowRecipe(recipe) => {
                self.selected_recipe = Some(recipe);
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{"Recipes"}</h1>
                <ul>
                    {for self.recipes.iter().map(|recipe| self.view_recipe_button(recipe))}
                </ul>
            </div>
        }
    }
}

impl Component for RecipeDetails {
    type Message = ();
    type Properties = Recipe;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { recipe: props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let ingredients = self.recipe.ingredients.iter().map(|ingredient| {
            let unit = ingredient.unit.clone().unwrap_or_else(|| "".to_string());
            html! {
                <li>{ format!("{} {} {}", ingredient.quantity, unit, ingredient.name)}</li>
            }
        });
        let instructions = self
            .recipe
            .instructions
            .iter()
            .enumerate()
            .map(|index, instruction| {
                html! {
                    <li>{format!("{}. {}", index + 1, instruction)}</li>
                }
            });
        html! {
            <div>
                <h2>{&self.recipe.name}</h2>
                <h3>{"Ingredients"}</h3>
                <ul>{for ingredients}</ul>
                <h3>{"Instructions"}</h3>
                <ol>{for instructions}</ol>
            </div>
        }
    }
}

impl RecipeList {
    fn view_recipe_button(&self, recipe: &Recipe) -> Html {
        let recipe_name = recipe.name.clone();
        let recipe_details = recipe.clone();
        html! {
            <div>
                <li>
                    <button onclick=self.link.callback(move |_| Msg::ShowRecipe(recipe_details.clone()))>
                    {recipe_name}
                    </button>
                </li>
                {if let Some(selected_recipe) = &self.selected_recipe {
                    html! {<RecipeDetails {selected_recipe.clone()} />}
                } else {
                        html! {}
                }}
            </div>
        }
    }
}

fn main() {
    yew::start_app::<RecipeList>();
}
