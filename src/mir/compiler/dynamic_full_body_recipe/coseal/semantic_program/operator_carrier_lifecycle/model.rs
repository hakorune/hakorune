use crate::mir::dynamic_invocation_contract::DynamicInvocationInputHomeV1;
use crate::mir::dynamic_operator_contract::VerifiedDynamicOperatorExecutionEnvelopeV1;
use crate::mir::loop_recipe_contract::{
    LoopBindingKeyV1, LoopItemKeyV1, LoopNodeKeyV1, LoopValueKeyV1,
};
use crate::mir::resolved_semantics::{BindingRefV1, SourceExprSiteV1, SourceStmtSiteV1};

pub(super) const OPERATOR_CARRIER_LIFECYCLE_COUNT_V1: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum DynamicOperatorCarrierPublicationV1 {
    OnNormalResultPublication,
}

#[derive(Debug)]
pub(super) enum DynamicOperatorCarrierDestinationV1 {
    EndAfterInvocationNormalOrFaultOutcome {
        invocation: LoopItemKeyV1,
        argument_ordinal: u32,
        input_contract: DynamicInvocationInputHomeV1,
    },
    ForwardToBindingAtRebindCommit {
        write: LoopItemKeyV1,
        binding: LoopBindingKeyV1,
        source_binding: BindingRefV1,
        assignment_source: SourceStmtSiteV1,
        target_source: SourceExprSiteV1,
        backedge_loop: LoopNodeKeyV1,
    },
}

#[derive(Debug)]
pub(super) struct DynamicOperatorCarrierLifecycleRowV1 {
    pub(super) producer: LoopItemKeyV1,
    pub(super) producer_source: SourceExprSiteV1,
    pub(super) operands: [LoopValueKeyV1; 2],
    pub(super) result: LoopValueKeyV1,
    pub(super) publication: DynamicOperatorCarrierPublicationV1,
    pub(super) contract: &'static VerifiedDynamicOperatorExecutionEnvelopeV1,
    pub(super) destination: DynamicOperatorCarrierDestinationV1,
}

#[derive(Debug)]
pub(in crate::mir) struct DynamicOperatorCarrierLifecycleRowRefV1<'program> {
    pub(super) row: &'program DynamicOperatorCarrierLifecycleRowV1,
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) enum DynamicOperatorCarrierDestinationRefV1<'program> {
    EndAfterInvocationNormalOrFaultOutcome {
        invocation: LoopItemKeyV1,
        argument_ordinal: u32,
        input_contract: DynamicInvocationInputHomeV1,
    },
    ForwardToBindingAtRebindCommit {
        write: LoopItemKeyV1,
        binding: LoopBindingKeyV1,
        source_binding: BindingRefV1,
        assignment_source: &'program SourceStmtSiteV1,
        target_source: &'program SourceExprSiteV1,
        backedge_loop: LoopNodeKeyV1,
    },
}

impl DynamicOperatorCarrierLifecycleRowRefV1<'_> {
    pub(in crate::mir) const fn producer(&self) -> LoopItemKeyV1 {
        self.row.producer
    }

    pub(in crate::mir) const fn producer_source(&self) -> &SourceExprSiteV1 {
        &self.row.producer_source
    }

    pub(in crate::mir) const fn operands(&self) -> [LoopValueKeyV1; 2] {
        self.row.operands
    }

    pub(in crate::mir) const fn result(&self) -> LoopValueKeyV1 {
        self.row.result
    }

    pub(in crate::mir) const fn publication(&self) -> DynamicOperatorCarrierPublicationV1 {
        self.row.publication
    }

    pub(in crate::mir) const fn contract(
        &self,
    ) -> &'static VerifiedDynamicOperatorExecutionEnvelopeV1 {
        self.row.contract
    }

    pub(in crate::mir) const fn destination(&self) -> DynamicOperatorCarrierDestinationRefV1<'_> {
        match &self.row.destination {
            DynamicOperatorCarrierDestinationV1::EndAfterInvocationNormalOrFaultOutcome {
                invocation,
                argument_ordinal,
                input_contract,
            } => DynamicOperatorCarrierDestinationRefV1::EndAfterInvocationNormalOrFaultOutcome {
                invocation: *invocation,
                argument_ordinal: *argument_ordinal,
                input_contract: *input_contract,
            },
            DynamicOperatorCarrierDestinationV1::ForwardToBindingAtRebindCommit {
                write,
                binding,
                source_binding,
                assignment_source,
                target_source,
                backedge_loop,
            } => DynamicOperatorCarrierDestinationRefV1::ForwardToBindingAtRebindCommit {
                write: *write,
                binding: *binding,
                source_binding: *source_binding,
                assignment_source,
                target_source,
                backedge_loop: *backedge_loop,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum DynamicOperatorCarrierLifecycleProgramRejectV1 {
    Coverage,
    SourceRelation,
    RecipeRelation,
    OperatorContract,
    InvocationRelation,
    FaultRelation,
    BackedgeRelation,
}

#[derive(Debug)]
pub(in crate::mir::compiler::dynamic_full_body_recipe::coseal::semantic_program) struct VerifiedDynamicOperatorCarrierLifecycleCatalogV1
{
    pub(super) rows: [DynamicOperatorCarrierLifecycleRowV1; OPERATOR_CARRIER_LIFECYCLE_COUNT_V1],
}

#[derive(Debug)]
pub(in crate::mir) struct DynamicOperatorCarrierLifecycleCatalogRefV1<'program> {
    rows: &'program [DynamicOperatorCarrierLifecycleRowV1; OPERATOR_CARRIER_LIFECYCLE_COUNT_V1],
}

impl<'program> DynamicOperatorCarrierLifecycleCatalogRefV1<'program> {
    pub(in crate::mir) fn rows(
        &self,
    ) -> impl ExactSizeIterator<Item = DynamicOperatorCarrierLifecycleRowRefV1<'program>> + '_ {
        self.rows
            .iter()
            .map(|row| DynamicOperatorCarrierLifecycleRowRefV1 { row })
    }
}

impl VerifiedDynamicOperatorCarrierLifecycleCatalogV1 {
    pub(in crate::mir::compiler::dynamic_full_body_recipe::coseal::semantic_program) const fn borrow(
        &self,
    ) -> DynamicOperatorCarrierLifecycleCatalogRefV1<'_> {
        DynamicOperatorCarrierLifecycleCatalogRefV1 { rows: &self.rows }
    }
}
