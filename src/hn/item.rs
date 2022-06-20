use serde::Deserialize;
use yew::prelude::*;

#[derive(Deserialize, Default)]
pub struct Item {
    loaded: bool,
    by: Option<String>,
    title: Option<String>,
}

#[derive(Properties, PartialEq)]
pub struct ItemProps {
    pub item_id: i64,
}

impl Component for Item {
    type Message = ();
    type Properties = ItemProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Item::default()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>{"loading..."}</div>
        }
    }
}
