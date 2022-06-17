
use chrono::{DateTime, Duration, Local};
use gloo_timers::callback::Interval;
use yew::prelude::*;

pub struct Clock {
    now: DateTime<Local>,
}

pub enum ClockMessage {
    Tick,
}

#[derive(PartialEq)]
#[allow(dead_code)]
pub enum Alignment {
    Left,
    Right
}

#[derive(Properties, PartialEq)]
pub struct ClockProperties {
    #[prop_or_default]
    pub shift: u8,
    pub alignment: Alignment
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

    fn view(&self, ctx: &Context<Self>) -> Html {
        let alignment_class = match ctx.props().alignment {
            Alignment::Left => "text-start",
            Alignment::Right => "text-end",
        };
        html! {
            <div class={classes!("clock", alignment_class)}>{ self.now.format("%v %T") }</div>
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
