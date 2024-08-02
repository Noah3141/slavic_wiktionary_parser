use std::{fmt::format, fs::File, io::{BufReader, BufWriter, Write}};
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

    let out_file =  match overwrite {
        true =>  File::options()
            .append(true)
            .create(true)
            .truncate(true)
            .open(out)
            .expect("creation of out file with overwrite"),
        false => File::options()
            .write(true)
            .create_new(true)
            .open(out)
            .expect("creation of out file without overwriting")
    };

    println!("\nBeginning Form Lemma process.");
    let mut i:usize = 0;

    let mut writer = BufWriter::with_capacity(1024 * 1024 * 1, out_file);
    writer.write(format!("form, lemma\n").as_bytes()).expect("writing of bytes");
    let client = reqwest::Client::builder()
        .pool_idle_timeout(None)
        .build()
        .expect("Client build process");

    match language {
        Language::Russian => {
            println!("Processing Russian...");
        
            let ru_conjugations = wiki_macros.iter()
                .filter_map(|m| match m { WiktionaryMacro::RuConj(n) => Some(n), _ => None })
                .filter(|t| !t.is_old());
            for ru_conj in ru_conjugations {
                i += 1;
                if i % 50 == 0 {
                    println!("Processing: {}", &ru_conj.macro_text)
                }
                let tups = ru_conj.form_and_lemma(&client).await;
                for tup in tups {
                    writer.write(format!("{}, {}\n", tup.0, tup.1).as_bytes()).expect("writing of bytes");
                }
            }

            i = 0;
            println!("Verbs complete.");
        
            let ru_noun_declensions = wiki_macros.iter()
                .filter_map(|m| match m { WiktionaryMacro::RuNounTable(n) => Some(n), _ => None })
                .filter(|t| !t.is_old());
            for ru_noun_table in ru_noun_declensions {
                i+=1;
                if i % 50 == 0 {
                    println!("Processing: {}", &ru_noun_table.macro_text)
                }
                let tups = ru_noun_table.form_and_lemma(&client).await;
                for tup in tups {
                    writer.write(format!("{}, {}\n", tup.0, tup.1).as_bytes()).expect("writing of bytes");
                }
            }

            i = 0;
            println!("Nouns complete.");
        
            let ru_adj_declensions = wiki_macros.iter()
                .filter_map(|m| match m { WiktionaryMacro::RuDeclAdj(n) => Some(n), _ => None });
            for ru_adj_decl in ru_adj_declensions {
                i+=1;
                if i % 50 == 0 {
                    println!("Processing: {}", &ru_adj_decl.macro_text)
                }
                let tups = ru_adj_decl.form_and_lemma(&client).await;
                for tup in tups {
                    writer.write(format!("{}, {}\n", tup.0, tup.1).as_bytes()).expect("writing of bytes");
                }
            }

            println!("Adjectives complete!");


            writer.flush().expect("flush!");
        
            println!("Form-lemma complete!");

            Ok(())
        },
        Language::Ukrainian => {
            let be_conjugations = wiki_macros.iter()
                .filter_map(|m| match m { WiktionaryMacro::BeConj(n) => Some(n), _ => None });
            for be_conj in be_conjugations {
                let tups = be_conj.form_and_lemma(&client).await;
                for tup in tups {
                    writer.write(format!("{}, {}\n", tup.0, tup.1).as_bytes()).expect("writing of bytes");
                }
            }



            writer.flush().expect("flush!");

            todo!()
        },
        Language::Belarusian => {


            writer.flush().expect("flush!");

            todo!()
        },
    }
}