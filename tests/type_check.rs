#[macro_use]
extern crate morq;

#[cfg(test)]
mod type_check_tests {
    use morq::prelude::*;

    #[test]
    fn test_a_string() {
        morq!(
            expect("boo".to_string()).to.be.a(String);
        );
    }

    #[test]
    fn test_a_str() {
        morq!(
            expect("boo").to.be.a(&str);
        );
    }

    #[test]
    fn test_a_vec() {
        morq!(
            expect(vec![1, 2, 3]).to.be.a(Vec<i32>);
            expect(vec!['a', 'b', 'c']).to.be.a(Vec<char>);
            expect(vec![1, 2, 3]).to.not.be.a(Vec<char>);
        );
    }

    #[test]
    fn test_a_integer() {
        morq!(
            expect(123).to.be.an(i32);
            expect(123i64).to.be.an(i64);
            expect(-123).to.be.an(i32);
        );
    }

    #[test]
    fn test_a_float() {
        morq!(
            expect(123.1).to.be.a(f64);
            expect(123.1f32).to.be.a(f32);
        );
    }

    #[test]
    fn test_a_bool() {
        morq!(
            expect(true).to.be.a(bool);
            expect(false).to.be.a(bool);
        );
    }
}
