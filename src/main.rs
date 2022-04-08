#[derive(Debug)]
struct Stack<T> {
    list: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack { list: vec![] }
    }

    fn push(&mut self, value: T) {
        self.list.push(value);
    }

    fn pop(&mut self) -> Option<T> {
        self.list.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.list.last()
    }

    fn len(&self) -> usize {
        self.list.len()
    }
}

#[derive(Debug)]
struct Queue<T> {
    list: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Queue<T> {
        Queue { list: vec![] }
    }

    fn enqueue(&mut self, value: T) {
        self.list.push(value);
    }

    fn dequeue(&mut self) -> Option<T> {
        if !self.list.is_empty() {
            Some(self.list.remove(0))
        } else {
            None
        }
    }

    fn front(&self) -> Option<&T> {
        self.list.first()
    }

    fn len(&self) -> usize {
        self.list.len()
    }
}

fn main() {
    let mut stack: Stack<&str> = Stack::new();
    stack.push("Hello");
    stack.push("World");
    stack.push("!");
    stack.push("Hi");
    dbg!(&stack);
    dbg!(&stack.peek());
    dbg!(&stack.len());
    let last = stack.pop().unwrap();
    dbg!(&last);
    dbg!(&stack.len());
    dbg!(&stack.peek());
    dbg!(&stack);

    let mut queue: Queue<&str> = Queue::new();
    queue.enqueue("Hi");
    queue.enqueue("Hello");
    queue.enqueue("World");
    queue.enqueue("!");
    dbg!(&queue);
    dbg!(&queue.front());
    dbg!(&queue.len());
    let first = queue.dequeue().unwrap();
    dbg!(&first);
    dbg!(&queue.len());
    dbg!(&queue.front());
    dbg!(&queue);
}
