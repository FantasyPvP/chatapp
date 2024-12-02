use yew::prelude::*;
use yew_router::prelude::*;
use gloo::storage::{LocalStorage, Storage};

mod hooks {
    pub mod websocket;
}

mod components {
    pub mod chat;
    pub mod signup;
    pub mod login;
}

use components::{
    chat::Chat,
    login::Login,
    signup::Signup,
};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Root,
    #[at("/login")]
    Login,
    #[at("/signup")]
    Signup,
    #[at("/chat")]
    Chat,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Root => html! { <Redirect<Route> to={Route::Login}/> },
        Route::Login => html! { <Login /> },
        Route::Signup => html! { <Signup /> },
        Route::Chat => {
            if let Ok(token) = LocalStorage::get::<String>("auth_token") {
                html! { <Chat /> }
            } else {
                html! { <Redirect<Route> to={Route::Login}/> }
            }
        }
        Route::NotFound => html! { <h1>{"404 Not Found"}</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}