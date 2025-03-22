// tests/integration_test.rs

use reverse_string;

#[test]
fn test_reverse_normal_string() {
    let input = "Olá";
    let expected_output = "álO";
    assert_eq!(reverse_string::reverse(input), expected_output);
}

#[test]
fn test_reverse_empty_string() {
    let input = "";
    let expected_output = "";
    assert_eq!(reverse_string::reverse(input), expected_output);
}

#[test]
fn test_reverse_single_character() {
    let input = "A";
    let expected_output = "A";
    assert_eq!(reverse_string::reverse(input), expected_output);
}
