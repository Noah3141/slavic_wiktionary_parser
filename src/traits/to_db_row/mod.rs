mod test;

pub trait ToDbRow {
    fn to_russian_noun(self) -> Result<rubit_api_db::dictionary_info::russian::RussianNoun, ()>;
    fn to_russian_verb(self) -> Result<rubit_api_db::dictionary_info::russian::RussianVerb, ()>;
    fn to_russian_adj(self) -> Result<rubit_api_db::dictionary_info::russian::RussianAdjective, ()>;
    fn to_russian_adv(self) -> Result<rubit_api_db::dictionary_info::russian::RussianAdverb, ()>;
}