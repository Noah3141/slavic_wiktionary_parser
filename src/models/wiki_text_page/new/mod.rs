use super::WikiTextPage;


struct WikiTextPageBuilder;
pub mod from_xml;


impl WikiTextPage<'_> {
    pub fn new() -> WikiTextPageBuilder { WikiTextPageBuilder }
}