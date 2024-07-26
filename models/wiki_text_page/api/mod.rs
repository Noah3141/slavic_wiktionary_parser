use super::WikiTextPage;

pub mod parse;
pub mod parse_with;

struct WikiTextPageApi<'a> {
    page: &'a WikiTextPage
}


impl WikiTextPage {
    pub fn api(&self) -> WikiTextPageApi { WikiTextPageApi {page: &self} }
}