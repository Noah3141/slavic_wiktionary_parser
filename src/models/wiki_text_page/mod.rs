use super::language_section::LanguageSection;

#[derive(Debug)]
pub struct WikiTextPage<'page> {
    pub page_id: String,
    pub title: String,
    pub russian_section: Option<LanguageSection<'page>>,
    pub ukrainian_section: Option<LanguageSection<'page>>,
    pub belarusian_section: Option<LanguageSection<'page>>,

}

pub mod new;
pub mod api;


