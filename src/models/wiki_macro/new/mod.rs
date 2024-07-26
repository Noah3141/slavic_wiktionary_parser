use core::panic;
use crate::utils::extract_macro_name::extract_macro_name;
use super::{macro_of::MacroOf, WikiMacro};


impl<'page> WikiMacro<'page> {
    pub fn new(title: &'page str, text: &str) -> Self {
        match extract_macro_name(text) {
            MacroOf::RussianVerb(text) =>             WikiMacro { text: MacroOf::RussianVerb(text), page_title: title, is_head: true, is_infl: false, is_noun: true, is_verb: true, is_adj: false, is_adv: false },
            MacroOf::InflRussianVerb(text) =>         WikiMacro { text: MacroOf::InflRussianVerb(text), page_title: title, is_head: false, is_infl: true, is_noun: true, is_verb: true, is_adj: false, is_adv: false },
            MacroOf::RussianNoun(text) =>             WikiMacro { text: MacroOf::RussianNoun(text), page_title: title, is_head: true, is_infl: false, is_noun: true, is_verb: false, is_adj: false, is_adv: false },
            MacroOf::InflRussianNoun(text) =>         WikiMacro { text: MacroOf::InflRussianNoun(text), page_title: title, is_head: false, is_infl: true, is_noun: true, is_verb: false, is_adj: false, is_adv: false },
            MacroOf::RussianAdjective(text) =>        WikiMacro { text: MacroOf::RussianAdjective(text), page_title: title, is_head: true, is_infl: false, is_noun: false, is_verb: false, is_adj: true, is_adv: false },
            MacroOf::InflRussianAdjective(text) =>    WikiMacro { text: MacroOf::InflRussianAdjective(text), page_title: title, is_head: false, is_infl: true, is_noun: false, is_verb: false, is_adj: true, is_adv: false },
            MacroOf::RussianAdverb(text) =>           WikiMacro { text: MacroOf::RussianAdverb(text), page_title: title, is_head: true, is_infl: false, is_noun: false, is_verb: false, is_adj: false, is_adv: true },
            MacroOf::UkrainianVerb(text) =>           WikiMacro { text: MacroOf::UkrainianVerb(text), page_title: title, is_head: true, is_infl: false, is_noun: true, is_verb: true, is_adj: false, is_adv: false },
            MacroOf::InflUkrainianVerb(text) =>       WikiMacro { text: MacroOf::InflUkrainianVerb(text), page_title: title, is_head: false, is_infl: true, is_noun: true, is_verb: true, is_adj: false, is_adv: false },
            MacroOf::UkrainianNoun(text) =>           WikiMacro { text: MacroOf::UkrainianNoun(text), page_title: title, is_head: true, is_infl: false, is_noun: true, is_verb: false, is_adj: false, is_adv: false },
            MacroOf::InflUkrainianNoun(text) =>       WikiMacro { text: MacroOf::InflUkrainianNoun(text), page_title: title, is_head: false, is_infl: true, is_noun: true, is_verb: false, is_adj: false, is_adv: false },
            MacroOf::UkrainianAdjective(text) =>      WikiMacro { text: MacroOf::UkrainianAdjective(text), page_title: title, is_head: true, is_infl: false, is_noun: false, is_verb: false, is_adj: true, is_adv: false },
            MacroOf::InflUkrainianAdjective(text) =>  WikiMacro { text: MacroOf::InflUkrainianAdjective(text), page_title: title, is_head: false, is_infl: true, is_noun: false, is_verb: false, is_adj: true, is_adv: false },
            MacroOf::UkrainianAdverb(text) =>         WikiMacro { text: MacroOf::UkrainianAdverb(text), page_title: title, is_head: true, is_infl: false, is_noun: false, is_verb: false, is_adj: false, is_adv: true },
            MacroOf::BelarusianVerb(text) =>          WikiMacro { text: MacroOf::BelarusianVerb(text), page_title: title, is_head: true, is_infl: false, is_noun: true, is_verb: true, is_adj: false, is_adv: false },
            MacroOf::InflBelarusianVerb(text) =>      WikiMacro { text: MacroOf::InflBelarusianVerb(text), page_title: title, is_head: false, is_infl: true, is_noun: true, is_verb: true, is_adj: false, is_adv: false },
            MacroOf::BelarusianNoun(text) =>          WikiMacro { text: MacroOf::BelarusianNoun(text), page_title: title, is_head: true, is_infl: false, is_noun: true, is_verb: false, is_adj: false, is_adv: false },
            MacroOf::InflBelarusianNoun(text) =>      WikiMacro { text: MacroOf::InflBelarusianNoun(text), page_title: title, is_head: false, is_infl: true, is_noun: true, is_verb: false, is_adj: false, is_adv: false },
            MacroOf::BelarusianAdjective(text) =>     WikiMacro { text: MacroOf::BelarusianAdjective(text), page_title: title, is_head: true, is_infl: false, is_noun: false, is_verb: false, is_adj: true, is_adv: false },
            MacroOf::InflBelarusianAdjective(text) => WikiMacro { text: MacroOf::InflBelarusianAdjective(text), page_title: title, is_head: false, is_infl: true, is_noun: false, is_verb: false, is_adj: true, is_adv: false },
            MacroOf::BelarusianAdverb(text) =>        WikiMacro { text: MacroOf::BelarusianAdverb(text), page_title: title, is_head: true, is_infl: false, is_noun: false, is_verb: false, is_adj: false, is_adv: true },
        }
    }
}