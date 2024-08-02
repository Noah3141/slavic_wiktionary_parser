
use std::{fs::File, io::{BufReader, BufWriter, Read, Write}};

use wiktionary_parser::{models::{
    language::Language, 
    wiktionary_macro::{
        Expand, 
        WiktionaryMacro
    }
}, utils::select_from};


pub async fn json_to_entry_csv(
    json_file: &str, 
    out: &str, 
    language: Language,
) -> Result<(), ()> {
    println!("Reading file...");
    let reader = BufReader::with_capacity(
        1024*1024*250, 
        File::open(json_file).unwrap()
    );
    let wiki_macros: Vec<WiktionaryMacro> = serde_json::from_reader(reader).unwrap();
    println!("Finished!");

    let mut out_file = File::options()
        .read(true)
        .append(true)
        .truncate(false)
        .create(true)
        .open(out)
        .expect("creation of out file with overwrite");

    println!("Building cache reference...");

    let mut holding_string = String::with_capacity(1024*1024*2);
    out_file.read_to_string(&mut holding_string).expect("reading to holding string");
    let mut present_lemmas: Vec<&str> = holding_string
        .lines()
        .map(|line| {select_from(line, ",", ",").unwrap().trim() })
        .collect();

    // Manual overrides:
    present_lemmas.insert(0,"ка\u{301}мхорить");
    present_lemmas.insert(0,"ны\u{301}кать");

    println!("Cache reference ready with {} forms.\nLast form reached was '{}'", present_lemmas.len(), present_lemmas.last().unwrap_or(&"None yet"));

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

            //
            // Verbs
            //
            let client = reqwest::Client::builder()
                .pool_idle_timeout(None)
                .build()
                .expect("Client build process");

            let ru_conjugations = wiki_macros.iter()
                .filter_map(|m| match m { WiktionaryMacro::RuConj(n) => Some(n), _ => None })
                .filter(|m| !present_lemmas.contains(&m.lemma().trim()))
                .filter(|m| !m.is_old());
            
            println!("Verbs not yet processed: {}", ru_conjugations.clone().count());
            for ru_conj in ru_conjugations {
                i += 1;
                if i % 50 == 0 {
                    println!("Processing: {}", &ru_conj.macro_text)
                }
                let ipa_string = wiki_macros.iter()
                    .filter_map(|m| match m { WiktionaryMacro::RuIpa(n) => Some(n), _ => None})
                    .find(|ipa_m| { 
                                ipa_m.page_id == ru_conj.page_id
                    })
                    .expect(format!("ipa_string corresponding to ru_conj: {}", &ru_conj.macro_text).as_str())
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
            

            //
            // Noun
            //
            drop(client);
            let client = reqwest::Client::builder()
                .pool_idle_timeout(None)
                .build()
                .expect("Client build process");
    
            let ru_noun_declensions = wiki_macros.iter()
                .filter_map(|m| match m { WiktionaryMacro::RuNounTable(n) => Some(n), _ => None })
                .filter(|m| !present_lemmas.contains(&m.lemma().as_str().trim()))
                .filter(|m| !m.is_old());
            
            println!("Nouns not yet processed: {}", ru_noun_declensions.clone().count());
            for ru_noun_table in ru_noun_declensions {
                i += 1;
                if i % 50 == 0 {
                    println!("Processing: {}", &ru_noun_table.macro_text)
                }
                
                let ipa_string = { 
                    if let Some(m) = wiki_macros.iter()
                        .filter_map(|m| match m { WiktionaryMacro::RuIpa(m) => Some(m), _ => None})
                        .find(|ipa_m| { 
                                    ipa_m.page_id == ru_noun_table.page_id // This can be wrong due to complex pages, but should probably be over 80% accurate 95% of the time... 
                        }) {
                            m.to_ipa_string(&client).await
                        }
                    else {
                        wiki_macros.iter()
                            .filter_map(|m| match m { WiktionaryMacro::Ipa(ipa) => Some(ipa), _ => None})
                            .find(|ipa_m| { 
                                        ipa_m.page_id == ru_noun_table.page_id // This can be wrong due to complex pages, but should probably be over 80% accurate 95% of the time... 
                            }).expect("Finding IPA macro after not finding ru-IPA").to_ipa_string(&client).await
                    }
                };

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
            
            //
            // Adjectives
            //
            drop(client);
            let client = reqwest::Client::builder()
                .pool_idle_timeout(None)
                .build()
                .expect("Client build process");
        
            let ru_adj_declensions = wiki_macros.iter()
                .filter_map(|m| match m { WiktionaryMacro::RuDeclAdj(n) => Some(n), _ => None })
                .filter(|m| !present_lemmas.contains(&m.lemma().trim()))
                .filter(|m| !m.is_old());
            
            println!("Adjectives not yet processed: {}", ru_adj_declensions.clone().count());
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