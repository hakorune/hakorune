//! Construction identity carried by the existing MIR and Core allocation products.
//! Current source producers stay named until intrinsic consumer acceptance.
//! Intrinsic source selection is gated on the subsequent consumer cutover.
//! There is deliberately no Deref/as_str/implicit conversion to a provider name.

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConstructionTarget {
    Named(String),
    IntrinsicArray,
}
