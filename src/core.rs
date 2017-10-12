/// Result of an assert
pub type AssertResult = Result<(), String>;

/// Assert trait
pub trait Assert {
    fn compare<L: PartialEq<R>, R>(self, left: L, right: R) -> AssertResult;
}
