use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props{
    pub alert_type: AttrValue,
    pub message: AttrValue,
    
}

#[function_component(Alert)]
pub fn input(props: &Props) -> Html {
    html!(
        <div class={format!("alert alert-{}", props.alert_type.clone())} role="alert">{props.message.clone()}</div>
    )
}