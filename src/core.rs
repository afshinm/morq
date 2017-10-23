use std::fmt;

/// Result of an assert
pub type AssertResult = Result<(), String>;

/// Assert trait
pub trait Assert<L: fmt::Debug> {
    fn compare<R: PartialEq<L> + fmt::Debug>(self, target: R) -> AssertResult;
}

/// accepts AssertResult and panics or prints "."
pub fn evaluate(assert: AssertResult) {
    match assert {
        Ok(_) => print!("."),
        Err(msg) => panic!(msg),
    };
}
