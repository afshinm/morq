#[macro_export]
macro_rules! morq {
    (expect( $VALUE:expr ) . $($rest:tt)*) => {
        //println!("Start -> {}", stringify!($VALUE));

        // here we are passing the value to be used
        // in other target rules
        // TODO: is this the best way to share data between rules?
        //
        // Format is:
        //
        //  $positive, $value, $($rest)*
        //
        // where $positive shows if the chain is a positive sentence
        //       $value indicates the expected value
        //       $rest is a tt
        //
        morq!(true, $VALUE, $($rest)*);
    };

    // neutral rule
    ($POSITIVE:expr, $VALUE:expr, be . $($rest:tt)*) => {
        //println!("{} <- End: {}", stringify!($TOPIC), $VALUE);
        morq!($POSITIVE, $VALUE, $($rest)*);
    };

    // neutral rule
    ($POSITIVE:expr, $VALUE:expr, to . $($rest:tt)*) => {
        //println!("to");
        morq!($POSITIVE, $VALUE, $($rest)*);
    };

    // neutral rule
    ($POSITIVE:expr, $VALUE:expr, have . $($rest:tt)*) => {
        //println!("have");
        morq!($POSITIVE, $VALUE, $($rest)*);
    };

    // negate rule
    ($POSITIVE:expr, $VALUE:expr, not . $($rest:tt)*) => {
        //println!("not");
        morq!(!$POSITIVE, $VALUE, $($rest)*);
    };

    ($POSITIVE:expr, $VALUE:expr, equal ( $TARGET:expr ) $($rest:tt)*) => {
        //println!("{} <- Equal ({}): {}", stringify!($TARGET), $POSITIVE, $VALUE);

        if $POSITIVE {
            //assert_eq!($VALUE, $TARGET);
            evaluate(Equal::new($VALUE).compare($TARGET));
        } else {
            evaluate(NotEqual::new($VALUE).compare($TARGET));
        }

        morq!($($rest)*);
    };

    // end of one rule
    (; $($rest:tt)*) => {
        //println!("terminate");
        morq!($($rest)*);
    };
    () => ();
}
