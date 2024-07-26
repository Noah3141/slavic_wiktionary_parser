use super::wiki_macro::WikiMacro;

#[derive(Debug)]
pub struct LanguageSection<'page> {
    pub wiki_macros: Vec<WikiMacro<'page>>
}
