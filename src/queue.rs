#![allow(warnings)]
use std::collections::LinkedList;
use std::fmt;

// ----------------- Queue<T> First-In-First-Out (FIFO) ------------------

///
#[derive(Debug)]
pub struct Queue<T: Clone + fmt::Debug> {
    size: usize,
    inner_list: LinkedList<T>,
}

///
impl<T: Clone + fmt::Debug + PartialEq> Queue<T> {
    ///
    fn new() -> Self {
        Queue {
            size: 0usize,
            // We don't need the access backwards, that's why I pick 
            // `SingleLinkedList` rather the the `DoubleLinkedList`.
            inner_list: LinkedList::new(),
        }
    }

    ///
    fn len(&self) -> usize {
        self.size
    }

    ///
    fn dequeue(&mut self) -> Option<T> {
        let result = self.inner_list.pop_front();
        if result.is_some() {
            self.size -= 1;
        }

        result
    }

    ///
    fn enqueue(&mut self, data: T) {
        self.inner_list.push_back(data);
        self.size += 1;
    }

    ///
    fn is_empty(&self) -> bool {
        self.size == 0
    }

    ///
    pub fn is_contains(&self, data: T) -> bool {
        self.inner_list.iter().any(|element| *element == data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_should_work() {
        let my_queue = Queue::<u8>::new();
        assert_eq!(0, my_queue.len());
        assert_eq!(true, my_queue.is_empty());
    }

    #[test]
    fn enqueue_and_dequque_should_work() {
        let mut my_queue = Queue::<u8>::new();
        my_queue.enqueue(10);
        assert_eq!(false, my_queue.is_empty());
        assert_eq!(1, my_queue.len());

        my_queue.enqueue(20);
        my_queue.enqueue(30);
        assert_eq!(3, my_queue.len());
        println!("my_queue {:#?}", &my_queue);

        let mut dequeue_value = my_queue.dequeue();
        assert_eq!(2, my_queue.len());
        assert_eq!(dequeue_value, Some(10));

        dequeue_value = my_queue.dequeue();
        assert_eq!(1, my_queue.len());
        assert_eq!(dequeue_value, Some(20));
        println!("my_queue {:#?}", &my_queue);

        dequeue_value = my_queue.dequeue();
        assert_eq!(true, my_queue.is_empty());
        assert_eq!(0, my_queue.len());
        assert_eq!(dequeue_value, Some(30));
        println!("my_queue {:#?}", &my_queue);
    }

    #[test]
    fn should_work_with_struct() {
        #[derive(Debug, Clone, PartialEq)]
        struct Book<'b> {
            title: &'b str,
            author: &'b str,
        }

        let mut book_queue = Queue::<Book>::new();
        book_queue.enqueue(Book {
            title: "No 1 sales",
            author: "Nobody"
        });
        book_queue.enqueue(Book {
            title: "Homes",
            author: "NZ publish"
        });
        assert_eq!(false, book_queue.is_empty());
        assert_eq!(2, book_queue.len());
        println!("book_queue: {:#?}", &book_queue);

        let mut book = book_queue.dequeue();
        assert_eq!(true, book.is_some());
        assert_eq!("No 1 sales", book.as_ref().unwrap().title);

        book = book_queue.dequeue();
        assert_eq!(true, book.is_some());
        assert_eq!("Homes", book.as_ref().unwrap().title);

        assert_eq!(true, book_queue.is_empty());
        assert_eq!(true, book_queue.dequeue().is_none());
    }

    #[test]
    fn is_contains_should_work() {
        let my_queue = Queue::<u8>::new();
        assert_eq!(false, my_queue.is_contains(9));

        let mut my_queue_2 = Queue::<u8>::new();
        my_queue_2.enqueue(2);
        my_queue_2.enqueue(4);
        my_queue_2.enqueue(6);

        println!("my_queue_2 {:#?}", &my_queue_2);

        assert_eq!(false, my_queue_2.is_contains(9));
        assert_eq!(true, my_queue_2.is_contains(4));
    }
}
