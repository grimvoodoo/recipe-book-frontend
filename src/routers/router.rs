use crate::components::app::App;
use web_sys::window;
use yew::prelude::*;

pub fn switch(route: &str) -> Html {
    match route {
        "/" => {
            html! { <App /> }
        }
        _ => {
            html! { <div> {"404 Not Found"} </div> }
        }
    }
}

pub struct Router;

impl Component for Router {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Router
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let route = window().unwrap().location().pathname().unwrap();
        switch(&route)
    }
}
