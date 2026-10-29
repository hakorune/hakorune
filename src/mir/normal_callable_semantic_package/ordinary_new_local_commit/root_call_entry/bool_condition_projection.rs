//! One original Bool condition and its real source-ordered physical producers.
//! A fixed prefix may be lent during branch descent; final users require the
//! same affine outer-Branch seal. No semantic permission is inferred from CFG.
use super::*;
use crate::mir::builder::ops::CompletedLogicalShortCircuitV1;
use crate::mir::builder::{
    ConditionMaterializationV1, ExactLexicalReadV1, IfConditionMaterializationV1,
    LocalSsaPhysicalRelationV1,
};
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
use crate::mir::normal_callable_semantic_package::EvaluatedExpressionValueV1;
use crate::mir::resolved_semantics::home_new_prefix::CheckedBoolConditionFrameV1;
use crate::mir::resolved_semantics::ResolvedBinaryOperatorV1;
use std::cell::RefCell;
use std::rc::Rc;
type Binding = (BasicBlockId, MirInstruction);
type Finished = super::super::super::physical_boundary::FinishedBindings;

#[path = "bool_condition_control.rs"]
mod control;
#[path = "bool_condition_source.rs"]
mod source;
pub(in crate::mir) use source::borrowed_source_loop_phi_relations_v2;

/// Cursors are observations at the existing recursive descent boundary.
/// They never select source children or issue a value/operation contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) struct BoolConditionCursorV1 {
    pub(in crate::mir) block: BasicBlockId,
    pub(in crate::mir) instruction: usize,
}
#[derive(Debug)]
pub(in crate::mir) enum BoolConditionValueProofV1 {
    Literal(Binding),
    Reference {
        read: ExactLexicalReadV1,
        producer: Option<Binding>,
    },
    OriginalReference(super::expression_projection::ExpressionReferenceReadV1),
    Field(Binding),
    /// The direct `me.<array>` receiver read and the checked Count read of one node.
    FieldArrayCount { field: Binding, read: Binding },
    Call {
        packet: Rc<EmittedLexicalCallProjectionV1>,
        bindings: Box<[Binding]>,
    },
    ScheduledSourceLoopCall {
        packet: Rc<EmittedLexicalCallProjectionV1>,
        bindings: Box<[Binding]>,
    },
    Binary {
        append: Binding,
        operands: [Rc<LocalSsaPhysicalRelationV1>; 2],
    },
    ScalarInvocation {
        receiver: Option<ExactLexicalReadV1>,
        input_values: Box<[Option<EvaluatedExpressionValueV1>]>,
        dynamic_calls: Box<[Rc<crate::mir::normal_callable_semantic_package::EmittedDynamicInvocationCallProjectionV1>]>,
        input_reads: std::collections::BTreeMap<
            usize,
            super::expression_projection::ExpressionReferenceReadV1,
        >,
        entry: Binding,
        terminal: Binding,
        continuation: Binding,
        bindings: Box<[Binding]>,
    },
    Logical {
        completed: CompletedLogicalShortCircuitV1,
        conditions: [Rc<LocalSsaPhysicalRelationV1>; 2],
    },
}
#[derive(Debug)]
pub(in crate::mir) struct BoolConditionNodeProjectionV1 {
    pub(in crate::mir) node: usize,
    pub(in crate::mir) entry: BoolConditionCursorV1,
    pub(in crate::mir) exit: BoolConditionCursorV1,
    pub(in crate::mir) value: ValueId,
    pub(in crate::mir) proof: BoolConditionValueProofV1,
}
/// Physical correspondence has one source purpose. The Loop arm retains only
/// immutable source identity, so scoped children cannot cycle to their parent.
#[derive(Debug)]
enum BoolConditionOuterV2 {
    If(Rc<IfConditionMaterializationV1>),
    SourceLoop {
        product: Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>,
        frame: usize,
        materialization: Rc<ConditionMaterializationV1>,
        targets: (BasicBlockId, BasicBlockId),
    },
}
impl BoolConditionOuterV2 {
    fn physical(&self) -> &Rc<ConditionMaterializationV1> {
        match self {
            Self::If(pending) => pending.materialization(),
            Self::SourceLoop {
                materialization, ..
            } => materialization,
        }
    }
    fn arms(&self) -> (BasicBlockId, BasicBlockId) {
        match self {
            Self::If(pending) => pending.arms(),
            Self::SourceLoop { targets, .. } => *targets,
        }
    }
    fn validate_source(
        &self,
        frame: &Rc<CheckedBoolConditionFrameV1>,
        context: CallValidationContextV1<'_>,
    ) -> Result<(), String> {
        use crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1 as Root;
        match self {
            Self::If(_) if matches!(frame.source().root(), Root::IfCondition { .. }) => Ok(()),
            Self::SourceLoop {
                product,
                frame: index,
                targets,
                ..
            } => {
                if targets.0 == targets.1 {
                    return Err(freeze("bool-condition/loop-branch-targets"));
                }
                context.validate_source_loop_bool_frame_v2(product, *index, frame)
            }
            _ => Err(freeze("bool-condition/if-source-purpose")),
        }
    }
}
#[derive(Debug)]
pub(in crate::mir) struct EmittedBoolConditionProjectionV1 {
    frame: Rc<CheckedBoolConditionFrameV1>,
    nodes: Box<[BoolConditionNodeProjectionV1]>,
    entry: Binding,
    prefix: Vec<Binding>,
    materialization: BoolConditionOuterV2,
    outer: RefCell<Option<Binding>>,
}

