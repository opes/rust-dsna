#[derive(Debug)]
pub struct Stack<T> {
    list: Vec<T>,
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { list: vec![] }
    }

    pub fn push(&mut self, value: T) {
        self.list.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.list.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.list.last()
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }
}
