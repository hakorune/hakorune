//! Original Plain binding-rebind purpose over the shared expression body.
//! Qualification neither assigns a physical value nor closes a Loop Header.
use super::super::{BindingRefV1, SourceExprSiteV1};
use super::*;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::home_new_prefix::LoopHeaderValueBasisV1;
use crate::mir::resolved_semantics::{ResolvedAssignmentFormV1, ResolvedAssignmentTargetV1};
use crate::mir::resolved_value_profile::{
    expression_children::{ExpressionChildReachV1, ExpressionChildrenV1},
    expression_class::ExpressionValueClassV1 as Class,
    expression_plan::ExpressionPlanRootV1,
    expression_projection::ExpressionLeafClassV1 as Leaf,
};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BindingRebindDestinationV1 {
    Integer,
    Bool,
    DynamicLoopCarrier(Rc<LoopHeaderValueBasisV1>),
    TextStatic(SourceExprSiteV1),
    TextResidence(TextResidenceAcquisitionV1),
    TextLoopCarrier(Rc<LoopHeaderValueBasisV1>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CheckedBindingRebindFrameV1 {
    evaluation: CheckedExpressionEvaluationV1,
    target: BindingRefV1,
    destination: BindingRebindDestinationV1,
}
impl CheckedBindingRebindFrameV1 {
    pub(crate) fn evaluation(&self) -> &CheckedExpressionEvaluationV1 {
        &self.evaluation
    }
    pub(crate) fn source(&self) -> &Rc<ExpressionSourcePlanV1> {
        self.evaluation.source()
    }
    pub(crate) fn target(&self) -> BindingRefV1 {
        self.target
    }
    pub(crate) fn is_text_residence(&self) -> bool {
        matches!(
            self.destination,
            BindingRebindDestinationV1::TextStatic(_)
                | BindingRebindDestinationV1::TextResidence(_)
                | BindingRebindDestinationV1::TextLoopCarrier(_)
        )
    }
    pub(crate) fn materializes_root(&self) -> bool {
        self.is_text_residence() && self.evaluation.owned_text_root_node_v1().is_none()
    }
    pub(crate) fn previous_text(&self) -> Option<&TextResidenceAcquisitionV1> {
        match &self.destination {
            BindingRebindDestinationV1::TextResidence(previous) => Some(previous),
            _ => None,
        }
    }
    pub(crate) fn text_loop_carrier_v1(&self) -> Option<&Rc<LoopHeaderValueBasisV1>> {
        match &self.destination {
            BindingRebindDestinationV1::TextLoopCarrier(basis) => Some(basis),
            _ => None,
        }
    }
    pub(crate) fn is_dynamic_loop_carrier_v1(&self) -> bool {
        matches!(self.destination, BindingRebindDestinationV1::DynamicLoopCarrier(_))
    }
    pub(crate) fn text_acquisition_v1(self: &Rc<Self>) -> TextResidenceAcquisitionV1 {
        TextResidenceAcquisitionV1::from_rebind(Rc::clone(self))
    }
}

pub(crate) fn qualify_binding_rebind_expression_v1<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    prepared: super::leaves::PreparedExpressionCallsV1,
    destination: BindingRebindDestinationV1,
    static_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<super::super::StaticTextReadV1>,
    residence_reference: &mut impl FnMut(
        &SourceExprSiteV1,
    ) -> Option<(
        BindingRefV1,
        BindingRefV1,
        TextResidenceAcquisitionV1,
    )>,
    loop_text_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<super::super::LoopTextReadV1>,
    scalar_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<super::super::CurrentScalarReadV1>,
    field_request: &mut impl FnMut(&SourceExprSiteV1) -> Option<LocalFieldReadRequestV1>,
    can_install: &mut impl FnMut(&HomeExpressionPlanV1) -> bool,
    field_batch: &mut impl FnMut(
        &[LocalFieldReadRequestV1],
        super::super::LocalFieldReadDemandV1,
    ) -> Result<bool, E>,
) -> Result<Option<HomeExpressionPlanV1>, E> {
    qualify_binding_rebind_expression_with_loop_dynamic_v1(
        input,
        prepared,
        destination,
        static_reference,
        residence_reference,
        loop_text_reference,
        &mut |_| None,
        &mut |_| None,
        scalar_reference,
        field_request,
        can_install,
        field_batch,
    )
}

pub(crate) fn qualify_binding_rebind_expression_with_loop_dynamic_v1<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    mut prepared: super::leaves::PreparedExpressionCallsV1,
    destination: BindingRebindDestinationV1,
    static_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<super::super::StaticTextReadV1>,
    residence_reference: &mut impl FnMut(
        &SourceExprSiteV1,
    ) -> Option<(
        BindingRefV1,
        BindingRefV1,
        TextResidenceAcquisitionV1,
    )>,
    loop_text_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<super::super::LoopTextReadV1>,
    dynamic_invocation_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<Rc<CheckedInitializerExpressionFrameV1>>,
    loop_dynamic_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<super::super::LoopDynamicReadV1>,
    scalar_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<super::super::CurrentScalarReadV1>,
    field_request: &mut impl FnMut(&SourceExprSiteV1) -> Option<LocalFieldReadRequestV1>,
    can_install: &mut impl FnMut(&HomeExpressionPlanV1) -> bool,
    field_batch: &mut impl FnMut(
        &[LocalFieldReadRequestV1],
        super::super::LocalFieldReadDemandV1,
    ) -> Result<bool, E>,
) -> Result<Option<HomeExpressionPlanV1>, E> {
    if !prepared.is_binding_rebind() {
        return Ok(None);
    }
    let source = Rc::clone(prepared.source());
    let ExpressionPlanRootV1::Assignment { assignment } = source.root() else {
        return Ok(None);
    };
    macro_rules! reject_rebind {
        ($stage:literal) => {{ let _ = $stage; return Ok(None); }};
    }
    let Some(ResolvedAssignmentTargetV1::BindingRebind(target)) =
        input.function().assignment_target(assignment.target_site())
    else {
        return Ok(None);
    };
    if input.owner() != source.owner()
        || target.owner() != source.owner()
        || assignment.form() != ResolvedAssignmentFormV1::Plain
        || source
            .nodes()
            .first()
            .is_none_or(|root| root.site() != assignment.value_site())
        || input.body_shape().is_none_or(|shape| {
            shape
                .assignment_sources()
                .iter()
                .filter(|row| row.statement_site() == assignment.statement_site())
                .ne(std::iter::once(assignment))
        })
    {
        reject_rebind!("source-identity-or-assignment-shape");
    }
    let prior_homes = prepared.prior_homes().to_vec();
    let mut unique_homes = BTreeSet::new();
    if prior_homes
        .iter()
        .any(|home| home.owner() != source.owner() || !unique_homes.insert(home.binding()))
    {
        reject_rebind!("prior-homes");
    }
    match &destination {
        BindingRebindDestinationV1::Integer
        | BindingRebindDestinationV1::Bool
        | BindingRebindDestinationV1::TextStatic(_)
            if prior_homes.iter().any(|home| home.binding() == *target) =>
        {
            return Ok(None)
        }
        BindingRebindDestinationV1::TextResidence(previous)
            if previous.binding() != *target
                || prior_homes
                    .iter()
                    .filter(|home| **home == super::super::LiveHomeV1::Text(previous.clone()))
                    .count()
                    != 1 =>
        {
            return Ok(None)
        }
        BindingRebindDestinationV1::TextLoopCarrier(basis)
            if basis.binding() != *target
                || basis.source().owner() != source.owner()
                || basis.entry_producer().class() != Class::String =>
        {
            return Ok(None)
        }
        BindingRebindDestinationV1::DynamicLoopCarrier(basis)
            if basis.binding() != *target
                || basis.source().owner() != source.owner()
                || basis.entry_producer().class() != Class::Dynamic
                || prior_homes.iter().any(|home| home.binding() == *target) =>
        {
            return Ok(None)
        }
        _ => {}
    }
    let Ok(order) = source.ordered_postorder_v1() else {
        reject_rebind!("postorder");
    };
    if source.nodes().iter().any(|node| {
        !matches!(node.children(), ExpressionChildrenV1::Known(children)
        if children.iter().all(|child| child.reach == ExpressionChildReachV1::Always))
    }) {
        reject_rebind!("conditional-expression-child");
    }
    let reference_order: Vec<_> = (0..source.nodes().len()).collect();
    let initial_dynamic_references = source
        .nodes()
        .iter()
        .enumerate()
        .filter_map(|(node, original)| {
            (prepared.is_active_reference(node)
                && matches!(original.source(), ExpressionNodeSourceV1::Reference(_)))
                .then(|| dynamic_invocation_reference(original.site())
                    .map(|frame| (node, CheckedExpressionReferenceV1::DynamicInvocationResult(frame)))
                    .or_else(|| loop_dynamic_reference(original.site())
                        .map(|read| (node, CheckedExpressionReferenceV1::LoopDynamic(read))))
                ).flatten()
        })
        .collect();
    let Some(mut reference_proofs) =
        super::evaluation::seed_current_expression_references_with_loop_text_v1(
            &mut prepared,
            initial_dynamic_references,
            &reference_order,
            static_reference,
            residence_reference,
            loop_text_reference,
            &mut |site| {
                scalar_reference(site).map(super::evaluation::CheckedScalarReferenceV1::Original)
            },
        )
    else {
        reject_rebind!("reference-seed");
    };
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
        reject_rebind!("leaf-qualification");
    };
    let class = match destination {
        BindingRebindDestinationV1::Integer => Class::I64,
        BindingRebindDestinationV1::Bool => Class::Bool,
        BindingRebindDestinationV1::DynamicLoopCarrier(_) => Class::Dynamic,
        _ => Class::String,
    };
    let text = class == Class::String;
    if (class != Class::String && !references.is_empty())
        || transferred.contains(&0)
        || classes[0] != Leaf::Value(class)
    {
        reject_rebind!("root-class-or-transfer");
    }
    let Some((operations, owned_results)) = super::operations::qualify_arithmetic_operations_v1(
        &source,
        &order,
        &classes,
        &transferred,
    ) else {
        reject_rebind!("arithmetic-operations");
    };
    let owned_results =
        super::evaluation::owned_result_nodes(&order, &owned_results, &cores, &calls);
    // The checked purpose materializes non-owning Text roots; evaluation keeps
    // the original literal/read and its source acquisition set unchanged.
    if !text && owned_results.contains(&0) {
        reject_rebind!("owned-result-destination");
    }
    reference_proofs.retain(|node, _| !transferred.contains(node));
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
        reject_rebind!("core-text-entry-reads");
    }
    let frame = Rc::new(CheckedBindingRebindFrameV1 {
        target: *target,
        destination,
        evaluation,
    });
    if !frame.evaluation().reference_coverage_is_exact() {
        reject_rebind!("reference-coverage");
    }
    let candidate = HomeExpressionPlanV1 {
        source: Rc::clone(&source),
        scalar_operations: Vec::new(),
        source_fields: Vec::new(),
        source_field_homes: None,
        argument_frame: None,
        initializer_frame: None,
        rebind_frame: Some(frame),
        print_frame: None,
        condition_frame: None,
        loop_recipe: None,
        field_write: None,
    };
    let Some(candidate) = candidate.with_source_field_requirements(&fields) else {
        reject_rebind!("field-requirements");
    };
    if !can_install(&candidate) {
        reject_rebind!("installation");
    }
    if !field_batch(
        &fields,
        super::super::LocalFieldReadDemandV1::SourceExpression { probe: false, plan: Rc::clone(&source) },
    )? {
        reject_rebind!("field-batch");
    }
    Ok(Some(candidate))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
    use crate::mir::resolved_semantics::home_new_prefix::local_flow::PrefixLocalFlow;
    use crate::mir::resolved_value_profile::expression_plan::assignment_expression_source_plan_v1;

    #[test]
    fn production_rebind_walk_refuses_owned_unknown_and_noninteger_destinations() {
        for case in 0..7 {
            let program = crate::parser::NyashParser::parse_from_string(
                "function probe() { local x = 0 x = x + 1 return 0 }",
            )
            .unwrap();
            let crate::ast::ASTNode::Program { mut statements, .. } = program else {
                panic!("Program")
            };
            let unit =
                VerifiedResolvedSourceUnitV1::resolve_function(statements.remove(0)).unwrap();
            let input = unit.root_function_input().unwrap();
            let initializer = input
                .function()
                .expression_source()
                .initializers()
                .next()
                .unwrap();
            let binding = initializer.binding();
            let mut locals = PrefixLocalFlow::new(input);
            use super::super::super::local_flow::OrdinaryObservation as Observation;
            match case {
                0 | 1 | 6 => locals.install_observed(binding, Observation::Integer(0)),
                2 => locals.install_observed(binding, Observation::Bool(false)),
                3 => locals.install_observed(binding, Observation::Null),
                4 => locals.install_uninitialized(binding),
                5 => locals.install_observed(binding, Observation::Handle(binding)),
                _ => unreachable!(),
            }
            let mut homes = if case == 1 {
                vec![super::super::super::LiveHomeV1::Stable(binding)]
            } else {
                vec![]
            };
            let body = input.source().root_body().unwrap();
            let statement = input.source().body_stmt(&body, 1).unwrap();
            let mut calls = Vec::new();
            let mut paths = Default::default();
            let accepted = super::super::super::binding_rebind::observe::<()>(
                input,
                &statement,
                &mut locals,
                &mut homes,
                case != 6,
                &mut calls,
                &mut paths,
                &mut |fields, _| {
                    assert!(fields.is_empty());
                    Ok(Some(Vec::new()))
                },
                &mut |_, _| panic!("fixture has no calls"),
            )
            .unwrap();
            assert_eq!(accepted, case == 0, "destination case {case}");
            assert_eq!(locals.expression_plans().len(), usize::from(case == 0));
        }
    }

    #[test]
    fn rebind_qualification_preserves_assignment_purpose_and_atomic_source_installation() {
        for (assignment, eligible) in [
            ("x = 1", true),
            ("x = 1 + 2", true),
            ("x = x + 1", true),
            ("x = x", true),
            ("x += 1", false),
            ("p.value = 1", false),
            ("x = true", false),
            ("x = \"a\" + \"b\"", false),
            ("x = 1 == 1", false),
            ("x = p", false),
        ] {
            let program = crate::parser::NyashParser::parse_from_string(&format!(
                "function probe(p) {{ local x = 0 {assignment} return x }}"
            ))
            .unwrap();
            let crate::ast::ASTNode::Program { mut statements, .. } = program else {
                panic!("Program")
            };
            let unit =
                VerifiedResolvedSourceUnitV1::resolve_function(statements.remove(0)).unwrap();
            let input = unit.root_function_input().unwrap();
            let ledger = input
                .forest()
                .callable_source_ledger(input.owner())
                .unwrap();
            let row = &input.body_shape().unwrap().assignment_sources()[0];
            let source =
                assignment_expression_source_plan_v1(&ledger, input.body_shape().unwrap(), row)
                    .unwrap();
            let initializer = input
                .function()
                .expression_source()
                .initializers()
                .next()
                .unwrap();
            for (profile, known, batch_ok) in [
                (leaves::ExpressionLeafProfileV1::BindingRebind, true, false),
                (leaves::ExpressionLeafProfileV1::BindingRebind, true, true),
                (leaves::ExpressionLeafProfileV1::BindingRebind, false, true),
                (
                    leaves::ExpressionLeafProfileV1::LocalInitializer,
                    true,
                    true,
                ),
            ] {
                let mut locals = PrefixLocalFlow::new(input);
                let observed = locals
                    .observe(initializer.initializer_site().unwrap())
                    .unwrap();
                locals.install_observed(initializer.binding(), observed);
                if !known {
                    locals.install_uninitialized(initializer.binding());
                }
                let sibling = locals.clone();
                let prepared = leaves::prepare_expression_calls_v1::<()>(
                    &source,
                    profile,
                    &Default::default(),
                    &[],
                    &mut |_| panic!("no Call in fixture"),
                )
                .unwrap()
                .unwrap();
                let mut installation = None;
                let mut batches = 0;
                let plan = qualify_binding_rebind_expression_v1::<()>(input, prepared, BindingRebindDestinationV1::Integer,
                    &mut |_| None, &mut |_| None, &mut |_| None, &mut |site| locals.current_scalar_read_v1(site, &[]),
                    &mut |_| None, &mut |plan| {
                        let mut pending = locals.expression_installation();
                        if !pending.approve(plan) { return false; }
                        installation = Some(pending); true
                    },
                    &mut |requests, demand| {
                        assert!(requests.is_empty());
                        assert!(matches!(demand, super::super::super::LocalFieldReadDemandV1::SourceExpression { plan, .. }
                            if Rc::ptr_eq(&plan, &source)));
                        batches += 1; Ok(batch_ok)
                    }).unwrap();
                let reference = source
                    .nodes()
                    .iter()
                    .any(|node| matches!(node.source(), ExpressionNodeSourceV1::Reference(_)));
                let qualifies = eligible
                    && (known || !reference)
                    && profile == leaves::ExpressionLeafProfileV1::BindingRebind;
                assert_eq!(
                    plan.is_some(),
                    qualifies && batch_ok,
                    "{assignment}, known={known}"
                );
                assert_eq!(batches, usize::from(qualifies), "{assignment}");
                if let Some(plan) = plan {
                    let frame = Rc::clone(plan.rebind_frame().unwrap());
                    assert!(Rc::ptr_eq(frame.source(), &source));
                    assert_eq!(frame.target(), initializer.binding());
                    assert_eq!(frame.evaluation().root_class(), Leaf::Value(Class::I64));
                    assert!(frame.evaluation().owned_result_nodes().is_empty());
                    assert!(
                        plan.initializer_frame().is_none() && plan.field_write_frame().is_none()
                    );
                    assert!(plan.scalar_operations().next().is_none());
                    installation.take().unwrap().commit();
                    let installed = sibling.expression_plans();
                    assert!(Rc::ptr_eq(installed[0].rebind_frame().unwrap(), &frame));
                    let mut duplicate = sibling.expression_installation();
                    assert!(!duplicate.approve(&plan));
                } else {
                    drop(installation);
                    assert!(
                        sibling.expression_plans().is_empty(),
                        "failed batch must not publish a frame"
                    );
                }
            }
        }
    }

    #[test]
    fn source_scalar_reads_keep_rebind_producers_before_following_aliases() {
        let program = crate::parser::NyashParser::parse_from_string(
            "function probe() { local x = 0 x = x + 1 local y = x x = x local z = x return y + z + x }").unwrap();
        let crate::ast::ASTNode::Program { mut statements, .. } = program else {
            panic!("Program")
        };
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(statements.remove(0)).unwrap();
        let input = unit.root_function_input().unwrap();
        let ledger = input
            .forest()
            .callable_source_ledger(input.owner())
            .unwrap();
        let initializers: Vec<_> = input
            .function()
            .expression_source()
            .initializers()
            .collect();
        let mut locals = PrefixLocalFlow::new(input);
        let observed = locals
            .observe(initializers[0].initializer_site().unwrap())
            .unwrap();
        locals.install_observed(initializers[0].binding(), observed);
        let mut frames = Vec::new();
        for (index, assignment) in input
            .body_shape()
            .unwrap()
            .assignment_sources()
            .iter()
            .enumerate()
        {
            let source = assignment_expression_source_plan_v1(
                &ledger,
                input.body_shape().unwrap(),
                assignment,
            )
            .unwrap();
            let prepared = leaves::prepare_expression_calls_v1::<()>(
                &source,
                leaves::ExpressionLeafProfileV1::BindingRebind,
                &Default::default(),
                &[],
                &mut |_| panic!("no Call"),
            )
            .unwrap()
            .unwrap();
            let mut installation = None;
            let plan = qualify_binding_rebind_expression_v1::<()>(
                input,
                prepared,
                BindingRebindDestinationV1::Integer,
                &mut |_| None,
                &mut |_| None,
                &mut |_| None,
                &mut |site| locals.current_scalar_read_v1(site, &[]),
                &mut |_| None,
                &mut |plan| {
                    let mut pending = locals.expression_installation();
                    if !pending.approve(plan) {
                        return false;
                    }
                    installation = Some(pending);
                    true
                },
                &mut |_, _| Ok(true),
            )
            .unwrap()
            .unwrap();
            let frame = Rc::clone(plan.rebind_frame().unwrap());
            if index == 1 {
                assert!(frame.evaluation().references().values().any(|read| matches!(read,
                    CheckedExpressionReferenceV1::Scalar(read) if read.original_read().is_some_and(|read|
                        matches!(read.producer(), crate::mir::resolved_semantics::home_new_prefix::CurrentScalarProducerV1::BindingRebind(original)
                            if Rc::ptr_eq(original, &frames[0]))))));
            }
            installation.take().unwrap().commit();
            drop(installation);
            frames.push(frame);
            if index == 0 {
                let observed = locals
                    .observe(initializers[1].initializer_site().unwrap())
                    .unwrap();
                locals.install_observed(initializers[1].binding(), observed);
            }
        }
        let observed = locals
            .observe(initializers[2].initializer_site().unwrap())
            .unwrap();
        locals.install_observed(initializers[2].binding(), observed);
        let returned = input.function().variable_refs().filter(|(site, _)| {
            site.node().segments().first()
                == Some(&crate::mir::resolved_semantics::SourcePathSegmentV1::Body(
                    5,
                ))
        });
        let mut count = 0;
        for (site, _) in returned {
            let read = locals.current_scalar_read_v1(site, &[]).unwrap();
            let crate::mir::resolved_semantics::home_new_prefix::CurrentScalarProducerV1::BindingRebind(frame) = read.producer()
                else { panic!("an assignment is not an initializer/entry") };
            let expected = if read.binding() == initializers[1].binding() {
                &frames[0]
            } else {
                &frames[1]
            };
            assert!(Rc::ptr_eq(frame, expected));
            count += 1;
        }
        assert_eq!(count, 3);
    }
}
