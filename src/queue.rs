/// A generic queue that wraps a `Vec<T>`.
///
/// These methods wrap existing methods on `Vec<T>`. The only reason this
/// struct exists was for my own benefit of implementing.
#[derive(Debug, PartialEq)]
pub struct Queue<T>(Vec<T>);

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Queue(Vec::default())
    }
}

impl<T> Queue<T> {
    /// Create a new queue.
    pub fn new(input: Vec<T>) -> Self {
        Queue(input)
    }

    /// Remove an element from the queue.
    ///
    /// # Panics
    ///
    /// The caller is responsible for ensuring the precondition that the stack
    /// is not empty.
    fn dequeue(&mut self) -> T {
        let len = self.0.len();
        if len == 0 {
            panic!("cannot dequeue from an empty queue")
        }

        self.0.remove(0)
    }

    /// Enqueue an item at the back of the queue.
    pub fn enqueue(&mut self, value: T) {
        self.0.push(value)
    }

    /// Check if the queue is empty.
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Peek the value at the front of the queue.
    ///
    /// # Panics
    ///
    /// The caller is responsible for ensuring the precondition that the stack
    /// is not empty.
    fn peek(&self) -> &T {
        &self.0.get(0).expect("cannot peek into an empty queue")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_queue() {
        let queue: Queue<i32> = Queue::default();
        assert!(queue.is_empty())
    }

    #[test]
    #[should_panic(expected = "cannot peek into an empty queue")]
    fn dequeue_new_queue() {
        let queue: Queue<i32> = Queue::default();
        let _ = queue.peek();
    }

    #[test]
    fn test_queue_operations() {
        let mut queue: Queue<i32> = Queue::default();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(queue.peek(), &1);
        assert_eq!(queue.dequeue(), 1);
        assert_eq!(queue.dequeue(), 2);
        assert_eq!(queue.peek(), &3);
        assert_eq!(queue.dequeue(), 3);
        assert!(queue.is_empty());
    }
}
