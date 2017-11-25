use core::AssertResult;
use core::Assert;
use std::fmt::Debug;
use std;

pub struct Contain;

impl Contain {
    pub fn new() -> Contain {
        Contain {}
    }
}

impl<
    L: Debug + IntoIterator + Clone,
    R: PartialEq<<L as std::iter::IntoIterator>::Item>,
> Assert<L, R> for Contain {
    fn compare(&self, expected: L, target: R) -> AssertResult {
        if expected.clone().into_iter().any(|x| target == x) {
            Ok(format!(
                "Given iterator {:?} has one or more items.",
                expected
            ))
        } else {
            Err(format!("Iterator {:?} does not have any items.", expected))
        }

    }
}
