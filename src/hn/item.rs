use reqwasm::http::Request;
use serde::Deserialize;
use yew::prelude::*;

#[derive(Deserialize)]
pub struct ItemData {
    by: String,
    title: String,
    url: String,
}

pub enum ItemValue {
    NotAsked,
    Loaded(ItemData),
}

pub struct Item {
    value: ItemValue,
}

pub enum Msg {
    FetchFinished(Result<Option<ItemData>, reqwasm::Error>),
}

#[derive(Properties, PartialEq)]
pub struct ItemProps {
    pub item_id: i64,
}

impl Component for Item {
    type Message = Msg;
    type Properties = ItemProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: ItemValue::NotAsked,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchFinished(res) => {
                if let Ok(Some(item_data)) = res {
                    self.value = ItemValue::Loaded(ItemData {
                        url: item_data.url,
                        by: item_data.by,
                        title: item_data.title,
                    });
                }
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            if let ItemValue::Loaded(item_data) = &self.value {
                <div><h4> <a href={item_data.url.to_string()}>{&item_data.title}</a></h4></div>
            } else {
                <div>{"loading..."}</div>
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }

        let link = ctx.link().clone();
        let url = format!(
            "https://hacker-news.firebaseio.com/v0/item/{}.json",
            ctx.props().item_id
        );
        wasm_bindgen_futures::spawn_local(async move {
            let item = Request::get(&url).send().await.unwrap().json().await;
            link.send_message(Msg::FetchFinished(item))
            //self.item_ids = fetched_top_stories;
        });
    }
}
