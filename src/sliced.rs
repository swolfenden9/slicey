use std::ops::{Deref, DerefMut};

use crate::Span;

/// A value of type `T` associated with a slice of the source text.
#[derive(Debug)]
pub struct Sliced<'source, T> {
    inner: T,
    span: Span,
    source: &'source str,
}

impl<'source, T> Sliced<'source, T> {
    /// Creates a new `Sliced` value.
    ///
    /// # Parameters
    /// - `inner`: The value to be wrapped.
    /// - `span`: The range in the source text that corresponds to the value.
    /// - `source`: The source text that the wrapped value corresponds to a slice of.
    pub fn new(inner: T, span: Span, source: &'source str) -> Self {
        Self {
            inner,
            span,
            source,
        }
    }

    /// The span associated with the wrapped value.
    pub fn span(&self) -> Span {
        self.span.clone()
    }

    /// The slice associated with the wrapped value.
    pub fn slice(&self) -> &'source str {
        &self.source[self.span.clone()]
    }

    /// The source string associated with the wrapped value.
    pub fn source(&self) -> &'source str {
        self.source
    }
}

impl<'source, T, E> Sliced<'source, Result<T, E>> {
    /// Unzips an `Sliced<Result<T, E>>` into a `Result<Sliced<T>, Sliced<E>>`.
    ///
    /// If `inner` is of the `Ok` variant this method returns `Ok(Sliced { inner: t, .. }`.
    /// Otherwise, `Err(Sliced { inner: e, .. }` is returned. Where `t` and `e` represent
    /// the `Ok` and `Err` values of the inner `Result`.
    pub fn unzip(self) -> Result<Sliced<'source, T>, Sliced<'source, E>> {
        match self.inner {
            Ok(t) => Ok(Sliced::new(t, self.span, self.source)),
            Err(e) => Err(Sliced::new(e, self.span, self.source)),
        }
    }
}

impl<'source, T> Sliced<'source, Option<T>> {
    /// Unzips a `Sliced<Option<T>>` into a `Option<Sliced<T>>`.
    ///
    /// If `self` is `Sliced { inner: Some(a), .. }` this method returns `Some(Sliced { inner: a, ..})`.
    /// Otherwise, `None` is returned.
    pub fn unzip(self) -> Option<Sliced<'source, T>> {
        match self.inner {
            Some(t) => Some(Sliced::new(t, self.span, self.source)),
            None => None,
        }
    }
}

impl<'source, T> Sliced<'source, &T> {
    /// Maps a `Sliced<&T>` to a `Sliced<T>` by copying the contents of the spanned.
    pub fn copied(self) -> Sliced<'source, T>
    where
        T: Copy,
    {
        Sliced {
            inner: *self.inner,
            span: self.span.clone(),
            source: self.source,
        }
    }

    /// Maps a `Sliced<&T>` to a `Sliced<T>` by cloning the contents of the spanned.
    pub fn cloned(self) -> Sliced<'source, T>
    where
        T: Clone,
    {
        Sliced {
            inner: self.inner.clone(),
            span: self.span.clone(),
            source: self.source,
        }
    }
}

impl<'source, T> Sliced<'source, &mut T> {
    /// Maps a `Sliced<&mut T>` to a `Sliced<T>` by copying the contents of the spanned.
    pub fn copied(self) -> Sliced<'source, T>
    where
        T: Copy,
    {
        Sliced {
            inner: *self.inner,
            span: self.span.clone(),
            source: self.source,
        }
    }

    /// Maps a `Sliced<&mut T>` to a `Sliced<T>` by cloning the contents of the spanned.
    pub fn cloned(self) -> Sliced<'source, T>
    where
        T: Clone,
    {
        Sliced {
            inner: self.inner.clone(),
            span: self.span.clone(),
            source: self.source,
        }
    }
}

impl<'source, T> Clone for Sliced<'source, T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Sliced {
            inner: self.inner.clone(),
            span: self.span.clone(),
            source: self.source,
        }
    }
}

impl<'source, T> Deref for Sliced<'source, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'source, T> DerefMut for Sliced<'source, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'source, T> AsRef<T> for Sliced<'source, T>
where
    <Sliced<'source, T> as Deref>::Target: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        self.deref().as_ref()
    }
}

impl<'source, T> AsMut<T> for Sliced<'source, T>
where
    <Sliced<'source, T> as Deref>::Target: AsMut<T>,
{
    fn as_mut(&mut self) -> &mut T {
        self.deref_mut().as_mut()
    }
}

impl<'a, 'source, T> From<&'a Sliced<'source, T>> for Sliced<'source, &'a T> {
    fn from(value: &'a Sliced<'source, T>) -> Self {
        Sliced {
            inner: &value.inner,
            span: value.span.clone(),
            source: value.source,
        }
    }
}

impl<'a, 'source, T> From<&'a mut Sliced<'source, T>> for Sliced<'source, &'a mut T> {
    fn from(value: &'a mut Sliced<'source, T>) -> Self {
        Sliced {
            inner: &mut value.inner,
            span: value.span.clone(),
            source: value.source,
        }
    }
}

impl<'source, T: PartialEq> PartialEq for Sliced<'source, T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner && self.span == other.span && self.source == other.source
    }
}
