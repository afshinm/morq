use core::AssertResult;
use core::Assert;
use std::fmt;

pub struct NotEqual<L> {
    expected: L,
}

impl<L> NotEqual<L> {
    pub fn new(expected: L) -> NotEqual<L> {
        NotEqual { expected }
    }
}

impl<L: fmt::Debug> Assert<L> for NotEqual<L>
{
    fn compare<R: PartialEq<L> + fmt::Debug>(self, target: R) -> AssertResult 
    {
        if target == self.expected {
            Err(format!("{:?} should not be equal to {:?}", self.expected, target))
        } else {
            Ok(())
        }
    }
}
