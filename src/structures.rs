pub mod queue;
pub mod stack;

pub mod examples {
    use crate::structures::queue::Queue;
    use crate::structures::stack::Stack;
    pub fn run_collections() {
        // Stacks implement a LIFO list,
        // where the last/top-most items are
        // pushed on and popped off the stack.

        // Create a new Stack of string slices:
        let mut stack: Stack<&str> = Stack::new();

        // Add some slices to the stack:
        stack.push("Hello");
        stack.push("World");
        stack.push("!");
        stack.push("Hi");

        // See the contents of the stack:
        dbg!(&stack);

        // Get the last/top-most item:
        dbg!(&stack.peek());

        // Get the count of items in the stack:
        dbg!(&stack.len());

        // Remove the last item from the stack:
        let last = stack.pop().unwrap();
        dbg!(&last);

        // Verify its size is updated:
        dbg!(&stack.len());

        // And the last/top-most item changed:
        dbg!(&stack.peek());

        // Then see it in its entirety:
        dbg!(&stack);

        // Queues implement a FIFO list,
        // where items are enqueued to the end
        // of the list, and dequeued from the front
        // of the list (eg. like a waiting line)

        // Create a new queue of string slices:
        let mut queue: Queue<&str> = Queue::new();

        // Add items to the queue:
        queue.enqueue("Hi");
        queue.enqueue("Hello");
        queue.enqueue("World");
        queue.enqueue("!");

        // Pay attention to the ordering:
        dbg!(&queue);

        // And who we can access next:
        dbg!(&queue.next());

        // We can also see the total queued items count:
        dbg!(&queue.len());

        // ...and take the first item off the queue:
        let first = queue.dequeue().unwrap();
        dbg!(&first);

        // ...verifying the size was updated:
        dbg!(&queue.len());

        // ...and the remaining items in the queue were
        // shifted to the beginning:
        dbg!(&queue.next());
        dbg!(&queue);
    }
}
