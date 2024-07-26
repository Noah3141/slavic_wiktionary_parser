#[cfg(test)]
mod test;

use crate::traits::Seekable;

pub fn redact_comment_block(page_bytes: &mut [u8]) {
    let comment_starts = page_bytes.as_ref().find_all(b"<comment>");
    for comment_start in comment_starts {
        let maybe_comment_end = page_bytes[..].as_ref().find_before(b"</comment>", b"<text").expect("text block openner tag");
        if let Some(comment_end) = maybe_comment_end {
            page_bytes[comment_start..comment_end + b"</comment>".len()].fill(0)
        }
    }
}



