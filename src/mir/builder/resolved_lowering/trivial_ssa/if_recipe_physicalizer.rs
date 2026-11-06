//! Physical consumer for the selected fixed-shell If demand.
//!
//! The portable artifact and JoinSig are consumed here, exactly once.  They
//! prove the source correspondence and fixed logical topology; the existing
//! canonical function session remains the only CFG/SSA/PHI writer.  Leaf
//! expressions and branch bodies are still emitted through the lowerer's
//! immutable source view.

use crate::mir::compiler::located::LocatedStmtV1;
use crate::mir::if_recipe_contract::{
    IfJoinEdgeRoleV1, IfJoinPortV1, IfSourcePathStepV1, IfValueClassV1,
};
use crate::mir::resolved_semantics::{BindingRefV1, SourcePathSegmentV1};
use crate::mir::resolved_value_profile::product::TrivialRepresentationV1;
use crate::mir::{BasicBlockId, ValueId};

use super::super::if_recipe_adapter::{
    CanonicalIfPhysicalCorrespondenceV1, CanonicalIfPhysicalDemandV1,
};
use super::lowerer::CanonicalTrivialSsaLowererV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct CanonicalIfPhysicalSuccessV1;

/// The only topology capabilities the selected physicalizer can mint. They are
/// created after the demand/JoinSig proof, so selected lowering cannot accept
/// an arbitrary bool or re-select topology from source syntax.
#[derive(Debug)]
pub(super) struct CanonicalIfRecipeExplicitElseTopologyV1 {
    binding: BindingRefV1,
}

impl CanonicalIfRecipeExplicitElseTopologyV1 {
    pub(super) const fn new(binding: BindingRefV1) -> Self {
        Self { binding }
    }

    pub(super) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }
}

#[derive(Debug)]
pub(super) struct CanonicalIfRecipeImplicitFallthroughTopologyV1 {
    binding: BindingRefV1,
}

impl CanonicalIfRecipeImplicitFallthroughTopologyV1 {
    pub(super) const fn new(binding: BindingRefV1) -> Self {
        Self { binding }
    }

    pub(super) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }
}

#[derive(Debug)]
pub(super) enum CanonicalIfRecipeTopologyV1 {
    ExplicitElse(CanonicalIfRecipeExplicitElseTopologyV1),
    ImplicitFallthrough(CanonicalIfRecipeImplicitFallthroughTopologyV1),
}

impl CanonicalIfRecipeTopologyV1 {
    pub(super) const fn binding(&self) -> BindingRefV1 {
        match self {
            Self::ExplicitElse(topology) => topology.binding(),
            Self::ImplicitFallthrough(topology) => topology.binding(),
        }
    }

    pub(super) const fn is_explicit_else(&self) -> bool {
        matches!(self, Self::ExplicitElse(_))
    }
}

#[derive(Debug)]
pub(super) struct CanonicalIfPhysicalReceiptExplicitElseV1 {
    header: BasicBlockId,
    condition: ValueId,
    then_block: BasicBlockId,
    else_block: BasicBlockId,
    then_exit: BasicBlockId,
    else_exit: BasicBlockId,
    merge: BasicBlockId,
    predecessors: [BasicBlockId; 2],
    values: [ValueId; 2],
    binding: BindingRefV1,
}

#[derive(Debug)]
pub(super) struct CanonicalIfPhysicalReceiptImplicitElseV1 {
    header: BasicBlockId,
    condition: ValueId,
    then_block: BasicBlockId,
    then_exit: BasicBlockId,
    merge: BasicBlockId,
    predecessors: [BasicBlockId; 2],
    values: [ValueId; 2],
    binding: BindingRefV1,
}

#[derive(Debug)]
pub(super) enum CanonicalIfPhysicalReceiptV1 {
    ExplicitElse(CanonicalIfPhysicalReceiptExplicitElseV1),
    ImplicitFallthrough(CanonicalIfPhysicalReceiptImplicitElseV1),
}

