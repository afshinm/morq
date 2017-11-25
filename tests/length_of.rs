#[macro_use]
extern crate morq;

#[cfg(test)]
mod length_of_tests {
    use morq::prelude::*;

    #[test]
    fn test_correct_length() {
        morq!(
            expect(vec![1, 2, 3]).to.have.length_of(3usize);
            expect(0..3).to.have.length_of(3usize);
        );
    }

    #[test]
    fn test_incorrect_length() {
        morq!(
            expect(vec![1, 2, 3]).to.not.have.length_of(1usize);
            expect(0..3).to.not.have.length_of(0usize);
        );
    }
}
