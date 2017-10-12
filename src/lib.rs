
mod core;
pub mod asserts;

macro_rules! morq {
    (( $TOPIC:expr ) . $($rest:tt)*) => {
        println!("Start -> {}", stringify!($TOPIC));
        morq!($TOPIC, $($rest)*);
    };
    ($VAL:expr, be ( $TOPIC:expr ) $($rest:tt)*) => {
        println!("{} <- End: {}", stringify!($TOPIC), $VAL);
        assert_eq!($TOPIC, $VAL);
        morq!($($rest)*);
    };
    ($VAL:expr, to.$($rest:tt)+) => {
        println!("to");
        morq!($VAL, $($rest)*);
    };
    (; $($rest:tt)*) => {
        println!("terminate");
        morq!($($rest)*);
    };
    () => ()
}


#[cfg(test)]
mod tests {
    #[test]
    fn grammar_test() {

        morq!(
            (3 * 5).to.be(15);
            (50).to.be(5 * 10);
        );

        //assert_eq!(2 + 2, 4);
    }
}
