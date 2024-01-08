#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

//! A simple crate that shorten's `color_eyre`'s names.
//!
//! Run `cew::init()` to initialize `color_eyre`
//!
//! `cew::R` is short for `color_eyre::Result`
//!
//! `cew::U` is short for `color_eyre::Result<()>`
//!
//! `cew::e!(..)` is short for `color_eyre::eyre::eyre!(..)`
//!
//! `cew::me!(..)` is short for `Err(color_eyre::eyre::eyre!(..))`

pub use color_eyre;

pub use color_eyre::Result as R;

/// type alias for `Result<(), Report>`
pub type U = R<()>;

pub use color_eyre::eyre::eyre as e;

/// Construct an ad-hoc `color_eyre::Result::Err` from a string
#[macro_export]
macro_rules! me {
    ($($t:tt)*) => {
        core::result::Result::Err(cew::e!($($t)*))
    };
}

/// Initializes `color_eyre`
///
/// # Errors
/// When called more than once
pub fn init() -> U {
    color_eyre::install()
}
