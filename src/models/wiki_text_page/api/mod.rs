use super::WikiTextPage;

pub mod parse;
pub mod parse_with;

struct WikiTextPageApi<'a> {
    page: &'a WikiTextPage<'a>
}


impl WikiTextPage<'_> {
    pub fn api(&self) -> WikiTextPageApi { WikiTextPageApi {page: &self} }
}