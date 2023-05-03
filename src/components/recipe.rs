use yew::prelude::*;

use crate::models::recipe_model::RecipeComponentProps;

#[function_component(RecipeComponent)]
pub fn recipe_component(props: &RecipeComponentProps) -> Html {
    let RecipeComponentProps { recipe } = props;

    let ingredients_rows = recipe
        .ingredients
        .iter()
        .map(|ingredient| {
            html! {
                <tr>
                    <td>{ingredient.name.clone()}</td>
                    <td>{ingredient.quantity}</td>
                    <td>{ingredient.unit.clone().unwrap_or_else(|| "".to_string())}</td>
                </tr>
            }
        })
        .collect::<Html>();

    let instructions_list = recipe
        .instructions
        .iter()
        .map(|instruction| {
            html! {
                <li>{instruction.clone()}</li>
            }
        })
        .collect::<Html>();

    html! {
        <div class="recipe">
            <h2 class="recipe-name">{recipe.name.clone()}</h2>
            <div class="recipe-ingredients">
                <h3>{"Ingredients"}</h3>
                <table>
                    <thead>
                        <tr>
                            <th>{"Name"}</th>
                            <th>{"Qnt"}</th>
                            <th>{"Unit"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {ingredients_rows}
                    </tbody>
                </table>
            </div>
            <div class="recipe-instructions">
                <h3>{"Instructions"}</h3>
                <ol>
                    {instructions_list}
                </ol>
            </div>
        </div>
    }
}
