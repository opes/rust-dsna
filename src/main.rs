mod structures;
pub use crate::structures::collections::{Queue, Stack};

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
