//! Physical free-function symbol projection shared by canonical header owners.
//!
//! This value carries spelling only.  Callable identity, scalar ABI, source
//! admission, and duplicate policy remain with their existing owners.

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct CanonicalCallableSymbolV1(Box<str>);

impl CanonicalCallableSymbolV1 {
    pub(crate) fn from_name_arity(name: &str, arity: usize) -> Self {
        Self(format!("{name}/{arity}").into_boxed_str())
    }

    pub(crate) fn as_mir_name(&self) -> &str {
        &self.0
    }
}
