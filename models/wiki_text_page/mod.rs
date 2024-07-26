use serde::{Deserialize, Serialize};
use super::language_section::LanguageSection;

#[derive(Debug, Serialize, Deserialize)]
pub struct WikiTextPage {
    pub page_id: String,
    pub title: String,
    pub russian_section: Option<LanguageSection>,
    pub ukrainian_section: Option<LanguageSection>,
    pub belarusian_section: Option<LanguageSection>,

}

pub mod new;
pub mod api;


