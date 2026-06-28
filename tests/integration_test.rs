use rust_lib_template::process;

#[test]
fn process_via_library() {
    let result = process("hello").unwrap();
    assert!(result.contains("HELLO"));
}

#[test]
fn process_empty_via_library() {
    let result = process("").unwrap();
    assert!(result.contains("(empty)"));
}
