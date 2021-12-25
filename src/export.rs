//! Opinionated list of standard imports.

#[doc(no_inline)]
pub use crate::Timer;
#[doc(no_inline)]
pub use color_eyre::eyre::{eyre, Result, WrapErr};
#[doc(no_inline)]
pub use itertools::Itertools;
#[doc(no_inline)]
pub use log::{debug, error, info, log, trace, warn};
#[doc(no_inline)]
pub use std::borrow::Cow;
#[doc(no_inline)]
pub use std::future::Future;
#[doc(no_inline)]
pub use std::io::{Read, Write};
#[doc(no_inline)]
pub use std::marker::{PhantomData, Send, Sync};
#[doc(no_inline)]
pub use std::rc::Rc;
#[doc(no_inline)]
pub use std::sync::Arc;

/// Unit typed [`Result`].
pub type Unit = Result<()>;
