//! # Slicey
//!
//! Slicey provides two simple ways to associate data with parts of a string:
//! - [`Spamned`]: Represents data and a range.
//! - [`Sliced`]: Represents data, a range, and a source string.
//!
//! See their documentation for more info.

use std::ops::Range;

pub use sliced::Sliced;
pub use slicey_derive::{Sliced, Spanned};
pub use spanned::Spanned;

mod sliced;
mod spanned;

/// Represents a range in a source text.
/// `Span` is a shorthand for a range of indices in the source, defined as `Range<usize>`.
pub type Span = Range<usize>;
