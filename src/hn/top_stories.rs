use crate::hn::top_stories::Msg::FetchFinished;
use reqwasm::http::Request;
use serde::Deserialize;
use yew::prelude::*;

use super::item::*;

const URL: &str = "https://hacker-news.firebaseio.com/v0/topstories.json";

pub struct TopStories {
    fetch_url: String,
    pub item_ids: Vec<u32>,
}

pub enum Msg {
    FetchFinished(Result<Vec<u32>, reqwasm::Error>),
}

#[derive(Properties, PartialEq)]
pub struct TopStoriesProperties {
    #[prop_or_default]
    pub fetch_url: Option<String>,
}

impl Component for TopStories {
    type Message = Msg;
    type Properties = TopStoriesProperties;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            fetch_url: ctx
                .props()
                .fetch_url
                .as_ref()
                .unwrap_or(&URL.to_string())
                .to_string(),
            item_ids: vec![],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            FetchFinished(res) => {
                if let Ok(items) = res {
                    self.item_ids = items;
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let item_ids = &self.item_ids;
        html! {
            <div>
                {
                    item_ids.into_iter().take(10).map(|item_id| {
                        html!{<Item item_id={*item_id as i64} />}
                    }).collect::<Html>()
                }
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }

        let url = self.fetch_url.clone();
        let link = ctx.link().clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_top_stories: Result<Vec<u32>, reqwasm::Error> =
                Request::get(&url).send().await.unwrap().json().await;
            link.send_message(FetchFinished(fetched_top_stories))
            //self.item_ids = fetched_top_stories;
        });

        //ctx.link().send_message(Msg::FetchFinished(top_stories));
    }
}
