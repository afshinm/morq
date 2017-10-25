#[macro_use]
extern crate morq;

#[cfg(test)]
mod close_tests {
    use morq::prelude::*;

    #[test]
    fn test_close_to() {
        morq!(
            expect(1.0f64).to.be.close(1.0);
            expect(1e-40f32).to.be.close(0.0);
        );
    }
}
