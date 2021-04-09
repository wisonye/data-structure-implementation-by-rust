use crate::linked_list::single_linked_list_node::{NextNode, Node};
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::ptr::NonNull;

///
/// # `SingleLinkedList`
///
/// It looks like this:
///
/// Head --> Node1 --> Node2 --> Tail
///
pub struct SingleLinkedList<T: Debug + PartialEq> {
    size: usize,
    head: NextNode<T>,
    tail: NextNode<T>,
}

///
impl<T: Debug + PartialEq> Debug for SingleLinkedList<T> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let head_option_ref: Option<&Node<T>> = match self.head {
            Some(temp_head) => {
                // Get back raw pointer to `Node<T>` from `NonNull<Node<T>>`
                let ptr: *mut Node<T> = temp_head.as_ptr();
                unsafe { Some(&*ptr) }
            }
            None => None,
        };

        let tail_option_ref: Option<&Node<T>> = match self.tail {
            Some(temp_tail) => {
                // Get back raw pointer to `Node<T>` from `NonNull<Node<T>>`
                let ptr: *mut Node<T> = temp_tail.as_ptr();
                unsafe { Some(&*ptr) }
            }
            None => None,
        };

        f.debug_struct("SingleLinkedList")
            .field("size", &self.size)
            .field("head", &head_option_ref)
            .field("tail", &tail_option_ref)
            .finish()
    }
}

///
impl<T: Debug + PartialEq> SingleLinkedList<T> {
    ///
    pub fn new() -> Self {
        SingleLinkedList {
            size: 0,
            head: None,
            tail: None,
        }
    }

    ///
    pub fn size(&self) -> usize {
        self.size
    }

    ///
    pub fn get_head(&self) -> Option<&T> {
        match self.head {
            Some(ptr_to_node) => {
                let ptr: *mut Node<T> = ptr_to_node.as_ptr();
                unsafe { Some(&(*ptr).data) }
            }
            None => None,
        }
    }

    ///
    pub fn get_tail(&self) -> Option<&T> {
        match self.tail {
            Some(ptr_to_node) => {
                let ptr: *mut Node<T> = ptr_to_node.as_ptr();
                unsafe { Some(&(*ptr).data) }
            }
            None => None,
        }
    }

    ///
    pub fn pop_head(&mut self) -> Option<T> {
        if self.head.is_none() || self.tail.is_none() || self.size <= 0 {
            return None;
        }

        // Consume `self.head`
        let ptr_to_head_node: NonNull<Node<T>> = self.head.unwrap();
        let box_of_head_node: Box<Node<T>> = unsafe { Box::from_raw(ptr_to_head_node.as_ptr()) };

        let head_data: T = box_of_head_node.data;
        let head_next: NextNode<T> = box_of_head_node.next;

        // Update `self.head`, make it point to the next node
        self.head = head_next;
        self.size -= 1;

        // Update `self.tail` if empty
        if self.head.is_none() {
            self.tail = None;
        }

        Some(head_data)
    }

    ///
    pub fn pop_tail(&mut self) -> Option<T> {
        if self.tail.is_none() || self.head.is_none() || self.size <= 0 {
            return None;
        }

        // Single node case
        if self.size == 1 {
            // Consume `self.head`
            let ptr_to_head_node: NonNull<Node<T>> = self.head.unwrap();
            let box_of_head_node: Box<Node<T>> =
                unsafe { Box::from_raw(ptr_to_head_node.as_ptr()) };

            let head_data: T = box_of_head_node.data;

            self.head = None;
            self.tail = None;
            self.size = 0;

            return Some(head_data);
        }

        //
        // Otherwise, let's walk through from the `self.head` to the previous of `self.tail`
        // and cut the tail
        //
        let mut ptr_to_current_node: *mut Node<T> = self.head.as_ref().unwrap().as_ptr();
        let ptr_to_tail_node: *mut Node<T> = self.tail.as_ref().unwrap().as_ptr();
        let mut found = false;

        while !found {
            let ptr_to_current_node_next: *mut Node<T> =
                unsafe { (*ptr_to_current_node).next.unwrap().as_ptr() };

            // If the pointer is the same, then we found it. So we need to stop the loop
            // and reset the `next`. As `self.tail` still point to the last node, so it won't cause
            // the memory leak. We will consume the last node last before return:)
            if ptr_to_current_node_next == ptr_to_tail_node {
                found = true;
                unsafe { (*ptr_to_current_node).next = None };
            }
            // Otherwise, update the pointer to try again
            else {
                ptr_to_current_node = ptr_to_current_node_next;
            }
        }

        // Consume the `self.tail` (the last node instance)
        let box_of_tail_node: Box<Node<T>> = unsafe { Box::from_raw(self.tail.unwrap().as_ptr()) };
        let tail_data: T = box_of_tail_node.data;

        // Update
        self.tail = NonNull::new(ptr_to_current_node);
        self.size -= 1;

        Some(tail_data)
    }

