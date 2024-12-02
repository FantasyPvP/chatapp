use serde::{Deserialize, Serialize};
use yew::prelude::*;
use web_sys::HtmlInputElement;
use chrono::prelude::*;
use crate::hooks::websocket::use_websocket;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RealTimeMessage {
    pub message_id: i32,
    pub user_id: i32,
    pub display_name: String,
    pub created_at: i64,
    pub content: String,
}

#[function_component(Chat)]
pub fn chat() -> Html {
    let ws = use_websocket("ws://localhost:8000/messenger/connect/1");
    let input_ref = use_node_ref();
    let dark_theme = use_state(|| true);

    // let theme_toggle = {
    //     let dark_theme = dark_theme.clone();
    //     Callback::from(move |_| {
    //         dark_theme.set(!*dark_theme);
    //     })
    // };

    let onsubmit = {
        let ws = ws.ws.clone();
        let input_ref = input_ref.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                let message = input.value();
                if !message.is_empty() {
                    ws.send_with_str(&message).unwrap();
                    input.set_value("");
                }
            }
        })
    };

    html! {
        <div class={classes!("app-container", if *dark_theme { "dark-theme" } else { "light-theme" })}>
            <nav class="navbar">
                <div class="nav-brand">{"Chat App"}</div>
                // <div class="theme-toggle">
                //     <button onclick={theme_toggle} class="theme-button">
                //         if *dark_theme {
                //             {"🌞"}
                //         } else {
                //             {"🌙"}
                //         }
                //     </button>
                // </div>
            </nav>
            <div class="chat-container">
                <div class="messages-container">
                    {ws.messages.messages().iter().map(|msg| {
                        let timestamp = Local.timestamp_millis_opt(msg.created_at).unwrap();
                        let formatted_time = timestamp.format("%d/%m/%y %H:%M").to_string();
                        let userid = msg.user_id;

                        html! {
                            <div class="message">
                                <div class="profile-picture" style={ format!(
                                    "background-image: url('http://localhost:8000/static/pfp/{userid}.png')"
                                )}></div>
                                <div class="message-bubble">
                                    <div class="message-header">
                                        <span class="username">{&msg.display_name}</span>
                                        <span class="timestamp">{formatted_time}</span>
                                    </div>
                                    <div class="message-content">{&msg.content}</div>
                                </div>
                            </div>
                        }
                    }).collect::<Html>()}
                </div>
                if let Some(error) = (*ws.error).as_ref() {
                    <div class="error-message">
                        {format!("Error: {}", error)}
                    </div>
                }
                <form {onsubmit} class="message-form">
                    <input
                        type="text"
                        ref={input_ref}
                        class="message-input"
                        placeholder="Type a message..."
                    />
                    <button type="submit" class="send-button">{"Send"}</button>
                </form>
            </div>
        </div>
    }
}
