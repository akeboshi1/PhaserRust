struct IteratorList {
    curr: u32,
    next: u32,
}

impl Iterator for IteratorList {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}