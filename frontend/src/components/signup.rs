use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use yew::{function_component, html, use_node_ref, use_state, Callback, Html, SubmitEvent};
use yew_router::{navigator, prelude::*};
use gloo::storage::{LocalStorage, Storage};
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

use crate::{Route, API_URL};

#[function_component(Signup)]
pub fn signup_page() -> Html {
    let navigator = use_navigator().unwrap();
    let username_ref = use_node_ref();
    let password_ref = use_node_ref();
    let confirm_password_ref = use_node_ref();
    let token_ref = use_node_ref();
    let signup_error = use_state(|| None::<String>);

    let navigator_clone = navigator.clone();
    let username_ref_clone = username_ref.clone();
    let password_ref_clone = password_ref.clone();
    let token_ref_clone = token_ref.clone();
    let confirm_password_ref_clone = confirm_password_ref.clone();
    let signup_error_clone = signup_error.clone();
    
    let onsubmit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        let username = username_ref_clone.cast::<HtmlInputElement>().unwrap().value();
        let password = password_ref_clone.cast::<HtmlInputElement>().unwrap().value();
        let confirm_password = confirm_password_ref_clone.cast::<HtmlInputElement>().unwrap().value();
        let token = token_ref_clone.cast::<HtmlInputElement>().unwrap().value();

        if username.is_empty() || password.is_empty() || token.is_empty() {
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
            match signup(SignupRequest { 
                username, 
                password, 
                token 
            }).await {
                Ok(_) => navigator.push(&Route::Chat { id: "test".to_string() }),
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
        <div class="form-container">
            <form {onsubmit} class="form-form">
                <h2 class="form-title">{"Sign Up"}</h2>
                <input 
                    ref={username_ref}
                    class="form-input"
                    type="text"
                    id="username"
                    name="username"
                    placeholder="Username"
                />
                <input 
                    ref={password_ref}
                    class="form-input"
                    type="password"
                    id="password"
                    name="password"
                    placeholder="Password"
                />
                <input 
                    ref={confirm_password_ref}
                    class="form-input"
                    type="password"
                    id="confirm_password"
                    name="confirm_password"
                    placeholder="Confirm Password"
                />
                <input 
                    ref={token_ref}
                    class="form-input"
                    type="password"
                    id="access_token"
                    name="access_token"
                    placeholder="Access Token"
                />
                <button class="form-button" type="submit">{"Sign Up"}</button>
                {
                    if let Some(error) = (*signup_error).clone() {
                        html! {
                            <p class="form-error">{error}</p>
                        }
                    } else {
                        html! {}
                    }
                }

                <p class="form-text">{"Already have an account?"}</p>
                <a onclick={go_to_login}
                    href=""
                    class="form-button"
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
    token: String,
}

async fn signup(req: SignupRequest) -> Result<(), String> {
    match Request::post(format!("{API_URL}/signup").as_str())
        .json(&req)
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