use crate::stack::stack::Stack;
use std::fmt::Debug;

/// BrowserHistoryStack<T> Last-in-first-out (LIFO)
#[derive(Debug)]
pub struct BrowserHistoryStack<T: Clone + Debug + PartialEq> {
    inner_stack: Stack<T>,
}

///
impl<T: Clone + Debug + PartialEq> BrowserHistoryStack<T> {
    ///
    pub fn new() -> Self {
        BrowserHistoryStack {
            inner_stack: Stack::<T>::new(),
        }
    }

    ///
    pub fn size(&self) -> usize {
        self.inner_stack.size()
    }

    ///
    pub fn pop(&mut self) -> Option<T> {
        self.inner_stack.pop()
    }

    /// `peek_tail` works like `pop_tail`, but it returns the immutable reference to the last node
    /// data rather than consume it.
    pub fn peek(&mut self) -> Option<&T> {
        self.inner_stack.peek()
    }

    ///
    pub fn push(&mut self, data: T) {
        self.inner_stack.push(data)
    }

    ///
    pub fn is_empty(&self) -> bool {
        self.inner_stack.is_empty()
    }

    ///
    pub fn contains(&self, data: T) -> bool {
        self.inner_stack.contains(data)
    }

    ///
    pub fn print_stack(&self) {
        self.inner_stack.print_stack();
    }

    ///
    pub fn get_stack_content(&self) -> String {
        self.inner_stack.get_stack_content()
    }
}
