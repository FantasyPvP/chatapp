use yew::{function_component, html, Html, Properties};
use crate::API_URL;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub username: String
}

#[function_component(MemeLicense)]
pub fn meme_license(props: &Props) -> Html {
    html! {
        <div class="app-container">
            <div class="meme-license-container" style="min-height: 500px;">
                <div class="profile-picture" style={ format!(
                        "background-image: 
                            url('{API_URL}/static/pfp/{}.png'),
                            url('{API_URL}/static/public/default_pfp.png');
                        
                        height: 25vw;
                        max-height: 100%;
                        width: revert;
                        aspect-ratio: 1/1;
                        border-radius: 40px 0px 40px 0px;
                        flex-shrink: 0;
                        box-shadow: 0 2px 8px var(--shadow-color);
                        ",
                        props.username
                    )}></div>
                <div style="display: flex; flex-grow: 1; flex-direction: column; align-items: center; padding: 2rem; gap: 2rem">
                    <h1 style="font-size: 4vw; text-align: center">{"Meme Stealing License"}</h1>
                    <div style="display: flex; flex-direction: column; align-items: left; width: max-content; justify-content: center">
                        {[
                            ("Username", props.username.as_str()),
                            ("Valid From", "2024"),
                            ("Expires", "2026"),
                            ("Issuer", "Steven"),
                        ].iter()
                            .map(|(field, value)| html! { 
                                <div style="display: flex; flex-direction: row; align-items: center; width: max-content; gap: 1rem">
                                    <p style="text-align: left; width: 10vw; font-size: 1.5vw">{field}</p>
                                    <p style="text-align: left; width: max-content; font-size: 1.5vw">{value}</p> 
                                </div>
                            })
                            .collect::<Html>()
                        }
                    </div>
                </div>
            </div>
        </div>
    }
}