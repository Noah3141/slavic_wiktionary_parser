#[cfg(test)]
mod test;

/// Calling this function with `from` expects to find `from`, but does not expect that another section follows (extending the returned slice unto the end of `text`)
pub fn select_unto_language_header<'text>(text: &'text str, from: &str) -> Result<&'text str, String> {
    match text.find(from) {
        None => Err(String::from("Starting text was not found")),
        Some(start) => {
            let language_section_onward = &text[start..];
        
            // Find the position of the next "\n==" where the character after is not '='
            let maybe_end = language_section_onward
                .match_indices("\n==")
                .find(|(pos, _)| {
                    language_section_onward[pos + 3..].chars().next().expect("The end should not be here yet") != '=' 
                });

            match maybe_end {
                Some((end,_)) => Ok(&language_section_onward[..end]),
                None => Ok(language_section_onward),
            }
        } 
    }
}