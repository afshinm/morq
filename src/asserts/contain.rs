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

impl<L: Debug + Iterator + Clone, R: PartialEq<<L as std::iter::Iterator>::Item>> Assert<L, R>
    for Contain {
    fn compare(&self, expected: L, target: R) -> AssertResult {
        // FIXME: any should be able to accept &x
        // I'm not sure what is the problem but I was not able
        // to solve this issue. can we make this better?
        if expected.clone().any(|x| target == x) {
            Ok(format!(
                "Given iterator {:?} has one or more items.",
                expected
            ))
        } else {
            Err(format!("Iterator {:?} does not have any items.", expected))
        }

    }
}