pub(super) enum CanonicalIfPhysicalValuesV1 {
    ExplicitElse {
        else_block: BasicBlockId,
        else_exit: BasicBlockId,
        else_value: ValueId,
    },
    ImplicitFallthrough {
        baseline_value: ValueId,
    },
}

impl CanonicalIfPhysicalReceiptV1 {
    pub(super) fn explicit(
        header: BasicBlockId,
        condition: ValueId,
        then_block: BasicBlockId,
        else_block: BasicBlockId,
        then_exit: BasicBlockId,
        else_exit: BasicBlockId,
        merge: BasicBlockId,
        then_value: ValueId,
        else_value: ValueId,
        binding: BindingRefV1,
    ) -> Self {
        Self::ExplicitElse(CanonicalIfPhysicalReceiptExplicitElseV1::new(
            header, condition, then_block, else_block, then_exit, else_exit, merge, then_value,
            else_value, binding,
        ))
    }

    pub(super) fn implicit(
        header: BasicBlockId,
        condition: ValueId,
        then_block: BasicBlockId,
        then_exit: BasicBlockId,
        merge: BasicBlockId,
        then_value: ValueId,
        baseline_value: ValueId,
        binding: BindingRefV1,
    ) -> Self {
        Self::ImplicitFallthrough(CanonicalIfPhysicalReceiptImplicitElseV1::new(
            header,
            condition,
            then_block,
            then_exit,
            merge,
            then_value,
            baseline_value,
            binding,
        ))
    }
}

pub(super) fn selected_receipt(
    topology: CanonicalIfRecipeTopologyV1,
    header: BasicBlockId,
    condition: ValueId,
    then_block: BasicBlockId,
    then_exit: BasicBlockId,
    merge: BasicBlockId,
    then_value: ValueId,
    values: CanonicalIfPhysicalValuesV1,
) -> Result<CanonicalIfPhysicalReceiptV1, String> {
    match topology {
        CanonicalIfRecipeTopologyV1::ExplicitElse(topology) => {
            let CanonicalIfPhysicalValuesV1::ExplicitElse {
                else_block,
                else_exit,
                else_value,
            } = values
            else {
                return Err("[freeze:contract][if_recipe/explicit_receipt_missing]".to_string());
            };
            Ok(CanonicalIfPhysicalReceiptV1::explicit(
                header,
                condition,
                then_block,
                else_block,
                then_exit,
                else_exit,
                merge,
                then_value,
                else_value,
                topology.binding(),
            ))
        }
        CanonicalIfRecipeTopologyV1::ImplicitFallthrough(topology) => {
            let CanonicalIfPhysicalValuesV1::ImplicitFallthrough { baseline_value } = values else {
                return Err(
                    "[freeze:contract][if_recipe/implicit_else_receipt_has_else]".to_string(),
                );
            };
            Ok(CanonicalIfPhysicalReceiptV1::implicit(
                header,
                condition,
                then_block,
                then_exit,
                merge,
                then_value,
                baseline_value,
                topology.binding(),
            ))
        }
    }
}

impl CanonicalIfPhysicalReceiptExplicitElseV1 {
    pub(super) fn new(
        header: BasicBlockId,
        condition: ValueId,
        then_block: BasicBlockId,
        else_block: BasicBlockId,
        then_exit: BasicBlockId,
        else_exit: BasicBlockId,
        merge: BasicBlockId,
        then_value: ValueId,
        else_value: ValueId,
        binding: BindingRefV1,
    ) -> Self {
        Self {
            header,
            condition,
            then_block,
            else_block,
            then_exit,
            else_exit,
            merge,
            predecessors: [then_exit, else_exit],
            values: [then_value, else_value],
            binding,
        }
    }

    pub(super) const fn header(&self) -> BasicBlockId {
        self.header
    }

    pub(super) const fn condition(&self) -> ValueId {
        self.condition
    }

    pub(super) const fn then_block(&self) -> BasicBlockId {
        self.then_block
    }

    pub(super) const fn else_block(&self) -> BasicBlockId {
        self.else_block
    }

    pub(super) const fn then_exit(&self) -> BasicBlockId {
        self.then_exit
    }

