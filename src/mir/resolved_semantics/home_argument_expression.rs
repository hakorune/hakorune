//! Completed semantic argument qualification at the real covered Prefix point.
//! Mandatory entry requirements remain obligations, not physical descriptors.
use super::super::{BindingRefV1, OwnedExprSiteV1, SourceExprSiteV1, SourceScalarKind};
use super::*;
use crate::mir::normal_callable_semantic_package::{
    ArgumentExpressionSourceLoanV1, BorrowedTextEntryReadRequirementV1,
};
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use crate::mir::resolved_value_profile::{
    expression_class::ExpressionValueClassV1 as Class, expression_plan::ExpressionPlanRootV1,
    expression_projection::ExpressionLeafClassV1 as Leaf,
};
use std::collections::BTreeMap;

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
            && self.evaluation.owned_text_root_node_v1().is_none()
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
    if !source.admits_prior_homes_v1(prior_homes) {
        return Ok(None);
    }
    if !source.children_always_known_v1() {
        return Ok(None);
    }
    if crate::config::env::builder_debug_enabled() {
        eprintln!(
            "[argument-expression] source preflight complete nodes={}",
            source.nodes().len()
        );
    }
    let mut seed_nodes = std::collections::BTreeSet::new();
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
            || !seed_nodes.insert(node)
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
            || !seed_nodes.insert(node)
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
            || !seed_nodes.insert(node)
            || loop_dynamic_proofs
                .insert(
                    node,
                    CheckedExpressionReferenceV1::LoopDynamic(read.clone()),
                )
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
            if read.read() != node.site() || read.binding() != *binding || !seed_nodes.insert(index)
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
                || !seed_nodes.insert(index)
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
            || !seed_nodes.insert(index) { return Ok(None); }
    }
    if crate::config::env::builder_debug_enabled() {
        eprintln!(
            "[argument-expression] reference seeds complete loop_dynamic={}",
            loop_dynamic_reads.len()
        );
    }
    let Some(prepared) = super::leaves::prepare_expression_calls_v1(
        &source,
        super::leaves::ExpressionLeafProfileV1::Argument,
        &BTreeMap::new(),
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
    let mut initial = Vec::new();
    for (node, proof) in finite_proofs
        .into_iter()
        .chain(dynamic_proofs)
        .chain(loop_dynamic_proofs)
    {
        if prepared.transfers_node_v1(node) {
            return Ok(None);
        }
        initial.push((node, proof));
    }
    for read in static_reads {
        let Some(node) = source
            .nodes()
            .iter()
            .position(|node| node.site() == read.read())
        else {
            return Ok(None);
        };
        if !prepared.transfers_node_v1(node) {
            initial.push((node, CheckedExpressionReferenceV1::StaticText(read)));
        }
    }
    for read in residence_reads {
        if !prepared.transfers_node_v1(read.node()) {
            initial.push((read.node(), CheckedExpressionReferenceV1::Residence(read)));
        }
    }
    for read in text_reads {
        if !prepared.transfers_node_v1(read.node()) {
            initial.push((
                read.node(),
                CheckedExpressionReferenceV1::ArgumentEntry(read.clone()),
            ));
        }
    }
    for (node, original) in source.nodes().iter().enumerate() {
        if !prepared.is_active_reference(node)
            || seed_nodes.contains(&node)
            || !(core.signed_operand_contains(original.site()) || core_i64_inputs.contains(&node))
        {
            continue;
        }
        let Some(read) = scalar_reference(original.site()) else {
            continue;
        };
        let proof = CheckedExpressionReferenceV1::Scalar(CheckedScalarReferenceV1::Original(read));
        if !proof.matches_node(&source, node)
            || !matches!(&proof, CheckedExpressionReferenceV1::Scalar(read) if read.kind() == SourceScalarKind::Integer)
        {
            return Ok(None);
        }
        initial.push((node, proof));
    }
    let Ok((evaluation, fields, _)) = super::qualification::qualify_expression_evaluation_v1(
        prepared,
        initial,
        &mut |_| None,
        &mut |_| None,
        &mut |_| None,
        &mut |_| None,
        field_request,
    )?
    else {
        return Ok(None);
    };
    let root_call = evaluation
        .calls()
        .iter()
        .find(|call| call.site().site() == root.site());
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
        || root_call.is_some_and(|call| {
            call.result()
                == crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::Dynamic
                && call.dynamic_invocation_result_lifecycle_v1().is_some()
        });
    if evaluation.root_class() != Leaf::Value(expected)
        || evaluation.transfers_node(0)
        || !dynamic_producer_matches
    {
        if crate::config::env::builder_debug_enabled() {
            eprintln!("[argument-expression] root rejected class={:?} expected={expected:?} transferred={}", evaluation.root_class(), evaluation.transfers_node(0));
        }
        return Ok(None);
    }
    if core.resolver_source().is_some_and(|original| {
        !evaluation
            .cores()
            .get(&0)
            .is_some_and(|retained| std::ptr::eq(retained.source(), original))
    }) {
        if crate::config::env::builder_debug_enabled() {
            eprintln!("[argument-expression] original Core source differs from retained Core node");
        }
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
        argument_frame: Some(frame),
        ..HomeExpressionPlanV1::for_source(Rc::clone(&source))
    };
    let Some(candidate) = candidate.with_source_field_requirements(&fields) else {
        return Ok(None);
    };
    if !can_install(&candidate) {
        return Ok(None);
    }
    if !field_batch(
        &fields,
        super::super::LocalFieldReadDemandV1::SourceExpression {
            probe: false,
            plan: source,
        },
    )? {
        return Ok(None);
    }
    Ok(Some(candidate))
}
