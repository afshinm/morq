use core::AssertResult;
use core::Assert;
use std::fmt::Debug;
use std::any::TypeId;

pub struct TypeMatch;

impl TypeMatch {
    pub fn new() -> TypeMatch {
        TypeMatch {}
    }
}

impl<L: Debug + 'static, R: Debug + PartialEq<TypeId>> Assert<L, R> for TypeMatch {
    fn compare(&self, expected: L, target: R) -> AssertResult {
        let expected_type_id = TypeId::of::<L>();

        if target == expected_type_id {
            Ok(format!(
                "{:?} has the same data type {:?}",
                expected,
                target
            ))
        } else {
            Err(format!(
                "{:?} has a different data type. Expected: {:?} - Received: {:?}",
                expected,
                target,
                expected_type_id
            ))
        }

    }
}
