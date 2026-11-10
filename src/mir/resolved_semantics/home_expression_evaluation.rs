//! Checked body of one original expression, borrowed by its purpose wrapper.
//! Acquisitions are source templates; only the physical Normal edge owns a value.
use super::*;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::{OwnedExprSiteV1, SourceExprSiteV1};
use crate::mir::resolved_semantics::home_new_prefix::local_flow::PrefixLocalFlow;
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1;
use crate::mir::resolved_value_profile::expression_projection::ExpressionLeafClassV1;
use std::collections::BTreeSet;

/// Templates for source-node acquisitions, not already acquired runtime owners.
/// Only the matching checked operation Normal may acquire an owned String.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CheckedExpressionEvaluationV1 {
    pub(super) source: Rc<ExpressionSourcePlanV1>,
    pub(super) prior_homes: Box<[LiveHomeV1]>,
    pub(super) order: Box<[usize]>,
    pub(super) classes: Box<[ExpressionLeafClassV1]>,
    pub(super) calls: Box<[LocalCallObservationV1]>,
    pub(super) cores: std::collections::BTreeMap<
        usize,
        crate::mir::normal_callable_semantic_package::ExpressionCoreSourceLoanV1,
    >,
    pub(super) references: std::collections::BTreeMap<usize, CheckedExpressionReferenceV1>,
    pub(super) operations: Box<[Rc<CheckedExpressionOperationLoanV1>]>,
    pub(super) owned_results: Box<[usize]>,
    pub(super) transferred_inputs: BTreeSet<usize>,
}
impl CheckedExpressionEvaluationV1 {
    /// The retained source laws, not an emitted opcode, decide whether this
    /// evaluation or its terminal consumer can need a Fault frame.
    pub(crate) fn requires_fault_frame_v1(
        &self,
        consumer_frame_free_proved: bool,
        mut checked_field: impl FnMut(&OwnedExprSiteV1) -> Option<bool>,
    ) -> bool {
        use crate::mir::resolved_value_profile::expression_root::StatementExpressionRoleV1;
        use crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1;
        if !consumer_frame_free_proved
            || !matches!(self.root_class(), ExpressionLeafClassV1::Value(
                ExpressionValueClassV1::I64 | ExpressionValueClassV1::Bool))
            || !self.calls.is_empty() || !self.owned_results.is_empty()
            || matches!(self.source.root(), ExpressionPlanRootV1::Statement {
                role: StatementExpressionRoleV1::PrintValue, ..
            })
            || self.cores.values().any(|core| core.source().target().frame_free_read_law_v1().is_none())
        {
            return true;
        }
        for loan in self.operations.iter() {
            if loan.scalar_copy_v1().is_none() && loan.integer_operator_v1().is_none()
                && self.exact_integer_comparison_nodes_v1(loan.node()).is_none()
                && !matches!(self.frame_free_text_comparison_v1(loan.node()), Ok(Some(_)))
                && loan.nullable_null_v1().is_none() && loan.nullable_field_null_v1().is_none()
            {
                return true;
            }
        }
        self.order.iter().any(|&index| {
            let node = &self.source.nodes()[index];
            matches!(node.source(), ExpressionNodeSourceV1::Shape(
                crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { .. }
            )) && checked_field(&OwnedExprSiteV1::new(self.source.owner(), node.site().clone()))
                != Some(false)
        })
    }
    pub(crate) fn source(&self) -> &Rc<ExpressionSourcePlanV1> {
        &self.source
    }
    pub(crate) fn prior_homes(&self) -> &[LiveHomeV1] {
        &self.prior_homes
    }
    pub(crate) fn ordered_nodes(&self) -> &[usize] {
        &self.order
    }
    pub(crate) fn calls(&self) -> &[LocalCallObservationV1] {
        &self.calls
    }
    pub(crate) fn cores(
        &self,
    ) -> &std::collections::BTreeMap<
        usize,
        crate::mir::normal_callable_semantic_package::ExpressionCoreSourceLoanV1,
    > {
        &self.cores
    }
    /// A Dynamic local may act as an opaque String carrier only when this
    /// exact read is backed by its live, checked invocation-result Home.
    /// This keeps arbitrary Dynamic values out of the String operation domain.
    pub(crate) fn dynamic_local_carrier_read_v1(&self, node: usize) -> bool {
        let Some(reference @ CheckedExpressionReferenceV1::DynamicInvocationResult(initializer)) =
            self.references.get(&node)
        else {
            return false;
        };
        reference.matches_node(&self.source, node)
            && initializer.is_dynamic_value()
            && initializer.binding().owner() == self.source.owner()
            && initializer.source().owner() == self.source.owner()
            && !self.owned_result_nodes().contains(&node)
            && initializer
                .dynamic_home_binding_v1()
                .is_some_and(|owner| self.prior_homes.contains(&LiveHomeV1::Stable(owner)))
    }
    /// A Dynamic invocation-result local read on the String Add right side is
    /// consumed as an opaque carrier, never as a tagged formal.
    pub(crate) fn string_add_right_operand_v1(
        &self,
        site: &crate::mir::resolved_semantics::SourceExprSiteV1,
        operand: crate::mir::instruction::StringInvokeOperand,
    ) -> crate::mir::instruction::StringInvokeOperand {
        use crate::mir::instruction::StringInvokeOperand as Operand;
        let Some(node) = self.source.nodes().iter().position(|node| node.site() == site) else { return operand };
        match operand {
            Operand::Dynamic(carrier) if self.dynamic_local_carrier_read_v1(node) => Operand::DynamicCarrier(carrier),
            Operand::NormalInteger(value) if matches!(self.references.get(&node),
                Some(CheckedExpressionReferenceV1::Scalar(read)) if read.kind() == super::super::SourceScalarKind::Bool) =>
                Operand::NormalBool(value),
            _ => operand,
        }
    }
    pub(crate) fn references(
        &self,
    ) -> &std::collections::BTreeMap<usize, CheckedExpressionReferenceV1> {
        &self.references
    }
    /// Install source-qualified Core receiver reads into the same reference
    /// map consumed by expression evaluation and original-use closure.
    pub(crate) fn retain_core_text_entry_reads_v1(&mut self) -> bool {
        let mut reads = std::collections::BTreeMap::new();
        for core in self.cores.values() {
            let Some(read) = core.text_entry_receiver_read() else {
                continue;
            };
            if !core.matches_plan(&self.source, core.node())
                || !Rc::ptr_eq(read.source_plan(), &self.source)
                || reads
                    .insert(read.node(), read.clone())
                    .is_some_and(|previous| previous != *read)
            {
                return false;
            }
        }
        for (node, read) in reads {
            match self.references.get(&node) {
                Some(CheckedExpressionReferenceV1::ArgumentEntry(previous))
                    if previous == &read => {}
                // Initializer qualification may already hold the stronger
                // finite-domain entry read for this exact source node. The
                // Core receiver contract is co-located in that proof's
                // requirement; retain one source reference, not two lanes
                // competing for the same evaluated read.
                Some(CheckedExpressionReferenceV1::FiniteEntry { read: finite, .. })
                    if finite.requirement() == &read => {}
                Some(_) => return false,
                None => {
                    self.references
                        .insert(node, CheckedExpressionReferenceV1::ArgumentEntry(read));
                }
            }
        }
        true
    }
    /// Every active value reference has exactly one retained original proof.
    /// Receiver identities and transferred ordinary-call inputs keep their own
    /// owners; neither becomes a reference permission in this evaluation.
    pub(crate) fn reference_coverage_is_exact(&self) -> bool {
        let expected: BTreeSet<_> = self
            .order
            .iter()
            .copied()
            .filter(|&node| {
                !self.transfers_node(node)
                    && !self.node_is_identity(node)
                    && matches!(
                        self.source.nodes()[node].source(),
                        ExpressionNodeSourceV1::Reference(_)
                    )
            })
            .collect();
        expected.len() == self.references.len()
            && self.references.iter().all(|(&node, proof)| {
                expected.contains(&node) && proof.matches_node(&self.source, node)
            })
    }
    pub(crate) fn operations(&self) -> &[Rc<CheckedExpressionOperationLoanV1>] {
        &self.operations
    }
    pub(crate) fn owned_result_nodes(&self) -> &[usize] {
        &self.owned_results
    }
    pub(crate) fn class(&self, node: usize) -> Option<ExpressionLeafClassV1> {
        self.classes.get(node).copied()
    }
    pub(crate) fn root_class(&self) -> ExpressionLeafClassV1 {
        self.classes[0]
    }
    /// Borrow only the original eager inputs of one String comparison. The
    /// enclosing Bool tree still owns conditional reach; no AST is re-rooted.
    pub(crate) fn text_comparison_input_order_v1(&self, comparison: usize) -> Option<Vec<usize>> {
        self.comparison_input_order_v1(comparison, false)
    }
    /// Bool conditions may additionally compare one exact owned Dynamic call
    /// result with String. The Loop recipe keeps its narrower text-only lane.
    pub(crate) fn bool_condition_comparison_input_order_v1(
        &self,
        comparison: usize,
    ) -> Option<Vec<usize>> {
        self.comparison_input_order_v1(comparison, true)
    }
    /// Operation eligibility is independent of a purpose's input schedule.
    /// The complete expression and a conditional subtree lend the same source loan.
    pub(crate) fn text_comparison_operation_v1(
        &self,
        comparison: usize,
        allow_dynamic_call: bool,
    ) -> Option<()> {
        use crate::mir::dynamic_operator_contract::{
            DynamicOperatorNormalResultV1 as ResultKind, DynamicOperatorValueClassV1 as Class,
        };
        let root = self.source.nodes().get(comparison)?;
        let loan = self
            .operations
            .iter()
            .find(|loan| loan.node() == comparison)?;
        let envelope = loan.single_envelope()?;
        let domain = envelope.domain();
        let operand_domain = match (domain.left(), domain.right()) {
            (Class::String, Class::String) => true,
            (Class::Dynamic, Class::String) | (Class::String, Class::Dynamic) => allow_dynamic_call,
            _ => false,
        };
        if !Rc::ptr_eq(loan.source(), &self.source)
            || root.binary().is_none()
            || !operand_domain
            || envelope.normal_result() != ResultKind::TrivialBool
        {
            return None;
        }
        Some(())
    }
    /// The same eager-input scope serves Text comparison and synchronous
    /// field presence. Presence currently requires a passive exact Text leaf:
    /// computed key effects must not move ahead of receiver field evaluation.
    pub(crate) fn scalar_invoke_input_order_v1(&self, root: usize) -> Option<Vec<usize>> {
        if let Some((left, right, _)) = self.carrier_integer_comparison_nodes_v1(root) {
            let binary = self.source.nodes()[root].binary()?;
            return Some(if self.source.nodes()[left].site() == binary.lhs() { vec![left, right] } else { vec![right, left] });
        }
        if let Some(order) = self.bool_condition_comparison_input_order_v1(root) {
            return Some(order);
        }
        let core = self.cores.get(&root)?;
        if !core.matches_plan(&self.source, root)
            || core.field_map().is_none()
            || !core.source().target().checked_map_presence_law_v1()
        {
            return None;
        }
        let [argument] = core.source().source().arguments() else {
            return None;
        };
        let node = self
            .source
            .nodes()
            .iter()
            .position(|node| node.site() == argument.site())?;
        let original = &self.source.nodes()[node];
        if self.class(node) != Some(ExpressionLeafClassV1::Value(
            crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::String))
            || self.transfers_node(node)
            || !matches!(original.children(), crate::mir::resolved_value_profile::expression_children::ExpressionChildrenV1::Known(children) if children.is_empty())
            || !matches!(original.source(), ExpressionNodeSourceV1::Reference(_)
                | ExpressionNodeSourceV1::Literal(super::super::ResolvedLiteralSourceV1::String(_))) { return None; }
        Some(vec![node])
    }
    pub(crate) fn transfers_node(&self, node: usize) -> bool {
        self.transferred_inputs.contains(&node)
    }
    pub(crate) fn owned_root_node_v1(&self) -> Option<usize> {
        let mut node = 0;
        while let Some(tail) = self.source.transparent_tail_node_v1(node) {
            node = tail;
        }
        self.owned_result_nodes().contains(&node).then_some(node)
    }
    /// Owned carrier and owned Text are distinct acquisitions. A proved Text
    /// payload still needs a snapshot before a Text destination can own it.
    pub(crate) fn text_snapshot_root_v1(&self) -> Option<usize> {
        let node = self.owned_root_node_v1()?;
        (self.class(node) == Some(ExpressionLeafClassV1::Value(
            crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::String))
            && self.cores.get(&node).is_some_and(|core|
                core.is_array_dynamic_get_v1() && core.proves_text_result_v1())).then_some(node)
    }
    pub(crate) fn owned_text_root_node_v1(&self) -> Option<usize> {
        let node = self.owned_root_node_v1()?;
        (self.class(node) == Some(ExpressionLeafClassV1::Value(
            crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::String))
            && self.text_snapshot_root_v1().is_none()).then_some(node)
    }
    pub(crate) fn direct_array_receiver_v1(&self, node: usize)
        -> Option<&crate::mir::normal_callable_semantic_package::ExpressionCoreSourceLoanV1> {
        let site = self.source.nodes().get(node)?.site();
        let mut cores = self.cores.values().filter(|core| core.matches_plan(&self.source, core.node())
            && core.field_array_access_v1().is_some_and(|array| array.receiver_alias_v1().is_none()
                && array.receiver().site() == site && array.matches_expression_receiver_v1(&self.source)));
        let core = cores.next()?;
        cores.next().is_none().then_some(core)
    }
    pub(crate) fn node_is_identity(&self, node: usize) -> bool {
        self.classes.get(node) == Some(&ExpressionLeafClassV1::Identity)
    }
}

