use data_structure_implementation_by_rust::stack::browser_history_stack::BrowserHistoryStack;

fn create_test_stack() -> BrowserHistoryStack<String> {
    let mut browser_history_stack = BrowserHistoryStack::<String>::new();
    browser_history_stack.push("https://www.google.com".to_string());
    browser_history_stack.push("https://www.facebook.com".to_string());
    browser_history_stack.push("https://www.microsoft.com".to_string());
    browser_history_stack
}

#[test]
fn should_create_empty_browser_history_stack() {
    let mut browser_history_stack = BrowserHistoryStack::<String>::new();

    assert_eq!(browser_history_stack.size(), 0);
    assert_eq!(browser_history_stack.is_empty(), true);
    assert_eq!(browser_history_stack.pop(), None);
    assert_eq!(browser_history_stack.peek(), None);
    assert_eq!(
        browser_history_stack.contains("https://www.facebook.com".to_string()),
        false
    );
    browser_history_stack.print_stack();
    assert_eq!(browser_history_stack.get_stack_content(), "empty list");
}

#[test]
fn should_create_valid_browser_history_stack_with_the_correct_size() {
    let mut browser_history_stack = create_test_stack();

    assert_eq!(browser_history_stack.size(), 3);
    assert_eq!(
        browser_history_stack.peek(),
        Some(&"https://www.microsoft.com".to_string())
    );
    assert_eq!(
        browser_history_stack.pop(),
        Some("https://www.microsoft.com".to_string())
    );
    assert_eq!(browser_history_stack.size(), 2);
    browser_history_stack.print_stack();
    assert_eq!(
        browser_history_stack.contains("https://www.microsoft.com".to_string()),
        false
    );
    assert_eq!(
        browser_history_stack.contains("https://www.facebook.com".to_string()),
        true
    );
    assert_eq!(
        browser_history_stack.contains("https://www.google.com".to_string()),
        true
    );
    browser_history_stack.print_stack();
    assert_eq!(
        browser_history_stack.get_stack_content(),
        "(2 elements): \"https://www.google.com\" --> \"https://www.facebook.com\""
    );
}

#[test]
fn push_should_work_correctly() {
    let mut browser_history_stack = create_test_stack();

    browser_history_stack.push("https://www.github.com".to_string());
    assert_eq!(
        browser_history_stack.peek(),
        Some(&"https://www.github.com".to_string())
    );
    assert_eq!(
        browser_history_stack.get_stack_content(),
        "(4 elements): \"https://www.google.com\" --> \"https://www.facebook.com\" --> \"https://www.microsoft.com\" --> \"https://www.github.com\""
    );

    assert_eq!(browser_history_stack.pop(), Some("https://www.github.com".to_string()));
    assert_eq!(browser_history_stack.contains("https://www.github.com".to_string()), false);
    assert_eq!(browser_history_stack.contains("https://www.microsoft.com".to_string()), true);
    browser_history_stack.print_stack();
    assert_eq!(
        browser_history_stack.get_stack_content(),
        "(3 elements): \"https://www.google.com\" --> \"https://www.facebook.com\" --> \"https://www.microsoft.com\""
    );
}

#[test]
fn pop_should_work_correctly() {
    let mut browser_history_stack = create_test_stack();

    assert_eq!(
        browser_history_stack.pop(),
        Some("https://www.microsoft.com".to_string())
    );
    assert_eq!(browser_history_stack.contains("https://www.microsoft.com".to_string()), false);
    assert_eq!(browser_history_stack.contains("https://www.facebook.com".to_string()), true);
    assert_eq!(browser_history_stack.contains("https://www.google.com".to_string()), true);
    assert_eq!(
        browser_history_stack.get_stack_content(),
        "(2 elements): \"https://www.google.com\" --> \"https://www.facebook.com\""
    );

    assert_eq!(
        browser_history_stack.pop(),
        Some("https://www.facebook.com".to_string())
    );
    assert_eq!(browser_history_stack.contains("https://www.microsoft.com".to_string()), false);
    assert_eq!(browser_history_stack.contains("https://www.facebook.com".to_string()), false);
    assert_eq!(browser_history_stack.contains("https://www.google.com".to_string()), true);
    assert_eq!(browser_history_stack.is_empty(), false);
    assert_eq!(
        browser_history_stack.get_stack_content(),
        "(1 elements): \"https://www.google.com\""
    );

    assert_eq!(
        browser_history_stack.pop(),
        Some("https://www.google.com".to_string())
    );
    assert_eq!(browser_history_stack.contains("https://www.microsoft.com".to_string()), false);
    assert_eq!(browser_history_stack.contains("https://www.facebook.com".to_string()), false);
    assert_eq!(browser_history_stack.contains("https://www.google.com".to_string()), false);
    assert_eq!(browser_history_stack.is_empty(), true);
    assert_eq!(
        browser_history_stack.get_stack_content(),
        "empty list"
    );
}

