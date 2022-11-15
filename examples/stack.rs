use delegate::delegate;

pub struct Stack<T> {
    inner: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    #[delegate(inner)]
    pub fn is_empty(&self) -> bool {}

    #[delegate(inner)]
    pub fn len(&self) -> usize {}

    #[delegate(inner)]
    pub fn push(&mut self, value: T) {}

    #[delegate(inner)]
    pub fn pop(&mut self) -> Option<T> {}
}

fn main() {
    let mut stack = Stack::new();
    assert!(stack.is_empty());

    stack.push(5);
    assert_eq!(stack.len(), 1);

    stack.pop();
    assert_eq!(stack.len(), 0);
}
