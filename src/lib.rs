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

/// Initialize `color_eyre`
pub fn init() -> U {
    color_eyre::install()
}

#[cfg(test)]
mod tests {
    use crate as cew;

    #[test]
    #[should_panic]
    fn do_stuff() -> cew::U {
        cew::init()?;

        fn test_fn() -> cew::U {
            cew::me!("This should error")
        }

        test_fn()
    }
}
