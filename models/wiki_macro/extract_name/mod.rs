use crate::{models::wiki_macro::macro_of::russian_noun_macro::RussianNounMacro, utils::select_from::select_from};

use super::{macro_of::MacroOf, WikiMacro};

#[cfg(test)]
mod test;

impl WikiMacro {
    pub fn detect_type(macro_text: &str) -> Result<MacroOf, String> {
        assert!(macro_text.starts_with("{{"));
        let macro_name = select_from(macro_text, "{{", "|").expect("presence of macro start in macro_text");

        // if macro_text.ends_with("}}") {}

        let named_macro = match macro_name {
            "ru-noun+" => MacroOf::RussianNoun(RussianNounMacro(macro_text.to_string())),
            "ru-noun-table" => return Err(String::from("Noun table not yet supported")),
            "head" => return Err(String::from("We ignore head tags for now")),
            "infl of" | "inflection of" => {
                let language_param = select_from(macro_text, "|", "").expect("parameters provided to infl of");

                

            }
            _ => return Err(format!("Unrecognized macro: {macro_text}"))
        };

        Ok(named_macro)
    }
}