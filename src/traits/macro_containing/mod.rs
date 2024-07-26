#[cfg(test)]
mod test;

pub trait MacroContaining {
    /// Searches the self for {{macro...}}
    fn find_macros(&self) -> Vec<String>;
}

impl MacroContaining for String {
    fn find_macros(&self) -> Vec<String> {
        let mut macros = Vec::new();
        let mut stack = Vec::new();
        let mut macro_start = None;
        let mut chars = self.chars().enumerate().peekable();

        while let Some((i, c)) = chars.next() {
            if c == '{' {
                stack.push(i);
                if stack.len() == 2 {
                    macro_start = Some(i - 1);
                }
            } else if c == '}' {
                stack.pop();
                if stack.len() == 1 && macro_start.is_some() {
                    let start = macro_start.unwrap();
                    let end = i + 2;
                    let macro_string = self.chars().skip(start).take(end - start).collect::<String>();
                    macros.push(macro_string);
                    macro_start = None;
                }
            }
        }

        macros
    }
}