pub(super) fn owned_result_nodes(
    order: &[usize],
    add: &[usize],
    cores: &std::collections::BTreeMap<
        usize,
        crate::mir::normal_callable_semantic_package::ExpressionCoreSourceLoanV1,
    >,
    calls: &std::collections::BTreeMap<usize, LocalCallObservationV1>,
) -> Box<[usize]> {
    order.iter().copied().filter(|node| add.contains(node) || cores.get(node).is_some_and(|core| core.owns_result())
        || calls.get(node).is_some_and(|call| call.result() == super::super::LocalCallResultClassV1::Text
            && matches!(call.result_source(), Some(super::super::LocalCallResultSourceV1::Ordinary(loan)) if loan.result() == super::super::LocalCallResultClassV1::Text))
        || calls.get(node).is_some_and(|call| call.dynamic_invocation_result_lifecycle_v1().is_some())).collect()
}

/// Existing purpose proofs live once, at the exact active source node. These
/// obligations are not physical operands; entry/residence fulfillment is later.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CheckedExpressionReferenceV1 {
    NullableResult(NullableResultReferenceV1),
    /// A local Dynamic result borrowed from its exact initializer call frame.
    /// The frame grants neither a concrete payload type nor ownership transfer.
    DynamicInvocationResult(Rc<super::super::CheckedInitializerExpressionFrameV1>),
    StaticText(super::super::StaticTextReadV1),
    Residence(super::initializer_expression::TextResidenceReadV1),
    LoopText(super::super::LoopTextReadV1),
    ArgumentEntry(crate::mir::normal_callable_semantic_package::BorrowedTextEntryReadRequirementV1),
    DynamicEntry(crate::mir::normal_callable_semantic_package::DynamicEntryReadV1),
    LoopDynamic(super::super::LoopDynamicReadV1),
    FiniteEntry {
        read: crate::mir::normal_callable_semantic_package::FiniteEntryReadV1,
        exclusion: Option<Rc<super::super::SourceNullExclusionV1>>,
    },
    Scalar(CheckedScalarReferenceV1),
}
impl CheckedExpressionReferenceV1 {
    pub(crate) fn binding(&self) -> super::super::BindingRefV1 {
        match self {
            Self::NullableResult(frame) => frame.binding(),
            Self::DynamicInvocationResult(frame) => frame.binding(),
            Self::StaticText(read) => read.binding(),
            Self::Residence(read) => read.binding(),
            Self::LoopText(read) => read.binding(),
            Self::ArgumentEntry(read) => read.binding(),
            Self::DynamicEntry(read) => read.requirement().binding(),
            Self::LoopDynamic(read) => read.binding(),
            Self::FiniteEntry { read, .. } => read.requirement().binding(),
            Self::Scalar(read) => read.binding(),
        }
    }
    pub(crate) fn matches_node(&self, source: &Rc<ExpressionSourcePlanV1>, node: usize) -> bool {
        let Some(original) = source.nodes().get(node) else {
            return false;
        };
        if self.binding().owner() != source.owner()
            || !matches!(original.source(),
            ExpressionNodeSourceV1::Reference(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding))
                if *binding == self.binding())
        {
            return false;
        }
        match self {
            Self::NullableResult(frame) => {
                frame.site().owner() == source.owner() && frame.matches_source()
            }
            Self::DynamicInvocationResult(frame) => {
                frame.is_dynamic_value()
                    && frame.source().owner() == source.owner()
                    && frame.binding() == self.binding()
            }
            Self::StaticText(read) => read.read() == original.site(),
            Self::Residence(read) => {
                read.node() == node
                    && read.root() == read.acquisition().binding()
                    && read.acquisition().source().owner() == source.owner()
                    && read.acquisition().is_text_residence()
            }
            Self::LoopText(read) => read.matches_source_node(source, node),
            Self::ArgumentEntry(read) => {
                Rc::ptr_eq(read.source_plan(), source)
                    && read.node() == node
                    && read.read() == original.site()
            }
            Self::DynamicEntry(read) => {
                Rc::ptr_eq(read.requirement().source_plan(), source)
                    && read.requirement().node() == node
                    && read.requirement().read() == original.site()
                    && read.corroborates_original_read_v1()
            }
            Self::LoopDynamic(read) => read.matches_source_node(source, node),
            Self::FiniteEntry { read, exclusion } => {
                Rc::ptr_eq(read.requirement().source_plan(), source)
                    && read.requirement().node() == node
                    && read.requirement().read() == original.site()
                    && read.leaf_class(exclusion.as_deref()).is_some()
            }
            Self::Scalar(read) => read
                .original_read()
                .is_none_or(|read| read.read() == original.site()),
        }
    }
}

