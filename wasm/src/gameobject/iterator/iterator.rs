struct IteratorNode<'a, T> {
    curr: u32,
    next: u32,
}

impl Iterator for IteratorNode<'a, T> {
    type Item = &'a It;
    
    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

trait Iterator{
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}