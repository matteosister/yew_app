use chrono::{DateTime, Duration, Local};
use gloo_timers::callback::Interval;
use yew::prelude::*;

pub struct Clock {
    now: DateTime<Local>,
}

pub enum ClockMessage {
    Tick,
}

#[derive(Properties, PartialEq)]
pub struct ClockProperties {
    #[prop_or_default]
    pub shift: u8,
}

impl Component for Clock {
    type Message = ClockMessage;
    type Properties = ClockProperties;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            now: get_datetime(ctx.props().shift),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ClockMessage::Tick => {
                self.now = get_datetime(ctx.props().shift);
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <p>{ self.now.format("%v %T") }</p>
            </>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }
        let link = ctx.link().clone();
        let interval = Interval::new(1000, move || link.send_message(Self::Message::Tick));

        interval.forget();
    }
}

fn get_datetime(shift: u8) -> DateTime<Local> {
    Local::now() + Duration::seconds(shift as i64)
}
