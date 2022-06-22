use yew::function_component;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ScoreProps {
    #[prop_or_default]
    pub value: Option<i32>,
}

#[function_component(Score)]
pub fn score(props: &ScoreProps) -> Html {
    props
        .value
        .map(|score_value| html!({ format!("{} points", score_value) }))
        .unwrap_or(html!())
}
