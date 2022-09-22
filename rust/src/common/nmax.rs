use std::collections::VecDeque;

#[derive(Debug)]
pub struct NMax<T>{
    v: VecDeque<T>,
    n: usize,
}

impl<T> NMax<T> {
    pub fn new(n: usize) -> NMax<T> {
        NMax { v: VecDeque::new(), n }
    }

    pub fn push(&mut self, item: T) {
        self.v.push_front(item);
        self.v.truncate(self.n.clone());
    }

    pub fn iter(&self) -> std::collections::vec_deque::Iter<T> {
        self.v.iter()
    }
}
