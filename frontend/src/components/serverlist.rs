use std::fmt::Display;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

use crate::{components::button::Button, API_URL};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Server {
    id: String,
    name: String,
}

#[function_component(ServerList)]
pub fn serverlist() -> Html {

    let servers = use_state(|| Vec::<Server>::new());

    {
        let servers = servers.clone();

        use_effect_with((), move |_| {
            let servers = servers.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let fetched = if let Ok(fetched) = Request::get(format!("{API_URL}/servers").as_str())
                    .send()
                    .await
                {
                    fetched.json()
                        .await
                        .unwrap()
                } else {
                    Vec::new()
                };
                servers.set(fetched);
            });
        })
    }

    let onselect = {
        let servers = servers.clone();
        Callback::from(move |server: Server| {
            servers.set(vec![server]);
        })
    };

    html! {
        <div class="ui-server-list">
            <p style="text-align: center; padding: 1rem;"> {"Servers"} </p>
            { servers.iter().map(|server| {
                html! {
                    <Button 
                        text={server.name.clone()} 
                        onclick={
                            let onselect = onselect.clone();
                            let server = server.clone();
                            Callback::from(move |_| {
                                onselect.emit(server.clone());
                            })
                        }
                        class={Some("selector-button".to_string())}
                    />
                }
            }).collect::<Html>() }
        </div>
    }
}