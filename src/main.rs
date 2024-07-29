use std::{fs::File, io::BufReader};

use wiktionary_parser::models::{language::Language, wiktionary_macro::{
    russian::*, WiktionaryMacro 
}};

mod processes;
mod traits;
mod utils;
mod constants;

fn main() {
    
    // processes::dump_to_filtered(
    //     "C:/Users/Noah3/Code/slavic_wiktionary_parser/data/wiki_dumps/en_wiktionary.xml",
    //     "C:/Users/Noah3/Code/slavic_wiktionary_parser/data/filtered_wiki_dump/ukrainian.txt",
    //     &["==Ukrainian=="]
    // ).expect("success");
    // processes::dump_to_filtered(
    //     "C:/Users/Noah3/Code/slavic_wiktionary_parser/data/wiki_dumps/en_wiktionary.xml",
    //     "C:/Users/Noah3/Code/slavic_wiktionary_parser/data/filtered_wiki_dump/russian.txt",
    //     &["==Russian=="]
    // ).expect("success");
    // processes::dump_to_filtered(
    //     "C:/Users/Noah3/Code/slavic_wiktionary_parser/data/wiki_dumps/en_wiktionary.xml",
    //     "C:/Users/Noah3/Code/slavic_wiktionary_parser/data/filtered_wiki_dump/belarusian.txt",
    //     &["==Belarusian=="]
    // ).expect("success");

    // processes::filtered_to_json(
    //     "C:/Users/Noah3/Code/slavic_wiktionary_parser/data/filtered_wiki_dump/belarusian.txt",
    //     "C:/Users/Noah3/Code/slavic_wiktionary_parser/data/parsed/belarusian.json",
    //     true,
    //     Language::Belarusian
    // ).expect("success");
    // processes::filtered_to_json(
    //     "C:/Users/Noah3/Code/slavic_wiktionary_parser/data/filtered_wiki_dump/russian.txt",
    //     "C:/Users/Noah3/Code/slavic_wiktionary_parser/data/parsed/russian.json",
    //     true,
    //     Language::Russian
    // ).expect("success");
    // processes::filtered_to_json(
    //     "C:/Users/Noah3/Code/slavic_wiktionary_parser/data/filtered_wiki_dump/ukrainian.txt",
    //     "C:/Users/Noah3/Code/slavic_wiktionary_parser/data/parsed/ukrainian.json",
    //     true,
    //     Language::Ukrainian
    // ).expect("success");

    println!("Reading file...");
    let reader = BufReader::with_capacity(1024*1024*250, File::open("C:/Users/Noah3/Code/slavic_wiktionary_parser/data/parsed/belarusian.json").unwrap());
    let wiki_macros: Vec<WiktionaryMacro> = serde_json::from_reader(reader).unwrap();
    println!("Finished!");

    let noun_count = wiki_macros.iter()
        .filter_map(|m| match m {
            WiktionaryMacro::BeNoun(m) => Some(m),
            _ => None,
        })
        .count();
    let verb_count: usize = wiki_macros.iter()
        .filter_map(|m| match m {
            WiktionaryMacro::BeVerb(m) => Some(m),
            _ => None,
        })
        .count();
    let adj_count: usize = wiki_macros.iter()
        .filter_map(|m| match m {
            WiktionaryMacro::BeAdj(m) => Some(m),
            _ => None,
        })
        .count();

    let inflected_count: usize = wiki_macros.iter()
        .filter_map(|m| {match m {
            WiktionaryMacro::InflectionOf(m) => Some(m),
            _ => None,
        }})
        .filter(|infl| {infl.language == Language::Belarusian})
        .count();

    println!("--Belarusian--");
    println!("Nouns: {noun_count}");
    println!("Verbs: {verb_count}");
    println!("Adjs: {adj_count}");
    println!("Total: {}", noun_count + verb_count + adj_count );
    println!("-------------");

    println!("Inflected: {inflected_count}");

}
