#![allow(warnings)]
//! # `SingeLinkedList`
//!
//! It looks like this:
//!
//! Head --> Node1 --> Node2 --> Tail
//!

use std::fmt;
use std::ptr::NonNull;

/// The type here describes either a **valid pointer** or
/// a **null pointer**. That's why we use `Option` here
/// which only got 2 possible values:
///
/// `Some()` - **valid pointer**
/// `None` - **null pointer**
///
/// The **valid pointer* means:
///
/// 1. Non null, it must point to some `Node<T>` instance.
/// 2. `Node<T>` instance should live inside the **heap**.
///
/// That's why we use `Box<Node<T>>` which is a smart pointer
/// in **Rust**. For getting back a `NonNull<Node<T>>` type,
/// we can use:
///
/// ```
/// // let temp_node: Box<Node<T>> = Box::new(Node {});
/// // let non_null_ptr = NonNull::from(Box::leak(temp_node));
///
/// ```
type PointToNextNode<T> = Option<NonNull<Node<T>>>;

///
pub struct Node<T: Clone + Copy + fmt::Debug> {
    data: T,
    next: PointToNextNode<T>,
}

impl<T: Clone + Copy + fmt::Debug> fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Node")
            .field("data", &self.data)
            .field("next", &self.next)
            .finish()
    }
}

impl<T: Clone + Copy + fmt::Debug> Node<T> {
    pub fn get_data(&self) -> &T {
        &self.data
    }

    // pub fn get_next(&self) -> &Option<NonNull<T>> {
    // &self.next
    // }
}

///
pub struct SingeLinkedList<T: Clone + Copy + fmt::Debug> {
    size: usize,
    head: PointToNextNode<T>,
    tail: PointToNextNode<T>,
}

impl<T: Clone + Copy + fmt::Debug> fmt::Debug for SingeLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            // `self.head.as_ref()` get back `Option<&NonNull<Node<T>>>`
            // `self.head.as_ref().unwrap()` get back `&NonNull<Node<T>>`
            // `self.head.as_ref().unwrap().as_ptr()` get back `*mut Node<T>` (raw pointer to Node<T>)
            let head_ptr_to: *mut Node<T> = self.head.as_ref().unwrap().as_ptr();

            // Cast back to `Box<Node<T>>` from raw pointer
            let temp_head = Some(Box::from_raw(head_ptr_to));

            let tail_ptr_to: *mut Node<T> = self.tail.as_ref().unwrap().as_ptr();
            // let temp_tail = Some(Box::from_raw(tail_ptr_to));

            f.debug_struct("SingeLinkedList")
                .field("size", &self.size)
                .field("head", &temp_head)
                // .field("tail", &temp_tail)
                .finish()
        }
    }
}

impl<T: Clone + Copy + fmt::Debug> SingeLinkedList<T> {
    pub fn new(data: T) -> Self {
        let head_and_tail: Box<Node<T>> = Box::new(Node { data, next: None });
        let non_null_raw_ptr = NonNull::from(Box::leak(head_and_tail));

        SingeLinkedList {
            size: 1,
            head: Some(non_null_raw_ptr),
            tail: Some(non_null_raw_ptr),
        }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    // pub fn get_head(&self) -> &Node<T> {
    // &self.head
    // }

    // fn has_child_node(&self, node_to_check: &Node<T>) -> bool {
    // let head_without_child = self.head.next.is_none();
    // if head_without_child == true {
    // return false;
    // }
    //
    // let mut has_child_for_the_node = false;
    // let mut next_node: &Node<T> = self.head.next.as_ref().unwrap();
    // let mut walk_through_size = 1;
    // while walk_through_size <= self.size {
    // // Compare to see whether the same `&Node<T>`
    // if ptr::eq(next_node, node_to_check) {}
    //
    // walk_through_size += 1;
    // }
    //
    // has_child_for_the_node
    // }

    // pub fn add_to_end(&mut self, data: T) {
    // let head_without_child = self.head.next.is_none();
    // if head_without_child == true {
    // self.head.next = Some(Box::new(Node { data, next: None }));
    // self.size += 1;
    // // println!("Attached data to 'self.head'.");
    // return;
    // }
    //
    // let mut next_node: &mut Node<T> = self.head.next.as_mut().unwrap();
    // let mut walk_through_size = 1;
    // while next_node.next.is_some() {
    // next_node = next_node.next.as_mut().unwrap();
    // }
    //
    // next_node.next = Some(Box::new(Node { data, next: None }));
    // self.size += 1;
    // }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.size <= 0 {
            return None;
        }

        None
    }

    pub fn pop_end() -> Option<T> {
        None
    }

    pub fn push_to_end(&mut self, data: T) {
        // tail --> New Node (become new `tail`)

        // Create new tail node
        let new_tail_node = Box::new(Node { data, next: None });
        let new_tail_node_ptr = NonNull::from(Box::leak(new_tail_node));

        // `self.tail.next` is always `None`, so just reset it no need to worry about anything.
        unsafe {
            let tail_ptr_to = self.tail.unwrap().as_ptr();
            (*tail_ptr_to).next = Some(new_tail_node_ptr);
        }

        // Reset `self.tail`
        self.tail = Some(new_tail_node_ptr);
        self.size += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_empty_list() {
        let empty_list = SingeLinkedList::new("Wison Ye");
        println!("empty_list: {:#?}", &empty_list);

        assert_eq!(empty_list.get_size(), 1);
        println!("empty_list: {:#?}", &empty_list);
        // assert_eq!(empty_list.get_head().get_data(), &"Wison Ye");
        // assert_eq!(empty_list.get_head().get_next().is_none(), true);
    }

    // #[test]
    // fn push_to_end_should_work() {
    // let mut test_list = SingeLinkedList::new("Wison Ye");
    // test_list.push_to_end("Fion Li");
    // // test_list.push_to_end("Mike Ye");
    //
    // println!("test_list: {:#?}", test_list);
    // assert_eq!(test_list.get_size(), 3);
    // }
}
