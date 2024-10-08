use crate::{models::{language_section::LanguageSection, wiki_macro::WikiMacro, wiki_text_page::WikiTextPage}, traits::macro_containing::MacroContaining, utils::{select_from::select_from, select_unto_language_header::select_unto_language_header}};
use super::WikiTextPageBuilder;


impl WikiTextPageBuilder {
    /// Provide `page_xml` from <page> to </page>
    pub fn from_xml(self, page_xml: &str) -> WikiTextPage {
        
        let page_id = select_from(page_xml, "<id>", "</id>").expect("presence of page id");
        let title = select_from(page_xml, "<title>", "</title>").expect("page title");

        let russian_section = if page_xml.contains("==Russian==") {
            let text = select_unto_language_header(page_xml, "==Russian==").expect("successful language section extraction");
            let wiki_macros = text.to_string().find_macros();
            Some(LanguageSection { 
                wiki_macros: wiki_macros
                    .into_iter()
                    .filter_map(|wiki_macro: String| {
                        WikiMacro::new(title, &wiki_macro).ok()
                    })
                    .collect::<Vec<WikiMacro>>()
            })
        } else {
            None
        };

        let ukrainian_section = if page_xml.contains("==Ukrainian==") {
            let text = select_unto_language_header(page_xml, "==Ukrainian==").expect("successful language section extraction");
            let wiki_macros = text.to_string().find_macros();
            Some(LanguageSection { 
                wiki_macros: wiki_macros
                    .into_iter()
                    .filter_map(|wiki_macro: String| {
                        WikiMacro::new(title, &wiki_macro).ok()
                    })
                    .collect::<Vec<WikiMacro>>()
            })
        } else {
            None
        };

        let belarusian_section = if page_xml.contains("==Belarusian==") {
            let text = select_unto_language_header(page_xml, "==Belarusian==").expect("successful language section extraction");
            let wiki_macros = text.to_string().find_macros();
            Some(LanguageSection { 
                wiki_macros: wiki_macros
                    .into_iter()
                    .filter_map(|wiki_macro: String| {
                        WikiMacro::new(title, &wiki_macro).ok()
                    })
                    .collect::<Vec<WikiMacro>>()
            })
        } else {
            None
        };

        if self.verbose {
            println!(
r#"{{
    "title": "{title}",
    "russian": {russian_macros},
    "ukrainian": {ukrainian_macros},
    "belarusian": {belarusian_macros},
}}
"#,
russian_macros = match &russian_section { Some(s) => serde_json::to_string_pretty(&s.wiki_macros).unwrap(), None => "None".to_string() },
ukrainian_macros = match &ukrainian_section { Some(s) => serde_json::to_string_pretty(&s.wiki_macros).unwrap(), None => "None".to_string() },
belarusian_macros = match &belarusian_section { Some(s) => serde_json::to_string_pretty(&s.wiki_macros).unwrap(), None => "None".to_string() }
            )
        }

        WikiTextPage {
            page_id: page_id.to_string(),
            title: title.to_string(),
            russian_section,
            ukrainian_section,
            belarusian_section,
        }
    }
}