/// Existing class obligations and original current reads share one node slot.
/// Only Original may later be fulfilled as a physical operand.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CheckedScalarReferenceV1 {
    Obligation {
        binding: super::super::BindingRefV1,
        kind: super::super::SourceScalarKind,
    },
    Original(super::super::CurrentScalarReadV1),
}
impl CheckedScalarReferenceV1 {
    pub(crate) fn binding(&self) -> super::super::BindingRefV1 {
        match self {
            Self::Obligation { binding, .. } => *binding,
            Self::Original(read) => read.binding(),
        }
    }
    pub(crate) fn kind(&self) -> super::super::SourceScalarKind {
        match self {
            Self::Obligation { kind, .. } => *kind,
            Self::Original(read) => read.kind(),
        }
    }
    pub(crate) fn original_read(&self) -> Option<&super::super::CurrentScalarReadV1> {
        match self {
            Self::Original(read) => Some(read),
            Self::Obligation { .. } => None,
        }
    }
}

/// Seed original references once before leaf projection. Candidate selection is
/// purpose-owned; a missing candidate remains unresolved, while a malformed
/// candidate rejects the whole expression. No runtime owner is acquired here.
pub(super) fn seed_expression_references_v1(
    prepared: &mut super::leaves::PreparedExpressionCallsV1,
    initial: Vec<(usize, CheckedExpressionReferenceV1)>,
    order: &[usize],
    candidate: &mut impl FnMut(
        usize,
        &crate::mir::resolved_semantics::SourceExprSiteV1,
    ) -> Result<Option<CheckedExpressionReferenceV1>, ()>,
) -> Option<std::collections::BTreeMap<usize, CheckedExpressionReferenceV1>> {
    let source = Rc::clone(prepared.source());
    let mut proofs = std::collections::BTreeMap::new();
    // Initial loans are a list: duplicate source reads must reject, even after
    // the first loan has resolved its node and made it inactive.
    for (node, proof) in initial {
        seed_reference_v1(prepared, &source, node, proof, &mut proofs)?;
    }
    for &node in order {
        if !prepared.is_active_reference(node) {
            continue;
        }
        let original = source.nodes().get(node)?;
        let Some(proof) = candidate(node, original.site()).ok()? else {
            continue;
        };
        seed_reference_v1(prepared, &source, node, proof, &mut proofs)?;
    }
    Some(proofs)
}

