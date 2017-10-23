#![crate_name = "morq"]
#![crate_type = "lib"]

mod core;

#[macro_use]
pub mod grammar;
pub mod asserts;

#[cfg(test)]
mod tests {
    #[test]
    fn grammar_test() {
        use core::Assert;
        use core::evaluate;
        use asserts::equal::Equal;
        use asserts::not_equal::NotEqual;

        morq!(
            expect(3 * 5).to.be.equal(15);
            expect("afshin").to.be.equal("afshin");
            expect(50).to.be.equal(5 * 10);
            expect(5.10).to.be.equal(5.10);
            expect(5.0 + 0.1).to.be.equal(5.10);
            expect(5.0 * 0.1).to.be.equal(0.50);

            expect(vec![1, 2, 3]).to.be.equal(vec![1, 2, 3]);

            expect(3 * 5).to.not.be.equal(10);
            expect(50).to.not.be.equal(10);
            expect("afshin").to.not.be.equal("afshi2");
            expect(5.0 * 0.5).to.not.be.equal(5.0);
        );
    }
}
