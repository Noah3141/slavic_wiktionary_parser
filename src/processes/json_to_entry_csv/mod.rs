
use std::{fs::File, io::{BufReader, BufWriter, Read, Write}};
use wiktionary_parser::utils::select_from;
use wiktionary_parser::models::{
    language::Language, 
    wiktionary_macro::{
        Expand, 
        WiktionaryMacro
    }
};


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



    println!("Cache reference ready with {} forms.\nLast form reached was '{}'", present_lemmas.len(), present_lemmas.last().unwrap_or(&"None yet"));

    println!("\nBeginning Entry JSON process.");
    let mut writer = BufWriter::with_capacity(1024 * 1024 * 6, out_file);
    let mut i: usize = 0;

    
    match language {
        Language::Russian => {
            // Manual overrides:
            present_lemmas.insert(0,"ка\u{301}мхорить");
            present_lemmas.insert(0,"ны\u{301}кать");
            println!("Processing Russian...");

            use wiktionary_parser::models::wiktionary_macro::russian;
            use rubit_api_db::dictionary_info::russian::*;
            use wiktionary_parser::models::wiktionary_macro::russian::{
                RuConj,  ru_conj,
                RuDeclAdj,  ru_decl_adj,
                RuNounTable, ru_noun_table
            };

            // Russian
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
                if i % 50 == 0 { println!("Processing: {}", &ru_conj.macro_text) }
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
                    format!("{id}, {lemma}, {commonality}, {pos_type}, {dictionary_info}\n", id="NULL", lemma=ru_conj.lemma(), commonality="NULL", pos_type="Verb", dictionary_info=dictionary_info ).as_bytes()
                ).expect("writing of bytes");
            }
            println!("Verbs complete!");
            

            // Russian
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
                .filter(|m| !m.macro_text.starts_with("-"))
                .filter(|m| !m.is_old() && !m.is_pronoun());
            
            println!("Nouns not yet processed: {}", ru_noun_declensions.clone().count());
            for ru_noun_table in ru_noun_declensions {
                i += 1;
                if i % 50 == 0 { println!("Processing: {}", &ru_noun_table.macro_text) }
                
                let ipa_string = { 
                    if let Some(m) = wiki_macros.iter()
                        .filter_map(|m| match m { WiktionaryMacro::RuIpa(m) => Some(m), _ => None})
                        .find(|ipa_m| { 
                                    ipa_m.page_id == ru_noun_table.page_id // This can be wrong due to complex pages, but should probably be over 80% accurate 95% of the time... 
                        }) {
                            m.to_ipa_string(&client).await
                        }
                    else {
                        if let Some(ipa) = wiki_macros.iter()
                            .filter_map(|m| match m { WiktionaryMacro::Ipa(ipa) => Some(ipa), _ => None})
                            .find(|ipa_m| { 
                                        ipa_m.page_id == ru_noun_table.page_id // This can be wrong due to complex pages, but should probably be over 80% accurate 95% of the time... 
                            }) {
                                ipa.to_ipa_string(&client).await
                            }
                        else { println!("No ipa at all! {}", &ru_noun_table.macro_text); continue }
                    }
                };

                let declension = ru_noun_table.html(&client).await;
                let dictionary_info = serde_json::to_string(
                    &RussianNoun::build_from_ru_noun_table(&ru_noun_table, ipa_string, &declension)
                ).expect("serialization of db dictionary_info model as json");
                
                writer.write(
                    format!("{id}, {lemma}, {commonality}, {pos_type}, {dictionary_info}\n", id="NULL", lemma=ru_noun_table.lemma(), commonality="NULL", pos_type="Noun", dictionary_info=dictionary_info ).as_bytes()
                ).expect("writing of bytes");
            }
            println!("Nouns complete!");
            
            // Russian
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
                if i % 50 == 0 { println!("Processing: {}", &ru_adj_decl.macro_text) }
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
                    format!("{id}, {lemma}, {commonality}, {pos_type}, {dictionary_info}\n", id="NULL", lemma=ru_adj_decl.lemma(), commonality="NULL", pos_type="Adjective", dictionary_info=dictionary_info ).as_bytes()
                ).expect("writing of bytes");
            }

            println!("Adjectives complete!");

            writer.flush().expect("flush!");

            println!("Entry CSV complete!");
            
            Ok(())
        },
        Language::Ukrainian => {
            println!("Processing Ukrainian...");

            use wiktionary_parser::models::wiktionary_macro::ukrainian;
            use rubit_api_db::dictionary_info::ukrainian::*;
            use wiktionary_parser::models::wiktionary_macro::ukrainian::{
                UkConj,  uk_conj,
                UkADecl,  uk_adecl,
                UkNDecl, uk_ndecl
            };

            // Ukrainian
            // Verbs
            //
            let client = reqwest::Client::builder()
                .pool_idle_timeout(None)
                .build()
                .expect("Client build process");

            let uk_conjugations = wiki_macros.iter()
                .filter_map(|m| match m { WiktionaryMacro::UkConj(n) => Some(n), _ => None })
                .filter(|m| !present_lemmas.contains(&m.lemma().trim()));
                // .filter(|m| !m.is_old());
            
            println!("Verbs not yet processed: {}", uk_conjugations.clone().count());
            for uk_conj in uk_conjugations {
                i += 1;
                if i % 50 == 0 { println!("Processing: {}", &uk_conj.macro_text) }
                let ipa_string = wiki_macros.iter()
                    .filter_map(|m| match m { WiktionaryMacro::UkIpa(n) => Some(n), _ => None})
                    .find(|ipa_m| { 
                                ipa_m.page_id == uk_conj.page_id
                    })
                    .expect(format!("ipa_string corresponding to uk_conj: {}", &uk_conj.macro_text).as_str())
                    .to_ipa_string(&client)
                    .await;
                let conjugation = uk_conj.html(&client).await;
                let dictionary_info = serde_json::to_string(
                    &UkrainianVerb::build_from_uk_conj(&uk_conj, ipa_string, &conjugation)
                ).expect("serialization of db dictionary_info model as json");
                
                writer.write(
                    format!("{id}, {lemma}, {commonality}, {pos_type}, {dictionary_info}\n", id="NULL", lemma=uk_conj.lemma(), commonality="NULL", pos_type="Verb", dictionary_info=dictionary_info ).as_bytes()
                ).expect("writing of bytes");
            }
            println!("Verbs complete!");
            

            // Ukrainian
            // Noun
            //
            drop(client);
            let client = reqwest::Client::builder()
                .pool_idle_timeout(None)
                .build()
                .expect("Client build process");
    
            let uk_noun_tables = wiki_macros.iter()
                .filter_map(|m| match m { WiktionaryMacro::UkNDecl(n) => Some(n), _ => None })
                .filter(|m| !present_lemmas.contains(&m.lemma().trim()))
                .filter(|m| !m.macro_text.starts_with("-"));
                // .filter(|m| !m.is_old() && !m.is_pronoun());
            
            println!("Nouns not yet processed: {}", uk_noun_tables.clone().count());
            for uk_noun_table in uk_noun_tables {
                i += 1;
                if i % 50 == 0 { println!("Processing: {}", &uk_noun_table.macro_text) }
                
                let ipa_string = { 
                    if let Some(m) = wiki_macros.iter()
                        .filter_map(|m| match m { WiktionaryMacro::UkIpa(m) => Some(m), _ => None})
                        .find(|ipa_m| { 
                                    ipa_m.page_id == uk_noun_table.page_id // This can be wrong due to complex pages, but should probably be over 80% accurate 95% of the time... 
                        }) {
                            m.to_ipa_string(&client).await
                        }
                    else {
                        if let Some(ipa) = wiki_macros.iter()
                            .filter_map(|m| match m { WiktionaryMacro::Ipa(ipa) => Some(ipa), _ => None})
                            .find(|ipa_m| { 
                                        ipa_m.page_id == uk_noun_table.page_id // This can be wrong due to complex pages, but should probably be over 80% accurate 95% of the time... 
                            }) {
                                ipa.to_ipa_string(&client).await
                            }
                        else { println!("No ipa at all! {}", &uk_noun_table.macro_text); continue }
                    }
                };

                let declension = uk_noun_table.html(&client).await;
                let dictionary_info = serde_json::to_string(
                    &UkrainianNoun::build_from_uk_ndecl(&uk_noun_table, ipa_string, &declension)
                ).expect("serialization of db dictionary_info model as json");
                
                writer.write(
                    format!("{id}, {lemma}, {commonality}, {pos_type}, {dictionary_info}\n", id="NULL", lemma=uk_noun_table.lemma(), commonality="NULL", pos_type="Noun", dictionary_info=dictionary_info ).as_bytes()
                ).expect("writing of bytes");
            }
            println!("Nouns complete!");
            
            // Ukrainian
            // Adjectives
            //
            drop(client);
            let client = reqwest::Client::builder()
                .pool_idle_timeout(None)
                .build()
                .expect("Client build process");
        
            let uk_adj_tables = wiki_macros.iter()
                .filter_map(|m| match m { WiktionaryMacro::UkADecl(n) => Some(n), _ => None })
                .filter(|m| !present_lemmas.contains(&m.lemma().trim()));
                // .filter(|m| !m.is_old());
            
            println!("Adjectives not yet processed: {}", uk_adj_tables.clone().count());
            for uk_adj_table in uk_adj_tables {
                i += 1;
                if i % 50 == 0 { println!("Processing: {}", &uk_adj_table.macro_text) }
                let ipa_string = wiki_macros.iter()
                    .filter_map(|m| match m { WiktionaryMacro::UkIpa(n) => Some(n), _ => None})
                    .find(|ipa_m| { 
                                ipa_m.page_id == uk_adj_table.page_id // This can be wrong due to complex pages, but should probably be over 80% accurate 95% of the time... 
                    })
                    .expect("ipa_string corresponding to ru_adj_Decl")
                    .to_ipa_string(&client)
                    .await;
                let declension = uk_adj_table.html(&client).await;
                let dictionary_info = serde_json::to_string(
                    &UkrainianAdjective::build_from_uk_adecl(&uk_adj_table, ipa_string, &declension)
                ).expect("serialization of db dictionary_info model as json");
                
                writer.write(
                    format!("{id}, {lemma}, {commonality}, {pos_type}, {dictionary_info}\n", id="NULL", lemma=uk_adj_table.lemma(), commonality="NULL", pos_type="Adjective", dictionary_info=dictionary_info ).as_bytes()
                ).expect("writing of bytes");
            }

            println!("Adjectives complete!");

            writer.flush().expect("flush!");

            println!("Entry CSV complete!");
            
            Ok(())
        },
        Language::Belarusian => {
            println!("Processing Belarusian...");

            present_lemmas.insert(0, "да́ныя");
            present_lemmas.insert(0, "клубни́ка");
            present_lemmas.insert(0, "Бяро́заsg&gt;");

            use wiktionary_parser::models::wiktionary_macro::belarusian;
            use rubit_api_db::dictionary_info::belarusian::*;
            use wiktionary_parser::models::wiktionary_macro::belarusian::{
                BeConj,  be_conj,
                BeADecl,  be_adecl,
                BeNDecl, be_ndecl
            };

            // Belarusian
            // Verbs
            //
            let client = reqwest::Client::builder()
                .pool_idle_timeout(None)
                .build()
                .expect("Client build process");

            let be_conjugations = wiki_macros.iter()
                .filter_map(|m| match m { WiktionaryMacro::BeConj(n) => Some(n), _ => None })
                .filter(|m| !present_lemmas.contains(&m.lemma().trim()));
                // .filter(|m| !m.is_old());
            
            println!("Verbs not yet processed: {}", be_conjugations.clone().count());
            for be_conj in be_conjugations {
                i += 1;
                if i % 50 == 0 { println!("Processing: {}", &be_conj.macro_text) }
                let ipa_string = { 
                    if let Some(m) = wiki_macros.iter()
                        .filter_map(|m| match m { WiktionaryMacro::BeIpa(m) => Some(m), _ => None})
                        .find(|ipa_m| { 
                                    ipa_m.page_id == be_conj.page_id // This can be wrong due to complex pages, but should probably be over 80% accurate 95% of the time... 
                        }) {
                            m.to_ipa_string(&client).await
                        }
                    else {
                        if let Some(ipa) = wiki_macros.iter()
                            .filter_map(|m| match m { WiktionaryMacro::Ipa(ipa) => Some(ipa), _ => None})
                            .find(|ipa_m| { 
                                        ipa_m.page_id == be_conj.page_id // This can be wrong due to complex pages, but should probably be over 80% accurate 95% of the time... 
                            }) {
                                ipa.to_ipa_string(&client).await
                            }
                        else { println!("No ipa at all! {}", &be_conj.macro_text); continue }
                    }
                };
                let conjugation = be_conj.html(&client).await;
                let dictionary_info = serde_json::to_string(
                    &BelarusianVerb::build_from_be_conj(&be_conj, ipa_string, &conjugation)
                ).expect("serialization of db dictionary_info model as json");
                
                writer.write(
                    format!("{id}, {lemma}, {commonality}, {pos_type}, {dictionary_info}\n", id="NULL", lemma=be_conj.lemma(), commonality="NULL", pos_type="Verb", dictionary_info=dictionary_info ).as_bytes()
                ).expect("writing of bytes");
            }
            println!("Verbs complete!");
            

            // Belarusian
            // Noun
            //
            drop(client);
            let client = reqwest::Client::builder()
                .pool_idle_timeout(None)
                .build()
                .expect("Client build process");
    
            let be_noun_tables = wiki_macros.iter()
                .filter_map(|m| match m { WiktionaryMacro::BeNDecl(n) => Some(n), _ => None })
                .filter(|m| !present_lemmas.contains(&m.lemma().trim()))
                .filter(|m| !m.macro_text.starts_with("-"));
                // .filter(|m| !m.is_old() && !m.is_pronoun());
            
            println!("Nouns not yet processed: {}", be_noun_tables.clone().count());
            for be_noun_table in be_noun_tables {
                i += 1;
                if i % 50 == 0 { println!("Processing: {}", &be_noun_table.macro_text) }
                
                let ipa_string = { 
                    if let Some(m) = wiki_macros.iter()
                        .filter_map(|m| match m { WiktionaryMacro::BeIpa(m) => Some(m), _ => None})
                        .find(|ipa_m| { 
                                    ipa_m.page_id == be_noun_table.page_id // This can be wrong due to complex pages, but should probably be over 80% accurate 95% of the time... 
                        }) {
                            m.to_ipa_string(&client).await
                        }
                    else {
                        if let Some(ipa) = wiki_macros.iter()
                            .filter_map(|m| match m { WiktionaryMacro::Ipa(ipa) => Some(ipa), _ => None})
                            .find(|ipa_m| { 
                                        ipa_m.page_id == be_noun_table.page_id // This can be wrong due to complex pages, but should probably be over 80% accurate 95% of the time... 
                            }) {
                                ipa.to_ipa_string(&client).await
                            }
                        else { println!("No ipa at all! {}", &be_noun_table.macro_text); continue }
                    }
                };

                let declension = be_noun_table.html(&client).await;
                let dictionary_info = serde_json::to_string(
                    &BelarusianNoun::build_from_be_ndecl(&be_noun_table, ipa_string, &declension)
                ).expect("serialization of db dictionary_info model as json");
                
                writer.write(
                    format!("{id}, {lemma}, {commonality}, {pos_type}, {dictionary_info}\n", id="NULL", lemma=be_noun_table.lemma(), commonality="NULL", pos_type="Noun", dictionary_info=dictionary_info ).as_bytes()
                ).expect("writing of bytes");
            }
            println!("Nouns complete!");
            
            // Belarusian
            // Adjectives
            //
            drop(client);
            let client = reqwest::Client::builder()
                .pool_idle_timeout(None)
                .build()
                .expect("Client build process");
        
            let be_adj_tables = wiki_macros.iter()
                .filter_map(|m| match m { WiktionaryMacro::BeADecl(n) => Some(n), _ => None })
                .filter(|m| !present_lemmas.contains(&m.lemma().trim()))
                .filter(|m| !m.is_surname());
                // .filter(|m| !m.is_old());
            
            println!("Adjectives not yet processed: {}", be_adj_tables.clone().count());
            for be_adj_table in be_adj_tables {
                i += 1;
                if i % 50 == 0 { println!("Processing: {}", &be_adj_table.macro_text) }
                let ipa_string = wiki_macros.iter()
                    .filter_map(|m| match m { WiktionaryMacro::BeIpa(n) => Some(n), _ => None})
                    .find(|ipa_m| { 
                                ipa_m.page_id == be_adj_table.page_id // This can be wrong due to complex pages, but should probably be over 80% accurate 95% of the time... 
                    })
                    .expect("ipa_string corresponding to ru_adj_Decl")
                    .to_ipa_string(&client)
                    .await;
                let declension = be_adj_table.html(&client).await;
                let dictionary_info = serde_json::to_string(
                    &BelarusianAdjective::build_from_be_adecl(&be_adj_table, ipa_string, &declension)
                ).expect("serialization of db dictionary_info model as json");
                
                writer.write(
                    format!("{id}, {lemma}, {commonality}, {pos_type}, {dictionary_info}\n", id="NULL", lemma=be_adj_table.lemma(), commonality="NULL", pos_type="Adjective", dictionary_info=dictionary_info ).as_bytes()
                ).expect("writing of bytes");
            }

            println!("Adjectives complete!");

            writer.flush().expect("flush!");

            println!("Entry CSV complete!");
            
            Ok(())
        },
    }
}