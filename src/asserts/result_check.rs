use core::AssertResult;
use core::Assert;
use std::fmt::Debug;

pub struct ResultCheck;

impl ResultCheck {
    pub fn new() -> ResultCheck {
        ResultCheck {}
    }
}

impl<L: Iterator + Clone + Debug, R> Assert<L, R> for ResultCheck {
    fn compare(&self, expected: L, _target: R) -> AssertResult {
        if expected.clone().count() > 0 {
            Ok(format!("Given expression returned Ok: {:?}", expected))
        } else {
            Err(format!("Given expression returned Err: {:?}", expected))
        }
    }
}
