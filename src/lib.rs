#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::missing_errors_doc)]

//! A personal utility crate that shorten's [`color_eyre`]'s names and a little extra
//!
//! Run [`cew::init()`] to initialize [`color_eyre`]
//!
//! [`cew::R`] is short for [`color_eyre::Result`]
//!
//! [`cew::U`] is short for [`color_eyre::Result<()>`]
//!
//! [`cew::e!(..)`] is short for [`color_eyre::eyre::eyre!(..)`]
//!
//! [`cew::me!(..)`] is short for [`Err(color_eyre::eyre::eyre!(..))`]
//!
//! Also adds the globally implemented [`Pipe`], [`Inspect`], and [`Lay`]
//! traits that provides a function to reduce the amount of stacked parenthesis.

#[cfg(feature = "color_eyre")]
pub use color_eyre_reexports::*;

#[cfg(feature = "color_eyre")]
mod color_eyre_reexports {

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
}

/// Trait to prevent big wrapping '()'s everywhere.
///
/// # Example
///
/// ```rust
/// // Take this function 'foo', which takes a value and returns itself
/// fn foo(val: &str) -> &str {
///     val
/// }
/// // That we want to use on this:
/// let bar = String::new();
///
/// // When using it, you need something like this:
/// let _ = foo(bar.as_str());
/// // But if you forgot that you needed that `foo` call at the beginning,
/// // you have to backtrack to the start of the call, then add braces, which is
/// // where this crate comes in.
///
/// use cew::Piper;
/// // It turns that mess into this:
/// let _ = bar.as_str().pipe(foo);
///
/// ```
pub trait Pipe {
    #[must_use]
    fn pipe<T>(self, f: impl FnOnce(Self) -> T) -> T
    where
        Self: Sized,
    {
        f(self)
    }
}
impl<T> Pipe for T {}

pub trait Inspect {
    /// Will ignore the result of `f`. If `f` returns a result or option, check
    /// [`inspect_try`] or [`inspect_maybe`]
    #[must_use]
    fn inspect<T>(self, f: impl FnOnce(&Self) -> T) -> Self
    where
        Self: Sized,
    {
        f(&self);
        self
    }

    fn inspect_try<T, E>(self, f: impl FnOnce(&Self) -> Result<T, E>) -> Result<Self, E>
    where
        Self: Sized,
    {
        f(&self)?;
        Ok(self)
    }

    #[must_use]
    fn inspect_maybe<T>(self, f: impl FnOnce(&Self) -> Option<T>) -> Option<Self>
    where
        Self: Sized,
    {
        f(&self)?;
        Some(self)
    }
}
impl<T> Inspect for T {}

pub trait Lay {
    #[must_use]
    fn lay<T>(mut self, f: impl FnOnce(&mut Self) -> T) -> Self
    where
        Self: Sized,
    {
        f(&mut self);
        self
    }

    fn lay_try<T, E>(mut self, f: impl FnOnce(&mut Self) -> Result<T, E>) -> Result<Self, E>
    where
        Self: Sized,
    {
        f(&mut self)?;
        Ok(self)
    }

    #[must_use]
    fn lay_maybe<T>(mut self, f: impl FnOnce(&mut Self) -> Option<T>) -> Option<Self>
    where
        Self: Sized,
    {
        f(&mut self)?;
        Some(self)
    }
}
impl<T> Lay for T {}
