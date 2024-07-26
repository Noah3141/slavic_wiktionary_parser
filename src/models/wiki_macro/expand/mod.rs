use super::WikiMacro;



impl WikiMacro<'_> {
    pub fn expand(&self) {

    }

    pub fn expand_with(&self, client: reqwest::Client) {
        match self.text {
            super::macro_of::MacroOf::RussianVerb(_) => todo!(),
        //    ...
            _ => ()
        }
    }
}