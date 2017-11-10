#[macro_use]
extern crate morq;

#[cfg(test)]
mod bool_tests {
    use morq::prelude::*;

    #[test]
    fn test_true() {
        morq!(
            expect(true).to.be.true();
            expect(true).to.not.be.false();
        );
    }

    #[test]
    fn test_false() {
        morq!(
            expect(false).to.not.be.true();
            expect(false).to.be.false();
        );
    }
}
