#[derive(Debug)]
pub struct Queue<T> {
    list: Vec<T>,
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue { list: vec![] }
    }

    pub fn enqueue(&mut self, value: T) {
        self.list.push(value);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if !self.list.is_empty() {
            Some(self.list.remove(0))
        } else {
            None
        }
    }

    pub fn next(&self) -> Option<&T> {
        self.list.first()
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }
}
