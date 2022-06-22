use yew::function_component;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ByProps {
    #[prop_or_default]
    pub value: Option<String>,
}

#[function_component(By)]
pub fn by(props: &ByProps) -> Html {
    props
        .value
        .as_ref()
        .map(|by_value| html!(<><span class="text-secondary">{" by "}</span> {by_value}</>))
        .unwrap_or(html!())
}
