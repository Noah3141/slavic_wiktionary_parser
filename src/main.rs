use std::{fs::File, io::{BufReader, Write}};

use russian::{ru_conj, RuConj};
use wiktionary_parser::models::{language::Language, wiktionary_macro::*};

mod processes;
mod traits;
mod utils;
mod constants;

#[tokio::main]
async fn main() {
    
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
    //     "C:/Users/Noah3/Code/slavic_wiktionary_parser/data/parsed/russian2.json",
    //     true,
    //     Language::Russian
    // ).expect("success");
    // processes::filtered_to_json(
    //     "C:/Users/Noah3/Code/slavic_wiktionary_parser/data/filtered_wiki_dump/ukrainian.txt",
    //     "C:/Users/Noah3/Code/slavic_wiktionary_parser/data/parsed/ukrainian.json",
    //     true,
    //     Language::Ukrainian
    // ).expect("success");

    // processes::json_to_form_lemma(
    //     "C:\\Users\\Noah3\\Code\\slavic_wiktionary_parser\\data\\parsed\\russian.json", 
    //     "C:\\Users\\Noah3\\Code\\slavic_wiktionary_parser\\data\\russian_complete\\form_lemma_verbs.csv", 
    //     false, 
    //     Language::Russian,
    // ).await.expect("json_to_form_lemma success");

    // processes::json_to_entry_csv(
    //     "C:\\Users\\Noah3\\Code\\slavic_wiktionary_parser\\data\\parsed\\belarusian.json", 
    //     "C:\\Users\\Noah3\\Code\\slavic_wiktionary_parser\\data\\belarusian_complete\\entries.csv", 
    //     Language::Belarusian,
    // ).await.expect("json_to_entry_csv success");

    processes::entry_csv_to_lemma_csv(
        "C:\\Users\\Noah3\\Code\\slavic_wiktionary_parser\\data\\belarusian_complete\\entries.csv",
        "C:\\Users\\Noah3\\Code\\slavic_wiktionary_parser\\data\\belarusian_complete\\form_lemma.csv",
        true,
        Language::Belarusian
    ).expect("");

    // explore()
}



async fn explore() {
    println!("Reading file...");
    let reader = BufReader::with_capacity(
        1024*1024*250, 
        File::open("C:/Users/Noah3/Code/slavic_wiktionary_parser/data/parsed/russian.json").unwrap()
    );
    let wiki_macros: Vec<WiktionaryMacro> = serde_json::from_reader(reader).unwrap();
    println!("Finished!");


    let client = reqwest::Client::new();

    let items = wiki_macros.into_iter()
        .filter_map(|m| match m {
                WiktionaryMacro::RuNounTable(n) => {
                    if n.lemma().len() < 3 {
                        Some(n)
                    } else {
                        None
                    }
                },
                _ => None
        })
        .take(30);
        // .take(3);

    
    for item in items {
        println!("\n\n{} - {} -> {}", item.macro_text, item.page_title, item.lemma());
        let mut writer = File::options().write(true).create(true).truncate(true).open(format!(
            "C:\\Users\\Noah3\\Code\\slavic_wiktionary_parser\\data\\examples\\ru-decl-adj_html\\{}-err.html", item.page_title
        )).expect("writer file");

        let res = &item.expand_with(&client).await;
        writer.write(&mut res.as_bytes()).expect("writing to file");
    
    }
}