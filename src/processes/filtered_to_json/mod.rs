use std::{fs::File, io::Read};
use crate::constants::DELIMITER;
use wiktionary_parser::models::wiktionary_macro::WiktionaryMacro;

pub fn filtered_to_json(filtered_txt: &str, out: &str) -> Result<(), ()> {

    let mut filtered_data = String::with_capacity(40_000);
    let filtered_file = File::options()
        .create(false)
        .read(true)
        .open(filtered_txt)
        .expect("opening the filtered file")
        .read_to_string(&mut filtered_data);

    let mut out_file = File::options()
        .read(true)
        .create_new(true)
        .append(true)
        .open(out)
        .expect("out file");


    let pages = filtered_data.split(DELIMITER).skip(1);
    let mut wiki_macros: Vec<WiktionaryMacro> = Vec::with_capacity(30_000);
    for page in pages {
        let mut page_wiki_macros = WiktionaryMacro::parse_from_xml(page);
        wiki_macros.append(&mut page_wiki_macros);
    }
    serde_json::to_writer(&mut out_file, &wiki_macros).expect("serialization");
    Ok(())
}