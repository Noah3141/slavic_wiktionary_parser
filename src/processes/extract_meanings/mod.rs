use std::{fs::File, io::{Read, Write}};

use serde_json::json;
use wiktionary_parser::{models::{language::Language, section_header::SectionHeader, wiktionary_macro::{russian::{RuAdj, RuNounPlus, RuVerb}, WiktionaryMacro}}, utils::{select_from, select_unto_language_header, split_sections}};

use crate::constants::DELIMITER;



pub fn extract_meanings(
    filtered_txt: &str,
    out: &str,
    language: &Language,
    overwrite: bool
) -> Result<(), ()> {


    let mut filtered_data = String::with_capacity(40_000);
    File::options()
        .create(false)
        .read(true)
        .open(filtered_txt)
        .expect("opening the filtered file")
        .read_to_string(&mut filtered_data)
        .expect("reading into local data");

    let mut out_file = File::options()
        .truncate(overwrite)
        .create(overwrite)
        .create_new(!overwrite)
        .write(true)
        .open(out)
        .expect(format!("Output file ({out})").as_str());


    let pages = filtered_data.split(DELIMITER).skip(1);
    for page in pages {
        let page_id = u64::from_str_radix(
            select_from(page, "<id>", "</id>").expect("presence of page id"),
            10
        ).expect("conversion to int");
        let page_title = select_from(page, "<title>", "</title>").expect("page title").to_string();
        if  page_title.starts_with("Wiktionary:") || 
            page_title.starts_with("User:") ||
            page_title.starts_with("Talk:") ||
            page_title.starts_with("Module:") ||
            page_title.starts_with("Template:") ||
            page_title.starts_with("Appendix:")
            { 
                continue 
        }
        if page.contains(language.as_header()) {
            let language_section = select_unto_language_header(page, language.as_header()).expect("successful language section extraction");
            let sections = split_sections(language_section);
            for (section, section_text) in sections {
                if section == SectionHeader::Verb {
                    // let macro_text = section_text
                    //     .split("\n")
                    //     .filter(|line| !line.starts_with("[["))
                    //     .nth(2) // Should be the macro right after the header
                    //     .expect("1th").to_string();

                    // if !macro_text.starts_with("{{") { continue };
                    // let verb_macro = RuVerb {
                    //     page_title: page_title.clone(), 
                    //     page_id, 
                    //     section, 
                    //     language: *language, 
                    //     macro_text, 
                    // };

                    // println!("Verb: {}", verb_macro.macro_text);

                    // let meaning_lines = section_text
                    //     .split("\n")
                    //     .filter(|line| line.starts_with("#"))
                    //     .map(|string| String::from(string))
                    //     .collect::<Vec<String>>()
                    //     .join("\n");

                    // serde_json::to_writer_pretty(&mut out_file, &json!(
                    //     { verb_macro.lemma() : meaning_lines }
                    // )).expect("writing json");
                }
                if section == SectionHeader::Noun {
                    // let macro_text = section_text
                    //     .split("\n")
                    //     .filter(|line| !line.starts_with("[["))
                    //     .nth(2) // Should be the macro right after the header
                    //     .expect("1th").to_string();

                    // if !macro_text.starts_with("{{") { continue };
                    // let noun_macro = RuNounPlus {
                    //     page_title: page_title.clone(), 
                    //     page_id, 
                    //     section, 
                    //     language: *language, 
                    //     macro_text, 
                    // };

                    // println!("Noun: {}", noun_macro.macro_text);

                    // let meaning_lines = section_text
                    //     .split("\n")
                    //     .filter(|line| line.starts_with("#"))
                    //     .map(|string| String::from(string))
                    //     .collect::<Vec<String>>()
                    //     .join("\n");

                    // serde_json::to_writer_pretty(&mut out_file, &json!(
                    //     { noun_macro.lemma() : meaning_lines }
                    // )).expect("writing json");
                }
                if section == SectionHeader::Adjective {
                    let macro_text = section_text
                        .split("\n")
                        .filter(|line| !line.starts_with("[["))
                        .nth(2) // Should be the macro right after the header
                        .expect("1th").to_string();

                    if !macro_text.starts_with("{{") { continue };
                    let adj_macro = RuAdj {
                        page_title: page_title.clone(), 
                        page_id, 
                        section, 
                        language: *language, 
                        macro_text, 
                    };

                    println!("Adj: {}", adj_macro.macro_text);

                    let meaning_lines = section_text
                        .split("\n")
                        .filter(|line| line.starts_with("#"))
                        .map(|string| String::from(string))
                        .collect::<Vec<String>>()
                        .join("\n");

                    serde_json::to_writer_pretty(&mut out_file, &json!(
                        { adj_macro.lemma() : meaning_lines }
                    )).expect("writing json");
                }
            }
        }
    }
    Ok(())
}

