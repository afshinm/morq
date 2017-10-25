#[macro_use]
extern crate morq;

#[cfg(test)]
mod equal_tests {
    use morq::prelude::*;

    #[test]
    fn test_i16() {
        morq!(
            expect(3 * 5).to.be.equal(15i16);
            expect(50i16).to.be.equal(5 * 10);
            expect(50i16).to.be.equal(5 * 10i16);
        );
    }

    #[test]
    fn test_i32() {
        morq!(
            expect(3 * 5).to.be.equal(15i32);
            expect(50i32).to.be.equal(5 * 10);
        );
    }

    #[test]
    fn test_f32() {
        morq!(
            expect(5.10).to.be.equal(5.10);
            expect(5.0 + 0.1).to.be.equal(5.10);
            expect(5.0 * 0.1).to.be.equal(0.50);
        );
    }

    #[test]
    fn test_vec() {
        morq!(
            expect(vec![1, 2, 3]).to.be.equal(vec![1, 2, 3]);
            expect(vec![1; 2]).to.be.equal(vec![1; 2]);
        );
    }

    #[test]
    fn test_str() {
        morq!(
            expect("hola").to.be.equal("hola");
            expect("hola".to_owned()).to.be.equal("hola".to_owned());
        );
    }
}
