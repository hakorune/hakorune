//! Source-root shape for one semantic execution owner.
//!
//! Owner IDs remain shape-neutral. This tag prevents normalized Script,
//! declared-function, and Lambda graphs from becoming indistinguishable.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SemanticOwnerSourceKindV1 {
    DeclaredFunction,
    Script,
    Lambda,
}
