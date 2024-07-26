use serde::{Deserialize, Serialize};
use super::wiki_macro::WikiMacro;

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageSection {
    pub wiki_macros: Vec<WikiMacro>
}
