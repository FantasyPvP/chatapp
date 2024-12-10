use gloo::console::log;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use web_sys::HtmlInputElement;
use chrono::prelude::*;
use crate::{components::{navbar::Navbar, serverlist::ServerList}, hooks::websocket::use_websocket};
use crate::{WS_URL, API_URL};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RealTimeMessage {
    pub message_id: i32,
    pub user_id: String,
    pub display_name: String,
    pub created_at: i64,
    pub content: String,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub id: String
}

#[function_component(Chat)]
pub fn chat(props: &Props) -> Html {
    let id = props.id.clone();
    let ws = use_websocket(format!("{WS_URL}/messenger/connect/1/{id}").as_str());
    let input_ref = use_node_ref();

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
        <div class="ui-layout-horizontal">
            // <ServerList/>
            <div class="app-container">
                <div class="messages-container">
                    {ws.messages.messages().iter().map(|msg| {
                        let timestamp = Local.timestamp_millis_opt(msg.created_at).unwrap();
                        let formatted_time = timestamp.format("%d/%m/%y %H:%M").to_string();
                        let userid = msg.user_id.clone();
                        html! {
                            <div class="message">
                                // load profile, if not - load fallback / default
                                <div class="profile-picture" style={ format!(
                                    "background-image: 
                                        url('{API_URL}/static/pfp/{}.png'),
                                        url('{API_URL}/static/public/default_pfp.png')",
                                    userid
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
                <form {onsubmit} class="ui-layout-horizontal">
                    <input
                        type="text"
                        ref={input_ref}
                        class="message-input ui-element-standalone"
                        placeholder="Type a message..."
                    />
                    <button type="submit" class="ui-button ui-element-standalone">{"Send"}</button>
                </form>
            </div>
        </div>
    }
}
