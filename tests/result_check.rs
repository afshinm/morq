#[macro_use]
extern crate morq;

#[cfg(test)]
mod result_check_tests {
    use morq::prelude::*;

    #[test]
    fn test_is_ok() {
        let res: Result<String, String> = Ok(format!("boo"));

        morq!(
            expect(res).to.be.ok();
        );
    }


    #[test]
    fn test_is_err() {
        let res: Result<String, String> = Err(format!("boo"));

        morq!(
            expect(res).to.be.err();
        );
    }

    #[test]
    fn test_is_not_ok() {
        let res: Result<String, String> = Err(format!("boo"));

        morq!(
            expect(res).to.not.be.ok();
        );
    }

    #[test]
    fn test_is_not_err() {
        let res: Result<String, String> = Ok(format!("boo"));

        morq!(
            expect(res).to.not.be.err();
        );
    }
}
