use crate::hn::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/page/:num")]
    Page { num: u8 },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    log::debug!("routes {:?}", routes);
    match routes {
        Route::Home => html! {
            <TopStories />
        },
        Route::Page { num } => html! {
            <TopStories page={num}/>
        },
        Route::NotFound => html! {
            <h1>{ "404" }</h1>
        },
    }
}
