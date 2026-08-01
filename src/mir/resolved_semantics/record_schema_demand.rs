//! Neutral positive admission for RecordLiteral descendant demand.
//!
//! Script semantics may ask only whether an existing declaration authority
//! proves that a literal has no declaration-owned default descendants.  It
//! never receives schema AST, type contracts, or diagnostics.

use crate::ast::ASTNode;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct FullyExplicitRecordLiteralAdmissionV1 {
    explicit_field_count: u32,
}

impl FullyExplicitRecordLiteralAdmissionV1 {
    pub(crate) const fn new(explicit_field_count: u32) -> Self {
        Self {
            explicit_field_count,
        }
    }

    pub(crate) const fn explicit_field_count(self) -> u32 {
        self.explicit_field_count
    }
}

/// Source-only positive proof supplied by the declaration-facts owner.
///
/// `None` is deliberately non-diagnostic: the complete Script route is not
/// eligible and existing RootLower remains the sole user-error authority.
pub(crate) trait RecordSchemaDemandV1 {
    fn admit_fully_explicit_literal(
        &self,
        record_type_name: &str,
        fields: &[(String, ASTNode)],
    ) -> Option<FullyExplicitRecordLiteralAdmissionV1>;
}

impl RecordSchemaDemandV1 for () {
    fn admit_fully_explicit_literal(
        &self,
        _record_type_name: &str,
        _fields: &[(String, ASTNode)],
    ) -> Option<FullyExplicitRecordLiteralAdmissionV1> {
        None
    }
}
