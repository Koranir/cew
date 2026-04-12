#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::missing_errors_doc)]
#![doc = include_str!("../README.md")]

pub mod prelude {
    #[cfg(feature = "piping")]
    pub use super::{Inspect, Lay, Pipe};

    #[cfg(feature = "tracing")]
    pub use super::tracing::prelude::*;

    #[cfg(feature = "snafu")]
    pub use super::{OptionExt as _, ResultExt as _};
}

#[cfg(feature = "snafu")]
pub use snafu_reexports::*;
#[cfg(feature = "snafu")]
mod snafu_reexports {
    pub use snafu::{OptionExt, ResultExt, Snafu};

    pub use snafu::Whatever;
    pub type R<T, E = Whatever> = std::result::Result<T, E>;
    pub type U = R<()>;

    pub use snafu::whatever as e;

    #[macro_export]
    macro_rules! me {
        ($($t:tt)*) => {
            core::result::Result::Err(cew::e!($($t)*))
        };
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

    #[cfg(test)]
    mod test {
        use crate as cew;
        use cew::prelude::*;

        #[test]
        fn test_thing() {
            let _ = 100.pipe(|n| n + 10);
        }
    }
}
#[cfg(feature = "piping")]
pub use piping::*;

#[cfg(feature = "tracing")]
pub mod tracing {
    pub use tracing;
    pub use tracing_subscriber as subscriber;

    pub mod prelude {
        pub use tracing::{
            debug, debug_span, error, error_span, info, info_span, instrument, trace, trace_span,
            warn, warn_span, Instrument,
        };
    }

    #[derive(Debug)]
    pub enum Error {
        TryInit(subscriber::util::TryInitError),
        EnvFilter(subscriber::filter::ParseError),
        VarError(std::env::VarError),
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::TryInit(e) => write!(f, "{e}"),
                Self::EnvFilter(e) => write!(f, "{e}"),
                Self::VarError(e) => write!(f, "Failed to read filter from RUST_LOG: {e}"),
            }
        }
    }

    impl std::error::Error for Error {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match self {
                Self::TryInit(e) => Some(e),
                Self::EnvFilter(e) => Some(e),
                Self::VarError(e) => Some(e),
            }
        }
    }

    pub use subscriber::fmt::layer as fmt_layer;
    pub use tracing::level_filters::LevelFilter;
    pub use tracing::Level;

    /// Initialises a tracing subscriber with a given layer and an env filter.
    ///
    /// The env filter is read from the `RUST_LOG` environment variable, but if that variable is not set, it will use the provided default filter, using the regular env logger syntax.
    ///
    /// Tip: Use with [`fmt_layer`]
    pub fn init_filtered_w_env(
        layer: impl subscriber::Layer<subscriber::Registry> + Send + Sync + 'static,
        default_env_filter: &str,
    ) -> Result<(), Error> {
        use tracing_subscriber::layer::SubscriberExt;
        use tracing_subscriber::util::SubscriberInitExt;

        let env;
        let filter = match std::env::var(tracing_subscriber::EnvFilter::DEFAULT_ENV) {
            Ok(f) => {
                env = f;
                &env
            }
            Err(std::env::VarError::NotPresent) => default_env_filter,
            Err(e) => return Err(Error::VarError(e)),
        };
        let filter = tracing_subscriber::EnvFilter::try_new(filter).map_err(Error::EnvFilter)?;

        tracing_subscriber::registry()
            .with(layer.with_filter(filter))
            .try_init()
            .map_err(Error::TryInit)?;

        Ok(())
    }
}

#[cfg(feature = "snafu")]
pub use snafu;
