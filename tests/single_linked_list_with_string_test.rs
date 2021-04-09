use data_structure_implementation_by_rust::linked_list::single_linked_list::SingleLinkedList;

fn create_test_list() -> SingleLinkedList<String> {
    let mut string_list = SingleLinkedList::<String>::new();
    string_list.append("Rust".to_string());
    string_list.append("is".to_string());
    string_list.append("Awesome!".to_string());
    string_list
}

#[test]
fn string_list_test_should_create_empty_string_list() {
    let mut string_list = SingleLinkedList::<String>::new();

    assert_eq!(string_list.size(), 0);
    assert_eq!(string_list.get_head(), None);
    assert_eq!(string_list.get_tail(), None);
    assert_eq!(string_list.contain("Hello".to_string()), false);
    assert_eq!(string_list.pop_head(), None);
    assert_eq!(string_list.pop_tail(), None);
    assert_eq!(string_list.get_head(), None);
    assert_eq!(string_list.get_tail(), None);
    string_list.print_list();
    assert_eq!(string_list.get_list_content(), "empty list");
}

#[test]
fn string_list_test_should_create_valid_string_list_with_the_correct_size() {
    let string_list = create_test_list();

    assert_eq!(string_list.size(), 3);
    assert_eq!(string_list.get_head(), Some(&"Rust".to_string()));
    assert_eq!(string_list.get_tail(), Some(&"Awesome!".to_string()));
    assert_eq!(string_list.contain("Hello".to_string()), false);
    assert_eq!(string_list.contain("Rust".to_string()), true);
    assert_eq!(string_list.contain("is".to_string()), true);
    assert_eq!(string_list.contain("Awesome!".to_string()), true);
    string_list.print_list();
    assert_eq!(
        string_list.get_list_content(),
        "(3 elements): \"Rust\" --> \"is\" --> \"Awesome!\""
    );
}

#[test]
fn string_list_test_insert_at_head_and_append_should_work_correctly() {
    let mut string_list = create_test_list();

    string_list.insert_at_head("that: ".to_string());
    // println!("get_head(): {:?}", string_list.get_head());
    // println!("get_tail(): {:?}", string_list.get_tail());
    assert_eq!(string_list.get_head(), Some(&"that: ".to_string()));
    assert_eq!(string_list.get_tail(), Some(&"Awesome!".to_string()));
    assert_eq!(string_list.contain("that: ".to_string()), true);
    string_list.print_list();
    assert_eq!(
        string_list.get_list_content(),
        "(4 elements): \"that: \" --> \"Rust\" --> \"is\" --> \"Awesome!\""
    );

    string_list.insert_at_head("I can tell you".to_string());
    // println!("get_head(): {:?}", string_list.get_head());
    // println!("get_tail(): {:?}", string_list.get_tail());
    assert_eq!(string_list.get_head(), Some(&"I can tell you".to_string()));
    assert_eq!(string_list.get_tail(), Some(&"Awesome!".to_string()));
    assert_eq!(string_list.contain("I can tell you".to_string()), true);
    string_list.print_list();
    assert_eq!(
        string_list.get_list_content(),
        "(5 elements): \"I can tell you\" --> \"that: \" --> \"Rust\" --> \"is\" --> \"Awesome!\""
    );
}

#[test]
fn string_list_test_pop_head_should_work_correctly() {
    let mut string_list = create_test_list();

    assert_eq!(string_list.pop_head(), Some("Rust".to_string()));
    assert_eq!(string_list.get_head(), Some(&"is".to_string()));
    assert_eq!(string_list.get_tail(), Some(&"Awesome!".to_string()));
    assert_eq!(string_list.contain("Rust".to_string()), false);
    assert_eq!(string_list.contain("is".to_string()), true);
    assert_eq!(string_list.contain("Awesome!".to_string()), true);
    string_list.print_list();
    assert_eq!(
        string_list.get_list_content(),
        "(2 elements): \"is\" --> \"Awesome!\""
    );

    assert_eq!(string_list.pop_head(), Some("is".to_string()));
    assert_eq!(string_list.get_head(), Some(&"Awesome!".to_string()));
    assert_eq!(string_list.get_tail(), Some(&"Awesome!".to_string()));
    assert_eq!(string_list.contain("Rust".to_string()), false);
    assert_eq!(string_list.contain("is".to_string()), false);
    assert_eq!(string_list.contain("Awesome!".to_string()), true);
    string_list.print_list();
    assert_eq!(string_list.get_list_content(), "(1 elements): \"Awesome!\"");

    assert_eq!(string_list.pop_head(), Some("Awesome!".to_string()));
    assert_eq!(string_list.get_head(), None);
    assert_eq!(string_list.get_tail(), None);
    assert_eq!(string_list.contain("Rust".to_string()), false);
    assert_eq!(string_list.contain("is".to_string()), false);
    assert_eq!(string_list.contain("Awesome!".to_string()), false);
    string_list.print_list();
    assert_eq!(string_list.get_list_content(), "empty list");
}

#[test]
fn string_list_test_pop_tail_should_work_correctly() {
    let mut string_list = create_test_list();

    assert_eq!(string_list.pop_tail(), Some("Awesome!".to_string()));
    assert_eq!(string_list.get_head(), Some(&"Rust".to_string()));
    assert_eq!(string_list.get_tail(), Some(&"is".to_string()));
    assert_eq!(string_list.contain("Rust".to_string()), true);
    assert_eq!(string_list.contain("is".to_string()), true);
    assert_eq!(string_list.contain("Awesome!".to_string()), false);
    string_list.print_list();
    assert_eq!(
        string_list.get_list_content(),
        "(2 elements): \"Rust\" --> \"is\""
    );

    assert_eq!(string_list.pop_tail(), Some("is".to_string()));
    assert_eq!(string_list.get_head(), Some(&"Rust".to_string()));
    assert_eq!(string_list.get_tail(), Some(&"Rust".to_string()));
    assert_eq!(string_list.contain("Rust".to_string()), true);
    assert_eq!(string_list.contain("is".to_string()), false);
    assert_eq!(string_list.contain("Awesome!".to_string()), false);
    string_list.print_list();
    assert_eq!(string_list.get_list_content(), "(1 elements): \"Rust\"");

    assert_eq!(string_list.pop_tail(), Some("Rust".to_string()));
    assert_eq!(string_list.get_head(), None);
    assert_eq!(string_list.get_tail(), None);
    assert_eq!(string_list.contain("Rust".to_string()), false);
    assert_eq!(string_list.contain("is".to_string()), false);
    assert_eq!(string_list.contain("Awesome!".to_string()), false);
    string_list.print_list();
    assert_eq!(string_list.get_list_content(), "empty list");
}
