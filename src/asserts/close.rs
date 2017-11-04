use core::AssertResult;
use core::Assert;
use std::fmt::Debug;
use num::Float;

pub struct Close;

impl Close {
    pub fn new() -> Close {
        Close {}
    }
}

impl<L: Debug + Float> Assert<L, L> for Close {
    fn compare(&self, expected: L, target: L) -> AssertResult {
        let eps = Float::epsilon();

        if (target - expected) < eps {
            Ok(format!("{:?} is close to {:?}", target, expected))
        } else {
            Err(format!(
                "left: `{:?}`, right: `{:?}`, expect diff: `{:?}`, real diff: `{:?}`",
                target,
                expected,
                eps,
                (target - expected).abs()
            ))
        }
    }
}
