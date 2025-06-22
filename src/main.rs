mod stack;

use stack::node::Node;
use stack::stack::Stack;

fn main() {

    let mut stack: Stack<i32> = Stack::new();

    stack.push(1);
    stack.push(2);

    println!("{:?}", stack);
}
