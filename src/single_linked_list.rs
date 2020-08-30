#![allow(warnings)]
//! # `SingeLinkedList`
//!
//! It looks like this:
//!
//! Head --> Node1 --> Node2 --> Tail
//!

use std::fmt;
use std::ptr;

///
type NextNode<T> = Option<Box<Node<T>>>;

///
#[derive(PartialEq)]
pub struct Node<T: Clone + Copy + fmt::Debug> {
    data: T,
    next: NextNode<T>,
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

    pub fn get_next(&self) -> &Option<Box<Node<T>>> {
        &self.next
    }
}

///
pub struct SingeLinkedList<T: Clone + Copy + fmt::Debug> {
    size: usize,
    head: Node<T>,
    // pub tail: Node<T>,
}

impl<T: Clone + Copy + fmt::Debug> fmt::Debug for SingeLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SingeLinkedList")
            .field("size", &self.size)
            .field("head", &self.head)
            .finish()
    }
}

impl<T: Clone + Copy + fmt::Debug> SingeLinkedList<T> {
    pub fn new(data: T) -> Self {
        SingeLinkedList {
            size: 1,
            head: Node { data, next: None },
        }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_head(&self) -> &Node<T> {
        &self.head
    }

    fn has_child_node(&self, node_to_check: &Node<T>) -> bool {
        let head_without_child = self.head.next.is_none();
        if head_without_child == true {
            return false;
        }

        let mut has_child_for_the_node = false;
        let mut next_node: &Node<T> = self.head.next.as_ref().unwrap();
        let mut walk_through_size = 1;
        while walk_through_size <= self.size {
            // Compare to see whether the same `&Node<T>`
            if ptr::eq(next_node, node_to_check) {}

            walk_through_size += 1;
        }

        has_child_for_the_node
    }

    pub fn add_to_end(&mut self, data: T) {
        let head_without_child = self.head.next.is_none();
        if head_without_child == true {
            self.head.next = Some(Box::new(Node { data, next: None }));
            self.size += 1;
            println!("Attached data to 'self.head'.");
            return;
        }

        let mut next_node: &mut Node<T> = self.head.next.as_mut().unwrap();
        let mut walk_through_size = 1;
        while next_node.next.is_some() {
            next_node = next_node.next.as_mut().unwrap();
        }

        next_node.next = Some(Box::new(Node { data, next: None }));
        self.size += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_empty_list() {
        let empty_list = SingeLinkedList::new("Wison Ye");
        println!("empty_list: {:#?}", empty_list);

        assert_eq!(empty_list.get_size(), 1);
        assert_eq!(empty_list.get_head().get_data(), &"Wison Ye");
        assert_eq!(empty_list.get_head().get_next().is_none(), true);
    }

    #[test]
    fn add_note_to_list_should_works() {
        let mut test_list = SingeLinkedList::new("Wison Ye");
        test_list.add_to_end("Fion Li");
        test_list.add_to_end("Mike Ye");

        println!("test_list: {:#?}", test_list);
        assert_eq!(test_list.get_size(), 3);
    }
}
