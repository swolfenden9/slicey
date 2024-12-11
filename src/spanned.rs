use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use crate::Span;

/// A value of type `T` associated with a span in the source text.
#[derive(Debug)]
pub struct Spanned<T> {
    pub inner: T,
    span: Span,
}

impl<T> Spanned<T> {
    /// Creates a new `Spanned` value.
    ///
    /// # Parameters
    /// - `inner`: The value to be wrapped.
    /// - `span`: The range in the source text that corresponds to the value.
    pub fn new(inner: T, span: Span) -> Self {
        Self { inner, span }
    }

    /// Consume `self` and return the inner, wrapped value
    pub fn unwrap(self) -> T {
        self.inner
    }

    /// The span associated with the wrapped value.
    pub fn span(&self) -> Span {
        self.span.clone()
    }
}

impl<T, E> Spanned<Result<T, E>> {
    /// Unzips an `Spanned<Result<T, E>>` into a `Result<Spanned<T>, Spanned<E>>`.
    ///
    /// If `inner` is of the `Ok` variant this method returns `Ok(Spanned { inner: t, .. }`.
    /// Otherwise, `Err(Spanned { inner: e, .. }` is returned. Where `t` and `e` represent
    /// the `Ok` and `Err` values of the inner result.
    pub fn unzip(self) -> Result<Spanned<T>, Spanned<E>> {
        match self.inner {
            Ok(t) => Ok(Spanned::new(t, self.span)),
            Err(e) => Err(Spanned::new(e, self.span)),
        }
    }
}

impl<T> Spanned<Option<T>> {
    /// Unzips a `Spanned<Option<T>>` into a `Option<Spanned<T>>`.
    ///
    /// If `self` is `Spanned { inner: Some(a), .. }` this method returns `Some(Spanned { inner: a, ..})`.
    /// Otherwise, `None` is returned.
    pub fn unzip(self) -> Option<Spanned<T>> {
        match self.inner {
            Some(t) => Some(Spanned::new(t, self.span)),
            None => None,
        }
    }
}

impl<T> Spanned<&mut T> {
    /// Maps a `Spanned<&mut T>` to a `Spanned<T>` by copying the contents of the spanned.
    pub fn copied(self) -> Spanned<T>
    where
        T: Copy,
    {
        Spanned {
            inner: *self.inner,
            span: self.span.clone(),
        }
    }

    /// Maps a `Spanned<&mut T>` to a `Spanned<T>` by cloning the contents of the spanned.
    pub fn cloned(self) -> Spanned<T>
    where
        T: Clone,
    {
        Spanned {
            inner: self.inner.clone(),
            span: self.span.clone(),
        }
    }
}

impl<T> Spanned<&T> {
    /// Maps a `Spanned<&T>` to a `Spanned<T>` by copying the contents of the spanned.
    pub fn copied(self) -> Spanned<T>
    where
        T: Copy,
    {
        Spanned {
            inner: *self.inner,
            span: self.span.clone(),
        }
    }

    /// Maps a `Spanned<&T>` to a `Spanned<T>` by cloning the contents of the spanned.
    pub fn cloned(self) -> Spanned<T>
    where
        T: Clone,
    {
        Spanned {
            inner: self.inner.clone(),
            span: self.span.clone(),
        }
    }
}

impl<T> Clone for Spanned<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Spanned {
            inner: self.inner.clone(),
            span: self.span.clone(),
        }
    }
}

impl<T> Deref for Spanned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Spanned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> AsRef<T> for Spanned<T>
where
    <Spanned<T> as Deref>::Target: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        self.deref()
    }
}

impl<T> AsMut<T> for Spanned<T>
where
    <Spanned<T> as Deref>::Target: AsMut<T>,
{
    fn as_mut(&mut self) -> &mut T {
        self.deref_mut()
    }
}

impl<'a, T> From<&'a Spanned<T>> for Spanned<&'a T> {
    fn from(value: &'a Spanned<T>) -> Self {
        Spanned {
            inner: &value.inner,
            span: value.span.clone(),
        }
    }
}

impl<'a, T> From<&'a mut Spanned<T>> for Spanned<&'a mut T> {
    fn from(value: &'a mut Spanned<T>) -> Self {
        Spanned {
            inner: &mut value.inner,
            span: value.span.clone(),
        }
    }
}

impl<T: PartialEq> PartialEq for Spanned<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner && self.span == other.span
    }
}

impl<T: Display> Display for Spanned<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}(\"{:?}\")", self.inner, self.span)
    }
}
