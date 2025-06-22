use crate::stack::node::Node;

#[derive(Debug)]
pub struct Stack<T> {
    top: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            top: None,
            size: 0
        }
    }

    pub fn push(&mut self, value: T) {
        let node = Node::new(
            value,
            self.top.take()
        );
        self.set_top(Some(Box::new(node)));
        self.size += 1;
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    fn set_top(&mut self, node: Option<Box<Node<T>>>) {
        self.top = node
    }
}