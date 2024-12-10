use yew::{function_component, html, use_context, Html, Callback};
use yew_router::hooks::use_navigator;

use crate::{
    components::selector::Selector,
    hooks::theme::{use_theme, Theme, ThemeManager}, Route
};

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let ctx = use_context::<ThemeManager>().unwrap();
    let on_select_theme = {
        Callback::from(move |selected: Theme| {
            ctx.set_theme.emit(selected);
        })
    };

    let go_to_route = {
        let navigator = use_navigator().unwrap();
        Callback::from(move |selected: Route| {
            navigator.push(&selected);
        })
    };

    html! {
        <nav class="navbar ui-layout-horizontal">
            <div class="nav-brand">{"ZXQ5.Dev"}</div>

            <div style="width: 100%;"/>

            <Selector<Theme>
                text={"Theme".to_string()}
                args={vec![Theme::Default, Theme::Light, Theme::Dark]}
                onselect={on_select_theme}
            />
            <Selector<Route>
                text={"My Account".to_string()}
                args={vec![Route::Login, Route::Signup]}
                onselect={go_to_route}
            />
        </nav>
    }
}