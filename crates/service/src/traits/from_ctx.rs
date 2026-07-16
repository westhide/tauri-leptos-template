use std::any::type_name;

use crate::shared::error::{Error, err};

pub trait FromCtx {
    fn from_ctx() -> Self;
}

pub trait TryFromCtx: Sized {
    fn try_from_ctx() -> Result<Self, Error>;
}

impl<T> TryFromCtx for T
where
    T: Clone + 'static,
{
    fn try_from_ctx() -> Result<Self, Error> {
        match use_context::<Self>() {
            Some(value) => Ok(value),
            None => err!("use context: {} not found", type_name::<T>()),
        }
    }
}

#[macro_export]
macro_rules! impl_from_ctx {
    ($ty:tt) => {
        impl $crate::traits::from_ctx::FromCtx for $ty {
            #[track_caller]
            fn from_ctx() -> Self {
                match $crate::traits::from_ctx::use_context::<Self>() {
                    Some(value) => value,
                    None => {
                        $crate::traits::from_ctx::error!(
                            "use context {} failed",
                            std::any::type_name::<$ty>()
                        );
                        // TODO: const_format_args
                        unreachable!("context is provided")
                    },
                }
            }
        }
    };
}

pub use leptos::context::use_context;
pub use tracing::error;