/// Sole writer lease. Dropping it without sealing leaves an unpublishable
/// Pending packet; no final consumer can turn absence into completion.
pub(in crate::mir) struct BoolConditionSealV1 {
    packet: Rc<EmittedBoolConditionProjectionV1>,
}
impl BoolConditionSealV1 {
    pub(in crate::mir) fn seal(
        self,
        function: &MirFunction,
        materialization: &Rc<IfConditionMaterializationV1>,
        branch: Binding,
    ) -> Result<(), String> {
        if !matches!(&self.packet.materialization, BoolConditionOuterV2::If(original) if Rc::ptr_eq(materialization, original))
            || self.packet.outer.borrow().is_some()
        {
            return Err(freeze("bool-condition/outer-seal-identity"));
        }
        self.packet.verify_outer(function, &branch, None)?;
        *self.packet.outer.borrow_mut() = Some(branch);
        Ok(())
    }
    pub(in crate::mir) fn seal_source_loop_v2(
        self,
        function: &MirFunction,
        materialization: &Rc<ConditionMaterializationV1>,
        branch: Binding,
    ) -> Result<(), String> {
        if !matches!(&self.packet.materialization, BoolConditionOuterV2::SourceLoop { materialization: original, .. }
            if Rc::ptr_eq(materialization, original))
            || self.packet.outer.borrow().is_some()
        {
            return Err(freeze("bool-condition/loop-outer-seal-identity"));
        }
        self.packet.verify_outer(function, &branch, None)?;
        *self.packet.outer.borrow_mut() = Some(branch);
        Ok(())
    }
}
impl EmittedBoolConditionProjectionV1 {
    pub(super) fn comparison_inputs_v1<'a>(
        &'a self,
        node: &'a BoolConditionNodeProjectionV1,
    ) -> Result<super::expression_projection::ExpressionPhysicalProjectionRefV1<'a>, String> {
        let BoolConditionValueProofV1::ScalarInvocation {
            input_values,
            dynamic_calls,
            input_reads,
            bindings,
            receiver,
            ..
        } = &node.proof
        else {
            return Err(freeze("bool-condition/text-comparison-source"));
        };
        super::expression_projection::ExpressionPhysicalProjectionRefV1::new_with_dynamic(
            self.frame.evaluation(),
            input_values,
            &[],
            dynamic_calls,
            &[],
            input_reads,
            bindings,
        )
        .for_comparison_inputs(node.node, node.value, receiver.as_ref())
    }
    pub(in crate::mir) fn prepare(
        frame: Rc<CheckedBoolConditionFrameV1>,
        nodes: Vec<BoolConditionNodeProjectionV1>,
        entry: Binding,
        prefix: Vec<Binding>,
        materialization: Rc<IfConditionMaterializationV1>,
        ledger: &OrdinaryNewClaimLedgerV1,
        function: &MirFunction,
    ) -> Result<(Rc<Self>, BoolConditionSealV1), String> {
        Self::prepare_outer(
            frame,
            nodes,
            entry,
            prefix,
            BoolConditionOuterV2::If(materialization),
            ledger,
            function,
        )
    }
    pub(in crate::mir) fn prepare_source_loop_v2(
        frame: Rc<CheckedBoolConditionFrameV1>,
        nodes: Vec<BoolConditionNodeProjectionV1>,
        entry: Binding,
        prefix: Vec<Binding>,
        materialization: Rc<ConditionMaterializationV1>,
        product: Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>,
        index: usize,
        targets: (BasicBlockId, BasicBlockId),
        ledger: &OrdinaryNewClaimLedgerV1,
        function: &MirFunction,
    ) -> Result<(Rc<Self>, BoolConditionSealV1), String> {
        Self::prepare_outer(
            frame,
            nodes,
            entry,
            prefix,
            BoolConditionOuterV2::SourceLoop {
                product,
                frame: index,
                materialization,
                targets,
            },
            ledger,
            function,
        )
    }
    fn prepare_outer(
        frame: Rc<CheckedBoolConditionFrameV1>,
        nodes: Vec<BoolConditionNodeProjectionV1>,
        entry: Binding,
        prefix: Vec<Binding>,
        materialization: BoolConditionOuterV2,
        ledger: &OrdinaryNewClaimLedgerV1,
        function: &MirFunction,
    ) -> Result<(Rc<Self>, BoolConditionSealV1), String> {
        let packet = Rc::new(Self {
            frame,
            nodes: nodes.into_boxed_slice(),
            entry,
            prefix,
            materialization,
            outer: RefCell::new(None),
        });
        packet.validate_source(CallValidationContextV1::Lowering(ledger), function, None)?;
        packet.validate_prefix_recorded(&packet.prefix)?;
        packet.validate_original_scope(ledger, function)?;
        let lease = BoolConditionSealV1 {
            packet: Rc::clone(&packet),
        };
        Ok((packet, lease))
    }
    pub(super) fn corroborate_loop_frame_v2(
        &self,
        product: &Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>,
        index: usize,
        entry: BasicBlockId,
        end: BasicBlockId,
        value: ValueId,
        materialization: &Rc<ConditionMaterializationV1>,
        targets: (BasicBlockId, BasicBlockId),
    ) -> Result<(), String> {
        if self.entry.0 != entry
            || materialization.block() != end
            || materialization.values().0 != value
            || !matches!(&self.materialization, BoolConditionOuterV2::SourceLoop { product: original, frame, materialization: actual, targets: arms }
                if Rc::ptr_eq(original, product) && *frame == index && Rc::ptr_eq(actual, materialization) && *arms == targets)
        {
            return Err(freeze("source-loop/bool-frame-correspondence"));
        }
        self.require_sealed()
    }
    pub(in crate::mir) fn frame(&self) -> &Rc<CheckedBoolConditionFrameV1> {
        &self.frame
    }
    pub(in crate::mir) fn prefix(&self) -> &[Binding] {
        &self.prefix
    }
    pub(in crate::mir) fn require_sealed(&self) -> Result<(), String> {
        if self.outer.borrow().is_none() {
            return Err(freeze("bool-condition/outer-branch-pending"));
        }
        Ok(())
    }
    pub(in crate::mir) fn calls(&self) -> impl Iterator<Item = &EmittedLexicalCallProjectionV1> {
        self.nodes.iter().filter_map(|node| match &node.proof {
            BoolConditionValueProofV1::Call { packet, .. } => Some(packet.as_ref()),
            _ => None,
        })
    }
    fn dynamic_calls(&self) -> impl Iterator<Item = &crate::mir::normal_callable_semantic_package::EmittedDynamicInvocationCallProjectionV1> {
        self.nodes.iter().flat_map(|node| match &node.proof {
            BoolConditionValueProofV1::ScalarInvocation { dynamic_calls, .. } => dynamic_calls.iter().map(Rc::as_ref).collect::<Vec<_>>(),
            _ => Vec::new(),
        })
    }
    /// The direct-receiver field read and checked Count of one Array operation.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn array_operation_bindings_v1(
        &self, operation: &Rc<crate::mir::normal_callable_semantic_package::CheckedFieldArrayAccessSourceV1>,
    ) -> Result<Option<(Binding, Binding)>, String> {
        let mut pairs = self.nodes.iter().filter_map(|node| match &node.proof {
            BoolConditionValueProofV1::FieldArrayCount { field, read }
                if self.frame.evaluation().cores().get(&node.node)
                    .and_then(|core| core.field_array_access_v1())
                    .is_some_and(|source| source.as_ref() == operation.as_ref()) => Some((field.clone(), read.clone())),
            _ => None,
        });
        let pair = pairs.next();
        if pairs.next().is_some() { return Err(freeze("finished-array-field/source-membership")); }
        Ok(pair)
    }
    fn node(&self, index: usize) -> Result<&BoolConditionNodeProjectionV1, String> {
        let mut nodes = self.nodes.iter().filter(|node| node.node == index);
        let node = nodes
            .next()
            .ok_or_else(|| freeze("bool-condition/node-missing"))?;
        if nodes.next().is_some() {
            return Err(freeze("bool-condition/node-duplicate"));
        }
        Ok(node)
    }
    pub(super) fn validate_prefix_recorded(&self, bindings: &[Binding]) -> Result<(), String> {
        if self.prefix.is_empty() || !self.prefix.contains(&self.entry) {
            return Err(freeze("bool-condition/entry-membership"));
        }
        for binding in &self.prefix {
            if bindings.iter().filter(|actual| *actual == binding).count() != 1 {
                return Err(freeze("bool-condition/prefix-membership"));
            }
        }
        for packet in self.calls() {
            packet.validate_recorded(bindings)?;
        }
        for packet in self.dynamic_calls() {
            packet.validate_recorded_projected_v1(bindings, None)?;
        }
        Ok(())
    }
    pub(super) fn validate_recorded(
        &self,
        bindings: &[Binding],
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        coordinates: Option<&Finished>,
        finishing: Option<&Finished>,
    ) -> Result<(), String> {
        self.require_sealed()?;
        let ledger = context.ledger();
        self.validate_source(context, function, finishing)?;
        for original in &self.prefix {
            match mapped(original, coordinates)? {
                Some(actual) if bindings.iter().filter(|row| **row == actual).count() == 1 => {}
                None if matches!(
                    original.1,
                    MirInstruction::Const { .. }
                        | MirInstruction::Copy { .. }
                        | MirInstruction::Jump {
                            edge_args: None,
                            ..
                        }
                ) => {}
                _ => return Err(freeze("bool-condition/final-prefix-membership")),
            }
        }
        let outer = self.outer.borrow();
        self.verify_outer(function, outer.as_ref().expect("sealed above"), finishing)?;
        for packet in self.calls() {
            packet.validate_recorded_projected(bindings, coordinates)?;
        }
        if finishing.is_none() {
            self.validate_original_scope(ledger, function)?;
        }
        Ok(())
    }
    fn verify_outer(
        &self,
        function: &MirFunction,
        original: &Binding,
        finishing: Option<&Finished>,
    ) -> Result<(), String> {
        let (then_bb, else_bb) = self.materialization.arms();
        if original.0 != self.materialization.physical().block()
            || !matches!(&original.1, MirInstruction::Branch { condition, then_bb: actual_then, else_bb: actual_else,
                then_edge_args: Some(a), else_edge_args: Some(b) }
                if *condition == self.materialization.physical().values().1 && (*actual_then, *actual_else) == (then_bb, else_bb)
                    && a.layout == crate::mir::edge_args::JumpArgsLayout::CarriersOnly && a.values.is_empty()
                    && b.layout == crate::mir::edge_args::JumpArgsLayout::CarriersOnly && b.values.is_empty())
        {
            return Err(freeze("bool-condition/outer-branch-source"));
        }
        require_binding(function, original, finishing)
    }
}
impl CallValidationContextV1<'_> {
    /// Physical nonnull borrow uses the original get and the existing sealed
    /// Bool group. Source proof alone never selects a block or grants a read.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_stored_get_nonnull_read_v1(
        self,
        frame: &Rc<crate::mir::resolved_semantics::home_new_prefix::MapFieldGetLocalV1>,
        exclusion: &crate::mir::resolved_semantics::home_new_prefix::SourceNullExclusionV1,
        read: &ExactLexicalReadV1,
        function: &MirFunction,
        finishing: Option<&Finished>,
        consumer: &Binding,
    ) -> Result<(), String> {
        let (_, [_, normal]) = self.stored_map_get_read_v1(frame, read, function, finishing)?;
        self.with_local_call_group_tree_v2(frame.site().owner(), |groups| {
            let mut matches = groups.iter().filter_map(|group| {
                let packet = group.bool_condition()?;
                let guard = exclusion.condition_read_v1(packet.frame.source())?;
                Some((group, packet, guard))
            });
            let (group, packet, guard) = matches
                .next()
                .ok_or_else(|| freeze("bool-condition/reference-source"))?;
            if matches.next().is_some()
                || exclusion.root_binding() != frame.binding()
                || !matches!(packet.materialization, BoolConditionOuterV2::If(_))
            {
                return Err(freeze("bool-condition/reference-source"));
            }
            let operation = packet
                .frame
                .operations()
                .iter()
                .find(|loan| loan.node() == 0)
                .ok_or_else(|| freeze("bool-condition/operator-loan"))?;
            let (reference, _, original) = operation
                .nullable_null_v1()
                .ok_or_else(|| freeze("bool-condition/operator-loan"))?;
            if !Rc::ptr_eq(original, frame)
                || packet.frame.source().nodes()[reference].site() != guard
                || !matches!(
                    packet.node(reference)?.proof,
                    BoolConditionValueProofV1::OriginalReference(_)
                )
            {
                return Err(freeze("bool-condition/reference-source"));
            }
            group.validate_recorded_in_context(self, function, finishing)?;
            packet.validate_nonnull_consumer_v1(function, finishing, consumer, Some(&normal))?;
            Ok(())
        })
    }
}
fn mapped(original: &Binding, finishing: Option<&Finished>) -> Result<Option<Binding>, String> {
    match finishing {
        Some(view) => view.binding(original.0, &original.1),
        None => Ok(Some(original.clone())),
    }
}
fn require_binding(
    function: &MirFunction,
    original: &Binding,
    finishing: Option<&Finished>,
) -> Result<(), String> {
    let actual = mapped(original, finishing)?
        .ok_or_else(|| freeze("bool-condition/required-producer-removed"))?;
    if !super::super::super::physical_boundary::check_binding(function, None, actual.0, &actual.1)?
    {
        return Err(format!(
            "{} original={original:?} actual={actual:?} finished={}",
            freeze("bool-condition/producer-missing"),
            finishing.is_some()
        ));
    }
    Ok(())
}

impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn bool_condition_argument_frames_v1(
        &self,
        frame: &Rc<CheckedBoolConditionFrameV1>,
    ) -> Result<Vec<Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1>>, String> {
        use crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1;
        let owner = frame.source().owner();
        let statement = frame.source().root().statement();
        let Some(flow) = self
            .completion_for_owner(owner)
            .and_then(|completion| completion.cleanup().root_flow())
        else {
            return Err(freeze("bool-condition/argument-root-flow"));
        };
        let condition_members = flow.expression_plans().iter()
            .filter_map(|plan| plan.condition_frame())
            .filter(|candidate| candidate.source().root().statement() == statement);
        if condition_members.filter(|candidate| Rc::ptr_eq(candidate, frame)).count() != 1 {
            return Err(freeze("bool-condition/argument-frame-membership"));
        }
        let mut output = Vec::new();
        for argument_frame in flow.expression_plans().iter().filter_map(|plan| plan.argument_frame()) {
            let ExpressionPlanRootV1::CallArgument { call, ordinal, statement: argument_statement } =
                argument_frame.source().root() else { continue };
            if argument_statement != statement || call.owner() != owner {
                continue;
            }
            let Some(source_argument) = call.arguments().get(*ordinal as usize) else {
                return Err(freeze("bool-condition/argument-ordinal"));
            };
            if source_argument.ordinal() != *ordinal
                || argument_frame.source().nodes().first().is_none_or(|root| root.site() != source_argument.site())
                || argument_frame.source().owner() != owner
                || !Rc::ptr_eq(argument_frame.source_loan().plan(), argument_frame.source())
                || !argument_frame.evaluation().reference_coverage_is_exact()
            {
                return Err(freeze("bool-condition/argument-frame-drift"));
            }
            output.push(Rc::clone(argument_frame));
        }
        Ok(output)
    }

    pub(in crate::mir) fn bool_condition_frame_v1(
        &self,
        owner: FunctionOwnerIdV1,
        statement: &SourceStmtSiteV1,
    ) -> Result<Option<Rc<CheckedBoolConditionFrameV1>>, String> {
        let Some(flow) = self
            .completion_for_owner(owner)
            .and_then(|completion| completion.cleanup().root_flow())
        else {
            return Ok(None);
        };
        let mut frames = flow
            .expression_plans()
            .iter()
            .filter_map(|plan| plan.condition_frame())
            .filter(|frame| frame.source().root().statement() == statement);
        let frame = frames.next().cloned();
        if frames.next().is_some() {
            return Err(freeze("bool-condition/duplicate-source-frame"));
        }
        Ok(frame)
    }
    pub(in crate::mir) fn validate_bool_condition_source_v1(
        &self,
        frame: &Rc<CheckedBoolConditionFrameV1>,
    ) -> Result<(), String> {
        let owner = frame.source().owner();
        let site = OwnedExprSiteV1::new(owner, frame.source().nodes()[0].site().clone());
        let source = self.next_prefix_source_v2(owner, &site)?;
        if !matches!(source.as_ref(), Some(LifecyclePrefixSourceV1::BoolCondition { frame: expected, .. })
            if Rc::ptr_eq(frame, expected))
        {
            return Err(freeze("bool-condition/source-prefix-order"));
        }
        Ok(())
    }
    pub(in crate::mir) fn prepare_bool_condition_bindings_v1(
        &self,
        packet: Rc<EmittedBoolConditionProjectionV1>,
    ) -> Result<PreparedLifecyclePrefixBindingGroupV1<'_>, String> {
        self.prepare_local_call_binding_group(
            packet.frame().source().owner(),
            RootLifecyclePrefixBindingGroupV1::new_bool_pending(packet)?,
        )
    }
    /// Lend the same source-owned Phi tuples to draft repair. A symbol is
    /// only the existing root/child physical locator, never source authority.
    pub(in crate::mir) fn original_source_phis_for_repair_v1(
        &self,
        function: &MirFunction,
        root_key: &str,
    ) -> Result<Vec<Binding>, String> {
        let owner = if function.signature.name == root_key {
            self.root_owner()
        } else {
            let children = self.child_physical_validation.borrow();
            let mut owners = children.iter().filter_map(|(owner, state)| {
                let symbol = match state {
                    super::super::super::ChildPhysicalValidation::Checked { symbol, .. }
                    | super::super::super::ChildPhysicalValidation::FinishingChecked {
                        symbol,
                        ..
                    } => symbol,
                };
                (symbol == &function.signature.name).then_some(*owner)
            });
            let owner = owners.next();
            if owners.next().is_some() {
                return Err(freeze("bool-condition/repair-owner-duplicate"));
            }
            owner
        };
        let Some(owner) = owner else {
            return Ok(Vec::new());
        };
        let mut bindings = self.original_condition_bindings_v1(owner)?;
        bindings.extend(self.original_field_array_append_bindings_v1(owner)?);
        bindings.extend(self.original_borrowed_field_write_guard_bindings_v1(owner)?);
        bindings.extend(self.original_borrowed_array_set_bindings_v1(owner)?);
        bindings.extend(self.original_borrowed_field_write_bindings_v1(owner, false)?);
        for role in self.recorded_source_loop_roles_v2(owner) {
            let proof = role
                .source_proof_v2()
                .ok_or_else(|| freeze("source-loop/repair-proof"))?;
            bindings.extend(proof.source_dependency_bindings_v2()?);
        }
        Ok(bindings
            .into_iter()
            .filter(|row| matches!(row.1, MirInstruction::Phi { .. }))
            .collect())
    }

    /// Shared source lender for draft repair and required finishing liveness.
    /// The prefix stays in its original group; no physical reclassification.
    pub(in crate::mir) fn original_condition_bindings_v1(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<Vec<Binding>, String> {
        let roots = self
            .root_local_call_bindings
            .borrow()
            .get(&owner)
            .cloned()
            .unwrap_or_default();
        let groups = RootLifecyclePrefixBindingGroupV1::snapshot_tree_v2(&roots);
        let mut bindings = Vec::new();
        for group in &groups {
            let Some(packet) = group.bool_condition() else {
                continue;
            };
            packet.require_sealed()?;
            match &packet.materialization {
                BoolConditionOuterV2::If(_) => {
                    let frame = self
                        .bool_condition_frame_v1(owner, packet.frame.source().root().statement())?
                        .ok_or_else(|| freeze("bool-condition/repair-source-missing"))?;
                    if !Rc::ptr_eq(&frame, &packet.frame) {
                        return Err(freeze("bool-condition/repair-source-foreign"));
                    }
                }
                BoolConditionOuterV2::SourceLoop { .. } => packet
                    .materialization
                    .validate_source(&packet.frame, CallValidationContextV1::Lowering(self))?,
            }
            for row in &packet.prefix {
                if !bindings.contains(row) {
                    bindings.push(row.clone());
                }
            }
        }
        Ok(bindings)
    }

    pub(in crate::mir) fn require_sealed_condition_groups_v1(&self) -> Result<(), String> {
        for group in self.root_local_call_bindings.borrow().values().flatten() {
            group.require_sealed_condition_v1()?;
        }
        Ok(())
    }
}