    pub(super) const fn else_exit(&self) -> BasicBlockId {
        self.else_exit
    }

    pub(super) const fn merge(&self) -> BasicBlockId {
        self.merge
    }

    pub(super) const fn predecessors(&self) -> [BasicBlockId; 2] {
        self.predecessors
    }

    pub(super) const fn values(&self) -> [ValueId; 2] {
        self.values
    }

    pub(super) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }
}

impl CanonicalIfPhysicalReceiptImplicitElseV1 {
    pub(super) fn new(
        header: BasicBlockId,
        condition: ValueId,
        then_block: BasicBlockId,
        then_exit: BasicBlockId,
        merge: BasicBlockId,
        then_value: ValueId,
        baseline_value: ValueId,
        binding: BindingRefV1,
    ) -> Self {
        Self {
            header,
            condition,
            then_block,
            then_exit,
            merge,
            predecessors: [header, then_exit],
            values: [baseline_value, then_value],
            binding,
        }
    }

    pub(super) const fn header(&self) -> BasicBlockId {
        self.header
    }

    pub(super) const fn condition(&self) -> ValueId {
        self.condition
    }

    pub(super) const fn then_block(&self) -> BasicBlockId {
        self.then_block
    }

    pub(super) const fn then_exit(&self) -> BasicBlockId {
        self.then_exit
    }

    pub(super) const fn merge(&self) -> BasicBlockId {
        self.merge
    }

    pub(super) const fn predecessors(&self) -> [BasicBlockId; 2] {
        self.predecessors
    }

    pub(super) const fn values(&self) -> [ValueId; 2] {
        self.values
    }

    pub(super) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }
}

pub(super) fn physicalize_if_recipe_v1<'builder, 'source>(
    lowerer: &mut CanonicalTrivialSsaLowererV1<'builder, 'source>,
    statement: &LocatedStmtV1<'source>,
    demand: CanonicalIfPhysicalDemandV1,
) -> Result<CanonicalIfPhysicalSuccessV1, String> {
    let (physical_input, correspondence) = demand.into_parts();
    let (artifact, join_sig) = physical_input.into_parts();
    verify_demand(statement, &correspondence, &artifact, join_sig.as_sig())?;
    let topology = if correspondence.is_explicit_else() {
        CanonicalIfRecipeTopologyV1::ExplicitElse(CanonicalIfRecipeExplicitElseTopologyV1::new(
            correspondence.entry_binding(),
        ))
    } else {
        CanonicalIfRecipeTopologyV1::ImplicitFallthrough(
            CanonicalIfRecipeImplicitFallthroughTopologyV1::new(correspondence.entry_binding()),
        )
    };
    let receipt = lowerer.lower_if_recipe_selected(statement, topology)?;
    verify_physical_receipt(&correspondence, join_sig.as_sig(), &receipt)?;
    Ok(CanonicalIfPhysicalSuccessV1)
}

