//! Neutral positive admission for direct enum-variant construction.
//!
//! The Script semantic path learns only that an existing declaration authority
//! has already selected the direct enum route. It does not receive diagnostics,
//! mutable compilation state, or an ordinary `FromCall` fallback.

use crate::ast::ASTNode;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct EnumVariantAdmissionV1 {
    tag: u32,
    argument_count: u32,
    declared_payload_type_name: Option<Box<str>>,
}

impl EnumVariantAdmissionV1 {
    pub(crate) fn new(
        tag: u32,
        argument_count: u32,
        declared_payload_type_name: Option<Box<str>>,
    ) -> Self {
        Self {
            tag,
            argument_count,
            declared_payload_type_name,
        }
    }

    pub(crate) const fn tag(&self) -> u32 {
        self.tag
    }

    pub(crate) const fn argument_count(&self) -> u32 {
        self.argument_count
    }

    pub(crate) fn declared_payload_type_name(&self) -> Option<&str> {
        self.declared_payload_type_name.as_deref()
    }
}

/// Source-only positive proof supplied by the Program declaration-facts owner.
///
/// `None` deliberately preserves the existing raw FromCall preflight and its
/// diagnostic authority. In particular it never means ordinary FromCall.
pub(crate) trait EnumVariantDemandV1 {
    fn admit_direct_variant(
        &self,
        enum_name: &str,
        variant_name: &str,
        arguments: &[ASTNode],
    ) -> Option<EnumVariantAdmissionV1>;
}

impl EnumVariantDemandV1 for () {
    fn admit_direct_variant(
        &self,
        _enum_name: &str,
        _variant_name: &str,
        _arguments: &[ASTNode],
    ) -> Option<EnumVariantAdmissionV1> {
        None
    }
}
