use std::any::{Any as StdAny, TypeId};
#[cfg(feature = "nightly")]
use std::convert::TryFrom;
#[cfg(feature = "nightly")]
use std::intrinsics;

#[cfg(feature = "nightly")]
pub(crate) fn type_name<T: StdAny + ?Sized>() -> &'static str {
    unsafe { intrinsics::type_name::<T>() }
}

#[cfg(not(feature = "nightly"))]
pub(crate) fn type_name<T: StdAny + ?Sized>() -> &'static str {
    "[ONLY ON NIGHTLY]"
}

/// FIXME(https://github.com/rust-lang/rust/issues/27745) remove this
pub trait Any: StdAny {
    fn type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    #[doc(hidden)]
    fn type_name(&self) -> &'static str {
        type_name::<Self>()
    }
}

impl<T> Any for T where T: StdAny + ?Sized {}
