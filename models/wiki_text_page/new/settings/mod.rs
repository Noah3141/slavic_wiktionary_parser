use super::WikiTextPageBuilder;



impl WikiTextPageBuilder {
    pub fn verbose(self) -> WikiTextPageBuilder {
        Self { verbose: true, ..self }
    } 
}