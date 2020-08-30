mod single_linked_list;

#[cfg(test)]
mod tests {
    use crate::single_linked_list::SingeLinkedList;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn create_empty_list() {
        let empty_list = SingeLinkedList::new("Wison Ye");

        assert_eq!(empty_list.get_size(), 1);
        assert_eq!(empty_list.get_head().get_data(), &"Wison Ye");
        assert_eq!(empty_list.get_head().get_next().is_none(), true);
    }
}
