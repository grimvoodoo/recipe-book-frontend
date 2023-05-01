use serde::{Deserialize, Serialize};
use yew::Properties;

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

pub type RecipeList = Vec<Recipe>;