fn seed_reference_v1(
    prepared: &mut super::leaves::PreparedExpressionCallsV1,
    source: &Rc<ExpressionSourcePlanV1>,
    node: usize,
    proof: CheckedExpressionReferenceV1,
    proofs: &mut std::collections::BTreeMap<usize, CheckedExpressionReferenceV1>,
) -> Option<()> {
    use crate::mir::resolved_value_profile::{
        expression_class::ExpressionValueClassV1 as Class,
        expression_projection::ExpressionLeafClassV1 as Leaf,
    };
    if proofs.contains_key(&node) || !proof.matches_node(source, node) {
        return None;
    }
    let class = match &proof {
        CheckedExpressionReferenceV1::NullableResult(frame) => {
            if !prepared.accepts_nullable_reference_v1(frame) || !frame.accepts_prior_homes(prepared.prior_homes()) {
                return None;
            }
            Leaf::Value(Class::Opaque)
        }
        CheckedExpressionReferenceV1::DynamicInvocationResult(frame) => {
            if !frame.is_dynamic_value() || frame.dynamic_home_binding_v1().is_none_or(|owner|
                !prepared.prior_homes().contains(&LiveHomeV1::Stable(owner))) {
                return None;
            }
            Leaf::Value(Class::Dynamic)
        }
        CheckedExpressionReferenceV1::Scalar(read) => Leaf::Value(match read.kind() {
            super::super::SourceScalarKind::Integer => Class::I64,
            super::super::SourceScalarKind::Bool => Class::Bool,
        }),
        CheckedExpressionReferenceV1::FiniteEntry { read, exclusion } => {
            read.leaf_class(exclusion.as_deref())?
        }
        CheckedExpressionReferenceV1::Residence(read) => {
            if !read.acquisition().is_text_residence()
                || !prepared
                    .prior_homes()
                    .contains(&LiveHomeV1::Text(read.acquisition().clone()))
            {
                return None;
            }
            Leaf::Value(Class::String)
        }
        CheckedExpressionReferenceV1::DynamicEntry(read) => read.leaf_class(),
        CheckedExpressionReferenceV1::LoopDynamic(_) => Leaf::Value(Class::Dynamic),
        CheckedExpressionReferenceV1::StaticText(_)
        | CheckedExpressionReferenceV1::ArgumentEntry(_)
        | CheckedExpressionReferenceV1::LoopText(_) => Leaf::Value(Class::String),
    };
    if !prepared.seed_reference(node, class) {
        return None;
    }
    proofs.insert(node, proof);
    Some(())
}

