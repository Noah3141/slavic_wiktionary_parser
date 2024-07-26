use super::WikiTextPage;


pub struct WikiTextPageBuilder {
    verbose: bool
}


pub mod from_xml;
pub mod settings;


impl WikiTextPage {
    pub fn new() -> WikiTextPageBuilder { 
        WikiTextPageBuilder { verbose: false } 
    }
}