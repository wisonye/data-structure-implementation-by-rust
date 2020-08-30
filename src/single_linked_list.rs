#![allow(warnings)]
//! # `SingeLinkedList`
//!
//! It looks like this:
//!
//! Head --> Node1 --> Node2 --> Tail
//!

use std::fmt;
use std::ptr::NonNull;

/// The type here describes either a **valid pointer** or a **null pointer**.
/// That's why we use `Option` here which only got 2 possible values:
///
/// `Some()` - means **valid pointer**
/// `None` - means **null pointer**
///
/// The **valid pointer* means:
///
/// 1. Non null, it must point to particular `Node<T>` instance.
/// 2. `Node<T>` instance should live inside the **heap**.
///
/// That's why we use `Box<Node<T>>` which is a smart pointer in `Rust`.
/// Below is the steps to get back a `NonNull<Node<T>>` from `Box<Node<T>>`:
///
/// ```
/// // 1. Create new box node instance
/// // let box_node = Box::new(Node {});
///
/// // 2. Get back a `&mut Node<T>` from box node instance
/// // let box_node_mut_ref: &mut Node<T> = Box::leak(box_node);
///
/// // 3. Get back `NonNull<Node<T>>` from `&mut Node<T>` (mutable reference just pointer)
/// // let non_null_node = NonNull::from(box_node_mut_ref);
///
/// // 4. Finally, get back `Option<NonNull<Node<T>>>`
/// // let head = Some(non_null_node);
///
/// // 5. All in one step:
/// // let head = Some(NonNull::from(Box::leak(Box::new(Node {}))));
/// ```
///
/// But how to get back `Option<&Node<T>>` from `Option<NonNull<Node<T>>>`???
///
/// `NonNull<Node<T>>.as_ptr()` get back a raw pointer to `Node<T>`, then We got 2 ways to do that:
///
/// ```
/// // let mut next_option_ref: Option<&Node<T>> = match self.next {
/// //     Some(temp_next) => {
/// //         let ptr: *mut Node<T> = temp_next.as_ptr();
/// //         unsafe { Some(&*ptr) }
/// //     }
/// //     None => None,
/// // };
///
/// // let mut next_option_ref: Option<&Node<T>> = None;
/// // if self.next.is_some() {
/// //     let unwrap_non_null = self.next.as_ref().unwrap();
/// //     let raw_ptr: *mut Node<T> = unwrap_non_null.as_ptr();
/// //     unsafe {
/// //         next_option_ref = Some(&*raw_ptr);
/// //     }
/// // }
/// ```
///
/// By the way, we **CANNOT** use `Some(Box::from_raw(temp_head.as_ptr()))` to get back a
/// `Option<Box<Node<T>>>`, as that will consume and drop the node instance when it out of scope
/// which will cause `pointer being freed was not allocated`!!!
///
type PointToNextNode<T> = Option<NonNull<Node<T>>>;

// ------------------------- Node<T> ---------------------------------

///
struct Node<T: Clone + fmt::Debug> {
    data: T,
    next: PointToNextNode<T>,
}

impl<T: Clone + fmt::Debug> fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut next_option_ref: Option<&Node<T>> = match self.next {
            Some(temp_next) => {
                // Get back raw pointer to `Node<T>` from `NonNull<Node<T>>`
                let ptr: *mut Node<T> = temp_next.as_ptr();
                unsafe { Some(&*ptr) }
            }
            None => None,
        };

        f.debug_struct("Node")
            .field("data", &self.data)
            .field("next", &next_option_ref)
            .finish()
    }
}

impl<T: Clone + fmt::Debug> Node<T> {
    pub fn get_data(&self) -> &T {
        &self.data
    }
}

// ------------------------- SingeLinkedList<T> ----------------------

///
pub struct SingeLinkedList<T: Clone + fmt::Debug> {
    size: usize,
    head: PointToNextNode<T>,
    tail: PointToNextNode<T>,
}

impl<T: Clone + fmt::Debug> fmt::Debug for SingeLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut head_option_ref: Option<&Node<T>> = match self.head {
            Some(temp_head) => {
                // Get back raw pointer to `Node<T>` from `NonNull<Node<T>>`
                let ptr: *mut Node<T> = temp_head.as_ptr();
                unsafe { Some(&*ptr) }
            }
            None => None,
        };

        let mut tail_option_ref: Option<&Node<T>> = match self.tail {
            Some(temp_tail) => {
                // Get back raw pointer to `Node<T>` from `NonNull<Node<T>>`
                let ptr: *mut Node<T> = temp_tail.as_ptr();
                unsafe { Some(&*ptr) }
            }
            None => None,
        };

        f.debug_struct("SingeLinkedList")
            .field("size", &self.size)
            .field("head", &head_option_ref)
            .field("tail", &tail_option_ref)
            .finish()
    }
}

impl<T: Clone + fmt::Debug> SingeLinkedList<T> {
    pub fn new() -> Self {
        SingeLinkedList {
            size: 0,
            head: None,
            tail: None,
        }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_empty_list() {
        let empty_list = SingeLinkedList::<String>::new();
        println!("empty_list: {:#?}", &empty_list);

        assert_eq!(empty_list.get_size(), 0);
        assert_eq!(empty_list.head.is_none(), true);
        assert_eq!(empty_list.tail.is_none(), true);
    }

    #[test]
    fn push_to_end_should_work() {
        let mut test_list = SingeLinkedList::new();
        test_list.push_to_end("Rust".to_owned());
        test_list.push_to_end("is".to_owned());
        test_list.push_to_end("Awesome".to_owned());

        println!("test_list: {:#?}", &test_list);
        assert_eq!(test_list.get_size(), 3);
    }
}
