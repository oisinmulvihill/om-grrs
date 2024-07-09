pub fn find_matches(line: &str, pattern: &str) -> bool {
    let found = line.contains(pattern);
    if found {
        return true;
    }
    false
}

#[test]
fn test_find_matches_use_cases() {
    // Configure logging in the test
    env_logger::init();

    assert_eq!(find_matches("hello there, how are you?", "hello"), true);
    assert_eq!(
        find_matches("hello there, how are you?", "how are you"),
        true
    );
    assert_eq!(find_matches("hello there, how are you?", "tony"), false);
    assert_eq!(find_matches("hello there, how are you?", "I'm fine"), false);
}
