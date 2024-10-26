use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
};
use crate::utils::find_ipa_for;
use wiktionary_parser::models::{section_header::SectionHeader,};
use wiktionary_parser::models::{
    language::Language,
    wiktionary_macro::{Expand, WiktionaryMacro},
};

pub async fn json_to_entry_csv(json_file: &str, out: &str, language: Language) -> Result<(), ()> {
    println!("Reading file...");
    let reader = BufReader::with_capacity(
        1024 * 1024 * 4,
        File::open(json_file).expect("buffer to initalize"),
    );
    let wiki_macros: Vec<WiktionaryMacro> =
        serde_json::from_reader(reader).expect("wiki macros to read from reader");
    println!("Finished!");

    let mut out_file = File::options()
        .read(true)
        .append(true)
        .truncate(false)
        .create(true)
        .open(out)
        .expect("creation of out file with overwrite");

    println!("Building cache reference...");

    let mut holding_string = String::with_capacity(1024 * 1024 * 2);
    out_file
        .read_to_string(&mut holding_string)
        .expect("reading to holding string");
    let mut present_lemmas: Vec<&str> = holding_string
        .lines()
        .map(|line| &line[..line.find("|").unwrap()])
        .collect();

    println!(
        "Cache reference ready with {} forms.\nLast form reached was '{}'",
        present_lemmas.len(),
        present_lemmas.last().unwrap_or(&"None yet")
    );

    println!("\nBeginning Entry JSON process.");
    let mut writer = BufWriter::with_capacity(1024 * 1024 * 6, out_file);
    let mut i: usize = 0;

    match language {
        Language::Russian => {
            // Manual overrides:
            present_lemmas.insert(0, "ка\u{301}мхорить");
            present_lemmas.insert(0, "ны\u{301}кать");
            present_lemmas.insert(0, "три");
            present_lemmas.insert(0, "сто");
            present_lemmas.insert(0, "сколько");
            present_lemmas.insert(0, "столько");
            present_lemmas.insert(0, "несколько");
            present_lemmas.insert(0, "кто");
            present_lemmas.insert(0, "что");
            present_lemmas.insert(0, "нечто");
            present_lemmas.insert(0, "не\u{301}чего");
            present_lemmas.insert(0, "не\u{301}кого");
            present_lemmas.insert(0, "некоторый");
            present_lemmas.insert(0, "четы́ре");
            present_lemmas.insert(0, "да\u{301}нные"); // let the adjective table take over
            present_lemmas.insert(0, "Со\u{301}чи");
            present_lemmas.insert(0, "чаевы́е");
            present_lemmas.insert(0, "хто");
            present_lemmas.insert(0, "[[ши́тый]]");
            present_lemmas.insert(0, "все (&quot;everyone&quot;)");
            present_lemmas.insert(0, "ме\u{301}сячные");
            present_lemmas.insert(0, "при́сные");
            println!("Processing Russian...");

            let ipa_macros: Vec<WiktionaryMacro> = wiki_macros.iter().filter(|m| {
                match m {
                    WiktionaryMacro::RuIpa(_) => true,
                    WiktionaryMacro::Ipa(_) => true,
                    _ => false
                }
            })
            .map(|m: &WiktionaryMacro| -> WiktionaryMacro { m.clone() })
            .collect();

            // use wiktionary_parser::models::wiktionary_macro::russian;
            use rubit_api_db::dictionary_info::russian::*;
            // use wiktionary_parser::models::wiktionary_macro::russian::{
            //     RuConj,  ru_conj,
            //     RuDeclAdj,  ru_decl_adj,
            //     RuNounTable, ru_noun_table
            // };

            // Russian
            // Verbs
            //
            let client = reqwest::Client::builder()
                .pool_idle_timeout(None)
                .build()
                .expect("Client build process");

            let ru_conjugations = wiki_macros
                .iter()
                .filter_map(|m| match m {
                    WiktionaryMacro::RuConj(n) => Some(n),
                    _ => None,
                })
                .filter(|m| !present_lemmas.contains(&m.lemma().trim()))
                .filter(|m| !m.is_old())
                .filter(|m| !m.is_impersonal());

            println!(
                "Verbs not yet processed: {}",
                ru_conjugations.clone().count()
            );
            for ru_conj in ru_conjugations {
                i += 1;
                if i % 50 == 0 {
                    writer.flush().expect("flush");
                    println!("Processing: {}", &ru_conj.macro_text)
                }
                let ipa_string = match find_ipa_for(ru_conj.page_id, &Language::Russian, &ipa_macros, &client).await {
                    Some(ipa_string) => ipa_string,
                    None => {
                        println!("No ipa at all! {}", &ru_conj.macro_text);
                        continue
                    }
                };
                let conjugation = ru_conj.html(&client).await;
                let dictionary_info = serde_json::to_string(&RussianVerb::build_from_ru_conj(
                    &ru_conj,
                    ipa_string,
                    &conjugation,
                ))
                .expect("serialization of db dictionary_info model as json");

                writer
                    .write(
                        format!(
                            "{lemma}|{commonality}|{pos_type}|{dictionary_info}\n",
                            lemma = ru_conj.lemma(),
                            commonality = "NULL",
                            pos_type = "Verb",
                            dictionary_info = dictionary_info
                        )
                        .as_bytes(),
                    )
                    .expect("writing of bytes");
            }

            writer.flush().expect("flush!");
            println!("Verbs complete!");

            // Russian
            // Noun
            //
            drop(client);
            let client = reqwest::Client::builder()
                .pool_idle_timeout(None)
                .build()
                .expect("Client build process");

            let ru_noun_declensions = wiki_macros
                .iter()
                .filter_map(|m| match m {
                    WiktionaryMacro::RuNounTable(n) => Some(n),
                    _ => None,
                })
                .filter(|m| !present_lemmas.contains(&m.lemma().as_str().trim()))
                .filter(|m| !m.macro_text.starts_with("-"))
                .filter(|m| {
                    !m.is_old() && !m.is_pronoun() && m.section != SectionHeader::ProperNoun
                })
                .filter(|m| !m.is_manual())
                .filter(|m| m.lemma().ends_with("ый"))
                .filter(|m| m.lemma().ends_with("ий"));

            println!(
                "Nouns not yet processed: {}",
                ru_noun_declensions.clone().count()
            );
            for ru_noun_table in ru_noun_declensions {
                i += 1;
                if i % 1 == 0 {
                    println!("Processing: {}", &ru_noun_table.macro_text)
                }

                let ipa_string = match find_ipa_for(ru_noun_table.page_id, &Language::Russian, &ipa_macros, &client).await {
                    Some(ipa_string) => ipa_string,
                    None => {
                        println!("No ipa at all! {}", &ru_noun_table.macro_text);
                        continue
                    }
                };

                let declension = ru_noun_table.html(&client).await;
                if ru_noun_table
                    .check_head(&declension, " adj")
                    .unwrap_or(false)
                {
                    continue;
                }
                if ru_noun_table
                    .check_head(&declension, " indecl")
                    .unwrap_or(false)
                {
                    continue;
                }

                let dictionary_info = serde_json::to_string(
                    &RussianNoun::build_from_ru_noun_table(&ru_noun_table, ipa_string, &declension),
                )
                .expect("serialization of db dictionary_info model as json");

                writer
                    .write(
                        format!(
                            "{lemma}|{commonality}|{pos_type}|{dictionary_info}\n",
                            lemma = ru_noun_table.lemma(),
                            commonality = "NULL",
                            pos_type = "Noun",
                            dictionary_info = dictionary_info
                        )
                        .as_bytes(),
                    )
                    .expect("writing of bytes");
            }

            writer.flush().expect("flush!");
            println!("Nouns complete!");

            // Russian
            // Adjectives
            //
            drop(client);
            let client = reqwest::Client::builder()
                .pool_idle_timeout(None)
                .build()
                .expect("Client build process");

            let ru_adj_declensions = wiki_macros
                .iter()
                .filter_map(|m| match m {
                    WiktionaryMacro::RuDeclAdj(n) => Some(n),
                    _ => None,
                })
                .filter(|m| !present_lemmas.contains(&m.lemma().trim()))
                .filter(|m| {
                    !m.is_old()
                        && !m.lemma().starts_with("-")
                        && !m.lemma().ends_with("ин")
                        && m.section != SectionHeader::ProperNoun
                        && m.section != SectionHeader::Determiner
                        && m.section != SectionHeader::Numeral
                });

            println!(
                "Adjectives not yet processed: {}",
                ru_adj_declensions.clone().count()
            );
            for ru_adj_decl in ru_adj_declensions {
                i += 1;
                if i % 50 == 0 {
                    writer.flush().expect("flush");
                    println!("Processing: {}", &ru_adj_decl.macro_text)
                }
                let ipa_string = match find_ipa_for(ru_adj_decl.page_id, &Language::Russian, &ipa_macros, &client).await {
                    Some(ipa_string) => ipa_string,
                    None => {
                        println!("No ipa at all! {}", &ru_adj_decl.macro_text);
                        continue
                    }
                };
                let declension = ru_adj_decl.html(&client).await;

                if ru_adj_decl
                    .check_head(&declension, "short-form-only")
                    .unwrap_or(false)
                {
                    continue;
                }
                if ru_adj_decl
                    .check_head(&declension, "Pre-reform")
                    .unwrap_or(false)
                {
                    continue;
                }

                let dictionary_info =
                    serde_json::to_string(&RussianAdjective::build_from_ru_decl_adj(
                        &ru_adj_decl,
                        ipa_string,
                        &declension,
                    ))
                    .expect("serialization of db dictionary_info model as json");

                writer
                    .write(
                        format!(
                            "{lemma}|{commonality}|{pos_type}|{dictionary_info}\n",
                            lemma = ru_adj_decl.lemma(),
                            commonality = "NULL",
                            pos_type = "Adjective",
                            dictionary_info = dictionary_info
                        )
                        .as_bytes(),
                    )
                    .expect("writing of bytes");
            }

            println!("Adjectives complete!");
            writer.flush().expect("flush!");
            

            // Russian
            // Adverb
            //
            drop(client);
            let client = reqwest::Client::builder()
                .pool_idle_timeout(None)
                .build()
                .expect("Client build process");

            
            let ru_adverbs = wiki_macros
                .iter()
                .filter_map(|m| match m {
                    WiktionaryMacro::RuAdv(n) => Some(n),
                    _ => None,
                })
                .filter(|m| !present_lemmas.contains(&m.lemma().trim()));
            
            println!(
                "Adverbs not yet processed: {}",
                ru_adverbs.clone().count()
            );
            for ru_adv in ru_adverbs {
                i += 1;
                if i % 50 == 0 {
                    writer.flush().expect("flush");
                    println!("Processing: {}", &ru_adv.macro_text)
                }
                let ipa_string = match find_ipa_for(ru_adv.page_id, &Language::Russian, &ipa_macros, &client).await {
                    Some(ipa_string) => ipa_string,
                    None => {
                        println!("No ipa at all! {}", &ru_adv.macro_text);
                        continue
                    }
                };
            

                let dictionary_info =
                    serde_json::to_string(&RussianAdverb::build_from_ru_adv(
                        &ru_adv,
                        ipa_string,
                    ))
                    .expect("serialization of db dictionary_info model as json");

                writer
                    .write(
                        format!(
                            "{lemma}|{commonality}|{pos_type}|{dictionary_info}\n",
                            lemma = ru_adv.lemma(),
                            commonality = "NULL",
                            pos_type = "Adverb",
                            dictionary_info = dictionary_info
                        )
                        .as_bytes(),
                    )
                    .expect("writing of bytes");
            }

            println!("Adverbs complete!");
            writer.flush().expect("flush!");

            println!("Entry CSV complete!");

            Ok(())
        }
        Language::Ukrainian => {
            println!("Processing Ukrainian...");

            use rubit_api_db::dictionary_info::ukrainian::*;
            use wiktionary_parser::models::wiktionary_macro::ukrainian;
            use wiktionary_parser::models::wiktionary_macro::ukrainian::{
                uk_adecl, uk_conj, uk_ndecl, UkADecl, UkConj, UkNDecl,
            };

            present_lemmas.insert(0, "бу́блик");
            present_lemmas.insert(0, "па\u{301}луба");
            present_lemmas.insert(0, "киргѝзомо́вний");

            // One day worth fixing but need to speed up iteration now
            {
                present_lemmas.insert(0, "па\u{301}луба");
                present_lemmas.insert(0, "рок");
                present_lemmas.insert(0, "ткани́на");
                present_lemmas.insert(0, "гри́вна");
                present_lemmas.insert(0, "листоно́ша");
                present_lemmas.insert(0, "Херсо́н");
                present_lemmas.insert(0, "поли́ва");
                present_lemmas.insert(0, "е́хо:ovi&gt");
                present_lemmas.insert(0, "пегмати́т");
                present_lemmas.insert(0, "криха́");
                present_lemmas.insert(0, "га́йда");
                present_lemmas.insert(0, "га́йда");
                present_lemmas.insert(0, "констру́кт");
                present_lemmas.insert(0, "дя́ка");
                present_lemmas.insert(0, "книгозбі́рня");
                present_lemmas.insert(0, "Болга́рщина");
                present_lemmas.insert(0, "вагани́");
                present_lemmas.insert(0, "сагайда́к");
                present_lemmas.insert(0, "путіні́зм");
                present_lemmas.insert(0, "вікісловни́к");
                present_lemmas.insert(0, "тризу́б");
                present_lemmas.insert(0, "гематоге́н");
                present_lemmas.insert(0, "Чорноба́й");
                present_lemmas.insert(0, "інтерне́т");
                present_lemmas.insert(0, "москва́");
                present_lemmas.insert(0, "по́цька");
                present_lemmas.insert(0, "Вакарчу́к");
                present_lemmas.insert(0, "бісексуалі́зм");
                present_lemmas.insert(0, "андрогі́нія");
                present_lemmas.insert(0, "Куматре́нко");
                present_lemmas.insert(0, "я́нгол");
                present_lemmas.insert(0, "-щи́на");
                present_lemmas.insert(0, "держкомі́сія");
                present_lemmas.insert(0, "неоти́п");
                present_lemmas.insert(0, "Діоні́сій");
                present_lemmas.insert(0, "-ї́вна");
                present_lemmas.insert(0, "маркі́з");
                present_lemmas.insert(0, "дофі́н");
                present_lemmas.insert(0, "коло́дій");
                present_lemmas.insert(0, "безві́з");
                present_lemmas.insert(0, "число́ з рухо́мою ко́мою");
                present_lemmas.insert(0, "чортопха́йка");
                present_lemmas.insert(0, "спе́ція");
                present_lemmas.insert(0, "ади́кція");
                present_lemmas.insert(0, "трильйо́н");
                present_lemmas.insert(0, "гниття́");
                present_lemmas.insert(0, "СБУ́шник");
                present_lemmas.insert(0, "наступнорічни́й");
                present_lemmas.insert(0, "тогорі́чний");
                present_lemmas.insert(0, "преподо́бний");
            };

            // Ukrainian
            // Verbs
            //
            let client = reqwest::Client::builder()
                .pool_idle_timeout(None)
                .build()
                .expect("Client build process");

            let uk_conjugations = wiki_macros
                .iter()
                .filter_map(|m| match m {
                    WiktionaryMacro::UkConj(n) => Some(n),
                    _ => None,
                })
                .filter(|m| !present_lemmas.contains(&m.lemma().trim()))
                .filter(|m| !m.is_impersonal());

            println!(
                "Verbs not yet processed: {}",
                uk_conjugations.clone().count()
            );
            for uk_conj in uk_conjugations {
                i += 1;
                if i % 50 == 0 {
                    writer.flush().expect("flush");
                    println!("Processing: {}", &uk_conj.macro_text)
                }
                let ipa_string = match find_ipa_for(uk_conj.page_id, &Language::Ukrainian, &wiki_macros, &client).await {
                    Some(ipa_string) => ipa_string,
                    None => {
                        println!("No ipa at all! {}", &uk_conj.macro_text);
                        continue
                    }
                };
                let conjugation = uk_conj.html(&client).await;
                let dictionary_info = serde_json::to_string(&UkrainianVerb::build_from_uk_conj(
                    &uk_conj,
                    ipa_string,
                    &conjugation,
                ))
                .expect("serialization of db dictionary_info model as json");

                writer
                    .write(
                        format!(
                            "{lemma}|{commonality}|{pos_type}|{dictionary_info}\n",
                            lemma = uk_conj.lemma(),
                            commonality = "NULL",
                            pos_type = "Verb",
                            dictionary_info = dictionary_info
                        )
                        .as_bytes(),
                    )
                    .expect("writing of bytes");
            }
            println!("Verbs complete!");
            writer.flush().expect("flush!");

            // Ukrainian
            // Noun
            //
            drop(client);
            let client = reqwest::Client::builder()
                .pool_idle_timeout(None)
                .build()
                .expect("Client build process");

            let uk_noun_tables = wiki_macros
                .iter()
                .filter_map(|m| match m {
                    WiktionaryMacro::UkNDecl(n) => Some(n),
                    _ => None,
                })
                .filter(|m| !present_lemmas.contains(&m.lemma().trim()))
                .filter(|m| !m.macro_text.starts_with("-"));
            // .filter(|m| !m.is_old() && !m.is_pronoun());

            println!(
                "Nouns not yet processed: {}",
                uk_noun_tables.clone().count()
            );
            for uk_noun_table in uk_noun_tables {
                i += 1;
                if i % 50 == 0 {
                    writer.flush().expect("flush");
                    println!("Processing: {}", &uk_noun_table.macro_text)
                }

                let ipa_string = match find_ipa_for(uk_noun_table.page_id, &Language::Ukrainian, &wiki_macros, &client).await {
                    Some(ipa_string) => ipa_string,
                    None => {
                        println!("No ipa at all! {}", &uk_noun_table.macro_text);
                        continue
                    }
                };

                let declension = uk_noun_table.html(&client).await;
                if uk_noun_table
                    .check_head(&declension, "indecl")
                    .expect("Checking head to word")
                    || uk_noun_table
                        .check_head(&declension, "adj ")
                        .expect("Checking head to word")
                {
                    continue;
                }
                println!("{}", uk_noun_table.macro_text);
                
                let dictionary_info = serde_json::to_string(&UkrainianNoun::build_from_uk_ndecl(
                    &uk_noun_table,
                    ipa_string,
                    &declension,
                ))
                .expect("serialization of db dictionary_info model as json");

                writer
                    .write(
                        format!(
                            "{lemma}|{commonality}|{pos_type}|{dictionary_info}\n",
                            lemma = uk_noun_table.lemma(),
                            commonality = "NULL",
                            pos_type = "Noun",
                            dictionary_info = dictionary_info
                        )
                        .as_bytes(),
                    )
                    .expect("writing of bytes");
            }
            println!("Nouns complete!");
            writer.flush().expect("flush!");

            // Ukrainian
            // Adjectives
            //
            drop(client);
            let client = reqwest::Client::builder()
                .pool_idle_timeout(None)
                .build()
                .expect("Client build process");

            let uk_adj_tables = wiki_macros
                .iter()
                .filter_map(|m| match m {
                    WiktionaryMacro::UkADecl(n) => Some(n),
                    _ => None,
                })
                .filter(|m| !present_lemmas.contains(&m.lemma().trim()))
                .filter(|m| !m.lemma().starts_with("-")
                    && !m.lemma().ends_with("ин")
                    && m.section != SectionHeader::ProperNoun
                    && m.section != SectionHeader::Determiner
                    && m.section != SectionHeader::Numeral
                );

            println!(
                "Adjectives not yet processed: {}",
                uk_adj_tables.clone().count()
            );
            for uk_adj_table in uk_adj_tables {
                i += 1;
                if i % 50 == 0 {
                    writer.flush().expect("flush");
                    println!("Processing: {}", &uk_adj_table.macro_text)
                }
                let ipa_string = match find_ipa_for(uk_adj_table.page_id, &Language::Ukrainian, &wiki_macros, &client).await {
                    Some(ipa_string) => ipa_string,
                    None => {
                        println!("No ipa at all! {}", &uk_adj_table.macro_text);
                        continue
                    }
                };

                let declension = uk_adj_table.html(&client).await;
                // println!("{}", declension.html());
                let dictionary_info =
                    serde_json::to_string(&UkrainianAdjective::build_from_uk_adecl(
                        &uk_adj_table,
                        ipa_string,
                        &declension,
                    ))
                    .expect("serialization of db dictionary_info model as json");

                writer
                    .write(
                        format!(
                            "{lemma}|{commonality}|{pos_type}|{dictionary_info}\n",
                            lemma = uk_adj_table.lemma(),
                            commonality = "NULL",
                            pos_type = "Adjective",
                            dictionary_info = dictionary_info
                        )
                        .as_bytes(),
                    )
                    .expect("writing of bytes");
            }

            println!("Adjectives complete!");

            writer.flush().expect("flush!");

            println!("Entry CSV complete!");

            Ok(())
        }
        Language::Belarusian => {
            println!("Processing Belarusian...");

            present_lemmas.insert(0, "да́ныя");
            present_lemmas.insert(0, "клубни́ка");
            present_lemmas.insert(0, "Бяро́заsg&gt;");

            // use wiktionary_parser::models::wiktionary_macro::belarusian;
            use rubit_api_db::dictionary_info::belarusian::*;
            // use wiktionary_parser::models::wiktionary_macro::belarusian::{
            //     BeConj,  be_conj,
            //     BeADecl,  be_adecl,
            //     BeNDecl, be_ndecl
            // };

            // Belarusian
            // Verbs
            //
            let client = reqwest::Client::builder()
                .pool_idle_timeout(None)
                .build()
                .expect("Client build process");

            let be_conjugations = wiki_macros
                .iter()
                .filter_map(|m| match m {
                    WiktionaryMacro::BeConj(n) => Some(n),
                    _ => None,
                })
                .filter(|m| !present_lemmas.contains(&m.lemma().trim()))
                .filter(|m| !m.is_impersonal());

            println!(
                "Verbs not yet processed: {}",
                be_conjugations.clone().count()
            );
            for be_conj in be_conjugations {
                i += 1;
                if i % 50 == 0 {
                    writer.flush().expect("flush");
                    println!("Processing: {}", &be_conj.macro_text)
                }
                let ipa_string = match find_ipa_for(be_conj.page_id, &Language::Belarusian, &wiki_macros, &client).await {
                    Some(ipa_string) => ipa_string,
                    None => {
                        println!("No ipa at all! {}", &be_conj.macro_text);
                        continue
                    }
                };
                let conjugation = be_conj.html(&client).await;

                let dictionary_info = serde_json::to_string(&BelarusianVerb::build_from_be_conj(
                    &be_conj,
                    ipa_string,
                    &conjugation,
                ))
                .expect("serialization of db dictionary_info model as json");

                writer
                    .write(
                        format!(
                            "{lemma}|{commonality}|{pos_type}|{dictionary_info}\n",
                            lemma = be_conj.lemma(),
                            commonality = "NULL",
                            pos_type = "Verb",
                            dictionary_info = dictionary_info
                        )
                        .as_bytes(),
                    )
                    .expect("writing of bytes");
            }
            println!("Verbs complete!");
            writer.flush().expect("flush!");

            // Belarusian
            // Noun
            //
            drop(client);
            let client = reqwest::Client::builder()
                .pool_idle_timeout(None)
                .build()
                .expect("Client build process");

            let be_noun_tables = wiki_macros
                .iter()
                .filter_map(|m| match m {
                    WiktionaryMacro::BeNDecl(n) => Some(n),
                    _ => None,
                })
                .filter(|m| !present_lemmas.contains(&m.lemma().trim()))
                .filter(|m| !m.macro_text.starts_with("-"));
            // .filter(|m| !m.is_old() && !m.is_pronoun());

            println!(
                "Nouns not yet processed: {}",
                be_noun_tables.clone().count()
            );
            for be_noun_table in be_noun_tables {
                i += 1;
                if i % 50 == 0 {
                    writer.flush().expect("flush");
                    println!("Processing: {}", &be_noun_table.macro_text)
                }

                let ipa_string = match find_ipa_for(be_noun_table.page_id, &Language::Belarusian, &wiki_macros, &client).await {
                    Some(ipa_string) => ipa_string,
                    None => {
                        println!("No ipa at all! {}", &be_noun_table.macro_text);
                        continue
                    }
                };

                let declension = be_noun_table.html(&client).await;
                let dictionary_info = serde_json::to_string(&BelarusianNoun::build_from_be_ndecl(
                    &be_noun_table,
                    ipa_string,
                    &declension,
                ))
                .expect("serialization of db dictionary_info model as json");

                writer
                    .write(
                        format!(
                            "{lemma}|{commonality}|{pos_type}|{dictionary_info}\n",
                            lemma = be_noun_table.lemma(),
                            commonality = "NULL",
                            pos_type = "Noun",
                            dictionary_info = dictionary_info
                        )
                        .as_bytes(),
                    )
                    .expect("writing of bytes");
            }
            println!("Nouns complete!");
            writer.flush().expect("flush!");

            // Belarusian
            // Adjectives
            //
            drop(client);
            let client = reqwest::Client::builder()
                .pool_idle_timeout(None)
                .build()
                .expect("Client build process");

            let be_adj_tables = wiki_macros
                .iter()
                .filter_map(|m| match m {
                    WiktionaryMacro::BeADecl(n) => Some(n),
                    _ => None,
                })
                .filter(|m| !present_lemmas.contains(&m.lemma().trim()))
                .filter(|m| !m.is_surname());
            // .filter(|m| !m.is_old());

            println!(
                "Adjectives not yet processed: {}",
                be_adj_tables.clone().count()
            );
            for be_adj_table in be_adj_tables {
                i += 1;
                if i % 50 == 0 {
                    writer.flush().expect("flush");
                    println!("Processing: {}", &be_adj_table.macro_text)
                }
                let ipa_string = wiki_macros
                    .iter()
                    .filter_map(|m| match m {
                        WiktionaryMacro::BeIpa(n) => Some(n),
                        _ => None,
                    })
                    .find(|ipa_m| {
                        ipa_m.page_id == be_adj_table.page_id // This can be wrong due to complex pages, but should probably be over 80% accurate 95% of the time...
                    })
                    .expect("ipa_string corresponding to ru_adj_Decl")
                    .to_ipa_string(&client)
                    .await;
                let declension = be_adj_table.html(&client).await;
                let dictionary_info =
                    serde_json::to_string(&BelarusianAdjective::build_from_be_adecl(
                        &be_adj_table,
                        ipa_string,
                        &declension,
                    ))
                    .expect("serialization of db dictionary_info model as json");

                writer
                    .write(
                        format!(
                            "{lemma}|{commonality}|{pos_type}|{dictionary_info}\n",
                            lemma = be_adj_table.lemma(),
                            commonality = "NULL",
                            pos_type = "Adjective",
                            dictionary_info = dictionary_info
                        )
                        .as_bytes(),
                    )
                    .expect("writing of bytes");
            }

            println!("Adjectives complete!");

            writer.flush().expect("flush!");

            println!("Entry CSV complete!");

            Ok(())
        }
    }
}
