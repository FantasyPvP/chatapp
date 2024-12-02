use yew::prelude::*;
use web_sys::{WebSocket, MessageEvent, ErrorEvent};
use wasm_bindgen::{prelude::*, JsCast};
use std::rc::Rc;
use crate::components::chat::RealTimeMessage;

pub struct UseWebSocketHandle {
    pub ws: Rc<WebSocket>,
    pub messages: UseReducerHandle<MessagesState>,
    pub error: UseStateHandle<Option<String>>,
}

#[derive(Clone)]
pub struct MessagesState {
    messages: Vec<RealTimeMessage>,
}

pub enum MessagesAction {
    AddMessage(RealTimeMessage),
}

impl Reducible for MessagesState {
    type Action = MessagesAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            MessagesAction::AddMessage(msg) => {
                let mut new_messages = self.messages.clone();
                new_messages.push(msg);
                Rc::new(MessagesState {
                    messages: new_messages,
                })
            }
        }
    }
}

impl MessagesState {
    pub fn messages(&self) -> Vec<RealTimeMessage> {
        self.messages.clone()
    } 
}


#[hook]
pub fn use_websocket(url: &str) -> UseWebSocketHandle {
    use web_sys::js_sys;

    let messages = use_reducer(|| MessagesState { messages: Vec::new() });
    let error = use_state(|| None::<String>);
    
    let ws = use_state_eq(|| {
        Rc::new(WebSocket::new(url).unwrap_or_else(|e| panic!("Failed to open WebSocket: {:?}", e)))
    });
    
    {
        let messages = messages.clone();
        let error = error.clone();
        let ws = (*ws).clone();
        
        use_effect_with((), move |_| {
            let ws_clone = ws.clone();
            let error_clone = error.clone();
            
            // Set up message handler
            let onmessage_callback = {
                let messages = messages.clone();
                Closure::wrap(Box::new(move |e: MessageEvent| {
                    if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                        if let Ok(msg) = serde_json::from_str::<RealTimeMessage>(&txt.as_string().unwrap()) {
                            messages.dispatch(MessagesAction::AddMessage(msg));
                        } else {
                            error_clone.set(Some("Failed to parse message".to_string()));
                        }
                    }
                }) as Box<dyn FnMut(MessageEvent)>)
            };
            
            ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
            onmessage_callback.forget();
            
            // Set up error handler
            let error_clone = error.clone();
            let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
                error_clone.set(Some(e.message()));
            }) as Box<dyn FnMut(ErrorEvent)>);
            
            ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
            onerror_callback.forget();
            
            move || {
                ws_clone.close().unwrap_or_else(|_| {});
            }
        });
    }
    
    UseWebSocketHandle {
        ws: (*ws).clone(),
        messages,
        error,
    }
}
