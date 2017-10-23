use core::AssertResult;
use core::Assert;
use std::fmt;

pub struct Equal<L> {
    expected: L
}

impl<L> Equal<L> {
    pub fn new(expected: L) -> Equal<L> {
        Equal {
            expected: expected 
        }
    }
}

impl<L: fmt::Debug> Assert<L> for Equal<L>
{
    fn compare<R: PartialEq<L> + fmt::Debug>(self, target: R) -> AssertResult 
    {
        if target == self.expected {
            Ok(())
        } else {
            Err(format!("Expected {:?}, received {:?}", self.expected, target))
        }
    }
}
