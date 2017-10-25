#![crate_name = "morq"]
#![crate_type = "lib"]

extern crate num;

#[macro_use]
pub mod grammar;
pub mod asserts;
pub mod core;

pub mod prelude {
    pub use core::Assert;
    pub use core::evaluate;
    pub use asserts::close::Close;
    pub use asserts::equal::Equal;
    pub use asserts::not_equal::NotEqual;
}
