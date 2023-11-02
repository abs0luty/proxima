use std::{
    path::{Path, PathBuf},
    sync::Mutex,
};

use lazy_static::lazy_static;
use string_interner::{backend::StringBackend, StringInterner, Symbol};

lazy_static! {
    static ref PATH_INTERNER: Mutex<StringInterner<StringBackend<SymbolUsize>>> =
        Mutex::new(StringInterner::new());
    static ref STRING_INTERNER: Mutex<StringInterner<StringBackend<SymbolUsize>>> =
        Mutex::new(StringInterner::new());
    static ref IDENTIFIER_INTERNER: Mutex<StringInterner<StringBackend<SymbolUsize>>> =
        Mutex::new(StringInterner::new());
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct SymbolUsize(usize);

impl From<usize> for SymbolUsize {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<SymbolUsize> for usize {
    #[inline]
    fn from(value: SymbolUsize) -> Self {
        value.0
    }
}

impl Symbol for SymbolUsize {
    #[inline]
    fn try_from_usize(index: usize) -> Option<Self> {
        Some(Self(index))
    }

    #[inline]
    fn to_usize(self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IdentifierId(SymbolUsize);

pub const DUMMY_IDENTIFIER_ID: IdentifierId = IdentifierId(SymbolUsize(usize::MAX - 1));

impl<S> From<S> for IdentifierId
where
    S: AsRef<str>,
{
    fn from(str: S) -> Self {
        Self(IDENTIFIER_INTERNER.lock().unwrap().get_or_intern(str))
    }
}

impl From<IdentifierId> for Option<String> {
    fn from(id: IdentifierId) -> Self {
        IDENTIFIER_INTERNER
            .lock()
            .unwrap()
            .resolve(id.0)
            .map(ToOwned::to_owned)
    }
}

impl From<IdentifierId> for String {
    fn from(id: IdentifierId) -> Self {
        Option::<String>::from(id).unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StringId(SymbolUsize);

pub const DUMMY_STRING_ID: StringId = StringId(SymbolUsize(usize::MAX - 1));

impl<S> From<S> for StringId
where
    S: AsRef<str>,
{
    fn from(value: S) -> Self {
        Self(STRING_INTERNER.lock().unwrap().get_or_intern(value))
    }
}

impl From<StringId> for Option<String> {
    fn from(id: StringId) -> Self {
        STRING_INTERNER
            .lock()
            .unwrap()
            .resolve(id.0)
            .map(ToOwned::to_owned)
    }
}

impl From<StringId> for String {
    fn from(id: StringId) -> Self {
        Option::<String>::from(id).unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PathId(SymbolUsize);

pub const DUMMY_PATH_ID: PathId = PathId(SymbolUsize(usize::MAX - 1));

impl<P> From<P> for PathId
where
    P: AsRef<Path>,
{
    fn from(path: P) -> Self {
        Self(
            PATH_INTERNER
                .lock()
                .unwrap()
                .get_or_intern(path.as_ref().to_str().unwrap()),
        )
    }
}

impl From<PathId> for Option<PathBuf> {
    fn from(id: PathId) -> Self {
        PATH_INTERNER.lock().unwrap().resolve(id.0).map(Into::into)
    }
}

impl From<PathId> for PathBuf {
    fn from(id: PathId) -> Self {
        Option::<PathBuf>::from(id).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_identifiers() {
        let a = IdentifierId::from("a");
        let b = IdentifierId::from("b");
        let a2 = IdentifierId::from("a");

        assert_eq!(a, a2);
        assert_ne!(a, b);
        assert_ne!(a2, b);
    }

    #[test]
    fn compare_paths() {
        let a = PathId::from("a.lzr");
        let b = PathId::from("b.lzr");
        let a2 = PathId::from("a.lzr");

        assert_eq!(a, a2);
        assert_ne!(a, b);
        assert_ne!(a2, b);
    }

    #[test]
    fn compare_strings() {
        let a = StringId::from("a.lzr");
        let b = StringId::from("b.lzr");
        let a2 = StringId::from("a.lzr");

        assert_eq!(a, a2);
        assert_ne!(a, b);
        assert_ne!(a2, b);
    }
}
