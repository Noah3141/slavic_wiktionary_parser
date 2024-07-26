#[cfg(test)]
mod test;

pub trait MacroContaining {
    /// Searches the self for {{macro...}}
    fn find_macros(&self) -> Vec<String>;
}

impl MacroContaining for String {
    /// Finds macros in a String
    fn find_macros(&self) -> Vec<String> {
        let mut macros = Vec::new();
        let mut stack = Vec::new();
        let mut macro_start: Option<usize> = None;
        let mut chars = self.chars().enumerate();

        while let Some((index, c)) = chars.next() {
            match c {
                '{' => {
                    stack.push(index);
                    if stack.len() == 2 {
                        macro_start = Some(index - 1);
                    }
                },
                '}' => {
                    stack.pop();
                    if stack.len() == 1 && macro_start.is_some() {
                        let start = macro_start.unwrap();
                        let end = index + 2;
                        let macro_string = self.chars().skip(start).take(end - start).collect::<String>();
                        if macro_string.starts_with("{{") && macro_string.ends_with("}}") {
                            macros.push(macro_string);
                        }
                        macro_start = None;
                    }
                },
                _ => (),
            }
        }

        macros
    }
}