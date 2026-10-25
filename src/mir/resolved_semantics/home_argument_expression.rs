//! Completed semantic argument qualification at the real covered Prefix point.
//! Mandatory entry requirements remain obligations, not physical descriptors.
use super::super::{
    BindingRefV1, LocalCallObservationV1, OwnedExprSiteV1, SourceExprSiteV1, SourceScalarKind,
};
use super::*;
use crate::mir::normal_callable_semantic_package::{
    ArgumentExpressionSourceLoanV1, BorrowedTextEntryReadRequirementV1,
};
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use crate::mir::resolved_value_profile::{
    expression_children::{ExpressionChildReachV1, ExpressionChildrenV1},
    expression_class::ExpressionValueClassV1 as Class,
    expression_plan::ExpressionPlanRootV1,
    expression_projection::ExpressionLeafClassV1 as Leaf,
};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CheckedArgumentExpressionFrameV1 {
    evaluation: CheckedExpressionEvaluationV1,
    source_loan: ArgumentExpressionSourceLoanV1,
}
impl CheckedArgumentExpressionFrameV1 {
    pub(crate) fn is_i64(&self) -> bool {
        self.evaluation.root_class() == Leaf::Value(Class::I64)
    }
    pub(crate) fn is_dynamic_carrier_append_v1(&self) -> bool {
        let Some(root) = self.source_loan.plan().nodes().first() else {
            return false;
        };
        self.evaluation.root_class() == Leaf::Value(Class::Dynamic)
            && self.source_loan.dynamic_array_carrier_append_root_v1()
            && self
                .source_loan
                .dynamic_array_carrier_call_is_v1(root.site())
            && self.evaluation.calls().iter().any(|call| {
                call.site().site() == root.site()
                    && call.result()
                        == crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::Dynamic
                    && call.dynamic_invocation_result_lifecycle_v1().is_some()
            })
    }
    /// Argument purpose obtains independent Text residence for a readonly root.
    pub(crate) fn materializes_root(&self) -> bool {
        !self.is_i64()
            && !self.is_dynamic_carrier_append_v1()
            && !self.evaluation.owned_result_nodes().contains(&0)
    }
    pub(crate) fn evaluation(&self) -> &CheckedExpressionEvaluationV1 {
        &self.evaluation
    }
    pub(crate) fn source_loan(&self) -> &ArgumentExpressionSourceLoanV1 {
        &self.source_loan
    }
    pub(crate) fn static_reads(&self) -> Vec<&super::super::StaticTextReadV1> {
        self.evaluation
            .references()
            .values()
            .filter_map(|read| match read {
                CheckedExpressionReferenceV1::StaticText(read) => Some(read),
                _ => None,
            })
            .collect()
    }
    pub(crate) fn residence_reads(
        &self,
    ) -> Vec<&super::initializer_expression::TextResidenceReadV1> {
        self.evaluation
            .references()
            .values()
            .filter_map(|read| match read {
                CheckedExpressionReferenceV1::Residence(read) => Some(read),
                _ => None,
            })
            .collect()
    }
    pub(crate) fn source(&self) -> &Rc<ExpressionSourcePlanV1> {
        self.evaluation.source()
    }
    pub(crate) fn prior_homes(&self) -> &[LiveHomeV1] {
        self.evaluation.prior_homes()
    }
    pub(crate) fn text_reads(&self) -> Vec<&BorrowedTextEntryReadRequirementV1> {
        self.evaluation
            .references()
            .values()
            .filter_map(|read| match read {
                CheckedExpressionReferenceV1::ArgumentEntry(read) => Some(read),
                _ => None,
            })
            .collect()
    }
    pub(crate) fn references(&self) -> BTreeMap<usize, (BindingRefV1, SourceScalarKind)> {
        self.evaluation
            .references()
            .iter()
            .filter_map(|(&node, read)| match read {
                CheckedExpressionReferenceV1::Scalar(read) => {
                    Some((node, (read.binding(), read.kind())))
                }
                _ => None,
            })
            .collect()
    }
    pub(crate) fn transfers_node(&self, node: usize) -> bool {
        self.evaluation.transfers_node(node)
    }
}

