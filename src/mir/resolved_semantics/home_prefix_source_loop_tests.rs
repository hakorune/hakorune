//! Original Header/body/targeted exits, with no After grant from assumptions.
use super::*;
use crate::mir::resolved_semantics::{ResolvedLiteralSourceV1, SourcePathV1, SourcePathSegmentV1 as P};
use crate::mir::resolved_semantics::home_new_prefix::{
    CurrentScalarProducerV1 as Producer, CheckedExpressionReferenceV1 as Reference,
    NoScalarFieldSourceV1, LocalFieldReadResultV1,
};

fn absent_integer_field<E>(
    _: &OwnedExprSiteV1,
    _: &SourceExprSiteV1,
    _: BindingRefV1,
    _: BindingRefV1,
    _: &str,
) -> Result<bool, E> {
    Ok(false)
}

pub(crate) fn assert_original_loop_source_walk_for_test<E: std::fmt::Debug>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    expected_control: bool,
    port: &mut impl FnMut(&OwnedExprSiteV1, super::super::super::super::BorrowedCallActualRequestV1<'_>)
        -> Result<Option<super::super::super::super::BorrowedCallArgumentsV1>, E>,
) {
    let mut locals = PrefixLocalFlow::new(input);
    for initializer in input.function().expression_source().initializers().take(3) {
        let site = initializer.initializer_site().unwrap();
        if matches!(input.function().expression_source().literal(site), Some(ResolvedLiteralSourceV1::String(_))) {
            locals.install_static_text(initializer.binding(), site.clone());
        } else {
            locals.install_observed(initializer.binding(), locals.observe(site).unwrap());
        }
    }
    let policy = crate::mir::builder::LoopFactsPolicyFrameV1::from_values(false, false, false, false, false, true);
    let loop_site = SourcePathV1::root_body(3).stmt();
    let plan = crate::mir::builder::issue_preissued_callable_loop_source_plan_v1(input, &loop_site, policy).unwrap();
    let source = Rc::clone(plan.original_source_v1());
    let body = input.source().root_body().unwrap();
    let statement = input.source().body_stmt(&body, 3).unwrap();
    let mut calls = Vec::new();
    let mut path_calls = BTreeSet::new();
    let mut homes = Vec::new();
    let mut covered = (0..=3).map(|index| SourcePathV1::root_body(index).stmt()).collect();
    let mut summary = crate::mir::resolved_semantics::home_new_prefix::source_loop::observe::<E>(
        input, &statement, &source,
        &BTreeSet::new(), &BTreeMap::new(), &BTreeSet::new(),
        &mut BTreeMap::new(), &mut BTreeMap::new(), &mut Vec::new(),
        &mut calls, &mut path_calls, &mut BTreeMap::new(), &mut BTreeMap::new(), &mut BTreeMap::new(),
        &mut locals, &mut homes, &mut covered, &mut Vec::new(), &mut None,
        &mut absent_integer_field::<E>,
        &mut |_, _| Ok(false), &mut |_| Ok(false), &mut |_| Ok(false), &mut |_| Ok(false),
        &mut |_| Ok(false), &mut |_| Ok(false), &mut |_| Ok(false), &mut |_| Ok(None),
        &mut |_, _, _, _, _| Ok(false), &mut NoScalarFieldSourceV1,
        &mut |_, _, _, _, _| Ok(false), &mut |_, _, _, _, _| Ok(false), &mut |_, _, _, _, _| Ok(false),
        &mut |requests, _| { assert!(requests.is_empty()); Ok(Some(Vec::<LocalFieldReadResultV1>::new())) },
        port, &mut |_, _| Ok(false), &mut |_| Ok(None), &mut |_| Ok(None),
    ).unwrap().expect("complete original source walk; Recipe/After still owed");
    assert!(Rc::ptr_eq(&summary.source, &source));
    let conditions = summary.bool_expressions.as_ref().expect("same checked conditions project to closed regions");
    assert_eq!(conditions.len(), 2);
    assert!(conditions[0].matches_frame(&summary.condition));
    assert!(!conditions[0].matches_frame(&Rc::new(summary.condition.as_ref().clone())),
        "equal reissued condition is not the retained source frame");
    assert!(conditions[0].operation(Vec::new(), crate::mir::loop_recipe_contract::LoopValueKeyV1::new(0)).is_err());
    assert!(!conditions[0].region().may_fault());
    assert!(conditions[1].region().may_fault());
    assert!(conditions[1].matches_frame(summary.body_expressions.iter().find_map(|plan| plan.condition_frame()).unwrap()));
    assert!(matches!(conditions[1].region().nodes.last(), Some(crate::mir::loop_recipe_contract::LoopBoolNodeV1::Or { .. })),
        "preserve original short-circuit operator; no eager Bool arithmetic");
    assert_eq!(conditions[1].imports().len(), 2, "each original ch read is a separate import");
    if !expected_control {
        assert!(!summary.all_ports_available(), "an incomplete execution tree must refuse the whole input");
        assert_eq!(summary.control.as_ref().unwrap_err(), &"source expression has no checked execution root");
        return;
    }
    assert!(summary.all_ports_available(), "refused={} ports={:?}", summary.refused,
        summary.ports.iter().map(|port| &port.unavailable).collect::<Vec<_>>());
    summary.validate_for_recipe_v1(input).unwrap();
    use super::super::control::CheckedSourceLoopStatementV1 as Control;
    let control = summary.control.as_ref().expect("same recursive scan closes exact source control");
    assert_eq!(control.site, input.source().child_body_from_stmt(&statement,
        crate::mir::resolved_semantics::BodyChildRoleV1::LoopBody).unwrap().site().clone());
    assert!(!control.terminated);
    assert_eq!(control.statements.len(), 3);
    assert!(matches!(&control.statements[0], Control::Expression { site, frame }
        if site == frame.source().root().statement() && frame.initializer_frame().is_some()));
    let Control::If { site, bundle, condition, then_body, else_body } = &control.statements[1] else { panic!("original If") };
    assert_eq!(input.function().if_region_bundle(site).unwrap(), bundle);
    assert!(Rc::ptr_eq(condition, conditions[1].frame()));
    assert!(else_body.is_none());
    assert!(then_body.terminated);
    assert_eq!(then_body.statements.len(), 1);
    let Control::Exit { site, record, port } = &then_body.statements[0] else { panic!("one original Break") };
    assert!(matches!(&summary.ports[*port].kind, SourceLoopPortKindV1::Break(original) if original == site));
    assert_eq!(input.function().resolved_exit(&crate::mir::resolved_semantics::ResolvedExitSiteV1::Statement(site.clone())), Some(record));
    assert!(matches!(&control.statements[2], Control::Expression { site, frame }
        if site == frame.source().root().statement() && frame.rebind_frame().is_some()));
    assert_eq!(summary.headers.len(), 1);
    let basis = summary.headers.values().next().unwrap();
    assert!(Rc::ptr_eq(basis.source(), &source));
    let incoming = summary.incoming.as_ref().expect("whole original predecessor census");
    assert_eq!(incoming.len(), 1);
    let incoming = &incoming[0];
    assert!(Rc::ptr_eq(&incoming.basis, basis));
    assert_eq!(basis.flow.loop_join(source.site(), basis.binding(), Join::LoopAfter).unwrap().0, incoming.after);
    assert_eq!(incoming.header_incoming.len(), 2);
    assert_eq!(incoming.after_incoming.len(), 2);
    let backedge = incoming.header_incoming.iter().find(|row| row.edge == Edge::Backedge).unwrap();
    assert!(matches!(&backedge.producer, Producer::BindingRebind(frame) if frame.target() == basis.binding()));
    for edge in incoming.after_incoming.iter() {
        assert_eq!(edge.value, basis.header());
        assert_eq!(edge.origin, basis.header());
        assert!(matches!(&edge.producer, Producer::LoopHeader(actual) if Rc::ptr_eq(actual, basis)));
    }
    assert_eq!(summary.ports.len(), 2);
    let mut breaks = 0;
    let mut backedges = 0;
    for port in &summary.ports {
        match &port.kind {
            SourceLoopPortKindV1::Break(site) => {
                breaks += 1;
                assert_eq!(site, &SourcePathV1::root_body(3).child(P::LoopBody(1)).child(P::IfThen(0)).stmt());
                assert_eq!(port.crossed.len(), 2);
            }
            SourceLoopPortKindV1::Backedge => { backedges += 1; assert_eq!(port.crossed.len(), 1); }
            _ => panic!("unexpected source exit"),
        }
        assert_eq!(port.homes.len(), 1);
        assert_eq!(port.normal_ends.as_ref(), port.homes.as_ref());
        let StoredLocal::Text { acquisition, .. } = &port.locals[&port.normal_ends[0].binding()] else { panic!("owned substring") };
        assert_eq!(crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1::Text(acquisition.clone()), port.normal_ends[0]);
        assert_eq!(input.function().scope(*port.crossed.last().unwrap()).unwrap().kind(), ScopeKindV1::LoopBody);
    }
    assert_eq!((breaks, backedges), (1, 1));
    // A summary is a complete census, never whichever paths happened to work.
    for removed in 0..summary.ports.len() {
        let mut ports = summary.ports.clone();
        ports.remove(removed);
        assert_eq!(verify_incoming(&locals, &source, &summary.headers, &ports, false, &calls).unwrap_err(),
            "source Loop predecessor census mismatch");
    }
    let mut ports = summary.ports.clone();
    ports.push(ports[0].clone());
    assert_eq!(verify_incoming(&locals, &source, &summary.headers, &ports, false, &calls).unwrap_err(),
        "duplicate source Loop port");
    ports.pop();
    for port in &mut ports { port.locals.insert(basis.binding(), StoredLocal::Trivial(Some(SourceScalarKind::Bool))); }
    assert_eq!(verify_incoming(&locals, &source, &summary.headers, &ports, false, &calls).unwrap_err(),
        "source Loop incoming scalar class");
    assert_eq!(verify_incoming(&locals, &source, &summary.headers, &summary.ports, true, &calls).unwrap_err(),
        "source Loop has an unavailable path");
    assert_eq!(verify_incoming(&locals, &source, &BTreeMap::new(), &summary.ports, false, &calls).unwrap_err(),
        "source Loop carrier census mismatch");
    let mut ports = summary.ports.clone();
    for port in &mut ports { port.covered = Box::new([]); }
    assert_eq!(verify_incoming(&locals, &source, &summary.headers, &ports, false, &calls).unwrap_err(),
        "source Loop incoming producer outside path");
    let references: Vec<_> = summary.condition.references().values().filter_map(|reference| match reference {
        Reference::Scalar(read) => read.original_read(), _ => None,
    }).collect();
    assert_eq!(references.len(), 2);
    assert!(references.iter().any(|read| matches!(read.producer(), Producer::LoopHeader(actual) if Rc::ptr_eq(actual, basis))));
    let plans = locals.expression_plans();
    assert_eq!(summary.body_expressions.len(), 3);
    assert_eq!(summary.body_expressions.iter().filter(|plan| plan.initializer_frame().is_some()).count(), 1);
    assert_eq!(summary.body_expressions.iter().filter(|plan| plan.condition_frame().is_some()).count(), 1);
    assert_eq!(summary.body_expressions.iter().filter(|plan| plan.rebind_frame().is_some()).count(), 1);
    for retained in summary.body_expressions.iter() {
        let original = plans.iter().find(|plan| Rc::ptr_eq(plan.source(), retained.source())).unwrap();
        match (retained.initializer_frame(), retained.condition_frame(), retained.rebind_frame()) {
            (Some(frame), _, _) => assert!(Rc::ptr_eq(frame, original.initializer_frame().unwrap())),
            (_, Some(frame), _) => assert!(Rc::ptr_eq(frame, original.condition_frame().unwrap())),
            (_, _, Some(frame)) => assert!(Rc::ptr_eq(frame, original.rebind_frame().unwrap())),
            _ => panic!("original checked body root"),
        }
        assert!(!Rc::ptr_eq(retained.source(), summary.condition.source()), "product must not retain its own inventory slot");
    }
    let rebind = plans.iter().find_map(|plan| plan.rebind_frame()).unwrap();
    assert!(rebind.evaluation().references().values().any(|reference|
        matches!(reference, Reference::Scalar(read) if matches!(read.original_read().map(|read| read.producer()),
            Some(Producer::LoopHeader(actual)) if Rc::ptr_eq(actual, basis)))));
    assert!(homes.is_empty(), "body acquisition never becomes an outer Home");
    assert!(!locals.in_source_loop_v1(), "scope assumptions do not escape");
    assert!(locals.current_scalar_read_v1(&SourcePathV1::root_body(4).child(P::Value).expr(), &calls).is_none(),
        "Header/body source success cannot grant After");
    // Each immutable component is necessary to close the whole source product.
    let roots = std::mem::replace(&mut summary.body_expressions, Box::new([]));
    assert_eq!(summary.validate_for_recipe_v1(input), Err("source Loop execution root census"));
    summary.body_expressions = roots;
    let ports = std::mem::take(&mut summary.ports);
    assert_eq!(summary.validate_for_recipe_v1(input), Err("source Loop Exit port missing"));
    summary.ports = ports;
    let conditions = std::mem::replace(&mut summary.bool_expressions, Err("missing projection".to_owned()));
    assert_eq!(summary.validate_for_recipe_v1(input), Err("source Loop Bool projection refused"));
    summary.bool_expressions = conditions;
    let incoming = std::mem::replace(&mut summary.incoming, Err("missing incoming closure"));
    assert_eq!(summary.validate_for_recipe_v1(input), Err("missing incoming closure"));
    summary.incoming = incoming;
    summary.validate_for_recipe_v1(input).unwrap();
    let product = crate::mir::loop_recipe_contract::produce_source_loop_recipe_v2(input, summary)
        .expect("same complete source tree reaches typed V2 and all-carrier JoinSig");
    assert!(Rc::ptr_eq(&product.source.source, &source));
    let recipe = product.recipe.as_recipe();
    assert_eq!(recipe.carriers.len(), 1, "only the original carried I64 has a Header");
    assert_eq!(recipe.read_only_inputs.len(), 2, "n/text are original readonly operands");
    use crate::mir::loop_recipe_contract::{LoopRecipeItemV2 as Item, LoopOperationV2 as Operation};
    assert_eq!(recipe.items.iter().filter(|row| matches!(row.item, Item::If { .. })).count(), 1);
    assert_eq!(recipe.exits.len(), 1, "one original Break, no internal Bool exit");
    assert_eq!(recipe.items.iter().filter(|row| matches!(row.item,
        Item::Operation { operation: Operation::WriteBinding { .. } })).count(), 1,
        "substring local is an SSA result, not another carried binding");
    assert_eq!(recipe.items.iter().filter(|row| matches!(row.item,
        Item::Operation { operation: Operation::CallSlot { .. } })).count(), 1);
    assert_eq!(recipe.items.iter().filter(|row| matches!(row.item,
        Item::Operation { operation: Operation::BoolExpression { .. } })).count(), 2);
    assert_eq!(product.source.ports.len(), 2, "retain both original cleanup obligations");
    assert_eq!(product.execution.len(), 4, "Loop condition, initializer, If condition and rebind");
    let faults: Vec<_> = product.execution.iter().flat_map(|evaluation| evaluation.faults.iter()
        .map(move |fault| (evaluation, fault))).collect();
    assert_eq!(faults.len(), 3, "substring plus two conditional Text comparisons");
    for (evaluation, fault) in &faults {
        assert!(fault.prior_acquisitions.is_empty(), "failed substring never owns its result");
        assert_eq!(fault.continuation.site().site(), evaluation.source.nodes()[fault.node].site());
        assert_eq!(fault.continuation.target_function(), input.function().function_region());
        assert_eq!(fault.continuation.crossed_scopes().len(), 1);
    }
    assert!(product.execution.iter().all(|evaluation| evaluation.normal_temporary_ends.is_empty()),
        "only substring root transfers to ch, which ends at the real Loop ports");
    let product = Rc::new(product);
    let after_site = SourcePathV1::root_body(4).child(P::Value).expr();
    let binding = *product.source.headers.keys().next().unwrap();
    assert!(!locals.apply_source_loop_after_v2(Rc::clone(&product), &[crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1::Stable(binding)]),
        "wrong entry Home snapshot cannot publish even one carrier");
    assert!(locals.current_scalar_read_v1(&after_site, &calls).is_none());
    assert!(locals.apply_source_loop_after_v2(Rc::clone(&product), &homes));
    assert!(!locals.apply_source_loop_after_v2(Rc::clone(&product), &homes), "one original slot attaches once");
    let read = locals.current_scalar_read_v1(&after_site, &calls).expect("original source After read");
    assert!(matches!(read.producer(), Producer::LoopAfter { product: actual, binding: actual_binding }
        if Rc::ptr_eq(actual, &product) && *actual_binding == binding));
    let weak = Rc::downgrade(&product);
    drop(read);
    drop(locals);
    let product = Rc::try_unwrap(product)
        .expect("completed product must not own its own inventory slot");
    assert!(weak.upgrade().is_none(), "completed product must not own its own inventory slot");
    // A pre-loop source producer cannot stand in for the actual backedge update.
    let mut candidate = product.source;
    let row = &mut candidate.incoming.as_mut().unwrap()[0];
    let backedge = row.header_incoming.iter_mut().find(|row| row.edge == Edge::Backedge).unwrap();
    backedge.producer = row.basis.entry_producer().clone();
    let error = crate::mir::loop_recipe_contract::produce_source_loop_recipe_v2(input, candidate).unwrap_err();
    assert!(error.contains("incoming-payload-value"), "{error}");
}
