use hooks::theme::{use_theme, Theme, ThemeManager};
use yew::prelude::*;
use yew_router::prelude::*;
use gloo::{console::log, storage::{LocalStorage, Storage}};

#[cfg(debug_assertions)]
pub(crate) const API_URL: &str = "http://localhost:8000";

#[cfg(debug_assertions)]
pub(crate) const WS_URL: &str = "ws://localhost:8000";

#[cfg(not(debug_assertions))]
pub(crate) const API_URL: &str = "https://api.zxq5.dev";

#[cfg(not(debug_assertions))]
pub(crate) const WS_URL: &str = "wss://api.zxq5.dev";

mod hooks {
    pub mod websocket;
    pub mod theme;
}

mod components {
    pub mod chat;
    pub mod signup;
    pub mod login;
    pub mod navbar;
    pub mod button;
    pub mod selector;
    pub mod serverlist;
    pub mod meme_license;
    pub mod incidents;
    pub mod form;
}

use components::{
    chat::Chat,
    login::Login,
    signup::Signup,
    navbar::Navbar,
    button::Button,
    selector::Selector,
    serverlist::ServerList,
    meme_license::MemeLicense,
    incidents::Incidents,
    form::Form,
};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Root,
    #[at("/login")]
    Login,
    #[at("/signup")]
    Signup,
    #[at("/chat/:id")]
    Chat { id: String },
    #[at("/profile")]
    Profile,
    #[at("/logout")]
    Logout,
    #[at("/invite")]
    Invite,
    #[at("/license/:username")]
    MemeLicense { username: String },
    #[at("/incidents/:incident")]
    Incidents { incident: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

impl std::fmt::Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Route::Root => "Home",
            Route::Login => "Login",
            Route::Signup => "Signup",
            Route::Chat { id: _ } => "Chat",
            Route::NotFound => "404",
            Route::Profile => "Profile",
            Route::Logout => "Logout",
            Route::Invite => "Invite",
            Route::MemeLicense { username: _ } => "Meme License",
            Route::Incidents { incident: _ } => "Incidents",
        })
    }
}

fn switch(route: Route) -> Html {

    // // check if user is logged in
    // if user not logged in:
    //     html! {}


    html ! {
        <div class="base-container">
            <Navbar/>
            { match route {
                Route::Root => html! { <Redirect<Route> to={Route::Login}/> },
                Route::Login => html! { <Login /> },
                Route::Signup => html! { <Signup /> },
                // Route::Chat { id: token } => {
                //     if let Ok(token) = LocalStorage::get::<String>("auth-token") {
                //         html! { <Chat id={token}/> }
                //     } else {
                //         html! { <Redirect<Route> to={Route::Login}/> }
                //     }
                // }
                Route::Chat { id: token } => html! { <Chat id={token}/> },
                Route::Incidents { incident: incident } => html! { <Incidents incident={incident}/> },
                Route::MemeLicense { username: username } => html! { <MemeLicense username={username}/> },
                _ => html! { <h1>{"404 Not Found"}</h1> },
            }}
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {

    let theme_handle = use_theme(Theme::Default);
    let ctx = use_state(|| ThemeManager::new(theme_handle.set_theme));
    
    {
        let ctx = ctx.clone();
        use_effect_with((), move |_| {
            ctx.set_theme.emit(Theme::Default);
        });
    }
    

    html! {
        <ContextProvider<ThemeManager> context={(*ctx).clone()}>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </ContextProvider<ThemeManager>>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}