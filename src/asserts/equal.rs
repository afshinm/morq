use core::AssertResult;
use core::Assert;
use std::fmt::Debug;

pub struct Equal;

impl Equal {
    pub fn new() -> Equal {
        Equal {}
    }
}

impl<L: Debug + PartialEq<L>> Assert<L> for Equal {
    fn compare(&self, expected: L, target: L) -> AssertResult {
        if expected == target {
            Ok(format!("{:?} is equal to {:?}", expected, target))
        } else {
            Err(format!("Expected {:?}, received {:?}", expected, target))
        }
    }
}
