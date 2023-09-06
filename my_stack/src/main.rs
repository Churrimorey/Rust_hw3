use std::cell::RefCell;

struct MyStack<T> {
    stack: RefCell<Vec<T>>,
}

impl<T> MyStack<T> {
    fn new() -> Self {
        Self {
            stack: RefCell::new(Vec::new()),
        }
    }

    fn push(&self, value: T) {
        self.stack.borrow_mut().push(value);
    }

    fn pop(&self) -> Option<T> {
        self.stack.borrow_mut().pop()
    }
}

fn main() {
    let stack = MyStack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    assert_eq!(3, stack.pop().unwrap());
    assert_eq!(2, stack.pop().unwrap());

    stack.push(4);

    assert_eq!(4, stack.pop().unwrap());
    assert_eq!(1, stack.pop().unwrap());
    assert_eq!(None, stack.pop());
}