    ///
    pub fn contain(&self, data_to_check: T) -> bool {
        if self.head.is_none() {
            // println!("container -> No head",);
            return false;
        }

        let mut current_node: Option<&NonNull<Node<T>>> = self.head.as_ref();

        while current_node.is_some() {
            // First, we need to get back the `&T` from `Option<&NonNull<Node<T>>>`
            let ptr_to_node = current_node.unwrap().as_ptr();
            let node_ref = unsafe { &*ptr_to_node };

            // println!("data_to_check {:?}", data_to_check);
            // println!("node_ref.data {:?}", node_ref.data);

            if data_to_check == node_ref.data {
                // println!("container -> matched, return eariler",);
                return true;
            }

            // Update `current_node` to next
            current_node = node_ref.next.as_ref();
        }

        // println!("container -> no match",);
        false
    }

    ///
    pub fn insert_at_head(&mut self, data: T) {
        // Create new box wrap the node instance
        let new_head_node_box = Box::new(Node {
            data,
            next: if self.head.is_some() { self.head } else { None },
        });

        // Convert `Box<Node<T>>` to `NonNull<Node<T>>`
        let new_head_node_ptr = NonNull::from(Box::leak(new_head_node_box));

        // Get back `Option<NonNull<Node<T>>>`
        let new_head_node = Some(new_head_node_ptr);

        if self.head.is_none() {
            self.head = new_head_node;
            self.tail = new_head_node;
        } else {
            self.head = new_head_node;
        }

        self.size += 1;
    }

    ///
    pub fn append(&mut self, data: T) {
        // tail --> New Node (become new `tail`)

        // Create new box wrap the node instance
        let new_tail_node_box = Box::new(Node { data, next: None });

        // Convert `Box<Node<T>>` to `NonNull<Node<T>>`
        let new_tail_node_ptr = NonNull::from(Box::leak(new_tail_node_box));

        // Get back `Option<NonNull<Node<T>>>`
        let new_tail_node = Some(new_tail_node_ptr);

        // `self.tail.next` is always `None`, so just reset it no need to worry about anything.
        unsafe {
            match self.tail {
                Some(temp_tail) => {
                    // `temp_tail` is `NonNull<Node<TT>>`, here we get back the raw pointer
                    let raw_ptr: *mut Node<T> = temp_tail.as_ptr();
                    // let tail_node_instance: Node<T> = *raw_ptr;
                    (*raw_ptr).next = new_tail_node;
                }
                None => self.head = new_tail_node,
            }
        }

        // Reset `self.tail`
        self.tail = new_tail_node;
        self.size += 1;
    }

    ///
    pub fn print_list(&self) {
        if self.head.is_none() {
            println!("empty list ");
            return;
        }

        let mut current_node: Option<&NonNull<Node<T>>> = self.head.as_ref();
        let mut list_content: Vec<String> = Vec::new();

        while current_node.is_some() {
            // First, we need to get back the `&T` from `Option<&NonNull<Node<T>>>`
            let ptr_to_node = current_node.unwrap().as_ptr();
            let node_ref = unsafe { &*ptr_to_node };
            let node_data = &node_ref.data;
            list_content.push(format!("{:?}", node_data));

            // Update `current_node` to next
            current_node = node_ref.next.as_ref();
        }

        println!(
            "{}{}",
            format!("({} elements): ", self.size),
            list_content.join(" --> ")
        );
    }

    /// The content string looks like "xxx ---> yyy", or "empty list".
    pub fn get_list_content(&self) -> String {
        if self.head.is_none() {
            return "empty list".to_string();
        }

        let mut current_node: Option<&NonNull<Node<T>>> = self.head.as_ref();
        let mut list_content: Vec<String> = Vec::new();

        while current_node.is_some() {
            // First, we need to get back the `&T` from `Option<&NonNull<Node<T>>>`
            let ptr_to_node = current_node.unwrap().as_ptr();
            let node_ref = unsafe { &*ptr_to_node };
            let node_data = &node_ref.data;
            list_content.push(format!("{:?}", node_data));

            // Update `current_node` to next
            current_node = node_ref.next.as_ref();
        }

        format!(
            "{}{}",
            format!("({} elements): ", self.size),
            list_content.join(" --> ")
        )
    }
}

// #[cfg(test)]
// #[path = "single_linked_list_test.rs"]
// mod single_linked_list_test;
//
// #[cfg(test)]
// #[path = "single_linked_list_with_integer_test.rs"]
// mod single_linked_list_with_integer_test;
