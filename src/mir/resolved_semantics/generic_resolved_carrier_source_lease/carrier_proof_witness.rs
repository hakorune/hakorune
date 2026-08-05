//! Bounded carrier-transfer proof over the resolver-owned source lease.
//!
//! This is deliberately not the full Generic semantic shape.  It proves one
//! relation only: the nested write and post-loop read refer to the same
//! resolver BindingRef.  The lease is retained in the returned handoff so a
//! later stage cannot split the proof from its source brand.

use super::{GenericSourceAncestryV1, GenericSourceLeaseV1, GenericSourceRoleClaimV1};
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOriginV1, FunctionOwnerIdV1, SemanticOwnerSourceKindV1, SourceExprSiteV1,
    SourceStmtSiteV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CarrierTransferV1 {
    NestedWriteToPostLoopRead,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CarrierProofRejectV1 {
    RoleCount,
    MissingRole,
    DuplicateRole,
    UnsupportedRole,
    BindingRelationMismatch,
    AncestryMismatch,
    AnchorMismatch,
    ForestFrameMismatch,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedGenericCarrierProofV1 {
    owner: FunctionOwnerIdV1,
    function_origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    root_site: SourceStmtSiteV1,
    loop_site: SourceStmtSiteV1,
    nested_write_site: SourceExprSiteV1,
    post_loop_read_site: SourceExprSiteV1,
    binding: BindingRefV1,
    transfer: CarrierTransferV1,
    _seal: CarrierProofSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct CarrierProofSealV1;

impl VerifiedGenericCarrierProofV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn function_origin(&self) -> FunctionOriginV1 {
        self.function_origin
    }

    pub(crate) const fn source_kind(&self) -> SemanticOwnerSourceKindV1 {
        self.source_kind
    }

    pub(crate) fn root_site(&self) -> &SourceStmtSiteV1 {
        &self.root_site
    }

    pub(crate) fn loop_site(&self) -> &SourceStmtSiteV1 {
        &self.loop_site
    }

    pub(crate) fn nested_write_site(&self) -> &SourceExprSiteV1 {
        &self.nested_write_site
    }

    pub(crate) fn post_loop_read_site(&self) -> &SourceExprSiteV1 {
        &self.post_loop_read_site
    }

    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(crate) const fn transfer(&self) -> CarrierTransferV1 {
        self.transfer
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedGenericCarrierProofHandoffV1 {
    lease: GenericSourceLeaseV1,
    proof: VerifiedGenericCarrierProofV1,
}

impl VerifiedGenericCarrierProofHandoffV1 {
    pub(crate) fn lease(&self) -> &GenericSourceLeaseV1 {
        &self.lease
    }

    pub(crate) fn proof(&self) -> &VerifiedGenericCarrierProofV1 {
        &self.proof
    }
}

pub(crate) fn issue_carrier_proof_v1(
    lease: GenericSourceLeaseV1,
) -> Result<VerifiedGenericCarrierProofHandoffV1, CarrierProofRejectV1> {
    if lease.roles().len() != 2 || lease.forest().members().len() != 2 {
        return Err(CarrierProofRejectV1::RoleCount);
    }
    if lease.frames().len() != lease.forest().members().len() {
        return Err(CarrierProofRejectV1::ForestFrameMismatch);
    }
    for (member, frame) in lease.forest().members().iter().zip(lease.frames()) {
        if member.source().site() != frame.site()
            || !member.source().frame_key().matches(frame.frame())
        {
            return Err(CarrierProofRejectV1::ForestFrameMismatch);
        }
    }

    let (nested_write, post_loop_read) = role_pair(lease.roles())?;
    if nested_write.ancestry() != GenericSourceAncestryV1::StrictAncestor {
        return Err(CarrierProofRejectV1::AncestryMismatch);
    }
    if !anchors_match(&lease, nested_write) || !anchors_match(&lease, post_loop_read) {
        return Err(CarrierProofRejectV1::AnchorMismatch);
    }
    if nested_write.binding() != post_loop_read.binding() {
        return Err(CarrierProofRejectV1::BindingRelationMismatch);
    }

    let proof = VerifiedGenericCarrierProofV1 {
        owner: lease.owner(),
        function_origin: lease.function_origin(),
        source_kind: lease.source_kind(),
        root_site: lease.root_site().clone(),
        loop_site: lease.loop_site().clone(),
        nested_write_site: nested_write.site().clone(),
        post_loop_read_site: post_loop_read.site().clone(),
        binding: nested_write.binding(),
        transfer: CarrierTransferV1::NestedWriteToPostLoopRead,
        _seal: CarrierProofSealV1,
    };
    Ok(VerifiedGenericCarrierProofHandoffV1 { lease, proof })
}

fn role_pair(
    roles: &[GenericSourceRoleClaimV1],
) -> Result<(&GenericSourceRoleClaimV1, &GenericSourceRoleClaimV1), CarrierProofRejectV1> {
    let mut nested_write = None;
    let mut post_loop_read = None;
    for role in roles {
        match role.kind() {
            super::GenericSourceRoleKindV1::NestedWrite => {
                if nested_write.replace(role).is_some() {
                    return Err(CarrierProofRejectV1::DuplicateRole);
                }
            }
            super::GenericSourceRoleKindV1::PostLoopRead => {
                if post_loop_read.replace(role).is_some() {
                    return Err(CarrierProofRejectV1::DuplicateRole);
                }
            }
            super::GenericSourceRoleKindV1::Unknown => {
                return Err(CarrierProofRejectV1::UnsupportedRole)
            }
        }
    }
    Ok((
        nested_write.ok_or(CarrierProofRejectV1::MissingRole)?,
        post_loop_read.ok_or(CarrierProofRejectV1::MissingRole)?,
    ))
}

fn anchors_match(lease: &GenericSourceLeaseV1, role: &GenericSourceRoleClaimV1) -> bool {
    role.root_anchor() == lease.root_site() && role.loop_anchor() == lease.loop_site()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::resolved_semantics::generic_resolved_carrier_source_lease::tests as lease_tests;

    const BINDING_MISMATCH_SOURCE: &str = r#"
function generic_both_mismatch(i, j) {
    loop(i < 3) {
        loop(j < 3) {
            j = j + 1
        }
        i = i + 1
    }
    return i
}
"#;

    fn positive_handoff() -> VerifiedGenericCarrierProofHandoffV1 {
        let unit = lease_tests::unit(lease_tests::SOURCE);
        let (input, root) = lease_tests::input_and_root(&unit);
        let lease = lease_tests::positive_lease(input, &root);
        issue_carrier_proof_v1(lease).expect("carrier proof")
    }

    #[test]
    fn carrier_proof_keeps_lease_brand_and_same_binding_transfer() {
        let handoff = positive_handoff();
        let proof = handoff.proof();
        assert_eq!(proof.owner(), handoff.lease().owner());
        assert_eq!(proof.function_origin(), handoff.lease().function_origin());
        assert_eq!(proof.source_kind(), handoff.lease().source_kind());
        assert_eq!(proof.binding(), handoff.lease().roles()[0].binding());
        assert_eq!(
            proof.transfer(),
            CarrierTransferV1::NestedWriteToPostLoopRead
        );
        assert_eq!(handoff.lease().roles().len(), 2);
        assert_ne!(proof.nested_write_site(), proof.post_loop_read_site());
    }

    #[test]
    fn carrier_proof_has_no_source_lifetime_after_unit_drop() {
        let handoff = positive_handoff();
        assert_eq!(handoff.proof().root_site(), handoff.lease().root_site());
        assert_eq!(handoff.proof().loop_site(), handoff.lease().loop_site());
    }

    #[test]
    fn carrier_proof_rejects_binding_relation_mismatch_before_publication() {
        let unit = lease_tests::unit(BINDING_MISMATCH_SOURCE);
        let (input, root) = lease_tests::input_and_root(&unit);
        let lease = lease_tests::positive_lease(input, &root);
        let error = issue_carrier_proof_v1(lease);
        assert_eq!(error, Err(CarrierProofRejectV1::BindingRelationMismatch));
    }
}
