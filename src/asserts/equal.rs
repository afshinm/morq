use core::AssertResult;
use core::Assert;

pub struct Equal;

impl Assert for Equal {
    fn compare<L: PartialEq<R>, R>(left: L, right: R) -> AssertResult {
        if left.eq(&right) {
            Ok(())
        } else {
            Err("Didn't match.".to_string())
        }
    }
}
