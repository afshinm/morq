#[macro_export]
macro_rules! morq {
    (expect( $VALUE:expr ) . $($rest:tt)*) => {
        // here we are passing the value to be used
        // in other target rules
        // TODO: is this the best way to share data between rules?
        //
        // Format is:
        //
        //  $ACTIVATOR, $NOT, $value, $($rest)*
        //
        // where $ACTIVATOR is the activation assert
        //       $NOT shows if the chain is a negative sentence
        //       $value indicates the expected value
        //       $rest is a tt
        //
        morq!(Unknown, false, $VALUE, $($rest)*);
    };

    // be - neutral rule
    ($ACTIVATOR:ident, $NOT:expr, $VALUE:expr, be . $($rest:tt)*) => {
        morq!($ACTIVATOR, $NOT, $VALUE, $($rest)*);
    };

    // to - neutral rule
    ($ACTIVATOR:ident, $NOT:expr, $VALUE:expr, to . $($rest:tt)*) => {
        morq!($ACTIVATOR, $NOT, $VALUE, $($rest)*);
    };

    // have - neutral rule
    ($ACTIVATOR:ident, $NOT:expr, $VALUE:expr, have . $($rest:tt)*) => {
        morq!($ACTIVATOR, $NOT, $VALUE, $($rest)*);
    };

    // not - negate rule
    ($ACTIVATOR:ident, $NOT:expr, $VALUE:expr, not . $($rest:tt)*) => {
        morq!($ACTIVATOR, !$NOT, $VALUE, $($rest)*);
    };

    // equal - negate rule
    ($ACTIVATOR:ident, $NOT:expr, $VALUE:expr, equal ( $TARGET:expr ) $($rest:tt)*) => {
        morq!(Equal, $NOT, $VALUE, $TARGET, $($rest)*);
    };

    // close - negate rule
    ($ACTIVATOR:ident, $NOT:expr, $VALUE:expr, close ( $TARGET:expr ) $($rest:tt)*) => {
        morq!(Close, $NOT, $VALUE, $TARGET, $($rest)*);
    };

    // end of one rule
    ($ACTIVATOR:ident, $NOT:expr, $VALUE:expr, $TARGET:expr, ; $($rest:tt)*) => {
        evaluate(&$ACTIVATOR::new(), $VALUE, $TARGET, $NOT);

        morq!($($rest)*);
    };
    () => ();
}
