use core::AssertResult;
use core::Assert;
use std::fmt::Debug;

pub struct NotEqual;

impl NotEqual {
    pub fn new() -> NotEqual {
        NotEqual { }
    }
}

impl<L: Debug + PartialEq<L>> Assert<L> for NotEqual
{
    fn compare(self, expected: L, target: L) -> AssertResult 
    {
        if expected == target {
            Err(format!("{:?} should not be equal to {:?}", expected, target))
        } else {
            Ok(())
        }
    }
}
