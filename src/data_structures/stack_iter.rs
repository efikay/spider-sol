pub struct StackIter<'a, T> {
    pub iter: std::slice::Iter<'a, T>,
}

impl<'a, T> Iterator for StackIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}
