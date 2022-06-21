use crate::hn::top_stories::Msg::FetchFinished;
use crate::utils::pager::Paginator;
use reqwasm::http::Request;
use yew::prelude::*;

use super::item::*;

const URL: &str = "https://hacker-news.firebaseio.com/v0/topstories.json";

pub struct TopStories {
    fetch_url: String,
    page: u8,
    pub item_ids: Vec<u32>,
}

pub enum Msg {
    FetchFinished(Result<Vec<u32>, reqwasm::Error>),
    NextPage,
}

#[derive(Properties, PartialEq)]
pub struct TopStoriesProperties {
    #[prop_or_default]
    pub fetch_url: Option<String>,
    #[prop_or(1)]
    pub page: u8,
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
            page: ctx.props().page,
            item_ids: vec![],
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            FetchFinished(res) => {
                if let Ok(items) = res {
                    log::debug!("num of items {}", items.len());
                    self.item_ids = items;
                }
            }
            Msg::NextPage => {
                self.page += 1;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        log::debug!("view called");
        let link = ctx.link();
        let item_ids = &self.item_ids;
        let paginator = Paginator::new(10);
        let actual_page = paginator.get_page(item_ids, self.page as usize);
        log::debug!("actual page {}", self.page);
        html! {
            <div class="top_stories">
                {actual_page.iter().map(|item_id| {
                    log::debug!("loading {}", item_id);
                    html!{<Item key={item_id.to_string()} item_id={*item_id as i64} />}
                }).collect::<Html>()}
                <button onclick={link.callback(|_| Msg::NextPage)} type="button" class="btn btn-primary">{"load more..."}</button>
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
        });
    }
}
