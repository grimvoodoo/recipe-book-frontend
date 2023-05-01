use yew::prelude::*;

use crate::models::recipe_model::RecipeListComponentProps;

#[function_component(RecipeListComponent)]
pub fn recipe_list_component(props: &RecipeListComponentProps) -> Html {
    let RecipeListComponentProps { recipe, on_click } = props;
    html! {
        <div class="recipe">
            <button class="name" onclick={on_click.clone()}>{recipe.name.to_owned()}</button>
        </div>
    }
}