fn verify_physical_receipt(
    correspondence: &CanonicalIfPhysicalCorrespondenceV1,
    join_sig: &crate::mir::if_recipe_contract::IfJoinSigV1,
    receipt: &CanonicalIfPhysicalReceiptV1,
) -> Result<(), String> {
    match (join_sig.disposition, receipt) {
        (
            crate::mir::if_recipe_contract::IfElseDispositionV1::Explicit,
            CanonicalIfPhysicalReceiptV1::ExplicitElse(receipt),
        ) => {
            if receipt.binding() != correspondence.entry_binding()
                || receipt.predecessors() != [receipt.then_exit(), receipt.else_exit()]
                || receipt
                    .values()
                    .iter()
                    .any(|value| *value == receipt.condition())
                || receipt.then_exit() != receipt.then_block()
                || receipt.else_exit() != receipt.else_block()
                || receipt.then_block() == receipt.else_block()
            {
                return Err("[freeze:contract][if_recipe/physical_receipt_mismatch]".to_string());
            }
            let blocks = [
                receipt.header(),
                receipt.then_block(),
                receipt.else_block(),
                receipt.merge(),
            ];
            if blocks
                .iter()
                .enumerate()
                .any(|(index, block)| blocks[..index].contains(block))
            {
                return Err("[freeze:contract][if_recipe/physical_blocks_overlap]".to_string());
            }
            if !correspondence.is_explicit_else() {
                return Err(
                    "[freeze:contract][if_recipe/correspondence_shape_mismatch]".to_string()
                );
            }
        }
        (
            crate::mir::if_recipe_contract::IfElseDispositionV1::ImplicitFallthrough,
            CanonicalIfPhysicalReceiptV1::ImplicitFallthrough(receipt),
        ) => {
            if receipt.binding() != correspondence.entry_binding()
                || receipt.predecessors() != [receipt.header(), receipt.then_exit()]
                || receipt.values()[1] == receipt.condition()
                || receipt.then_exit() != receipt.then_block()
                || receipt.header() == receipt.then_block()
                || receipt.header() == receipt.merge()
                || receipt.then_block() == receipt.merge()
            {
                return Err("[freeze:contract][if_recipe/implicit_receipt_mismatch]".to_string());
            }
            if correspondence.is_explicit_else() {
                return Err(
                    "[freeze:contract][if_recipe/correspondence_shape_mismatch]".to_string()
                );
            }
        }
        _ => return Err("[freeze:contract][if_recipe/receipt_shape_mismatch]".to_string()),
    }
    if join_sig.edges[3].to != IfJoinPortV1::Continuation
        || join_sig.edges[4].to != IfJoinPortV1::Continuation
    {
        return Err("[freeze:contract][if_recipe/physical_join_mismatch]".to_string());
    }
    Ok(())
}

fn verify_demand(
    statement: &LocatedStmtV1<'_>,
    correspondence: &CanonicalIfPhysicalCorrespondenceV1,
    artifact: &crate::mir::if_recipe_contract::VerifiedIfRecipeArtifactV1,
    join_sig: &crate::mir::if_recipe_contract::IfJoinSigV1,
) -> Result<(), String> {
    if statement.site() != correspondence.if_site() {
        return Err("[freeze:contract][if_recipe/source_site_mismatch]".to_string());
    }
    if correspondence.entry_binding().owner() != statement.owner() {
        return Err("[freeze:contract][if_recipe/binding_owner_mismatch]".to_string());
    }
    let Some(SourcePathSegmentV1::Body(root_index)) =
        correspondence.if_site().node().segments().first()
    else {
        return Err("[freeze:contract][if_recipe/if_site_not_root_body]".to_string());
    };
    if correspondence.if_site().node().segments().len() != 1 {
        return Err("[freeze:contract][if_recipe/nested_if_site]".to_string());
    }
    let Some(IfSourcePathStepV1::BodyItem { index }) = artifact
        .source_binding()
        .as_source_binding()
        .claims
        .first()
        .and_then(|claim| claim.path.steps.first())
    else {
        return Err("[freeze:contract][if_recipe/source_claim_missing]".to_string());
    };
    if index != root_index {
        return Err("[freeze:contract][if_recipe/source_claim_site_mismatch]".to_string());
    }
    let expected_ports = if correspondence.is_explicit_else() {
        [
            IfJoinPortV1::Entry,
            IfJoinPortV1::Condition,
            IfJoinPortV1::Then,
            IfJoinPortV1::Else,
            IfJoinPortV1::Continuation,
        ]
    } else {
        [
            IfJoinPortV1::Entry,
            IfJoinPortV1::Condition,
            IfJoinPortV1::Then,
            IfJoinPortV1::Baseline,
            IfJoinPortV1::Continuation,
        ]
    };
    if join_sig.ports != expected_ports {
        return Err("[freeze:contract][if_recipe/logical_ports_mismatch]".to_string());
    }
    let edges = &join_sig.edges;
    if edges[0].role != IfJoinEdgeRoleV1::Enter
        || edges[1].role != IfJoinEdgeRoleV1::True
        || edges[2].role != IfJoinEdgeRoleV1::False
        || edges[3].role != IfJoinEdgeRoleV1::ThenTransfer
        || edges[4].role
            != if correspondence.is_explicit_else() {
                IfJoinEdgeRoleV1::ElseTransfer
            } else {
                IfJoinEdgeRoleV1::ImplicitBaseline
            }
        || edges[3].to != IfJoinPortV1::Continuation
        || edges[4].to != IfJoinPortV1::Continuation
    {
        return Err("[freeze:contract][if_recipe/logical_edges_mismatch]".to_string());
    }
    if edges[1].value.value != artifact.recipe().as_recipe().condition
        || edges[2].value.value != artifact.recipe().as_recipe().condition
        || edges[1].value.class != IfValueClassV1::Bool
        || edges[2].value.class != IfValueClassV1::Bool
    {
        return Err("[freeze:contract][if_recipe/condition_mapping_mismatch]".to_string());
    }
    if class_for_representation(correspondence.representation())? != join_sig.join.class {
        return Err("[freeze:contract][if_recipe/join_class_mismatch]".to_string());
    }
    let recipe_join = artifact
        .recipe()
        .as_recipe()
        .joins
        .first()
        .ok_or_else(|| "[freeze:contract][if_recipe/join_row_missing]".to_string())?;
    if recipe_join.binding != join_sig.join.binding
        || recipe_join.entry_value != join_sig.join.entry_value
        || recipe_join.then_value != join_sig.join.then_value
        || recipe_join.else_value != join_sig.join.else_value
    {
        return Err("[freeze:contract][if_recipe/join_pair_mismatch]".to_string());
    }
    if correspondence.condition() == correspondence.then_value()
        || correspondence.continuation_read() == correspondence.condition()
        || correspondence
            .else_value()
            .is_some_and(|value| correspondence.condition() == value)
        || correspondence
            .else_assignment()
            .is_some_and(|assignment| correspondence.then_assignment() == assignment)
    {
        return Err("[freeze:contract][if_recipe/source_roles_overlap]".to_string());
    }
    Ok(())
}

