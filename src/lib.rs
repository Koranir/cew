#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::missing_errors_doc)]
#![doc = include_str!("../README.md")]

pub mod prelude {
    #[cfg(feature = "piping")]
    pub use super::{Inspect as _, Lay as _, Pipe as _};

    #[cfg(feature = "block_on")]
    pub use super::BlockOn as _;
}

#[cfg(feature = "color_eyre")]
pub use color_eyre_reexports::*;

#[cfg(feature = "color_eyre")]
mod color_eyre_reexports {

    // pub use color_eyre;

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

#[cfg(feature = "piping")]
mod piping {

    /// Apply a transformation to self, returning the result.
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

    /// Run a function on a shared reference to self, but return self.
    /// Use the fallible/nullable functions [`inspect_try`] and [`inspect_maybe`] if you want to propagate errors.
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

    /// Run a function that mutates self, returning self.
    /// Use the fallible/nullable functions [`lay_try`] and [`lay_maybe`] if you want to propagate errors.
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
}
#[cfg(feature = "piping")]
pub use piping::*;

#[cfg(feature = "block_on")]
mod block_on {
    /// Block on a future.
    pub trait BlockOn
    where
        Self: std::future::Future + Sized,
    {
        fn block_on(self) -> Self::Output {
            fn make_raw_waker() -> std::task::RawWaker {
                static RAW_VTABLE: std::task::RawWakerVTable =
                    std::task::RawWakerVTable::new(|_| make_raw_waker(), |_| {}, |_| {}, |_| {});
                std::task::RawWaker::new(std::ptr::null(), &RAW_VTABLE)
            }

            let mut fut = std::pin::pin!(self);
            let noop_waker = unsafe { std::task::Waker::from_raw(make_raw_waker()) };
            let mut context = std::task::Context::from_waker(&noop_waker);
            loop {
                if let std::task::Poll::Ready(output) =
                    std::future::Future::poll(fut.as_mut(), &mut context)
                {
                    return output;
                }
            }
        }
    }
    impl<T: std::future::Future> BlockOn for T {}

    #[cfg(test)]
    mod test {
        use crate::BlockOn;

        #[test]
        fn test_block_on() {
            async fn testfn() -> bool {
                true
            }

            let _out = testfn().block_on();

            assert!(_out)
        }
    }
}
#[cfg(feature = "block_on")]
pub use block_on::*;

#[cfg(all(test, feature = "piping"))]
mod test {
    use crate::prelude::*;

    #[test]
    fn test_thing() {
        let _ = 100.pipe(|n| n + 10);
    }
}
