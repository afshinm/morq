use core::AssertResult;
use core::Assert;
use std::fmt::Debug;

pub struct Empty;

impl Empty {
    pub fn new() -> Empty {
        Empty {}
    }
}

impl<L: Debug + Iterator + Clone, R> Assert<L, R> for Empty {
    fn compare(&self, expected: L, _target: R) -> AssertResult {
        match expected.clone().next() {
            None => Ok(format!(
                "Given iterator {:?} has one or more items.",
                expected
            )),
            Some(_) => Err(format!("Iterator {:?} does not have any items.", expected)),
        }
    }
}
