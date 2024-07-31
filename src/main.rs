use std::{fs::File, io::{BufReader, Write}};

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
    let reader = BufReader::with_capacity(
        1024*1024*250, 
        File::open("C:/Users/Noah3/Code/slavic_wiktionary_parser/data/parsed/russian.json").unwrap()
    );
    let wiki_macros: Vec<WiktionaryMacro> = serde_json::from_reader(reader).unwrap();
    println!("Finished!");

    // for item in wiki_macros.into_iter().filter_map(|m| match m {
    //     WiktionaryMacro::InflectionOf(m) => Some(m),
    //     _ => None
    // })
    // .step_by(15)
    // .take(150) {
    //     println!("{:#?}", item.form_and_lemma())
    // }

    let client = reqwest::Client::new();

    let items = wiki_macros.into_iter()
        .filter_map(|m| 
            match m {
                WiktionaryMacro::RuIpa(n) => Some(n),
                // WiktionaryMacro::RuAdj(n) => Some(n),
                // WiktionaryMacro::InflectionOf(m) => Some(m),
                _ => None
            }
        )
        .step_by(35)
        .take(3);

    
    for item in items {
        println!("{} - {}", item.macro_text, item.page_title);
        let mut writer = File::options().write(true).create(true).truncate(true).open(format!(
            "C:\\Users\\Noah3\\Code\\slavic_wiktionary_parser\\data\\examples\\ru-ipa_html\\{}.html", item.page_title
        )).expect("writer file");

        println!("{item:#?}");
        let res = &item.expand_with(&client).await;
        writer.write(&mut res.as_bytes()).expect("writing to file");
    
    }

}
