use data_structure_implementation_by_rust::linked_list::single_linked_list::SingleLinkedList;

fn create_test_list() -> SingleLinkedList<isize> {
    let mut integer_list = SingleLinkedList::<isize>::new();
    integer_list.append(1);
    integer_list.append(2);
    integer_list.append(3);
    integer_list.append(4);
    integer_list
}

#[test]
fn integer_list_test_should_create_empty_integer_list() {
    let mut integer_list = SingleLinkedList::<isize>::new();

    assert_eq!(integer_list.size(), 0);
    assert_eq!(integer_list.get_head(), None);
    assert_eq!(integer_list.get_tail(), None);
    assert_eq!(integer_list.contain(8), false);
    assert_eq!(integer_list.pop_head(), None);
    assert_eq!(integer_list.pop_tail(), None);
    assert_eq!(integer_list.get_head(), None);
    assert_eq!(integer_list.get_tail(), None);
    integer_list.print_list();
    assert_eq!(integer_list.get_list_content(), "empty list");
}

#[test]
fn integer_list_test_should_create_valid_integer_list_with_the_correct_size() {
    let integer_list = create_test_list();

    assert_eq!(integer_list.size(), 4);
    assert_eq!(integer_list.get_head(), Some(&1));
    assert_eq!(integer_list.get_tail(), Some(&4));
    assert_eq!(integer_list.contain(0), false);
    assert_eq!(integer_list.contain(1), true);
    assert_eq!(integer_list.contain(2), true);
    assert_eq!(integer_list.contain(3), true);
    assert_eq!(integer_list.contain(4), true);
    integer_list.print_list();
    assert_eq!(
        integer_list.get_list_content(),
        "(4 elements): 1 --> 2 --> 3 --> 4"
    );
}

#[test]
fn integer_list_test_insert_at_head_and_append_should_work_correctly() {
    let mut integer_list = create_test_list();

    integer_list.insert_at_head(0);
    // println!("get_head(): {:?}", integer_list.get_head());
    // println!("get_tail(): {:?}", integer_list.get_tail());
    assert_eq!(integer_list.get_head(), Some(&0));
    assert_eq!(integer_list.get_tail(), Some(&4));
    assert_eq!(integer_list.contain(0), true);
    integer_list.print_list();
    assert_eq!(
        integer_list.get_list_content(),
        "(5 elements): 0 --> 1 --> 2 --> 3 --> 4"
    );

    integer_list.insert_at_head(-1);
    // println!("get_head(): {:?}", integer_list.get_head());
    // println!("get_tail(): {:?}", integer_list.get_tail());
    assert_eq!(integer_list.get_head(), Some(&-1));
    assert_eq!(integer_list.get_tail(), Some(&4));
    assert_eq!(integer_list.contain(-1), true);
    assert_eq!(integer_list.contain(0), true);
    integer_list.print_list();
    assert_eq!(
        integer_list.get_list_content(),
        "(6 elements): -1 --> 0 --> 1 --> 2 --> 3 --> 4"
    );
}

#[test]
fn integer_list_test_pop_head_should_work_correctly() {
    let mut integer_list = create_test_list();
    //
    assert_eq!(integer_list.pop_head(), Some(1));
    assert_eq!(integer_list.get_head(), Some(&2));
    assert_eq!(integer_list.get_tail(), Some(&4));
    assert_eq!(integer_list.contain(1), false);
    assert_eq!(integer_list.contain(2), true);
    assert_eq!(integer_list.contain(3), true);
    assert_eq!(integer_list.contain(4), true);
    integer_list.print_list();
    assert_eq!(
        integer_list.get_list_content(),
        "(3 elements): 2 --> 3 --> 4"
    );
   
    assert_eq!(integer_list.pop_head(), Some(2));
    assert_eq!(integer_list.get_head(), Some(&3));
    assert_eq!(integer_list.get_tail(), Some(&4));
    assert_eq!(integer_list.contain(1), false);
    assert_eq!(integer_list.contain(2), false);
    assert_eq!(integer_list.contain(3), true);
    assert_eq!(integer_list.contain(4), true);
    integer_list.print_list();
    assert_eq!(integer_list.get_list_content(), "(2 elements): 3 --> 4");
    
    assert_eq!(integer_list.pop_head(), Some(3));
    assert_eq!(integer_list.get_head(), Some(&4));
    assert_eq!(integer_list.get_tail(), Some(&4));
    assert_eq!(integer_list.contain(1), false);
    assert_eq!(integer_list.contain(2), false);
    assert_eq!(integer_list.contain(3), false);
    assert_eq!(integer_list.contain(4), true);
    integer_list.print_list();
    assert_eq!(integer_list.get_list_content(), "(1 elements): 4");

    assert_eq!(integer_list.pop_head(), Some(4));
    assert_eq!(integer_list.get_head(), None);
    assert_eq!(integer_list.get_tail(), None);
    assert_eq!(integer_list.contain(1), false);
    assert_eq!(integer_list.contain(2), false);
    assert_eq!(integer_list.contain(3), false);
    assert_eq!(integer_list.contain(4), false);
    integer_list.print_list();
    assert_eq!(integer_list.get_list_content(), "empty list");
}

#[test]
fn integer_list_test_pop_tail_should_work_correctly() {
    let mut integer_list = create_test_list();

    assert_eq!(integer_list.pop_tail(), Some(4));
    assert_eq!(integer_list.get_head(), Some(&1));
    assert_eq!(integer_list.get_tail(), Some(&3));
    assert_eq!(integer_list.contain(1), true);
    assert_eq!(integer_list.contain(2), true);
    assert_eq!(integer_list.contain(3), true);
    assert_eq!(integer_list.contain(4), false);
    integer_list.print_list();
    assert_eq!(
        integer_list.get_list_content(),
        "(3 elements): 1 --> 2 --> 3"
    );

    assert_eq!(integer_list.pop_tail(), Some(3));
    assert_eq!(integer_list.get_head(), Some(&1));
    assert_eq!(integer_list.get_tail(), Some(&2));
    assert_eq!(integer_list.contain(1), true);
    assert_eq!(integer_list.contain(2), true);
    assert_eq!(integer_list.contain(3), false);
    assert_eq!(integer_list.contain(4), false);
    integer_list.print_list();
    assert_eq!(integer_list.get_list_content(), "(2 elements): 1 --> 2");

    assert_eq!(integer_list.pop_tail(), Some(2));
    assert_eq!(integer_list.get_head(), Some(&1));
    assert_eq!(integer_list.get_tail(), Some(&1));
    assert_eq!(integer_list.contain(1), true);
    assert_eq!(integer_list.contain(2), false);
    assert_eq!(integer_list.contain(3), false);
    assert_eq!(integer_list.contain(4), false);
    integer_list.print_list();
    assert_eq!(integer_list.get_list_content(), "(1 elements): 1");

    assert_eq!(integer_list.pop_tail(), Some(1));
    assert_eq!(integer_list.get_head(), None);
    assert_eq!(integer_list.get_tail(), None);
    assert_eq!(integer_list.contain(1), false);
    assert_eq!(integer_list.contain(2), false);
    assert_eq!(integer_list.contain(3), false);
    assert_eq!(integer_list.contain(4), false);
    integer_list.print_list();
    assert_eq!(integer_list.get_list_content(), "empty list");
}
