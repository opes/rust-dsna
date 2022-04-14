#[derive(Debug)]
pub struct Queue<T> {
    pub list: Vec<T>,
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

    pub fn front(&self) -> Option<&T> {
        self.list.first()
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }
}

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
