#[macro_export]
macro_rules! morq {
    (( $TOPIC:expr ) . $($rest:tt)*) => {
        println!("Start -> {}", stringify!($TOPIC));
        morq!($TOPIC, $($rest)*);
    };
    ($VAL:expr, be ( $TOPIC:expr ) $($rest:tt)*) => {
        println!("{} <- End: {}", stringify!($TOPIC), $VAL);

        Equal::new().compare($VAL, $TOPIC);

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
