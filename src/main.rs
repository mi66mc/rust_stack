mod stack;

use stack::node::Node;

fn main() {
    let node = Node::new(5, None);
    let node2 = Node::new(5,Some(Box::new(node)));
    println!("{:?}", node2);
}
