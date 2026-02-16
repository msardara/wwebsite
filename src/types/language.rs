#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    English,
    French,
    Italian,
}

#[allow(dead_code)]
impl Language {
    pub fn code(&self) -> &str {
        match self {
            Language::English => "en",
            Language::French => "fr",
            Language::Italian => "it",
        }
    }

    pub fn from_code(code: &str) -> Self {
        match code {
            "fr" => Language::French,
            "it" => Language::Italian,
            _ => Language::English,
        }
    }

    pub fn from_browser() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            use web_sys::window;
            if let Some(window) = window() {
                if let Some(navigator) = window.navigator().language() {
                    let lang = navigator.to_lowercase();
                    if lang.starts_with("fr") {
                        return Language::French;
                    } else if lang.starts_with("it") {
                        return Language::Italian;
                    }
                }
            }
        }
        Language::English
    }

    pub fn name(&self) -> &str {
        match self {
            Language::English => "English",
            Language::French => "Français",
            Language::Italian => "Italiano",
        }
    }
}
