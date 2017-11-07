#![crate_name = "morq"]
#![crate_type = "lib"]

extern crate num;

pub mod grammar;
pub mod asserts;
pub mod core;

pub mod prelude {
    pub use core::Assert;
    pub use core::evaluate;
    pub use asserts::close::Close;
    pub use asserts::equal::Equal;
    pub use asserts::type_match::TypeMatch;
    pub use asserts::empty::Empty;
    pub use asserts::length_of::LengthOf;
}
