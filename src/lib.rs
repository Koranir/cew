pub use color_eyre;

pub use color_eyre::Result as R;

pub type U = R<()>;

pub use color_eyre::eyre::eyre as e;

#[macro_export]
macro_rules! me {
    ($($t:tt)*) => {
        core::result::Result::Err(cew::e!($($t)*))
    };
}

#[cfg(test)]
mod tests {
    use crate as cew;

    #[test]
    #[should_panic]
    fn do_stuff() {
        fn test_fn() -> cew::U {
            cew::me!("This should error")
        }

        test_fn().unwrap()
    }
}
