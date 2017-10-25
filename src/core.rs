/// Result of an assert
pub type AssertResult = Result<(), String>;

/// Assert trait
pub trait Assert<L> {
    fn compare(self, expected: L, target: L) -> AssertResult;
}

/// accepts AssertResult and panics or prints "."
pub fn evaluate(assert: AssertResult) {
    match assert {
        Ok(_) => print!("."),
        Err(msg) => panic!(msg),
    };
}
