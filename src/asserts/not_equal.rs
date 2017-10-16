use core::AssertResult;
use core::Assert;

pub struct NotEqual;

impl NotEqual {
    pub fn new() -> NotEqual {
        NotEqual {}
    }
}

impl Assert for NotEqual {
    fn compare<L: PartialEq<R>, R>(self, left: L, right: R) -> AssertResult {
        if left.eq(&right) {
            Err(!format("{} equals to {}", left, right))
        } else {
            Ok(())
        }
    }
}