/// Current Home references use the same original proof selection for every
/// participating purpose. Purpose-specific entry loans remain with that issuer.
pub(super) fn seed_current_expression_references_v1(
    prepared: &mut super::leaves::PreparedExpressionCallsV1,
    initial: Vec<(usize, CheckedExpressionReferenceV1)>,
    order: &[usize],
    static_reference: &mut impl FnMut(
        &super::super::SourceExprSiteV1,
    ) -> Option<super::super::StaticTextReadV1>,
    residence_reference: &mut impl FnMut(
        &super::super::SourceExprSiteV1,
    ) -> Option<(
        super::super::BindingRefV1,
        super::super::BindingRefV1,
        TextResidenceAcquisitionV1,
    )>,
    scalar_reference: &mut impl FnMut(
        &super::super::SourceExprSiteV1,
    ) -> Option<CheckedScalarReferenceV1>,
) -> Option<std::collections::BTreeMap<usize, CheckedExpressionReferenceV1>> {
    seed_current_expression_references_with_loop_text_v1(
        prepared,
        initial,
        order,
        static_reference,
        residence_reference,
        &mut |_| None,
        scalar_reference,
    )
}

pub(super) fn seed_current_expression_references_with_loop_text_v1(
    prepared: &mut super::leaves::PreparedExpressionCallsV1,
    initial: Vec<(usize, CheckedExpressionReferenceV1)>,
    order: &[usize],
    static_reference: &mut impl FnMut(
        &super::super::SourceExprSiteV1,
    ) -> Option<super::super::StaticTextReadV1>,
    residence_reference: &mut impl FnMut(
        &super::super::SourceExprSiteV1,
    ) -> Option<(
        super::super::BindingRefV1,
        super::super::BindingRefV1,
        TextResidenceAcquisitionV1,
    )>,
    loop_text_reference: &mut impl FnMut(
        &super::super::SourceExprSiteV1,
    ) -> Option<super::super::LoopTextReadV1>,
    scalar_reference: &mut impl FnMut(
        &super::super::SourceExprSiteV1,
    ) -> Option<CheckedScalarReferenceV1>,
) -> Option<std::collections::BTreeMap<usize, CheckedExpressionReferenceV1>> {
    let bool_condition = prepared.is_bool_condition();
    seed_expression_references_v1(prepared, initial, order, &mut |node, site| {
        let proof = if let Some(read) = loop_text_reference(site) {
            CheckedExpressionReferenceV1::LoopText(read)
        } else if let Some(read) = static_reference(site) {
            CheckedExpressionReferenceV1::StaticText(read)
        } else if let Some((binding, root, acquisition)) = residence_reference(site) {
            CheckedExpressionReferenceV1::Residence(
                super::initializer_expression::TextResidenceReadV1::new(
                    node,
                    binding,
                    root,
                    acquisition,
                ),
            )
        } else if let Some(read) = scalar_reference(site) {
            // An exact Bool Local is a first-class operand (Text display, Bool
            // equality); no other non-Integer scalar exists.
            if !matches!(read.kind(), super::super::SourceScalarKind::Integer | super::super::SourceScalarKind::Bool) && !bool_condition {
                return Err(());
            }
            CheckedExpressionReferenceV1::Scalar(read)
        } else {
            return Ok(None);
        };
        Ok(Some(proof))
    })
}

