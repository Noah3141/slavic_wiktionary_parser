use std::{fs::File, io::{BufReader, Write}};

use rubit_api_db::dictionary_info;
use serde_json::json;
use wiktionary_parser::models::{
    language::Language, 
    wiktionary_macro::{russian, Expand, WiktionaryMacro
}};




pub async fn json_to_entry_csv(
    json_file: &str, 
    out: &str, 
    overwrite: bool,
    language: Language,
) {
    println!("Reading file...");
    let reader = BufReader::with_capacity(
        1024*1024*250, 
        File::open(json_file).unwrap()
    );
    let wiki_macros: Vec<WiktionaryMacro> = serde_json::from_reader(reader).unwrap();
    println!("Finished!");

    let mut out_file =  match overwrite {
        true =>  File::options()
            .append(true)
            .create(true)
            .truncate(true)
            .open(out)
            .expect("creation of out file with overwrite"),
        false => File::options()
            .append(true)
            .create_new(true)
            .truncate(false)
            .open(out)
            .expect("creation of out file without overwriting")
    };

    let client = reqwest::Client::new();
    
    match language {
        Language::Russian => {
            use wiktionary_parser::models::wiktionary_macro::russian;
            use wiktionary_parser::models::wiktionary_macro::russian::{
                RuConj,  ru_conj,
                RuDeclAdj,  ru_decl_adj,
                RuNounTable, ru_noun_table
            };

            let ru_conjugations = wiki_macros.iter()
            .filter_map(|m| match m { WiktionaryMacro::RuConj(n) => Some(n), _ => None })
            .filter(|t| !t.is_old());
            for ru_conj in ru_conjugations {

                let ipa_string = wiki_macros.iter()
                    .filter_map(|m| match m { WiktionaryMacro::RuIpa(n) => Some(n), _ => None})
                    .find(|ipa_m| { 
                                ipa_m.page_id == ru_conj.page_id // This can be wrong due to complex pages, but should probably be over 80% accurate 95% of the time... 
                    })
                    .expect("ipa_string corresponding to ru_conj")
                    .to_ipa_string(&client)
                    .await;

                let conjugation = ru_conj.html(&client).await;
                
                out_file.write(
                    format!("{id}, {lemma}, {commonality}, {pos_type}, {dictionary_info}",
                            id="NULL",
                            lemma=ru_conj.lemma(),
                            commonality="NULL",
                            pos_type="Verb",
                            dictionary_info=json!({
                                "dictionary_form": ru_conj.lemma(),
                                "ipa": ipa_string,
                                "is_imperfective": RuConj::is_imperfective(&conjugation),
                                // "я_form": RuConj::get_form(&conjugation, ru_conj::class_selectors:: )
                            })
                        ).as_bytes()
                ).expect("writing of bytes");
            }
    
            let ru_noun_declensions = wiki_macros.iter()
                .filter_map(|m| match m { WiktionaryMacro::RuNounTable(n) => Some(n), _ => None })
                .filter(|t| !t.is_old());
            for ru_noun_table in ru_noun_declensions {
            }
        
            let ru_adj_declensions = wiki_macros.iter()
                .filter_map(|m| match m { WiktionaryMacro::RuDeclAdj(n) => Some(n), _ => None });
            for ru_adj_decl in ru_adj_declensions {
            }

        },
        Language::Ukrainian => todo!(),
        Language::Belarusian => todo!(),
    }

}