use core::AssertResult;
use core::Assert;
use std::fmt::{Debug, Display};
use num::Unsigned;
use num::ToPrimitive;

pub struct LengthOf;

impl LengthOf {
    pub fn new() -> LengthOf {
        LengthOf {}
    }
}

impl<L: Debug + Iterator + Clone, R: Unsigned + ToPrimitive + Display> Assert<L, R> for LengthOf {
    fn compare(&self, expected: L, target: R) -> AssertResult {
        let expected_count = expected.clone().count();

        if expected_count == target.to_usize().unwrap() {
            Ok(format!(
                "Given iterator {:?} has {} items.",
                expected,
                target
            ))
        } else {
            Err(format!(
                "Iterator {:?} has {} items. Expected: {}",
                expected,
                expected_count,
                target
            ))
        }

    }
}
