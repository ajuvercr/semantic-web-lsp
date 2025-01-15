use std::{
    borrow::Borrow,
    ops::{Deref, DerefMut, Range},
};

use enum_methods::{EnumIntoGetters, EnumIsA, EnumToGetters};

#[derive(Debug, Clone)]
pub struct Spanned<T>(pub T, pub Range<usize>);
impl<T> Default for Spanned<T>
where
    T: Default,
{
    fn default() -> Self {
        Self(T::default(), 0..1)
    }
}
impl<T> Spanned<T> {
    pub fn map<O>(self, f: impl Fn(T) -> O) -> Spanned<O> {
        let v = f(self.0);
        Spanned(v, self.1)
    }
    pub fn map_ref<'a, O: 'a>(&'a self, f: impl Fn(&'a T) -> O) -> Spanned<O> {
        let v = f(&self.0);
        Spanned(v, self.1.clone())
    }
    pub fn as_ref<'a>(&'a self) -> Spanned<&'a T> {
        Spanned(&self.0, self.1.clone())
    }

    pub fn try_map_ref<'a, O>(&'a self, f: impl FnOnce(&'a T) -> Option<O>) -> Option<Spanned<O>> {
        if let Some(v) = f(&self.0) {
            Some(Spanned(v, self.1.clone()))
        } else {
            None
        }
    }
    pub fn try_map<O>(self, f: impl FnOnce(T) -> Option<O>) -> Option<Spanned<O>> {
        if let Some(v) = f(self.0) {
            Some(Spanned(v, self.1))
        } else {
            None
        }
    }
}
impl<T> Spanned<Option<T>> {
    pub fn transpose(self) -> Option<Spanned<T>> {
        self.0.map(|inner| Spanned(inner, self.1))
    }
}

impl<T: PartialEq> PartialEq for Spanned<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}
impl<T: std::hash::Hash> std::hash::Hash for Spanned<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}
impl<T: PartialEq> Eq for Spanned<T> {}

pub fn spanned<T>(t: T, span: Range<usize>) -> Spanned<T> {
    Spanned(t, span)
}

impl Borrow<str> for Spanned<String> {
    #[inline]
    fn borrow(&self) -> &str {
        &self[..]
    }
}

impl<T> Deref for Spanned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Spanned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Spanned<T> {
    pub fn into_value(self) -> T {
        self.0
    }
    pub fn into_span(self) -> Range<usize> {
        self.1
    }
    pub fn value(&self) -> &T {
        &self.0
    }
    pub fn value_mut(&mut self) -> &mut T {
        &mut self.0
    }
    pub fn span(&self) -> &Range<usize> {
        &self.1
    }
}

