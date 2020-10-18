use std::collections::LinkedList;
use std::fmt;

// ------------------------- Stack<T> Last-in-first-out -------------

///
#[derive(Debug)]
pub struct Stack<T: Clone + fmt::Debug> {
    size: usize,
    inner_list: LinkedList<T>,
}

///
impl<T: Clone + fmt::Debug> Stack<T> {
    ///
    pub fn new() -> Self {
        Stack {
            size: 0usize,
            inner_list: LinkedList::new(),
        }
    }

    ///
    pub fn len(&self) -> usize {
        self.size
    }

    ///
    pub fn pop(&mut self) -> Option<T> {
        let result = self.inner_list.pop_back();
        if result.is_some() {
            self.size -= 1;
        }

        result
    }

    ///
    pub fn peek(&mut self) -> Option<T> {
        // As the `LinkedList` doesn't implement the `peek`, then we need
        // to `pop` first and copy and then `push` back again
        let temp_pop = self.inner_list.pop_back();
        if temp_pop.is_some() {
            let original = temp_pop.unwrap();
            let copy = original.clone();
            self.inner_list.push_back(original);
            Some(copy)
        } else {
            None
        }
    }

    ///
    pub fn push(&mut self, data: T) {
        self.inner_list.push_back(data);
        self.size += 1;
    }

    ///
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_should_work() {
        let my_stack = Stack::<u8>::new();
        assert_eq!(0, my_stack.len());
        assert_eq!(true, my_stack.is_empty());
    }

    #[test]
    fn should_work_with_primitive_type() {
        let mut my_stack = Stack::<u8>::new();
        my_stack.push(10);
        assert_eq!(false, my_stack.is_empty());
        assert_eq!(1, my_stack.len());

        my_stack.push(20);
        my_stack.push(30);
        assert_eq!(3, my_stack.len());

        let mut pop_value = my_stack.pop();
        assert_eq!(2, my_stack.len());
        assert_eq!(pop_value, Some(30));

        pop_value = my_stack.pop();
        assert_eq!(1, my_stack.len());
        assert_eq!(pop_value, Some(20));
        println!("my_stack {:#?}", &my_stack);

        let peek_value = my_stack.peek();
        assert_eq!(peek_value, Some(10));
        assert_eq!(1, my_stack.len());

        pop_value = my_stack.pop();
        assert_eq!(true, my_stack.is_empty());
        assert_eq!(0, my_stack.len());
        assert_eq!(pop_value, Some(10));
        println!("my_stack {:#?}", &my_stack);

        // Empty
        assert_eq!(true, my_stack.is_empty());
        assert_eq!(None, my_stack.peek());
        assert_eq!(None, my_stack.pop());
    }

    #[test]
    fn should_work_with_struct() {
        #[derive(Debug, Clone)]
        struct Book<'b> {
            title: &'b str,
            author: &'b str,
        }

        let mut book_stack = Stack::<Book>::new();
        book_stack.push(Book {
            title: "No 1 sales",
            author: "Nobody",
        });
        book_stack.push(Book {
            title: "Homes",
            author: "NZ publish",
        });
        assert_eq!(false, book_stack.is_empty());
        assert_eq!(2, book_stack.len());

        let mut book = book_stack.pop();
        assert_eq!(true, book.is_some());
        assert_eq!("Homes", book.as_ref().unwrap().title);

        let peek = book_stack.peek();
        assert_eq!(true, peek.is_some());
        assert_eq!("No 1 sales", peek.as_ref().unwrap().title);
        println!("book_stack {:#?}", &book_stack);

        book = book_stack.pop();
        assert_eq!(true, book.is_some());
        assert_eq!("No 1 sales", book.as_ref().unwrap().title);

        // Empty
        assert_eq!(true, book_stack.is_empty());
        assert_eq!(true, book_stack.peek().is_none());
        assert_eq!(true, book_stack.pop().is_none());
    }
}
