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
            (3 * 5).to.be(15);
            (50).to.be(5 * 10);
        );

        //assert_eq!(2 + 2, 4);
    }
}
