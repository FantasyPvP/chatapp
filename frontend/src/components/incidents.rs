use yew::{function_component, html, Html, Properties};
use crate::API_URL;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub incident: String
}

#[function_component(Incidents)]
pub fn incidents(props: &Props) -> Html {

    let content = match props.incident.as_str() {
        "boats" => (
            "The Boat Incident.",
            "the boat incident involved many boats being dropped on panic attack's tower. he then proceedeed to accidentally blow himself up along with all his stuff using a tnt cannon."
        ),
        _ => ("", "")
    };

    html! {
        <div class="app-container">
            <div class="meme-license-container" style="min-height: 500px;">
                <div style="display: flex; flex-grow: 1; flex-direction: column; align-items: center; padding: 2rem; gap: 2rem">
                    <h1 style="font-size: 4vw; text-align: center">{content.0}</h1>
                    <div style="display: flex; flex-direction: column; align-items: left; width: max-content; justify-content: center">
                        <div style="display: flex; flex-direction: row; align-items: center; width: max-content; gap: 1rem">
                            <p style="text-align: left; width: 50vw; font-size: 1.5vw">{content.1}</p>
                            // <p style="text-align: left; width: max-content; font-size: 1.5vw">{value}</p> 
                        </div>      
                    </div>
                </div>
            </div>
        </div>
    }
}