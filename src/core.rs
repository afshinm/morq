/// Result of an assert
pub type AssertResult = Result<String, String>;

/// Assert trait
pub trait Assert<L> {
    fn compare(&self, expected: L, target: L) -> AssertResult;
}

/// accepts AssertResult and panics or prints "."
pub fn evaluate<L>(assert: &Assert<L>, expected: L, target: L, is_not: bool) {
    let assert_result = assert.compare(expected, target);

    // sorry about this negative condition :(
    match (assert_result, is_not) {
        (Ok(_), false) => print!("."),
        (Err(msg), false) => panic!(msg),
        (Err(_), true) => print!("."),
        (Ok(msg), true) => panic!(msg),
    };
}
