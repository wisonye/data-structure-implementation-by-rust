use data_structure_implementation_by_rust::linked_list::single_linked_list::SingleLinkedList;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: Option<u8>
}

fn create_test_list() -> SingleLinkedList<Person> {
    let mut person_list = SingleLinkedList::<Person>::new();
    person_list.append(Person { name: "David Chen".to_string(), age: Some(25) });
    person_list.append(Person { name: "Andy Liu".to_string(), age: Some(30) });
    person_list.append(Person { name: "Amy He".to_string(), age: Some(40) });
    person_list
}

#[test]
fn should_create_empty_person_list() {
    let mut person_list = SingleLinkedList::<isize>::new();

    assert_eq!(person_list.size(), 0);
    assert_eq!(person_list.get_head(), None);
    assert_eq!(person_list.get_tail(), None);
    assert_eq!(person_list.contains(8), false);
    assert_eq!(person_list.pop_head(), None);
    assert_eq!(person_list.pop_tail(), None);
    assert_eq!(person_list.get_head(), None);
    assert_eq!(person_list.get_tail(), None);
    person_list.print_list();
    assert_eq!(person_list.get_list_content(), "empty list");
}

#[test]
fn should_create_valid_person_list_with_the_correct_size() {
    let person_list = create_test_list();

    assert_eq!(person_list.size(), 3);
    assert_eq!(person_list.get_head(), Some(&Person { name: "David Chen".to_string(), age: Some(25) }));
    assert_eq!(person_list.get_tail(), Some(&Person { name: "Amy He".to_string(), age: Some(40) }));
    assert_eq!(person_list.contains(Person { name: "David Chen".to_string(), age: Some(25) }), true);
    assert_eq!(person_list.contains(Person { name: "Amy He".to_string(), age: Some(40) }), true);
    assert_eq!(person_list.contains(Person { name: "David Ye".to_string(), age: None }), false);
    assert_eq!(person_list.contains(Person { name: "David Liu".to_string(), age: Some(88) }), false);
    person_list.print_list();
    assert_eq!(
        person_list.get_list_content(),
        "(3 elements): Person { name: \"David Chen\", age: Some(25) } --> Person { name: \"Andy Liu\", age: Some(30) } --> Person { name: \"Amy He\", age: Some(40) }"
    );
}

#[test]
fn insert_at_head_and_append_should_work_correctly() {
    let mut person_list = create_test_list();

    person_list.insert_at_head(Person { name: "Mary".to_string(), age: None });
    // println!("get_head(): {:?}", person_list.get_head());
    // println!("get_tail(): {:?}", person_list.get_tail());
    assert_eq!(person_list.get_head(), Some(&Person { name: "Mary".to_string(), age: None }));
    assert_eq!(person_list.get_tail(), Some(&Person { name: "Amy He".to_string(), age: Some(40) }));
    assert_eq!(person_list.contains(Person { name: "Mary".to_string(), age: None }), true);
    person_list.print_list();
    assert_eq!(
        person_list.get_list_content(),
        "(4 elements): Person { name: \"Mary\", age: None } --> Person { name: \"David Chen\", age: Some(25) } --> Person { name: \"Andy Liu\", age: Some(30) } --> Person { name: \"Amy He\", age: Some(40) }"
    );

    person_list.insert_at_head(Person { name: "Willam".to_string(), age: Some(10) });
    // println!("get_head(): {:?}", person_list.get_head());
    // println!("get_tail(): {:?}", person_list.get_tail());
    assert_eq!(person_list.get_head(), Some(&Person { name: "Willam".to_string(), age: Some(10) }));
    assert_eq!(person_list.get_tail(), Some(&Person { name: "Amy He".to_string(), age: Some(40) }));
    assert_eq!(person_list.contains(Person { name: "Willam".to_string(), age: Some(10) }), true);
    person_list.print_list();
    assert_eq!(
        person_list.get_list_content(),
        "(5 elements): Person { name: \"Willam\", age: Some(10) } --> Person { name: \"Mary\", age: None } --> Person { name: \"David Chen\", age: Some(25) } --> Person { name: \"Andy Liu\", age: Some(30) } --> Person { name: \"Amy He\", age: Some(40) }"
    );
}

#[test]
fn pop_head_should_work_correctly() {
    let mut person_list = create_test_list();
   
    assert_eq!(person_list.pop_head(), Some(Person { name: "David Chen".to_string(), age: Some(25) }));
    assert_eq!(person_list.get_head(), Some(&Person { name: "Andy Liu".to_string(), age: Some(30) }));
    assert_eq!(person_list.get_tail(), Some(&Person { name: "Amy He".to_string(), age: Some(40) }));
    assert_eq!(person_list.contains(Person { name: "David Chen".to_string(), age: Some(25) }), false);
    assert_eq!(person_list.contains(Person { name: "Andy Liu".to_string(), age: Some(30) }), true);
    person_list.print_list();
    assert_eq!(person_list.get_list_content(), 
        "(2 elements): Person { name: \"Andy Liu\", age: Some(30) } --> Person { name: \"Amy He\", age: Some(40) }"
        );
    
    assert_eq!(person_list.pop_head(), Some(Person { name: "Andy Liu".to_string(), age: Some(30) }));
    assert_eq!(person_list.get_head(), Some(&Person { name: "Amy He".to_string(), age: Some(40) }));
    assert_eq!(person_list.get_tail(), Some(&Person { name: "Amy He".to_string(), age: Some(40) }));
    assert_eq!(person_list.contains(Person { name: "Andy Liu".to_string(), age: Some(30) }), false);
    person_list.print_list();
    assert_eq!(person_list.get_list_content(), 
        "(1 elements): Person { name: \"Amy He\", age: Some(40) }"
        );

    assert_eq!(person_list.pop_head(), Some(Person { name: "Amy He".to_string(), age: Some(40) }));
    assert_eq!(person_list.get_head(), None);
    assert_eq!(person_list.get_tail(), None);
    person_list.print_list();
    assert_eq!(person_list.get_list_content(), "empty list");
}
