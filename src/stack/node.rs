#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>
}

impl<T> Node<T> {
    pub fn new(value: T, next: Option<Box<Node<T>>>) -> Self {
        Node {
            value,
            next
        }
    }

    pub fn peek_value(&self) -> &T {
        &self.value
    }

    pub fn set_next(&mut self, next: Option<Box<Node<T>>>) {
        self.next = next;
    }
}