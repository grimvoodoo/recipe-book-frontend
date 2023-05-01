use yew::prelude::*;

use crate::models::recipe_model::RecipeComponentProps;

#[function_component(RecipeComponent)]
pub fn recipe_component(props: &RecipeComponentProps) -> Html {
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
