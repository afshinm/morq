use core::AssertResult;
use core::Assert;

pub struct Equal<T> {
    expected: T
}

impl Equal<T> {
    pub fn new(expected: T) -> Equal<T> {
        Equal {
            expected: expected 
        }
    }
}

impl<T> Assert for Equal<T> {
    fn compare<L: PartialEq<R>, R>(self, target: R) -> AssertResult {
        if self.expected.eq(&target) {
            Ok(())
        } else {
            Err(!format("Expected {}, received {}", self.expected, target))
        }
    }
}
