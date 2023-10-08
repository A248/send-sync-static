#![forbid(unsafe_code)]
#![no_std]

//!
//! A very basic crate, whose only purpose is to allow marking types [`Send`], [`Sync`], and `'static`
//!

use core::future::Future;

///
/// A marker which identifies a type as [`Send`], [`Sync`], and `'static`.
/// This trait is automatically implemented for any types which fulfill these requirements,
/// and it is intended to be used as a shorthand for writing out each bound. For example:
/// ```
/// use send_sync_static::SSS;
/// pub async fn send_data<D>(data: D) where D: SSS {
///   // Do something here
/// }
/// ```
///
/// Code written explicitly using [`Send`], [`Sync`], and `'static` is fully interchangeable with this trait.
///
pub trait SSS: Send + Sync + 'static {}

impl<S> SSS for S where S: Send + Sync + 'static {}

///
/// A marker which identifies a [`Future`] (but not necessarily its output) as [`Send`], [`Sync`], and `'static`.
///
/// This trait is automatically implemented for futures which fulfill these requirements,
/// and it is intended to be used as a shorthand for writing out each bound. For example:
/// ```
/// use send_sync_static::{FutureSSS, SSS};
/// pub fn send_data<D: SSS>(data: D) -> impl FutureSSS {
///   // Guarantees the async block is always Send, Sync, and 'static
///   async move {
///     // Do something here
///     drop(data)
///   }
/// }
/// ```
/// This can be combined with RPITIT, an upcoming stable feature.
/// ```ignore
/// use send_sync_static::FutureSSS;
/// pub trait Database {
///   fn add_user(&self, user: String) -> impl FutureSSS<Output = ()>;
/// }
/// ```
///
/// Code written explicitly using [`Send`], [`Sync`], and `'static` is fully interchangeable with this trait.
///
pub trait FutureSSS: Future + SSS {}

impl<F> FutureSSS for F where F: Future + SSS {}

#[cfg(test)]
mod tests {
    use crate::{FutureSSS, SSS};

    #[test]
    fn implemented() {
        fn assert_sss<V: SSS>(_val: V) {}
        fn assert_future_sss<F: FutureSSS<Output = usize>>(f: F) -> Option<usize> {
            assert_sss(f);
            None
        }

        assert_sss(0usize);
        assert_future_sss(async { 0usize });
    }
}
