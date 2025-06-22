use crate::stack::node::Node;

pub struct Stack<T> {
    top: Option<Box<Node<T>>>,
    size: usize,
}