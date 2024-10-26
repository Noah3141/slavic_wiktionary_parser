use std::{
    fs::File,
    io::{BufReader, Write}, os::windows::process,
};
use wiktionary_parser::models::{language::Language, wiktionary_macro::*};

mod constants;
mod processes;
mod traits;
mod utils;
mod funcs;

/// 1) `processes::dump_to_filtered`
/// 2) `processes::filtered_to_json`
/// 3) `processes::json_to_entry_csv`
/// 4) `processes::entry_csv_to_lemma_csv`
#[tokio::main]
async fn main() {
    // processes::json_to_entry_csv(
    //     "C:\\Users\\Noah3\\Code\\slavic_wiktionary_parser\\data\\parsed\\russian.json",
    //     "C:\\Users\\Noah3\\Code\\slavic_wiktionary_parser\\data\\russian_complete\\entries_002.csv",
    //     Language::Russian,
    // )
    // .await
    // .expect("the best");

    // processes::entry_csv_to_lemma_csv(
    //     "C:\\Users\\Noah3\\Code\\slavic_wiktionary_parser\\data\\russian_complete\\entries_002.csv",
    //     "C:\\Users\\Noah3\\Code\\slavic_wiktionary_parser\\data\\russian_complete\\form_lemma_002.csv",
    //     true,
    //     Language::Russian,
    // )
    // .expect("success")


    processes::extract_meanings(
        "./data/filtered_wiki_dump/russian.txt",
        "./data/meanings/russian.txt",
        &Language::Russian,
        true
    ).expect("Success!");
}

async fn explore() {
    println!("Reading file...");
    let reader = BufReader::with_capacity(
        1024 * 1024 * 250,
        File::open("C:/Users/Noah3/Code/slavic_wiktionary_parser/data/parsed/russian.json")
            .unwrap(),
    );
    let wiki_macros: Vec<WiktionaryMacro> = serde_json::from_reader(reader).unwrap();
    println!("Finished!");

    let client = reqwest::Client::new();

    let items = wiki_macros
        .into_iter()
        .filter_map(|m| match m {
            WiktionaryMacro::RuNounTable(n) => {
                if n.lemma().len() < 3 {
                    Some(n)
                } else {
                    None
                }
            }
            _ => None,
        })
        .take(30);
    // .take(3);

    for item in items {
        println!(
            "\n\n{} - {} -> {}",
            item.macro_text,
            item.page_title,
            item.lemma()
        );
        let mut writer = File::options().write(true).create(true).truncate(true).open(format!(
            "C:\\Users\\Noah3\\Code\\slavic_wiktionary_parser\\data\\examples\\ru-decl-adj_html\\{}-err.html", item.page_title
        )).expect("writer file");

        let res = &item.expand_with(&client).await;
        writer.write(&mut res.as_bytes()).expect("writing to file");
    }
}
