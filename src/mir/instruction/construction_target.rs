//! Construction identity carried by the existing MIR and Core allocation products.
//! This first migration keeps all current construction explicitly named.
//! Intrinsic source selection is gated on the subsequent consumer cutover.
//! There is deliberately no Deref/as_str/implicit conversion to a provider name.

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConstructionTarget {
    Named(String),
}
