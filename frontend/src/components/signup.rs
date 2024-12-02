use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use yew::{function_component, html, use_node_ref, use_state, Callback, Html, SubmitEvent};
use yew_router::{navigator, prelude::*};
use gloo::storage::{LocalStorage, Storage};
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

use crate::Route;

#[function_component(Signup)]
pub fn signup_page() -> Html {
    let navigator = use_navigator().unwrap();
    let username_ref = use_node_ref();
    let password_ref = use_node_ref();
    let confirm_password_ref = use_node_ref();
    let signup_error = use_state(|| None::<String>);

    let navigator_clone = navigator.clone();
    let username_ref_clone = username_ref.clone();
    let password_ref_clone = password_ref.clone();
    let confirm_password_ref_clone = confirm_password_ref.clone();
    let signup_error_clone = signup_error.clone();
    
    let onsubmit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        let username = username_ref_clone.cast::<HtmlInputElement>().unwrap().value();
        let password = password_ref_clone.cast::<HtmlInputElement>().unwrap().value();
        let confirm_password = confirm_password_ref_clone.cast::<HtmlInputElement>().unwrap().value();

        if username.is_empty() || password.is_empty() {
            signup_error_clone.set(Some("Please fill in all fields".to_string()));
            return;
        }

        if password != confirm_password {
            signup_error_clone.set(Some("Passwords do not match".to_string()));
            return;
        }

        let navigator = navigator_clone.clone();
        let signup_error = signup_error_clone.clone();
        spawn_local(async move {
            match signup(username, password).await {
                Ok(_) => navigator.push(&Route::Chat),
                Err(e) => signup_error.set(Some(e)),
            }
        });
    });

    let go_to_login = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::Login);
        })
    };

    html! {
        <div class="login-container">
            <form {onsubmit} class="login-form">
                <h2 class="login-title">{"Sign Up"}</h2>
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
                <input 
                    ref={confirm_password_ref}
                    class="login-input"
                    type="password"
                    id="confirm_password"
                    name="confirm_password"
                    placeholder="Confirm Password"
                />
                <button class="login-button" type="submit">{"Sign Up"}</button>
                {
                    if let Some(error) = (*signup_error).clone() {
                        html! {
                            <p class="login-error">{error}</p>
                        }
                    } else {
                        html! {}
                    }
                }

                <p class="login-text">{"Already have an account?"}</p>
                <a onclick={go_to_login}
                    href=""
                    class="login-button"
                >
                    {"Login"}
                </a>
            </form>
        </div>
    }
}

#[derive(Serialize, Deserialize)]
struct SignupRequest {
    username: String,
    password: String,
}

async fn signup(username: String, password: String) -> Result<(), String> {
    let signup_request = SignupRequest {
        username,
        password,
    };

    match Request::post("http://127.0.0.1:8000/signup")
        .json(&signup_request)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?
        .status() 
    {
        200 => Ok(()),
        409 => Err("Username already exists".to_string()),
        x => Err(format!("Signup failed with status code {}", x)),
    }
}