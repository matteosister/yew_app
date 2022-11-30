use yew::prelude::*;
use yew_router::prelude::*;

mod clock;
mod hn;
mod route;
mod utils;

use clock::Clock;
use route::{switch, Route};

enum Msg {}

struct App {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <div class="container-lg">
                    <div class="row">
                        <div class="col"><h1>{ "Hacker NYews" }</h1></div>
                        <div class="col"><Clock alignment={clock::Alignment::Right} /></div>
                    </div>
                    <hr />
                    <Switch<Route> render={switch} />
                </div>
            </BrowserRouter>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
