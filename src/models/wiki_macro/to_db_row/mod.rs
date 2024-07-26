// use crate::traits::to_db_row::ToDbRow;

// use super::WikiMacro;
// use super::macro_of::MacroOf;


// impl<'page> ToDbRow for WikiMacro<'_> {
//     fn to_russian_adj(self) -> Result<rubit_api_db::dictionary_info::russian::RussianAdjective, ()> {
//         if !self.is_adj { return Err(()) }

//         match self.text {
//             MacroOf::InflRussianAdjective(text) => {
//                 // self.expand -> 
//             },
//             MacroOf::RussianAdjective(text) => (),
//             _ => return Err(())
//         }
//     }
    
//     fn to_russian_noun(self) -> Result<rubit_api_db::dictionary_info::russian::RussianNoun, ()> {
//         todo!()
//     }
    
//     fn to_russian_verb(self) -> Result<rubit_api_db::dictionary_info::russian::RussianVerb, ()> {
//         todo!()
//     }
    
//     fn to_russian_adv(self) -> Result<rubit_api_db::dictionary_info::russian::RussianAdverb, ()> {
//         todo!()
//     }
    
// }