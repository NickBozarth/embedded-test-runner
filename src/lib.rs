#![no_std]

extern crate alloc;

pub mod embedded_utils;
pub mod test_case;
pub mod test_runner;
pub mod macros;

#[cfg(all(feature = "ram-16k", feature = "ram-64k"))]
compile_error!("Features `ram-16k` and `ram-64k` cannot be built together");

#[cfg(not(any(feature = "ram-16k", feature = "ram-64k")))]
compile_error!("Select ONE memory target feature: --features [`ram-16k`, `ram-64k`]");
