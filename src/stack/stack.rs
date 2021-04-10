use crate::linked_list::single_linked_list::SingleLinkedList;
use std::fmt::Debug;

/// Stack<T> Last-in-first-out (LIFO)
#[derive(Debug)]
pub struct Stack<T: Clone + Debug + PartialEq> {
    size: usize,
    inner_list: SingleLinkedList<T>,
}

///
impl<T: Clone + Debug + PartialEq> Stack<T> {
    ///
    pub fn new() -> Self {
        Stack {
            size: 0usize,
            // We don't need the access backwards, that's why I pick
            // `SingleLinkedList` rather the `DoubleLinkedList`.
            inner_list: SingleLinkedList::<T>::new(),
        }
    }

    ///
    pub fn size(&self) -> usize {
        self.size
    }

    ///
    pub fn pop(&mut self) -> Option<T> {
        let result = self.inner_list.pop_tail();
        if result.is_some() {
            self.size -= 1;
        }

        result
    }

    /// `peek_tail` works like `pop_tail`, but it returns the immutable reference to the last node
    /// data rather than consume it.
    pub fn peek(&mut self) -> Option<&T> {
        self.inner_list.peek_tail()
    }

    ///
    pub fn push(&mut self, data: T) {
        self.inner_list.append(data);
        self.size += 1;
    }

    ///
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    ///
    pub fn contains(&self, data: T) -> bool {
        self.inner_list.contains(data)
    }

    ///
    pub fn print_stack(&self) {
        self.inner_list.print_list();
    }

    ///
    pub fn get_stack_content(&self) -> String {
        self.inner_list.get_list_content()
    }
}
