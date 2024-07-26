mod test;

///
pub trait Seekable {
    fn find(&self, item: Self) -> Option<usize>;
    fn find_all(&self, item: Self) -> Vec<usize>;
    /// Attempts to find an instance of `item` occurring before `marker`. If no no marker is present, returns Err. If the item does not occur before the marker, returns None.
    fn find_before(&self, item: Self, marker: Self) -> Result<Option<usize>, ()>;
}

impl Seekable for &[u8] {
    fn find(&self, item: Self) -> Option<usize> {
        self.windows(item.len())
            .position(|where_window| where_window == item)
    }

    fn find_all(&self, item: Self) -> Vec<usize> {
        let mut idxs = vec![];
        let mut start_idx: usize = 0;
        while let Some(idx) = self[start_idx..].as_ref().find(item) {
            let global_idx = start_idx + idx;
            idxs.push(global_idx);
            start_idx = global_idx + item.len();
        }
        return idxs;
    }

    /// Searches for an occurrence of the search item before any occurrence of the marker
    fn find_before(&self, item: Self, marker: Self) -> Result<Option<usize>, ()> {
        match self.find(marker) {
            Some(marker_idx) => {
                match self.find(item) {
                    Some(item_idx) => {
                        if item_idx < marker_idx { return Ok(Some(item_idx)) }
                        else { return Ok(None) }
                    }
                    None => Ok(None),
                }
            },
            None => return Err(()),
        }
    }
}