fn class_for_representation(
    representation: TrivialRepresentationV1,
) -> Result<IfValueClassV1, String> {
    match representation {
        TrivialRepresentationV1::InlineI64 => Ok(IfValueClassV1::I64),
        TrivialRepresentationV1::InlineBool => Ok(IfValueClassV1::Bool),
        _ => Err("[freeze:contract][if_recipe/unsupported_join_representation]".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::if_recipe_contract::{
        IfElseDispositionV1, IfJoinEdgeRoleV1, IfJoinEdgeV1, IfJoinObligationV1, IfJoinPortV1,
        IfJoinSigV1, IfJoinValueEdgeV1, IfValueKeyV1,
    };
    use crate::mir::resolved_semantics::{
        FunctionOwnerIssuerV1, SourceExprSiteV1, SourceNodeSiteV1, SourceStmtSiteV1,
    };
    use hakorune_mir_core::BindingId;

    fn binding() -> BindingRefV1 {
        let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().expect("owner issuer");
        BindingRefV1::new(issuer.issue().expect("owner"), BindingId::new(0))
    }

    fn stmt_site(index: u32) -> SourceStmtSiteV1 {
        SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
            SourcePathSegmentV1::Body(index),
        ]))
    }

    fn expr_site(index: u32, role: SourcePathSegmentV1) -> SourceExprSiteV1 {
        SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
            SourcePathSegmentV1::Body(index),
            role,
        ]))
    }

    fn explicit_correspondence(binding: BindingRefV1) -> CanonicalIfPhysicalCorrespondenceV1 {
        CanonicalIfPhysicalCorrespondenceV1::from_test_parts(
            stmt_site(1),
            expr_site(1, SourcePathSegmentV1::IfCondition),
            binding,
            TrivialRepresentationV1::InlineI64,
            stmt_site(2),
            expr_site(2, SourcePathSegmentV1::Value),
            Some(stmt_site(3)),
            Some(expr_site(3, SourcePathSegmentV1::Value)),
            expr_site(4, SourcePathSegmentV1::Value),
        )
    }

    fn explicit_join_sig() -> IfJoinSigV1 {
        let edge = |from, to, role| IfJoinEdgeV1 {
            from,
            to,
            role,
            value: IfJoinValueEdgeV1 {
                value: IfValueKeyV1::new(0),
                class: crate::mir::if_recipe_contract::IfValueClassV1::I64,
            },
        };
        IfJoinSigV1 {
            disposition: IfElseDispositionV1::Explicit,
            ports: [
                IfJoinPortV1::Entry,
                IfJoinPortV1::Condition,
                IfJoinPortV1::Then,
                IfJoinPortV1::Else,
                IfJoinPortV1::Continuation,
            ],
            edges: [
                edge(
                    IfJoinPortV1::Entry,
                    IfJoinPortV1::Condition,
                    IfJoinEdgeRoleV1::Enter,
                ),
                edge(
                    IfJoinPortV1::Condition,
                    IfJoinPortV1::Then,
                    IfJoinEdgeRoleV1::True,
                ),
                edge(
                    IfJoinPortV1::Condition,
                    IfJoinPortV1::Else,
                    IfJoinEdgeRoleV1::False,
                ),
                edge(
                    IfJoinPortV1::Then,
                    IfJoinPortV1::Continuation,
                    IfJoinEdgeRoleV1::ThenTransfer,
                ),
                edge(
                    IfJoinPortV1::Else,
                    IfJoinPortV1::Continuation,
                    IfJoinEdgeRoleV1::ElseTransfer,
                ),
            ],
            join: IfJoinObligationV1 {
                binding: crate::mir::if_recipe_contract::IfBindingKeyV1::new(0),
                class: crate::mir::if_recipe_contract::IfValueClassV1::I64,
                entry_value: IfValueKeyV1::new(0),
                then_value: IfValueKeyV1::new(1),
                else_value: IfValueKeyV1::new(2),
            },
        }
    }

    #[test]
    fn receipt_rejects_non_adjacent_block_overlap() {
        let binding = binding();
        let correspondence = explicit_correspondence(binding);
        let receipt = CanonicalIfPhysicalReceiptV1::explicit(
            BasicBlockId::new(0),
            ValueId::new(9),
            BasicBlockId::new(1),
            BasicBlockId::new(0),
            BasicBlockId::new(1),
            BasicBlockId::new(0),
            BasicBlockId::new(3),
            ValueId::new(10),
            ValueId::new(11),
            binding,
        );

        assert_eq!(
            verify_physical_receipt(&correspondence, &explicit_join_sig(), &receipt),
            Err("[freeze:contract][if_recipe/physical_blocks_overlap]".to_string())
        );
    }

    #[test]
    fn receipt_values_keep_explicit_and_implicit_topologies_typed() {
        let explicit = CanonicalIfRecipeTopologyV1::ExplicitElse(
            CanonicalIfRecipeExplicitElseTopologyV1::new(binding()),
        );
        assert!(selected_receipt(
            explicit,
            BasicBlockId::new(0),
            ValueId::new(1),
            BasicBlockId::new(2),
            BasicBlockId::new(2),
            BasicBlockId::new(3),
            ValueId::new(4),
            CanonicalIfPhysicalValuesV1::ImplicitFallthrough {
                baseline_value: ValueId::new(5),
            },
        )
        .is_err());

        let implicit = CanonicalIfRecipeTopologyV1::ImplicitFallthrough(
            CanonicalIfRecipeImplicitFallthroughTopologyV1::new(binding()),
        );
        assert!(selected_receipt(
            implicit,
            BasicBlockId::new(0),
            ValueId::new(1),
            BasicBlockId::new(2),
            BasicBlockId::new(2),
            BasicBlockId::new(3),
            ValueId::new(4),
            CanonicalIfPhysicalValuesV1::ExplicitElse {
                else_block: BasicBlockId::new(5),
                else_exit: BasicBlockId::new(5),
                else_value: ValueId::new(6),
            },
        )
        .is_err());
    }
}