impl OrdinaryNewClaimLedgerV1 {
    fn visit_verified_condition_groups_v1(
        &self,
        owner: FunctionOwnerIdV1,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        projection: &Finished,
        mut visit: impl FnMut(&EmittedBoolConditionProjectionV1) -> Result<(), String>,
    ) -> Result<(), String> {
        context.with_local_call_group_tree_v2(owner, |groups| {
            for group in groups {
                let Some(packet) = group.bool_condition() else {
                    continue;
                };
                group.validate_recorded_in_context(context, function, Some(projection))?;
                visit(packet)?;
            }
            Ok(())
        })
    }

    /// Admission comes from the same source guard and its sealed, finished
    /// execution. This lends no typed payload or successor non-null permission.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn visit_finished_null_compare_uses_v1(
        &self,
        owner: FunctionOwnerIdV1,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        projection: &Finished,
        mut visit: impl FnMut(BindingRefV1, BindingRefV1, &OwnedExprSiteV1) -> Result<(), String>,
    ) -> Result<(), String> {
        self.visit_verified_condition_groups_v1(owner, context, function, projection, |packet| {
            for (&node, reference) in packet.frame.references() {
                let crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionReferenceV1::FiniteEntry {
                    read, exclusion: None,
                } = reference else { continue };
                let Some(binary) = read.null_compare_v1() else { continue };
                let source = packet.frame.source();
                if !Rc::ptr_eq(read.requirement().source_plan(), source) || binary.owner() != owner
                    || source.nodes().get(node).is_none_or(|node| node.site() != read.requirement().read())
                    || !matches!(&packet.node(node)?.proof, BoolConditionValueProofV1::OriginalReference(_)) {
                    return Err(freeze("bool-condition/reference-source"));
                }
                visit(read.requirement().binding(), read.requirement().formal().1, binary)?;
            }
            Ok(())
        })
    }

    /// Lend exact finished prefix and sealed outer Branch, never repair drafts.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn visit_finished_bool_bindings_v1(
        &self,
        owner: FunctionOwnerIdV1,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        projection: &Finished,
        mut visit: impl FnMut(Binding) -> Result<(), String>,
    ) -> Result<(), String> {
        self.visit_verified_condition_groups_v1(owner, context, function, projection, |packet| {
            for original in &packet.prefix {
                if let Some(actual) = mapped(original, Some(projection))? {
                    visit(actual)?;
                }
            }
            // The Loop owns these scalar Header Phis, so the condition prefix
            // excludes them. Lend the SAME exact source-read relation without
            // turning its output into a borrowed-formal alias.
            for relation in borrowed_source_loop_phi_relations_v2(
                &packet.frame, &packet.nodes, context, None,
            )? {
                relation.require_completed_v1()?;
                relation.verify_projected(function, &mut |row|
                    projection.relation_binding(function, row))?;
                let original = relation.identity_phi_binding_v1()?;
                let actual = projection.relation_binding(function, original)?
                    .ok_or_else(|| freeze("source-loop/phi-kind"))?;
                visit(actual)?;
            }
            let outer = packet.outer.borrow();
            let actual = mapped(
                outer.as_ref().expect("verified sealed group"),
                Some(projection),
            )?
            .ok_or_else(|| freeze("bool-condition/required-producer-removed"))?;
            visit(actual)
        })
    }


}

#[path = "scalar_condition_operation.rs"]
mod scalar_operation;
pub(crate) use scalar_operation::checked_scalar_invoke_operation_v1;
pub(super) use scalar_operation::scalar_condition_bindings_v1;
