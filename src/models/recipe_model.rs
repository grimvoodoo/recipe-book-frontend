use serde::{Deserialize, Serialize};
use web_sys::MouseEvent;
use yew::{Callback, Properties};

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

#[derive(PartialEq, Properties)]
pub struct RecipeComponentProps {
    pub recipe: Recipe,
}

#[derive(Properties, PartialEq, Clone)]
pub struct RecipeListComponentProps {
    pub recipe: Recipe,
    pub on_click: Callback<MouseEvent>,
}

pub type RecipeList = Vec<Recipe>;
