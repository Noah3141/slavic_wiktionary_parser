#[cfg(test)]
mod test;

pub fn select_from<'text>(text: &'text str, from: &str,  to: &str) -> Option<&'text str> {
    if let Some(start) = text.find(from) {
        let start = start + from.len(); // Exclude the `from` from our attempt to find `to`, in case they are the same
        if let Some(end) = text[start..].find(to) {
            Some(&text[start..start + end])
        } else {
            Some(&text[start..])
        }
    } else {
        None
    }
}