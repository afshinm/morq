#[macro_use]
extern crate morq;

#[cfg(test)]
mod not_equal_tests {
    use morq::prelude::*;

    #[test]
    fn test_i16() {
        morq!(
            expect(3 * 5).to.not.be.equal(10);
        );
    }

    #[test]
    fn test_i32() {
        morq!(
            expect(50i32).to.not.be.equal(10);
        );
    }

    #[test]
    fn test_f32() {
        morq!(
            expect(5.0 * 0.5).to.not.be.equal(5.0);
        );
    }

    #[test]
    fn test_vec() {
        morq!(
            expect(vec![1, 3]).to.not.be.equal(vec![1, 2, 3]);
            expect(vec![5; 2]).to.not.be.equal(vec![1; 2]);
        );
    }

    #[test]
    fn test_str() {
        morq!(
            expect("afshin").to.not.be.equal("afshi2");
        );
    }
}
