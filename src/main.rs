mod stack;

use stack::stack::Stack;

fn main() {
    let mut stack: Stack<i32> = Stack::new();

    stack.push(5);
    stack.push(6);

    println!("{:?}", stack.get_size());
    println!("{:?}", stack.peek());

    stack.push(9);
    stack.push(100);

    println!("{:?}", stack.get_size());
    println!("{:?}", stack.peek());

    let last = stack.pop();

    println!("{:?}", stack.get_size());
    println!("{:?}", stack.peek());
    println!("{:?}", last);
    
    println!("{:?}", stack.is_empty())
}
