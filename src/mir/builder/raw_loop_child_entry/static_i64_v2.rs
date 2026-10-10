//! Select the original Static I64 Loop before the generic V1 facts issuer.
//! The producer is semantic-only; this route stays closed until its Home and
//! physical SSA session can consume the verified V2 product.

use super::super::normal_callable_loop_source_facts::VerifiedStaticI64LoopSemanticV2;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1;
use crate::mir::normal_callable_semantic_package::PhysicalCallableLaneRoleV1;
use crate::mir::normal_callable_semantic_package::ResolvedCallablePhysicalSignatureLoanV1;
use crate::mir::normal_callable_semantic_package::VerifiedStaticLoopTaggedEntrySourceV1;
use crate::mir::resolved_semantics::SourceStmtSiteV1;

#[derive(Debug)]
pub(in crate::mir) struct StaticI64LoopFunctionEntryV2 {
    semantic: VerifiedStaticI64LoopSemanticV2,
    tagged: VerifiedStaticLoopTaggedEntrySourceV1,
}

impl StaticI64LoopFunctionEntryV2 {
    pub(in crate::mir) fn from_verified_source(
        semantic: VerifiedStaticI64LoopSemanticV2,
        tagged: VerifiedStaticLoopTaggedEntrySourceV1,
    ) -> Self {
        Self { semantic, tagged }
    }

    pub(in crate::mir) fn semantic(&self) -> &VerifiedStaticI64LoopSemanticV2 {
        &self.semantic
    }

    pub(in crate::mir) fn home_source(
        &self,
    ) -> crate::mir::builder::VerifiedStaticI64LoopHomeSourceV2 {
        self.semantic.home_source()
    }

    pub(in crate::mir) fn tagged_formal(&self) -> crate::mir::resolved_semantics::BindingRefV1 {
        self.tagged.formal()
    }

    pub(in crate::mir) fn packet_source_inputs(
        &self,
    ) -> (
        &crate::mir::normal_callable_semantic_package::LoopEntryStaticI64SourceLoanV1,
        &VerifiedStaticLoopTaggedEntrySourceV1,
    ) {
        (self.semantic.source_calls().0, &self.tagged)
    }

    /// Join the package's source proof to its one physical formal lane. The
    /// OrdinaryScalar role alone does not authorize a tagged carrier.
    pub(in crate::mir) fn join_physical_signature_v2(
        &self,
        signature: &ResolvedCallablePhysicalSignatureLoanV1<'_>,
    ) -> Result<StaticI64LoopTaggedPhysicalFormalV2, String> {
        use crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1;
        let reject =
            || "[freeze:contract][callable-loop/static-tagged-signature-mismatch]".to_owned();
        let formal = self.tagged.formal();
        let [lane] = signature.lanes() else {
            return Err(reject());
        };
        if signature.owner() != formal.owner()
            || signature.mode() != CallableParameterDeclarationModeV1::StaticBoxMethod
            || signature.source_logical_arity() != 1
            || signature.receiver_lane_count() != 0
            || signature.physical_formal_lane_count() != 1
            || signature.physical_callable_lane_count() != 1
            || lane.index() != 0
            || lane.logical_ordinal() != Some(0)
            || lane.role() != PhysicalCallableLaneRoleV1::OrdinaryScalar
            || lane.binding() != formal
        {
            return Err(reject());
        }
        Ok(StaticI64LoopTaggedPhysicalFormalV2 {
            formal,
            lane_index: lane.index(),
        })
    }
}

/// One joined source/signature representation request; no ValueId or packet.
#[derive(Debug)]
pub(in crate::mir) struct StaticI64LoopTaggedPhysicalFormalV2 {
    formal: crate::mir::resolved_semantics::BindingRefV1,
    lane_index: u32,
}

impl StaticI64LoopTaggedPhysicalFormalV2 {
    pub(in crate::mir) const fn formal(&self) -> crate::mir::resolved_semantics::BindingRefV1 {
        self.formal
    }

    pub(in crate::mir) const fn lane_index(&self) -> u32 {
        self.lane_index
    }

    pub(in crate::mir) const fn carrier(
        &self,
    ) -> crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1
    {
        crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1::BorrowedTaggedValue
    }
}

pub(in crate::mir) fn take_at_function_entry_v2(
    input: ResolvedFunctionLoweringInputV1<'_>,
    claims: &OrdinaryNewClaimLedgerV1,
) -> Result<Option<StaticI64LoopFunctionEntryV2>, String> {
    let mut selected = input
        .function()
        .loop_sites()
        .filter(|site| claims.expects_loop_static_source_loan_v1(input.owner(), site));
    let Some(loop_site) = selected.next() else {
        return Ok(None);
    };
    if selected.next().is_some() {
        return Err(
            "[freeze:contract][callable-loop/static-i64-v2/duplicate-selected-loop]".to_owned(),
        );
    }
    take_selected_semantic_product(input, claims, loop_site)?
        .ok_or_else(|| {
            "[freeze:contract][callable-loop/static-i64-v2/source-unavailable]".to_owned()
        })
        .map(Some)
}

pub(in crate::mir) fn stop_after_selected_semantic_product(
    input: ResolvedFunctionLoweringInputV1<'_>,
    claims: &OrdinaryNewClaimLedgerV1,
    loop_site: &SourceStmtSiteV1,
) -> Result<(), String> {
    if take_selected_semantic_product(input, claims, loop_site)?.is_some() {
        return Err(
            "[freeze:contract][callable-loop/static-i64-v2/physical-unavailable]".to_owned(),
        );
    }
    Ok(())
}

fn take_selected_semantic_product(
    input: ResolvedFunctionLoweringInputV1<'_>,
    claims: &OrdinaryNewClaimLedgerV1,
    loop_site: &SourceStmtSiteV1,
) -> Result<Option<StaticI64LoopFunctionEntryV2>, String> {
    if !claims.expects_loop_static_source_loan_v1(input.owner(), loop_site) {
        return Ok(None);
    }
    let reject = || "[freeze:contract][callable-loop/static-i64-v2/source-unavailable]".to_owned();
    let product = claims
        .take_preissued_static_i64_product_v2(input.owner())
        .ok_or_else(reject)??;
    if product.semantic().roles().loop_site != *loop_site {
        return Err(reject());
    }
    Ok(Some(product))
}
