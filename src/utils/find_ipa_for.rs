use reqwest::Client;
use wiktionary_parser::models::{language::Language, wiktionary_macro::WiktionaryMacro};




pub async fn find_ipa_for(page_id: u64, language: &Language, wiki_macros: &Vec<WiktionaryMacro>, client: &Client) -> Option<String> {
    let language_ipa = match language {
        Language::Russian => {
            if let Some(ru_ipa) = wiki_macros
                .iter()
                .filter_map(|macros| match macros { WiktionaryMacro::RuIpa(m) => Some(m), _ => None, })
                .find(|ipa_m| { ipa_m.page_id == page_id }) 
                { Some(ru_ipa.to_ipa_string(&client).await) } 
            else { None }
        },
        Language::Ukrainian => {
            if let Some(uk_ipa) = wiki_macros
                .iter()
                .filter_map(|macros| match macros { WiktionaryMacro::UkIpa(m) => Some(m), _ => None, })
                .find(|ipa_m| { ipa_m.page_id == page_id })
                { Some(uk_ipa.to_ipa_string(&client).await) } 
            else { None }
        },
        Language::Belarusian => {
            if let Some(be_ipa) = wiki_macros
                .iter()
                .filter_map(|macros| match macros { WiktionaryMacro::UkIpa(m) => Some(m), _ => None, })
                .find(|ipa_m| { ipa_m.page_id == page_id })
                { Some(be_ipa.to_ipa_string(&client).await) }   
            else { None }
        },
    };

    match language_ipa {
        Some(ipa_string) => return Some(ipa_string),
        None => {
            if let Some(m) = wiki_macros
                .iter()
                .filter_map(|m| match m { WiktionaryMacro::Ipa(m) => Some(m), _ => None, })
                .find(|ipa_m| { ipa_m.page_id == page_id })
                { Some(m.to_ipa_string(&client).await) }   
            else { None }
        },
    }

}