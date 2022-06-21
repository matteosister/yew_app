use yew::prelude::*;

mod clock;
mod hn;
mod utils;

use clock::Clock;
use hn::*;

enum Msg {}

struct Model {}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="container-lg">
                <div class="row">
                    <div class="col"><h1>{ "Hacker NYews" }</h1></div>
                    <div class="col"><Clock alignment={clock::Alignment::Right} /></div>
                </div>
                <hr />
                <TopStories />
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
