#[macro_export]
macro_rules! morq {
    (expect( $VALUE:expr ) . $($rest:tt)*) => {
        // here we are passing the value to be used
        // in other target rules
        //
        // Format is:
        //
        //  $ACTIVATOR, $NOT, $VALUE, $($rest)*
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

    // terminal rule
    // ok
    ($ACTIVATOR:ident, $NOT:expr, $VALUE:expr, ok ( ) $($rest:tt)*) => {
        // TODO: ugh. this is not a good way to test Result
        // Can we make it better?
        morq!(ResultCheck, $NOT, $VALUE.into_iter(), 0, $($rest)*);
    };

    // terminal rule
    // ok
    ($ACTIVATOR:ident, $NOT:expr, $VALUE:expr, err ( ) $($rest:tt)*) => {
        morq!(ResultCheck, !$NOT, $VALUE.into_iter(), 0, $($rest)*);
    };

    // terminal rule
    // equal
    ($ACTIVATOR:ident, $NOT:expr, $VALUE:expr, equal ( $TARGET:expr ) $($rest:tt)*) => {
        morq!(Equal, $NOT, $VALUE, $TARGET, $($rest)*);
    };

    // terminal rule
    // close
    ($ACTIVATOR:ident, $NOT:expr, $VALUE:expr, close ( $TARGET:expr ) $($rest:tt)*) => {
        morq!(Close, $NOT, $VALUE, $TARGET, $($rest)*);
    };

    // terminal rule
    // empty
    ($ACTIVATOR:ident, $NOT:expr, $VALUE:expr, empty ( ) $($rest:tt)*) => {
        morq!(Empty, $NOT, $VALUE, 0, $($rest)*);
    };

    // terminal rule
    // empty
    ($ACTIVATOR:ident, $NOT:expr, $VALUE:expr, lengthOf ( $TARGET:expr ) $($rest:tt)*) => {
        morq!(LengthOf, $NOT, $VALUE, $TARGET, $($rest)*);
    };

    // terminal rule
    // empty
    ($ACTIVATOR:ident, $NOT:expr, $VALUE:expr, contain ( $TARGET:expr ) $($rest:tt)*) => {
        morq!(Contain, $NOT, $VALUE, $TARGET, $($rest)*);
    };

    // terminal rule
    // a (to match data type)
    ($ACTIVATOR:ident, $NOT:expr, $VALUE:expr, a ( $TARGET:ty ) $($rest:tt)*) => {
        morq!(TypeMatch, $NOT, $VALUE, $TARGET, $($rest)*);
    };

    // terminal rule
    // an (to match data type)
    ($ACTIVATOR:ident, $NOT:expr, $VALUE:expr, an ( $TARGET:ty ) $($rest:tt)*) => {
        morq!(TypeMatch, $NOT, $VALUE, $TARGET, $($rest)*);
    };

    // end of a rule
    // used for type check only
    // TODO: can we merge this one with the $TARGET:expr target?
    ($ACTIVATOR:ident, $NOT:expr, $VALUE:expr, $TARGET:ty, ; $($rest:tt)*) => {
        evaluate(&$ACTIVATOR::new(), $VALUE, ::std::any::TypeId::of::<$TARGET>(), $NOT);

        morq!($($rest)*);
    };

    // end of a rule
    ($ACTIVATOR:ident, $NOT:expr, $VALUE:expr, $TARGET:expr, ; $($rest:tt)*) => {
        evaluate(&$ACTIVATOR::new(), $VALUE, $TARGET, $NOT);

        morq!($($rest)*);
    };
    () => ();
}
