use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::ptr::NonNull;

/// The type here describes either a **valid pointer** or a **null pointer**.
/// That's why we use `Option` here which only got 2 possible values:
///
/// - `Some()` - means **valid pointer**
/// - `None` - means **null pointer**
///
/// The **valid pointer** means:
///
/// - Non null, it must point to the particular `Node<T>` instance.
/// - `Node<T>` instance should live inside the **heap**.
///
/// That's why we use `Box<Node<T>>` which is a smart pointer in `Rust`.
/// Below is the steps to get back a `NonNull<Node<T>>` from `Box<Node<T>>`:
///
/// ```ignore
/// // 1. Create new box node instance
/// let box_node = Box::new(Node {});
///
/// // 2. Get back a `&mut Node<T>` from box node instance
/// let box_node_mut_ref: &mut Node<T> = Box::leak(box_node);
///
/// // 3. Get back `NonNull<Node<T>>` from `&mut Node<T>` (mutable reference just pointer)
/// let non_null_node = NonNull::from(box_node_mut_ref);
///
/// // 4. Finally, get back `Option<NonNull<Node<T>>>`
/// let head = Some(non_null_node);
///
/// // Or make all of those into a single step:
/// let head = Some(NonNull::from(Box::leak(Box::new(Node {}))));
/// ```
///
/// </br>
///
/// But how to get back `Option<&Node<T>>` from `Option<NonNull<Node<T>>>`???
///
/// `NonNull<Node<T>>.as_ptr()` gets back a raw pointer to `Node<T>`, we got 2 options to make that
/// happen:
///
/// - Option A:
///
///     ```ignore
///     // For example, the `next_node` is the instance of `NextNode<T>`
///     let mut next_option_ref: Option<&Node<T>> = match next_node {
///         Some(temp_next) => {
///             let ptr: *mut Node<T> = temp_next.as_ptr();
///             unsafe { Some(&*ptr) }
///         }
///         None => None,
///     };
///
/// - Option B:
///
///     ```ignore
///     // For example, the `next_node` is the instance of `NextNode<T>`
///     let mut next_option_ref: Option<&Node<T>> = None;
///     if next_node.is_some() {
///         let unwrap_non_null = self.next.as_ref().unwrap();
///         let raw_ptr: *mut Node<T> = unwrap_non_null.as_ptr();
///         unsafe {
///             next_option_ref = Some(&*raw_ptr);
///         }
///     }
///     ```
///
/// By the way, we **CANNOT** use `Some(Box::from_raw(temp_head.as_ptr()))` to get back a
/// `Option<Box<Node<T>>>`, as that will consume and drop the node instance when it out of scope
/// which will cause `pointer being freed was not allocated`!!!
///
pub type NextNode<T> = Option<NonNull<Node<T>>>;

/// The node to hold data (in the head) and the pointer to [`next node`](NextNode)
///
/// **All fields in this struct only visible to the current crate!!!**
pub struct Node<T: Debug + PartialEq> {
    pub(crate) data: T,
    pub(crate) next: NextNode<T>,
}

/// Only output the data value
impl<T: Debug + PartialEq> Debug for Node<T> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self.next {
            Some(temp_next) => {
                // Get back raw pointer to `Node<T>` from `NonNull<Node<T>>`
                let ptr: *mut Node<T> = temp_next.as_ptr();
                let node_ref = unsafe { &*ptr };
                f.write_fmt(format_args!("{:?} --> {:?}", self.data, node_ref))
            }
            None => f.write_fmt(format_args!("{:?}", self.data)),
        }
    }
}
