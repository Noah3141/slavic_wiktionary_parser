
use std::{fs::File, io::{BufReader, BufWriter, Write}};

use wiktionary_parser::models::{
    language::Language, 
    wiktionary_macro::{
        Expand, 
        WiktionaryMacro
    }
};


pub async fn json_to_entry_json(
    json_file: &str, 
    out: &str, 
    overwrite: bool,
    language: Language,
) -> Result<(), ()> {
    println!("Reading file...");
    let reader = BufReader::with_capacity(
        1024*1024*250, 
        File::open(json_file).unwrap()
    );
    let wiki_macros: Vec<WiktionaryMacro> = serde_json::from_reader(reader).unwrap();
    println!("Finished!");

    let out_file =  match overwrite {
        true =>  File::options()
            .write(true)
            .truncate(true)
            .create(true)
            .open(out)
            .expect("creation of out file with overwrite"),
        false => File::options()
            .write(true)
            .create_new(true)
            .open(out)
            .expect("creation of out file without overwriting")
    };

    println!("\nBeginning Entry JSON process.");
    let mut writer = BufWriter::with_capacity(1024 * 1024 * 6, out_file);
    let mut i: usize = 0;

    
    match language {
        Language::Russian => {
            println!("Processing Russian...");

            use wiktionary_parser::models::wiktionary_macro::russian;
            use rubit_api_db::dictionary_info::russian::*;
            use wiktionary_parser::models::wiktionary_macro::russian::{
                RuConj,  ru_conj,
                RuDeclAdj,  ru_decl_adj,
                RuNounTable, ru_noun_table
            };

            let client = reqwest::Client::builder()
                .pool_idle_timeout(None)
                .build()
                .expect("Client build process");

            let ru_conjugations = wiki_macros.iter()
                .filter_map(|m| match m { WiktionaryMacro::RuConj(n) => Some(n), _ => None })
                .filter(|t| !t.is_old());
            
            for ru_conj in ru_conjugations {
                i += 1;
                if i % 50 == 0 {
                    println!("Processing: {}", &ru_conj.macro_text)
                }
                let ipa_string = wiki_macros.iter()
                    .filter_map(|m| match m { WiktionaryMacro::RuIpa(n) => Some(n), _ => None})
                    .find(|ipa_m| { 
                                ipa_m.page_id == ru_conj.page_id // This can be wrong due to complex pages, but should probably be over 80% accurate 95% of the time... 
                    })
                    .expect("ipa_string corresponding to ru_conj")
                    .to_ipa_string(&client)
                    .await;
                let conjugation = ru_conj.html(&client).await;
                let dictionary_info = serde_json::to_string(
                    &RussianVerb::build_from_ru_conj(&ru_conj, ipa_string, &conjugation)
                ).expect("serialization of db dictionary_info model as json");
                
                writer.write(
                    format!("{id}, {lemma}, {commonality}, {pos_type}, {dictionary_info}\n",
                            id="NULL",
                            lemma=ru_conj.lemma(),
                            commonality="NULL",
                            pos_type="Verb",
                            dictionary_info=dictionary_info
                    ).as_bytes()
                ).expect("writing of bytes");
            }
            println!("Verbs complete!");
            drop(client);

            let client = reqwest::Client::builder()
                .pool_idle_timeout(None)
                .build()
                .expect("Client build process");
    
            let ru_noun_declensions = wiki_macros.iter()
                .filter_map(|m| match m { WiktionaryMacro::RuNounTable(n) => Some(n), _ => None })
                .filter(|t| !t.is_old());
            for ru_noun_table in ru_noun_declensions {
                i += 1;
                if i % 50 == 0 {
                    println!("Processing: {}", &ru_noun_table.macro_text)
                }
                let ipa_string = wiki_macros.iter()
                    .filter_map(|m| match m { WiktionaryMacro::RuIpa(n) => Some(n), _ => None})
                    .find(|ipa_m| { 
                                ipa_m.page_id == ru_noun_table.page_id // This can be wrong due to complex pages, but should probably be over 80% accurate 95% of the time... 
                    })
                    .expect("ipa_string corresponding to ru_noun_table")
                    .to_ipa_string(&client)
                    .await;
                let declension = ru_noun_table.html(&client).await;
                let dictionary_info = serde_json::to_string(
                    &RussianNoun::build_from_ru_noun_table(&ru_noun_table, ipa_string, &declension)
                ).expect("serialization of db dictionary_info model as json");
                
                writer.write(
                    format!("{id}, {lemma}, {commonality}, {pos_type}, {dictionary_info}\n",
                            id="NULL",
                            lemma=ru_noun_table.lemma(),
                            commonality="NULL",
                            pos_type="Noun",
                            dictionary_info=dictionary_info
                    ).as_bytes()
                ).expect("writing of bytes");
            }
            println!("Nouns complete!");
            drop(client);

            let client = reqwest::Client::builder()
                .pool_idle_timeout(None)
                .build()
                .expect("Client build process");
        
            let ru_adj_declensions = wiki_macros.iter()
                .filter_map(|m| match m { WiktionaryMacro::RuDeclAdj(n) => Some(n), _ => None })
                .filter(|t| !t.is_old());

            for ru_adj_decl in ru_adj_declensions {
                i += 1;
                if i % 50 == 0 {
                    println!("Processing: {}", &ru_adj_decl.macro_text)
                }
                let ipa_string = wiki_macros.iter()
                    .filter_map(|m| match m { WiktionaryMacro::RuIpa(n) => Some(n), _ => None})
                    .find(|ipa_m| { 
                                ipa_m.page_id == ru_adj_decl.page_id // This can be wrong due to complex pages, but should probably be over 80% accurate 95% of the time... 
                    })
                    .expect("ipa_string corresponding to ru_adj_Decl")
                    .to_ipa_string(&client)
                    .await;
                let declension = ru_adj_decl.html(&client).await;
                let dictionary_info = serde_json::to_string(
                    &RussianAdjective::build_from_ru_decl_adj(&ru_adj_decl, ipa_string, &declension)
                ).expect("serialization of db dictionary_info model as json");
                
                writer.write(
                    format!("{id}, {lemma}, {commonality}, {pos_type}, {dictionary_info}\n",
                            id="NULL",
                            lemma=ru_adj_decl.lemma(),
                            commonality="NULL",
                            pos_type="Adjective",
                            dictionary_info=dictionary_info
                    ).as_bytes()
                ).expect("writing of bytes");
            }

            println!("Adjectives complete!");

            writer.flush().expect("flush!");

            println!("Entry CSV complete!");
            
            Ok(())
        },
        Language::Ukrainian => todo!(),
        Language::Belarusian => todo!(),
    }
}