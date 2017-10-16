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
        use asserts::equal::Equal;

        morq!(
            expect(3 * 5).to.be.equal(15);
            expect(3 * 5).to.not.be.equal(15);
            expect(50).to.be.equal(5 * 10);
        );
    }
}
