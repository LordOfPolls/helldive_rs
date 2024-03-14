
/// Languages supported by the API
#[non_exhaustive]
pub enum Language {
    English,
    German,
    Spanish,
    Russian,
    French,
    Italian,
    Polish,
    Chinese
}

impl Language {
    pub fn to_str(&self) -> &str {
        match self {
            Language::English => "en-US",
            Language::German => "de-DE",
            Language::Spanish => "es-ES",
            Language::Russian => "ru-RU",
            Language::French => "fr-FR",
            Language::Italian => "it-IT",
            Language::Polish => "pl-PL",
            Language::Chinese => "zh-Hans",
        }
    }
}
