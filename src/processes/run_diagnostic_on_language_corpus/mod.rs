// use std::{fs::File, io::{BufReader, Read}};

// use wiktionary_parser::models::{language::Language, section_header::SectionHeader, wiktionary_macro::WiktionaryMacro};




// pub fn run_diagnostic_on_language_corpus(
//     json_file: &str, 
//     analysis: dyn FnOnce(Vec<WiktionaryMacro>) -> ()
// ) {
//     println!("\n\n==========================================");

//     println!("Diagnostics");
//     println!("==============");

//     println!("Reading file...");
//     let reader = BufReader::with_capacity(
//         1024*1024*4, 
//         File::open(json_file).unwrap()
//     );
//     let wiki_macros: Vec<WiktionaryMacro> = serde_json::from_reader(reader).unwrap();
//     println!("Finished!");

//     println!("==============");

//     analysis(wiki_macros);

// }


// pub fn belarusian_word_counts(wiki_macros: Vec<WiktionaryMacro>) {
//             let verbs = wiki_macros.iter()
//                 .filter_map(|m| match m {WiktionaryMacro::BeVerb(adj) => Some(adj), _ => None});

//             println!("Verbs:");
//                 println!("\tCount: {}", verbs.clone().count());
//                 println!("\tForms: {}  (Count x 18)", verbs.clone().count() * 18); // sort of an average
                
//                 let nouns = wiki_macros.iter()
//                 .filter_map(|m| match m {WiktionaryMacro::BeNoun(adj) => Some(adj), _ => None});
            
//             println!("Nouns:");
//             println!("\tCount: {}", nouns.clone().count());
//             println!("\tForms: {}  (Count x 10)", nouns.clone().count() * 10); // sort of an average
            
//             let adjectives = wiki_macros.iter()
//                 .filter_map(|m| match m {WiktionaryMacro::BeAdj(adj) => Some(adj), _ => None});

//             println!("Adjectives:");
//             println!("\tCount: {}", adjectives.clone().count());
//             println!("\tForms: {}  (Count x 12)", adjectives.clone().count() * 12); // 12 overtly distinct adjective forms


//             let adverbs = wiki_macros.iter()
//                 .filter_map(|m| match m {WiktionaryMacro::BeAdv(adj) => Some(adj), _ => None});

//             println!("Adverbs:");
//             println!("\tCount: {}", adverbs.clone().count());

//             println!("Total: {}", 
//                 nouns.clone().count() + adjectives.clone().count() + verbs.clone().count() + adverbs.clone().count()
//             );


// }


// pub fn ukrainian_word_counts(wiki_macros: Vec<WiktionaryMacro>) {
//             let verbs = wiki_macros.iter()
//                 .filter_map(|m| match m {WiktionaryMacro::UkVerb(adj) => Some(adj), _ => None});

//             println!("Verbs:");
//                 println!("\tCount: {}", verbs.clone().count());
//                 println!("\tForms: {}  (Count x 18)", verbs.clone().count() * 18); // sort of an average
                
//             let nouns = wiki_macros.iter()
//                 .filter_map(|m| match m {WiktionaryMacro::UkNoun(adj) => Some(adj), _ => None});

            
//             println!("Nouns:");
//             println!("\tCount: {}", nouns.clone().count());
//             println!("\tForms: {}  (Count x 10)", nouns.clone().count() * 10); // sort of an average
            
//             let adjectives = wiki_macros.iter()
//                 .filter_map(|m| match m {WiktionaryMacro::UkAdj(adj) => Some(adj), _ => None});

//             println!("Adjectives:");
//             println!("\tCount: {}", adjectives.clone().count());
//             println!("\tForms: {}  (Count x 12)", adjectives.clone().count() * 12); // 12 overtly distinct adjective forms


//             let adverbs = wiki_macros.iter()
//                 .filter_map(|m| match m {WiktionaryMacro::UkAdv(adj) => Some(adj), _ => None});

//             println!("Adverbs:");
//             println!("\tCount: {}", adverbs.clone().count());

//             println!("Total: {}", 
//                 nouns.clone().count() + adjectives.clone().count() + verbs.clone().count() + adverbs.clone().count()
//             );


// }

// pub fn russian_word_counts(wiki_macros: Vec<WiktionaryMacro>) {
//     let verbs = wiki_macros.iter()
//         .filter_map(|m| match m {WiktionaryMacro::RuVerb(adj) => Some(adj), _ => None});

//     println!("Verbs:");
//     println!("\tCount: {}", verbs.clone().count());
//     println!("\tForms: {}  (Count x 18)", verbs.clone().count() * 18); // sort of an average
        
//     let nouns = wiki_macros.iter()
//         .filter_map(|m| match m {WiktionaryMacro::RuNounPlus(adj) => Some(adj), _ => None});
//     let noun_tables = wiki_macros.iter()
//         .filter_map(|m| match m {WiktionaryMacro::RuNounTable(adj) => Some(adj), _ => None});

//     println!("Nouns:");
//     println!("\tCount: {}", nouns.clone().count());
//     println!("\tForms: {}  (Count x 10)", nouns.clone().count() * 10); // sort of an average
    
//     let adjectives = wiki_macros.iter()
//         .filter_map(|m| match m {WiktionaryMacro::RuAdj(adj) => Some(adj), _ => None});

//     println!("Adjectives:");
//     println!("\tCount: {}", adjectives.clone().count());
//     println!("\tForms: {}  (Count x 12)", adjectives.clone().count() * 12); // 12 overtly distinct adjective forms

//     let adverbs = wiki_macros.iter()
//         .filter_map(|m| match m {WiktionaryMacro::RuAdv(adj) => Some(adj), _ => None});

//     println!("Adverbs:");
//     println!("\tCount: {}", adverbs.clone().count());

//     println!("Total: {}", 
//         nouns.clone().count() + adjectives.clone().count() + verbs.clone().count() + adverbs.clone().count()
//     );
// }
