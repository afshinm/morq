#[macro_use]
extern crate morq;

#[cfg(test)]
mod empty_tests {
    use morq::prelude::*;

    #[test]
    fn test_range() {
        morq!(
            expect(0..2).to.not.be.empty();
            expect(0..0).to.be.empty();
        );
    }

    #[test]
    fn test_vec() {
        let empty_vec: Vec<i32> = Vec::new();

        morq!(
            expect(vec![1, 2, 3].iter()).to.not.be.empty();
            expect(empty_vec.iter()).to.be.empty();
        );
    }
}
