#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

/// Shorthand for `mod module; pub use module::*;`.
#[macro_export]
macro_rules! glob_export {
    ($module: ident) => {
        mod $module;
        pub use $module::*;
    };
}

#[cfg(test)]
mod test;

mod extensions;
pub mod bytes;
mod error;
mod traits;
mod u24;
mod vector;

pub use extensions::*;
pub use error::*;
pub use traits::*;
pub use u24::*;
pub use vector::*;