/// Shared original Text evaluation for terminal values and passive Map keys.
/// The purpose chooses the source root; this function grants no result/Home ABI.
pub(in crate::mir::resolved_semantics::home_new_prefix) fn qualify_text_evaluation_v1<
    E,
>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    source: std::rc::Rc<
        crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1,
    >,
    purpose_site: &OwnedExprSiteV1,
    locals: &PrefixLocalFlow<'_>,
    homes: &[crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1],
    calls: &[crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1],
    actuals: &mut impl FnMut(
        &OwnedExprSiteV1,
        crate::mir::resolved_semantics::home_new_prefix::BorrowedCallActualRequestV1<'_>,
    ) -> Result<
        Option<crate::mir::resolved_semantics::home_new_prefix::BorrowedCallArgumentsV1>,
        E,
    >,
    field_text_source: Option<
        &crate::mir::normal_callable_semantic_package::CheckedFieldArrayAccessSourceV1,
    >,
) -> Result<
    Result<
        std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1>,
        &'static str,
    >,
    E,
> {
    let (evaluation, fields) = match prepare_value_evaluation_v1(
        input,
        source,
        crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::String,
        purpose_site,
        locals,
        homes,
        calls,
        actuals,
        field_text_source,
        &std::collections::BTreeMap::new(),
        &mut |_| None,
    )? {
        Ok(prepared) => prepared,
        Err(reason) => return Ok(Err(reason)),
    };
    Ok(if fields.is_empty() { Ok(evaluation) } else { Err("evaluation-requests-field-reads") })
}

#[path = "home_expression_value_preparation.rs"]
mod value_preparation;
pub(in crate::mir::resolved_semantics::home_new_prefix) use value_preparation::prepare_value_evaluation_v1;

#[path = "home_expression_comparison_nodes.rs"]
mod comparison_nodes;
