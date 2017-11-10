#[macro_use]
extern crate morq;

#[cfg(test)]
mod contain_tests {
    use morq::prelude::*;

    #[test]
    fn test_i32() {
        morq!(
            expect(vec![1, 2, 3].iter()).to.contain(&2);
        );
    }

    #[test]
    fn test_f32() {
        morq!(
            expect(vec![1f32, 2f32, 3f32].iter()).to.contain(&2f32);
        );
    }

    #[test]
    fn test_char() {
        morq!(
            expect(vec!['a', 'b', 'c'].iter()).to.contain(&'c');
        );
    }

    #[test]
    fn test_bool() {
        morq!(
            expect(vec![false, false].iter()).to.not.contain(&true);
            expect(vec![true, false].iter()).to.contain(&true);
        );
    }
}