pub(crate) fn qualify_argument_expression_v1<E>(
    core: ArgumentExpressionSourceLoanV1,
    statement: &SourceStmtSiteV1,
    prior_homes: &[LiveHomeV1],
    text_reads: &[BorrowedTextEntryReadRequirementV1],
    finite_entry_reads: &[crate::mir::normal_callable_semantic_package::FiniteEntryReadV1],
    dynamic_entry_reads: &[crate::mir::normal_callable_semantic_package::DynamicEntryReadV1],
    loop_dynamic_reads: &[super::super::LoopDynamicReadV1],
    text_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<(BindingRefV1, BindingRefV1)>,
    static_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<super::super::StaticTextReadV1>,
    residence_reference: &mut impl FnMut(
        &SourceExprSiteV1,
    ) -> Option<(
        BindingRefV1,
        BindingRefV1,
        TextResidenceAcquisitionV1,
    )>,
    scalar_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<super::super::CurrentScalarReadV1>,
    field_request: &mut impl FnMut(&SourceExprSiteV1) -> Option<LocalFieldReadRequestV1>,
    source_call: &mut impl FnMut(
        &OwnedExprSiteV1,
    ) -> Result<Option<super::leaves::ExpressionCallSourceV1>, E>,
    can_install: &mut impl FnMut(&HomeExpressionPlanV1) -> bool,
    field_batch: &mut impl FnMut(
        &[LocalFieldReadRequestV1],
        super::super::LocalFieldReadDemandV1,
    ) -> Result<bool, E>,
) -> Result<Option<HomeExpressionPlanV1>, E> {
    let source = Rc::clone(core.plan());
    let ExpressionPlanRootV1::CallArgument {
        statement: original,
        call,
        ordinal,
    } = source.root()
    else {
        return Ok(None);
    };
    let Some(root) = source.nodes().first() else {
        return Ok(None);
    };
    if original != statement
        || call.owner() != source.owner()
        || call
            .arguments()
            .get(*ordinal as usize)
            .is_none_or(|arg| arg.ordinal() != *ordinal || arg.site() != root.site())
    {
        return Ok(None);
    }
    if let Some(resolver) = core.resolver_source() {
        let method = resolver.source();
        if method.owner() != source.owner()
            || root.site() != method.site()
            || !matches!(root.source(), ExpressionNodeSourceV1::MethodCall)
        {
            return Ok(None);
        }
    } else if !core.requires_i64_v1()
        && !(core.dynamic_array_carrier_append_root_v1()
            && core.dynamic_array_carrier_call_is_v1(root.site()))
        && !core.ordinary_call_is(root.site())
        && !matches!(
            root.source(),
            ExpressionNodeSourceV1::Literal(ResolvedLiteralSourceV1::String(_))
                | ExpressionNodeSourceV1::Reference(
                    crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(_)
                )
        )
        && !root.binary().is_some_and(|binary| {
            binary.operator() == crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::Add
        })
    {
        return Ok(None);
    }
    let mut homes = BTreeSet::new();
    if prior_homes
        .iter()
        .any(|home| home.owner() != source.owner() || !homes.insert(home.binding()))
    {
        return Ok(None);
    }
    let Ok(order) = source.ordered_postorder_v1() else {
        return Ok(None);
    };
    if source.nodes().iter().any(|node| {
        !matches!(node.children(), ExpressionChildrenV1::Known(children)
        if children.iter().all(|child| child.reach == ExpressionChildReachV1::Always))
    }) {
        return Ok(None);
    }
    if crate::config::env::builder_debug_enabled() {
        eprintln!("[argument-expression] source preflight complete nodes={}", source.nodes().len());
    }
    let mut seeded = BTreeMap::new();
    let mut finite_proofs = BTreeMap::new();
    for read in finite_entry_reads {
        let requirement = read.requirement();
        let node = requirement.node();
        if !core.signed_operand_contains(requirement.read())
            || text_reference(requirement.read())
                != Some((requirement.binding(), requirement.formal().1))
            || source.nodes().get(node).is_none_or(|original| {
                original.site() != requirement.read()
                    || !matches!(original.source(), ExpressionNodeSourceV1::Reference(
                        crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding)
                    ) if *binding == requirement.binding())
            })
            || !read.signed_only()
            || read.leaf_class(None) != Some(Leaf::Value(Class::I64))
            || seeded.insert(node, Leaf::Value(Class::I64)).is_some()
            || finite_proofs
                .insert(
                    node,
                    CheckedExpressionReferenceV1::FiniteEntry {
                        read: read.clone(),
                        exclusion: None,
                    },
                )
                .is_some()
        {
            return Ok(None);
        }
    }
    let mut dynamic_proofs = BTreeMap::new();
    for read in dynamic_entry_reads {
        let requirement = read.requirement();
        let node = requirement.node();
        if !core.dynamic_operand_contains(requirement.read())
            || text_reference(requirement.read())
                != Some((requirement.binding(), requirement.formal().1))
            || source.nodes().get(node).is_none_or(|original| {
                original.site() != requirement.read()
                    || !matches!(original.source(), ExpressionNodeSourceV1::Reference(
                        crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding)
                    ) if *binding == requirement.binding())
            })
            || !read.corroborates_original_read_v1()
            || seeded.insert(node, read.leaf_class()).is_some()
            || dynamic_proofs
                .insert(
                    node,
                    CheckedExpressionReferenceV1::DynamicEntry(read.clone()),
                )
                .is_some()
        {
            return Ok(None);
        }
    }
    let mut loop_dynamic_proofs = BTreeMap::new();
    for read in loop_dynamic_reads {
        let Some(node) = source
            .nodes()
            .iter()
            .position(|original| original.site() == read.read())
        else {
            return Ok(None);
        };
        if source
            .nodes()
            .iter()
            .filter(|original| original.site() == read.read())
            .count()
            != 1
            || !core.dynamic_operand_contains(read.read())
            || !read.matches_source_node(&source, node)
            || seeded.insert(node, Leaf::Value(Class::Dynamic)).is_some()
            || loop_dynamic_proofs
                .insert(node, CheckedExpressionReferenceV1::LoopDynamic(read.clone()))
                .is_some()
        {
            if crate::config::env::builder_debug_enabled() {
                eprintln!("[argument-expression] loop Dynamic source rejected read={:?} node={node} core_dynamic={} source_match={} duplicate_site={}",
                    read.read(), core.dynamic_operand_contains(read.read()), read.matches_source_node(&source, node),
                    source.nodes().iter().filter(|original| original.site() == read.read()).count() != 1);
            }
            return Ok(None);
        }
    }

    let mut static_reads = Vec::new();
    let mut residence_reads = Vec::new();
    for (index, node) in source.nodes().iter().enumerate() {
        let ExpressionNodeSourceV1::Reference(
            crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding),
        ) = node.source()
        else {
            continue;
        };
        if !core.text_operand_is(node.site()) {
            continue;
        }
        if let Some(read) = static_reference(node.site()) {
            if read.read() != node.site()
                || read.binding() != *binding
                || seeded.insert(index, Leaf::Value(Class::String)).is_some()
            {
                return Ok(None);
            }
            static_reads.push(read);
        } else if let Some((current, root, acquisition)) = residence_reference(node.site()) {
            if current != *binding
                || root != acquisition.binding()
                || acquisition.source().owner() != source.owner()
                || (acquisition.owns_root()
                    && !prior_homes.contains(&LiveHomeV1::Text(acquisition.clone())))
                || seeded.insert(index, Leaf::Value(Class::String)).is_some()
            {
                return Ok(None);
            }
            residence_reads.push(super::initializer_expression::TextResidenceReadV1::new(
                index,
                current,
                root,
                acquisition,
            ));
        }
    }
    for read in text_reads {
        if !Rc::ptr_eq(read.source_plan(), &source)
            || !core.text_operand_is(read.read())
            || text_reference(read.read()) != Some((read.binding(), read.formal().1))
        {
            return Ok(None);
        }
        let index = read.node();
        if source.nodes().get(index).is_none_or(|node| node.site() != read.read()
            || !matches!(node.source(), ExpressionNodeSourceV1::Reference(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding)) if *binding == read.binding()))
            || seeded.insert(index, Leaf::Value(Class::String)).is_some() { return Ok(None); }
    }
    if crate::config::env::builder_debug_enabled() {
        eprintln!("[argument-expression] reference seeds complete loop_dynamic={}", loop_dynamic_reads.len());
    }
    let Some(mut prepared) = super::leaves::prepare_expression_calls_v1(
        &source,
        super::leaves::ExpressionLeafProfileV1::Argument,
        &seeded,
        prior_homes,
        source_call,
    )?
    else {
        if crate::config::env::builder_debug_enabled() {
            eprintln!("[argument-expression] nested source call preparation failed");
        }
        return Ok(None);
    };
    let Some(core_i64_inputs) = prepared.i64_core_input_reference_nodes_v1() else {
        return Ok(None);
    };
    let mut scalar_proofs = BTreeMap::new();
    for (node, original) in source.nodes().iter().enumerate() {
        let active = prepared.is_active_reference(node);
        let signed = core.signed_operand_contains(original.site());
        let core_i64 = core_i64_inputs.contains(&node);
        if !active || (!core_i64 && !signed) {
            continue;
        }
        let Some(read) = scalar_reference(original.site()) else {
            if crate::config::env::builder_debug_enabled() {
                eprintln!("[argument-expression] call={call:?} ordinal={ordinal} signed operand {:?} has no current scalar read", original.site());
            }
            continue;
        };
        let proof = CheckedExpressionReferenceV1::Scalar(CheckedScalarReferenceV1::Original(read));
        if !proof.matches_node(&source, node)
            || !prepared.seed_reference(node, Leaf::Value(Class::I64))
            || scalar_proofs.insert(node, proof).is_some()
        {
            if crate::config::env::builder_debug_enabled() {
                eprintln!("[argument-expression] call={call:?} ordinal={ordinal} signed operand {:?} proof failed exact seeding", original.site());
            }
            return Ok(None);
        }
    }
    let Some(super::leaves::QualifiedExpressionLeavesV1 {
        classes,
        mut calls,
        cores,
        fields,
        transferred,
        references,
    }) = super::leaves::qualify_expression_leaves_with_calls_v1::<E>(
        prepared,
        &mut |_| None,
        &order,
        field_request,
    )?
    else {
        if crate::config::env::builder_debug_enabled() {
            eprintln!("[argument-expression] call={call:?} ordinal={ordinal} unresolved expression leaves");
        }
        return Ok(None);
    };
    let dynamic_array_carrier_root = core.dynamic_array_carrier_append_root_v1()
        && core.dynamic_array_carrier_call_is_v1(root.site());
    let expected = if core.requires_i64_v1() {
        Class::I64
    } else if dynamic_array_carrier_root {
        Class::Dynamic
    } else {
        Class::String
    };
    let dynamic_producer_matches = !dynamic_array_carrier_root
        || calls.get(&0).is_some_and(|call| {
            call.result() == crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::Dynamic
                && call.dynamic_invocation_result_lifecycle_v1().is_some()
        });
    if classes[0] != Leaf::Value(expected)
        || transferred.contains(&0)
        || !references.is_empty()
        || !dynamic_producer_matches
    {
        if crate::config::env::builder_debug_enabled() {
            eprintln!("[argument-expression] root rejected class={:?} expected={expected:?} transferred={} unresolved_refs={references:?}", classes.first(), transferred.contains(&0));
        }
        return Ok(None);
    }
    let Some((operations, owned_results)) = super::operations::qualify_arithmetic_operations_v1(
        &source,
        &order,
        &classes,
        &transferred,
    ) else {
        if crate::config::env::builder_debug_enabled() {
            eprintln!("[argument-expression] operator envelope rejected classes={classes:?} binaries={:?}", source.nodes().iter().enumerate().filter_map(|(node, original)| original.binary().map(|binary| (node, binary.operator(), binary.lhs(), binary.rhs()))).collect::<Vec<_>>());
        }
        return Ok(None);
    };
    if core.resolver_source().is_some_and(|original| {
        !cores
            .get(&0)
            .is_some_and(|retained| std::ptr::eq(retained.source(), original))
    }) {
        if crate::config::env::builder_debug_enabled() {
            eprintln!("[argument-expression] original Core source differs from retained Core node");
        }
        return Ok(None);
    }
    let mut reference_proofs = scalar_proofs;
    for (node, proof) in finite_proofs {
        if transferred.contains(&node) || reference_proofs.insert(node, proof).is_some() {
            return Ok(None);
        }
    }
    for (node, proof) in dynamic_proofs {
        if transferred.contains(&node) || reference_proofs.insert(node, proof).is_some() {
            return Ok(None);
        }
    }
    for (node, proof) in loop_dynamic_proofs {
        if transferred.contains(&node) || reference_proofs.insert(node, proof).is_some() {
            return Ok(None);
        }
    }
    for read in static_reads {
        let Some(node) = source
            .nodes()
            .iter()
            .position(|node| node.site() == read.read())
        else {
            return Ok(None);
        };
        if !transferred.contains(&node)
            && reference_proofs
                .insert(node, CheckedExpressionReferenceV1::StaticText(read))
                .is_some()
        {
            return Ok(None);
        }
    }
    for read in residence_reads {
        if !transferred.contains(&read.node())
            && reference_proofs
                .insert(read.node(), CheckedExpressionReferenceV1::Residence(read))
                .is_some()
        {
            return Ok(None);
        }
    }
    for read in text_reads {
        if !transferred.contains(&read.node())
            && reference_proofs
                .insert(
                    read.node(),
                    CheckedExpressionReferenceV1::ArgumentEntry(read.clone()),
                )
                .is_some()
        {
            return Ok(None);
        }
    }
    reference_proofs.retain(|node, _| !transferred.contains(node));
    if reference_proofs
        .iter()
        .any(|(&node, proof)| !proof.matches_node(&source, node))
    {
        return Ok(None);
    }
    let owned_results =
        super::evaluation::owned_result_nodes(&order, &owned_results, &cores, &calls);
    let mut evaluation = CheckedExpressionEvaluationV1 {
        source: Rc::clone(&source),
        prior_homes: prior_homes.into(),
        order: order.clone().into_boxed_slice(),
        classes: classes.into_boxed_slice(),
        references: reference_proofs,
        calls: order.iter().filter_map(|node| calls.remove(node)).collect(),
        operations: operations.into_boxed_slice(),
        owned_results,
        cores,
        transferred_inputs: transferred,
    };
    if !evaluation.retain_core_text_entry_reads_v1() {
        return Ok(None);
    }
    let frame = Rc::new(CheckedArgumentExpressionFrameV1 {
        evaluation,
        source_loan: core,
    });
    if !frame.evaluation().reference_coverage_is_exact() {
        return Ok(None);
    }
    let candidate = HomeExpressionPlanV1 {
        source: Rc::clone(&source),
        scalar_operations: Vec::new(),
        source_fields: Vec::new(),
        print_frame: None,
        condition_frame: None,
        loop_recipe: None,
        argument_frame: Some(frame),
        initializer_frame: None,
        rebind_frame: None,
        field_write: None,
        initializer_homes: None,
    };
    let Some(candidate) = candidate.with_source_field_requirements(&fields) else {
        return Ok(None);
    };
    if !can_install(&candidate) {
        return Ok(None);
    }
    if !field_batch(
        &fields,
        super::super::LocalFieldReadDemandV1::SourceExpression { probe: false, plan: source },
    )? {
        return Ok(None);
    }
    Ok(Some(candidate))
}
