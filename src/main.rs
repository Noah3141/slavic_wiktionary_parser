use wiktionary_parser::models::language::Language;

mod processes;
mod traits;
mod utils;
mod constants;

fn main() {
    
    // processes::dump_to_filtered(
    //     "C:/Users/Noah3/Code/slavic_wiktionary_parser/data/wiki_dumps/en_wiktionary.xml",
    //     "C:/Users/Noah3/Code/slavic_wiktionary_parser/data/filtered_wiki_dump/ukrainian.txt",
    //     &["==Ukrainian=="]
    // ).expect("success");
    // processes::dump_to_filtered(
    //     "C:/Users/Noah3/Code/slavic_wiktionary_parser/data/wiki_dumps/en_wiktionary.xml",
    //     "C:/Users/Noah3/Code/slavic_wiktionary_parser/data/filtered_wiki_dump/russian.txt",
    //     &["==Russian=="]
    // ).expect("success");
    // processes::dump_to_filtered(
    //     "C:/Users/Noah3/Code/slavic_wiktionary_parser/data/wiki_dumps/en_wiktionary.xml",
    //     "C:/Users/Noah3/Code/slavic_wiktionary_parser/data/filtered_wiki_dump/belarusian.txt",
    //     &["==Belarusian=="]
    // ).expect("success");

    // processes::filtered_to_json(
    //     "C:/Users/Noah3/Code/slavic_wiktionary_parser/data/filtered_wiki_dump/belarusian.txt",
    //     "C:/Users/Noah3/Code/slavic_wiktionary_parser/data/parsed/belarusian.json",
    //     true,
    //     Language::Belarusian
    // ).expect("success");
    // processes::filtered_to_json(
    //     "C:/Users/Noah3/Code/slavic_wiktionary_parser/data/filtered_wiki_dump/russian.txt",
    //     "C:/Users/Noah3/Code/slavic_wiktionary_parser/data/parsed/russian.json",
    //     true,
    //     Language::Russian
    // ).expect("success");
    processes::filtered_to_json(
        "C:/Users/Noah3/Code/slavic_wiktionary_parser/data/filtered_wiki_dump/ukrainian.txt",
        "C:/Users/Noah3/Code/slavic_wiktionary_parser/data/parsed/ukrainian.json",
        true,
        Language::Ukrainian
    ).expect("success");


}
