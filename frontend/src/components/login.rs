use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::{navigator, prelude::*};
use gloo::storage::{LocalStorage, Storage};
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

use crate::Route;

#[function_component(Login)]
pub fn login_page() -> Html {
    let navigator = use_navigator().unwrap();
    let username_ref = use_node_ref();
    let password_ref = use_node_ref();
    let login_success = use_state(|| true);

    let navigator_clone = navigator.clone();
    let username_ref_clone = username_ref.clone();
    let password_ref_clone = password_ref.clone();
    let login_success_clone = login_success.clone();
    let onsubmit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        let username = username_ref_clone.cast::<HtmlInputElement>().unwrap().value();
        let password = password_ref_clone.cast::<HtmlInputElement>().unwrap().value();

        // TODO: Replace this with actual authentication
        // For now, we'll just set a dummy token
        if !( username.is_empty() || password.is_empty() ) {
            let navigator = navigator_clone.clone();
            let login_success = login_success_clone.clone();
            spawn_local(async move {
                match login(username, password).await {
                    Ok(_) => navigator.push(&Route::Chat),
                    Err(_) => login_success.set(false),
                }
            });
        }
    });

    let go_to_signup = {
        let navigator_clone = navigator.clone();
        Callback::from(move |_| {
            navigator_clone.push(&Route::Signup);
        })
    };

    html! {
        <div class="login-container">
            <form {onsubmit} class="login-form">
                <h2 class="login-title">{"Login"}</h2>
                <input 
                    ref={username_ref}
                    class="login-input"
                    type="text"
                    id="username"
                    name="username"
                    placeholder="Username"
                />
                <input 
                    ref={password_ref}
                    class="login-input"
                    type="password"
                    id="password"
                    name="password"
                    placeholder="Password"
                />  
                <button class="login-button" type="submit">{"Login"}</button>
                {
                    if !(*login_success) {
                        html! {
                            <p class="login-error">{"Incorrect username or password"}</p>
                        }
                    } else {
                        html! {}
                    }
                }

                <p class="login-text">{"Don't have an account?"}</p>
                <a onclick={go_to_signup}
                    href=""
                    class="login-button"
                >
                    {"Create Account"}
                </a>
            </form>
        </div>
    }
}

#[derive(Serialize, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

async fn login(username: String, password: String) -> Result<(), String> {
    let login_request = LoginRequest {
        username,
        password,
    };

    match Request::post("http://127.0.0.1:8000/login")
        .json(&login_request)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?
        .status() 
    {
        200 => Ok(()),
        _ => Err("Login failed".to_string()),
    }
}