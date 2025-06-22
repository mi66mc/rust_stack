# Building a Stack from Scratch in Rust

This project is a simple and educational implementation of a `Stack` in Rust. It supports basic operations like insertion (`push`) and retrieval (`pop`).

---

## 📁 Structure

- `Node<T>`: Represents a node with a value and the next node.
- `Stack<T>`: A stack that implements the basics methods.

---

## 🚀 Features

## Stack<T>
> Generic type `T` for flexibility.
- `push(value: T)`: Adds a value to the top of the stack.
- `pop() -> Option<T>`: Removes and returns the value from the top of the stack.
- `peek() -> Option<&T>`: Returns a reference to the value at the top of the stack without removing it.
- `get_size() -> usize`: Returns the current size of the stack.
- `is_empty() -> bool`: Checks if the stack is empty.

---

## 🛠️ Example Usage

```rust
    let mut stack: Stack<i32> = Stack::new();

    stack.push(5);
    stack.push(6);

    println!("{:?}", stack.get_size()); // 2
    println!("{:?}", stack.peek());     // Some(6)      

    stack.push(9);
    stack.push(100);

    println!("{:?}", stack.get_size()); // 4
    println!("{:?}", stack.peek());     // Some(100)

    let last = stack.pop();

    println!("{:?}", stack.get_size()); // 3
    println!("{:?}", stack.peek());     // Some(9)
    println!("{:?}", last);             // Some(100)
    
    println!("{:?}", stack.is_empty()); // false
```