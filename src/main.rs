mod models;
mod processes;
mod traits;
mod utils;

fn main() {
    
    processes::dump_to_filtered(
        "C:/Users/Noah3/Code/slavic_wiktionary_parser/data/wiki_dumps/en_wiktionary.xml",
        "C:/Users/Noah3/Code/slavic_wiktionary_parser/data/filtered_wiki_dump/ukrainian.txt",
        &["==Ukrainian=="]
    ).expect("success");
}
