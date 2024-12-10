use yew::{function_component, html, Callback, Html, MouseEvent, Properties};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub onclick: Callback<MouseEvent>,
    pub text: String,
    pub class: Option<String>
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    html! {
        <button 
            class={props.class.clone().unwrap_or("button".to_string())} 
            onclick={props.onclick.clone()}>
            {props.text.clone()}
        </button>
    }
}