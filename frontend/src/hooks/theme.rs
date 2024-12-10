use std::fmt;

use yew::prelude::*;
use web_sys;
use gloo::console::log;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Theme {
    Default,
    Light,
    Dark,
}

impl Theme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Theme::Default => "default",
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Clone, PartialEq)]
pub struct ThemeManager {
    pub current: Theme,
    pub set_theme: Callback<Theme>,
}

impl ThemeManager {
    pub fn new(set_theme: Callback<Theme>) -> Self {
        Self {
            current: Theme::Default,
            set_theme,
        }
    }
}

#[derive(Clone)]
pub struct UseThemeHandle {
    pub set_theme: Callback<Theme>,
}

#[hook]
pub fn use_theme(initial: Theme) -> UseThemeHandle {
    let theme = use_state(|| initial);

    let set_theme = {
        let theme = theme.clone();
        Callback::from(move |new_theme: Theme| {
            if let Some(window) = web_sys::window() {
                if let Some(doc) = window.document() {
                    if let Some(body) = doc.body() {
                        if let Err(e) = body.set_attribute("theme", new_theme.as_str()) {
                            log!("Failed to set theme:", e);
                        }
                    }
                }
            }
            theme.set(new_theme);
        })
    };

    UseThemeHandle {
        set_theme,
    }
}