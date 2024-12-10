use std::fmt::Display;
use yew::prelude::*;

use crate::components::button::Button;

#[derive(Properties, PartialEq)]
pub struct Props<T: Display + Clone + PartialEq + 'static> {
    pub args: Vec<T>,
    pub onselect: Callback<T>,
    pub text: String,
}

#[function_component(Selector)]
pub fn selector<T: Display + Clone + PartialEq + 'static>(props: &Props<T>) -> Html {
    html! {
        <div class="selector">
            <div class="selector-label">{&props.text}</div>
            <div class="selector-items">
                {props.args.iter().map(|arg| {
                    let onclick = {
                        let arg = arg.clone();
                        let onselect = props.onselect.clone();
                        Callback::from(move |_| {
                            onselect.emit(arg.clone())
                        })
                    };
                    html! {
                        <Button class={Some("selector-button".to_string())} text={arg.to_string()} onclick={onclick} />
                    }
                }).collect::<Html>()}
            </div>
        </div>
    }
}