use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;

#[derive(Clone, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "dark" => Theme::Dark,
            _ => Theme::Light,
        }
    }

    pub fn toggle(&self) -> Self {
        match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        }
    }
}

fn apply_theme_to_document(theme: &Theme) {
    if let Some(window) = window() {
        if let Some(document) = window.document() {
            if let Some(html) = document.document_element() {
                html.set_attribute("data-theme", theme.as_str()).ok();
                if let Ok(html_element) = html.dyn_into::<web_sys::HtmlElement>() {
                    let class_list = html_element.class_list();
                    if theme.as_str() == "dark" {
                        class_list.add_1("dark").ok();
                    } else {
                        class_list.remove_1("dark").ok();
                    }
                }
            }
        }
    }
}

fn get_initial_theme() -> Theme {
    if let Some(window) = window() {
        // Check localStorage first
        if let Ok(Some(local_storage)) = window.local_storage() {
            if let Ok(Some(saved_theme)) = local_storage.get_item("theme") {
                return Theme::from_str(&saved_theme);
            }
        }
        // Check system preference
        if let Ok(Some(prefers_dark)) = window.match_media("(prefers-color-scheme: dark)") {
            if prefers_dark.matches() {
                return Theme::Dark;
            }
        }
    }
    Theme::Light
}

#[hook]
pub fn use_theme() -> (UseStateHandle<Theme>, Callback<()>) {
    let theme = use_state(|| {
        let initial = get_initial_theme();
        apply_theme_to_document(&initial);
        initial
    });

    let toggle_theme = {
        let theme = theme.clone();
        Callback::from(move |_| {
            let new_theme = theme.toggle();
            theme.set(new_theme.clone());
            
            // Save to localStorage
            if let Some(window) = window() {
                if let Ok(Some(local_storage)) = window.local_storage() {
                    local_storage.set_item("theme", new_theme.as_str()).ok();
                }
            }
            
            // Apply to document
            apply_theme_to_document(&new_theme);
        })
    };

    (theme, toggle_theme)
}
