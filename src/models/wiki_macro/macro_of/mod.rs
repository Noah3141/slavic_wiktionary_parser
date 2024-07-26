
#[derive(Debug)]
pub enum MacroOf {
    //
    RussianVerb(RussianVerbMacro), 
    InflRussianVerb(InflRussianVerbMacro),
    RussianNoun(RussianNounMacro),
    InflRussianNoun(InflRussianNounMacro),
    RussianAdjective(RussianAdjectiveMacro),
    InflRussianAdjective(InflRussianAdjectiveMacro),
    RussianAdverb(RussianAdverbMacro),
    //
    UkrainianVerb(UkrainianVerbMacro),
    InflUkrainianVerb(InflUkrainianVerbMacro),
    UkrainianNoun(UkrainianNounMacro),
    InflUkrainianNoun(InflUkrainianNounMacro),
    UkrainianAdjective(UkrainianAdjectiveMacro),
    InflUkrainianAdjective(InflUkrainianAdjectiveMacro),
    UkrainianAdverb(UkrainianAdverbMacro),
    //
    BelarusianVerb(BelarusianVerbMacro),
    InflBelarusianVerb(InflBelarusianVerbMacro),
    BelarusianNoun(BelarusianNounMacro),
    InflBelarusianNoun(InflBelarusianNounMacro),
    BelarusianAdjective(BelarusianAdjectiveMacro),
    InflBelarusianAdjective(InflBelarusianAdjectiveMacro),
    BelarusianAdverb(BelarusianAdverbMacro),
    //
}

pub mod russian_verb_macro;
pub mod infl_russian_verb_macro;
pub mod russian_noun_macro;
pub mod infl_russian_noun_macro;
pub mod russian_adjective_macro;
pub mod infl_russian_adjective_macro;
pub mod russian_adverb_macro;
pub mod ukrainian_verb_macro;
pub mod infl_ukrainian_verb_macro;
pub mod ukrainian_noun_macro;
pub mod infl_ukrainian_noun_macro;
pub mod ukrainian_adjective_macro;
pub mod infl_ukrainian_adjective_macro;
pub mod ukrainian_adverb_macro;
pub mod belarusian_verb_macro;
pub mod infl_belarusian_verb_macro;
pub mod belarusian_noun_macro;
pub mod infl_belarusian_noun_macro;
pub mod belarusian_adjective_macro;
pub mod infl_belarusian_adjective_macro;
pub mod belarusian_adverb_macro;
use russian_verb_macro::RussianVerbMacro;
use infl_russian_verb_macro::InflRussianVerbMacro;
use russian_noun_macro::RussianNounMacro;
use infl_russian_noun_macro::InflRussianNounMacro;
use russian_adjective_macro::RussianAdjectiveMacro;
use infl_russian_adjective_macro::InflRussianAdjectiveMacro;
use russian_adverb_macro::RussianAdverbMacro;
use ukrainian_verb_macro::UkrainianVerbMacro;
use infl_ukrainian_verb_macro::InflUkrainianVerbMacro;
use ukrainian_noun_macro::UkrainianNounMacro;
use infl_ukrainian_noun_macro::InflUkrainianNounMacro;
use ukrainian_adjective_macro::UkrainianAdjectiveMacro;
use infl_ukrainian_adjective_macro::InflUkrainianAdjectiveMacro;
use ukrainian_adverb_macro::UkrainianAdverbMacro;
use belarusian_verb_macro::BelarusianVerbMacro;
use infl_belarusian_verb_macro::InflBelarusianVerbMacro;
use belarusian_noun_macro::BelarusianNounMacro;
use infl_belarusian_noun_macro::InflBelarusianNounMacro;
use belarusian_adjective_macro::BelarusianAdjectiveMacro;
use infl_belarusian_adjective_macro::InflBelarusianAdjectiveMacro;
use belarusian_adverb_macro::BelarusianAdverbMacro;





















