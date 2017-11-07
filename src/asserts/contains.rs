use core::AssertResult;
use core::Assert;
use std::fmt::Debug;

pub struct Contains;

impl Contains {
    pub fn new() -> Contains {
        Contains {}
    }
}

impl<L: Debug + Iterator + Clone, R> Assert<L, R> for Contains {
    fn compare(&self, expected: L, target: R) -> AssertResult {
        match expected.clone().any(|&x| x == target) {
            None => Ok(format!(
                "Given iterator {:?} has one or more items.",
                expected
            )),
            Some(_) => Err(format!("Iterator {:?} does not have any items.", expected)),
        }
    }
}
