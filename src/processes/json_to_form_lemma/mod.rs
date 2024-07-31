use std::{fmt::format, fs::File, io::{BufReader, Write}};
use wiktionary_parser::models::{language::Language, wiktionary_macro::{Expand, WiktionaryMacro}};



pub async fn json_to_form_lemma(
    json_file: &str, 
    out: &str, 
    overwrite: bool,
    language: Language,
) -> Result<(), ()>{
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
        
            let ru_conjugations = wiki_macros.iter()
                .filter_map(|m| match m { WiktionaryMacro::RuConj(n) => Some(n), _ => None })
                .filter(|t| !t.is_old());
            for ru_conj in ru_conjugations {
                let tups = ru_conj.form_and_lemma(&client).await;
                for tup in tups {
                    out_file.write(format!("{}, {}", tup.0, tup.1).as_bytes()).expect("writing of bytes");
                }
            }
        
            let ru_noun_declensions = wiki_macros.iter()
                .filter_map(|m| match m { WiktionaryMacro::RuNounTable(n) => Some(n), _ => None })
                .filter(|t| !t.is_old());
            for ru_noun_table in ru_noun_declensions {
                let tups = ru_noun_table.form_and_lemma(&client).await;
                for tup in tups {
                    out_file.write(format!("{}, {}", tup.0, tup.1).as_bytes()).expect("writing of bytes");
                }
            }
        
            let ru_adj_declensions = wiki_macros.iter()
                .filter_map(|m| match m { WiktionaryMacro::RuDeclAdj(n) => Some(n), _ => None });
            for ru_adj_decl in ru_adj_declensions {
                let tups = ru_adj_decl.form_and_lemma(&client).await;
                for tup in tups {
                    out_file.write(format!("{}, {}", tup.0, tup.1).as_bytes()).expect("writing of bytes");
                }
            }
        
            Ok(())
        },
        Language::Ukrainian => {
            let be_conjugations = wiki_macros.iter()
                .filter_map(|m| match m { WiktionaryMacro::BeConj(n) => Some(n), _ => None });
            for be_conj in be_conjugations {
                let tups = be_conj.form_and_lemma(&client).await;
                for tup in tups {
                    out_file.write(format!("{}, {}", tup.0, tup.1).as_bytes()).expect("writing of bytes");
                }
            }

            todo!()
        },
        Language::Belarusian => {
            todo!()
        },
    }
}