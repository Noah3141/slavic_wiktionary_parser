use core::panic;
use super::{macro_of::MacroOf, WikiMacro};


impl WikiMacro {
    /// Composes a page's title with suspected macro text. If the macro text contains a recognizeable macro, Ok. If the text's macro is unrecognizable/-categorizable, Err.
    pub fn new(title: &str, macro_text: &str) -> Result<Self, String> {
        let macro_name = WikiMacro::detect_type(macro_text)?;

        // Literally just wrap the MacroOf in a WikiMacro
        let wiki_macro = match macro_name {
            MacroOf::RussianVerb(m) => WikiMacro { text: MacroOf::RussianVerb(m), page_title: title.to_string(), is_head: true, is_infl: false, is_noun: true, is_verb: true, is_adj: false, is_adv: false },
            MacroOf::InflRussianVerb(m) => WikiMacro { text: MacroOf::InflRussianVerb(m), page_title: title.to_string(), is_head: false, is_infl: true, is_noun: true, is_verb: true, is_adj: false, is_adv: false },
            MacroOf::RussianNoun(m) => WikiMacro { text: MacroOf::RussianNoun(m), page_title: title.to_string(), is_head: true, is_infl: false, is_noun: true, is_verb: false, is_adj: false, is_adv: false },
            MacroOf::InflRussianNoun(m) => WikiMacro { text: MacroOf::InflRussianNoun(m), page_title: title.to_string(), is_head: false, is_infl: true, is_noun: true, is_verb: false, is_adj: false, is_adv: false },
            MacroOf::RussianAdjective(m) =>WikiMacro { text: MacroOf::RussianAdjective(m), page_title: title.to_string(), is_head: true, is_infl: false, is_noun: false, is_verb: false, is_adj: true, is_adv: false },
            MacroOf::InflRussianAdjective(m) =>    WikiMacro { text: MacroOf::InflRussianAdjective(m), page_title: title.to_string(), is_head: false, is_infl: true, is_noun: false, is_verb: false, is_adj: true, is_adv: false },
            MacroOf::RussianAdverb(m) =>   WikiMacro { text: MacroOf::RussianAdverb(m), page_title: title.to_string(), is_head: true, is_infl: false, is_noun: false, is_verb: false, is_adj: false, is_adv: true },
            MacroOf::UkrainianVerb(m) =>   WikiMacro { text: MacroOf::UkrainianVerb(m), page_title: title.to_string(), is_head: true, is_infl: false, is_noun: true, is_verb: true, is_adj: false, is_adv: false },
            MacroOf::InflUkrainianVerb(m) =>   WikiMacro { text: MacroOf::InflUkrainianVerb(m), page_title: title.to_string(), is_head: false, is_infl: true, is_noun: true, is_verb: true, is_adj: false, is_adv: false },
            MacroOf::UkrainianNoun(m) =>   WikiMacro { text: MacroOf::UkrainianNoun(m), page_title: title.to_string(), is_head: true, is_infl: false, is_noun: true, is_verb: false, is_adj: false, is_adv: false },
            MacroOf::InflUkrainianNoun(m) =>   WikiMacro { text: MacroOf::InflUkrainianNoun(m), page_title: title.to_string(), is_head: false, is_infl: true, is_noun: true, is_verb: false, is_adj: false, is_adv: false },
            MacroOf::UkrainianAdjective(m) =>  WikiMacro { text: MacroOf::UkrainianAdjective(m), page_title: title.to_string(), is_head: true, is_infl: false, is_noun: false, is_verb: false, is_adj: true, is_adv: false },
            MacroOf::InflUkrainianAdjective(m) =>  WikiMacro { text: MacroOf::InflUkrainianAdjective(m), page_title: title.to_string(), is_head: false, is_infl: true, is_noun: false, is_verb: false, is_adj: true, is_adv: false },
            MacroOf::UkrainianAdverb(m) => WikiMacro { text: MacroOf::UkrainianAdverb(m), page_title: title.to_string(), is_head: true, is_infl: false, is_noun: false, is_verb: false, is_adj: false, is_adv: true },
            MacroOf::BelarusianVerb(m) =>  WikiMacro { text: MacroOf::BelarusianVerb(m), page_title: title.to_string(), is_head: true, is_infl: false, is_noun: true, is_verb: true, is_adj: false, is_adv: false },
            MacroOf::InflBelarusianVerb(m) =>  WikiMacro { text: MacroOf::InflBelarusianVerb(m), page_title: title.to_string(), is_head: false, is_infl: true, is_noun: true, is_verb: true, is_adj: false, is_adv: false },
            MacroOf::BelarusianNoun(m) =>  WikiMacro { text: MacroOf::BelarusianNoun(m), page_title: title.to_string(), is_head: true, is_infl: false, is_noun: true, is_verb: false, is_adj: false, is_adv: false },
            MacroOf::InflBelarusianNoun(m) =>  WikiMacro { text: MacroOf::InflBelarusianNoun(m), page_title: title.to_string(), is_head: false, is_infl: true, is_noun: true, is_verb: false, is_adj: false, is_adv: false },
            MacroOf::BelarusianAdjective(m) => WikiMacro { text: MacroOf::BelarusianAdjective(m), page_title: title.to_string(), is_head: true, is_infl: false, is_noun: false, is_verb: false, is_adj: true, is_adv: false },
            MacroOf::InflBelarusianAdjective(m) => WikiMacro { text: MacroOf::InflBelarusianAdjective(m), page_title: title.to_string(), is_head: false, is_infl: true, is_noun: false, is_verb: false, is_adj: true, is_adv: false },
            MacroOf::BelarusianAdverb(m) =>WikiMacro { text: MacroOf::BelarusianAdverb(m), page_title: title.to_string(), is_head: true, is_infl: false, is_noun: false, is_verb: false, is_adj: false, is_adv: true },
        };

        Ok(wiki_macro)
    }

}