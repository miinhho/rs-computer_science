use std::collections::LinkedList;

#[derive(Debug)]
pub struct Queue<T> {
    elements: LinkedList<T>,
}

impl<T> Queue<T> {
    // Creates a new empty Queue
    pub fn new() -> Queue<T> {
        Queue {
            elements: LinkedList::new(),
        }
    }

    // Adds an element to the back of the queue
    pub fn enqueue(&mut self, value: T) {
        self.elements.push_back(value)
    }

    // Removes and returns the front element from the queue, or None is empty
    pub fn dequeue(&mut self) -> Option<T> {
        self.elements.pop_front()
    }

    // Returns a reference to the front element of the queue, or None if empty
    pub fn peek_front(&self) -> Option<&T> {
        self.elements.front()
    }

    // Returns a reference to the back element of the queue, or None if empty
    pub fn peek_back(&self) -> Option<&T> {
        self.elements.back()
    }

    // Returns the number of elements in the queue
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    // Checks if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn drain(&mut self) {
        self.elements.clear();
    }
}

// Implementing the Default trait for Queue
impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue::new()
    }
}
