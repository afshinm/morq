#[macro_export]
macro_rules! morq {
    (expect( $VALUE:expr ) . $($rest:tt)*) => {
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
        morq!($POSITIVE, $VALUE, $($rest)*);
    };

    // neutral rule
    ($POSITIVE:expr, $VALUE:expr, to . $($rest:tt)*) => {
        morq!($POSITIVE, $VALUE, $($rest)*);
    };

    // neutral rule
    ($POSITIVE:expr, $VALUE:expr, have . $($rest:tt)*) => {
        morq!($POSITIVE, $VALUE, $($rest)*);
    };

    // negate rule
    ($POSITIVE:expr, $VALUE:expr, not . $($rest:tt)*) => {
        morq!(!$POSITIVE, $VALUE, $($rest)*);
    };

    ($POSITIVE:expr, $VALUE:expr, equal ( $TARGET:expr ) $($rest:tt)*) => {
        if $POSITIVE {
            evaluate(Equal::new().compare($VALUE, $TARGET));
        } else {
            evaluate(NotEqual::new().compare($VALUE, $TARGET));
        }

        morq!($($rest)*);
    };

    ($POSITIVE:expr, $VALUE:expr, close ( $TARGET:expr ) $($rest:tt)*) => {
        if $POSITIVE {
            evaluate(Close::new().compare($VALUE, $TARGET));
        } else {
            //evaluate(close::Close::new($VALUE).compare($TARGET));
        }

        morq!($($rest)*);
    };

    // end of one rule
    (; $($rest:tt)*) => {
        morq!($($rest)*);
    };
    () => ();
}
