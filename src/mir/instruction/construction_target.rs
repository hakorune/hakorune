//! Construction identity carried by the existing MIR and Core allocation products.
//! Array literal lowering issues IntrinsicArray; explicit named construction
//! and the pending Map/Main producers retain Named until their consumer cutover.
//! There is deliberately no Deref/as_str/implicit conversion to a provider name.

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConstructionTarget {
    Named(String),
    IntrinsicArray,
    IntrinsicMap,
}
