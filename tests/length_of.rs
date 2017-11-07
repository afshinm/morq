#[macro_use]
extern crate morq;

#[cfg(test)]
mod length_of_tests {
    use morq::prelude::*;

    #[test]
    fn test_correct_length() {
        morq!(
            expect(vec![1, 2, 3].iter()).to.have.lengthOf(3usize);
            expect(0..3).to.have.lengthOf(3usize);
        );
    }

    #[test]
    fn test_incorrect_length() {
        morq!(
            expect(vec![1, 2, 3].iter()).to.not.have.lengthOf(1usize);
            expect(0..3).to.not.have.lengthOf(0usize);
        );
    }
}
