use std::{fs::File, io::Read};
use crate::constants::DELIMITER;
use wiktionary_parser::models::{language::Language, wiktionary_macro::WiktionaryMacro};

pub fn filtered_to_json(
    filtered_txt: &str, 
    out: &str, 
    overwrite: bool,
    language: Language,
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
    let mut wiki_macros: Vec<WiktionaryMacro> = Vec::with_capacity(30_000);
    for page in pages {
        let mut page_wiki_macros = match WiktionaryMacro::parse_from_xml(page, &language) { 
            Ok(v) => v, 
            _ => continue 
        };
        wiki_macros.append(&mut page_wiki_macros);
    }
    serde_json::to_writer_pretty(&mut out_file, &wiki_macros).expect("serialization");
    Ok(())
}