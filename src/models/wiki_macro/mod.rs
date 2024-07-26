use macro_of::MacroOf;


#[derive(Debug)]
pub struct WikiMacro<'page> {
    pub page_title: &'page str,
    pub is_head: bool,
    pub is_infl: bool,
    pub is_noun: bool,
    pub is_verb: bool,
    pub is_adj: bool,
    pub is_adv: bool,
    pub text: MacroOf,
}

pub mod new;
pub mod macro_of;
pub mod to_db_row;
pub mod expand;
