//! Scoped physical-signature sibling loan for an installed callable row.

use super::super::physical_signature::PhysicalCallableSignatureRowRefV1;

/// Non-owning, non-Clone signature sibling for one resolved lowering loan.
/// The package issuer is the only constructor; the module handoff consumes
/// this view before the surrounding HRTB callback returns.
pub(crate) struct ResolvedCallablePhysicalSignatureLoanV1<'loan> {
    row: PhysicalCallableSignatureRowRefV1<'loan>,
    _seal: ResolvedCallablePhysicalSignatureLoanSealV1,
}

struct ResolvedCallablePhysicalSignatureLoanSealV1;

impl<'loan> ResolvedCallablePhysicalSignatureLoanV1<'loan> {
    pub(super) fn new(row: PhysicalCallableSignatureRowRefV1<'loan>) -> Self {
        Self {
            row,
            _seal: ResolvedCallablePhysicalSignatureLoanSealV1,
        }
    }

    /// Internal adapter for the S6C loan. It preserves the package-owned
    /// signature row and does not create a second signature authority.
    pub(crate) fn from_s6c_row(row: PhysicalCallableSignatureRowRefV1<'loan>) -> Self {
        Self::new(row)
    }

    pub(crate) const fn owner(&self) -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
        self.row.owner()
    }

    pub(crate) fn identity(&self) -> &crate::parser::CallableDeclarationIdentityV1 {
        self.row.identity()
    }

    pub(crate) const fn physical_callable_lane_count(&self) -> u32 {
        self.row.physical_callable_lane_count()
    }

    pub(crate) const fn receiver_lane_count(&self) -> u32 {
        self.row.receiver_lane_count()
    }

    pub(crate) const fn source_logical_arity(&self) -> u32 {
        self.row.source_logical_arity()
    }

    pub(crate) const fn physical_formal_lane_count(&self) -> u32 {
        self.row.physical_formal_lane_count()
    }

    pub(crate) fn has_exact_text_formal(&self) -> bool {
        self.row.lanes().iter().any(|lane| {
            matches!(
                lane.role(),
                crate::mir::normal_callable_semantic_package::PhysicalCallableLaneRoleV1::
                    ExactTextSlot
            )
        })
    }

    /// Borrow the complete lane rows from the package-owned cohort. This is
    /// a scoped sibling view; it never copies or reissues the signature rows.
    pub(crate) fn lanes(
        &self,
    ) -> &[crate::mir::normal_callable_semantic_package::physical_signature::PhysicalCallableLaneV1]
    {
        self.row.lanes()
    }
}
