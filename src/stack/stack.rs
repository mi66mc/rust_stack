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

    pub fn pop(&mut self) -> Option<T> {
        self.top.take().map(|node| {
            self.set_top(node.next);
            self.size -= 1;
            node.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.top.as_ref().map(|node| node.peek_value())
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.top.is_none()
    }

    fn set_top(&mut self, node: Option<Box<Node<T>>>) {
        self.top = node
    }
}