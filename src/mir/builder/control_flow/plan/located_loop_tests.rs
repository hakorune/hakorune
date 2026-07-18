use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1,
    VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::callable_result_representation::{
    CallableResultLoopClaimScheduleErrorV1, LegacyStmtInputV1,
    VerifiedCallableResultActivationPlanV1, VerifiedCallableResultActivationRowsV1,
    VerifiedCallableResultLegacySourceViewV1, VerifiedSameModuleCallableResultCatalogV1,
};
use crate::mir::resolved_semantics::{
    SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1, SourceStmtSiteV1,
};
use crate::mir::source_call_target::{
    VerifiedSourceStaticCallTargetCatalogV1, VerifiedStaticImportAliasViewV1,
};
use crate::mir::{BasicBlockId, ValueId};
use crate::parser::NyashParser;

use super::{
    CoreCallSourceV1, CoreEffectPlan, CoreLoopPlan, CorePlan, LocatedCoreLoopPlanErrorV1,
    LoopStepMode, VerifiedLocatedCoreLoopPlanV1,
};
use crate::mir::builder::control_flow::edgecfg::api::Frag;

const SOURCE: &str = r#"
    box ParserBox {
        parse(text, pos) {
            local before = Helpers.before(pos)
            loop(Helpers.condition(pos)) {
                local inside = Helpers.outer(Helpers.inner(pos))
            }
            return Helpers.after(pos)
        }
    }
    box OtherBox {
        parse(text, pos) {
            loop(Helpers.condition(pos)) {
                local inside = Helpers.outer(Helpers.inner(pos))
            }
            return pos
        }
    }
    static box Helpers {
        before(value) { return value }
        condition(value) { return value }
        outer(value) { return value }
        inner(value) { return value }
        after(value) { return value }
    }
"#;

const EMPTY_LOOP_SOURCE: &str = r#"
    box ParserBox {
        parse(text, pos) {
            local before = Helpers.before(pos)
            loop(pos < 1) {
                pos = pos + 1
            }
            return Helpers.after(pos)
        }
    }
    static box Helpers {
        before(value) { return value }
        after(value) { return value }
    }
"#;

const TWO_LOOP_SOURCE: &str = r#"
    box ParserBox {
        parse(text, pos) {
            loop(Helpers.condition(pos)) {
                local first = Helpers.outer(pos)
            }
            loop(Helpers.condition(pos)) {
                local second = Helpers.inner(pos)
            }
            return pos
        }
    }
    static box Helpers {
        condition(value) { return value }
        outer(value) { return value }
        inner(value) { return value }
    }
"#;

fn site(segments: Vec<SourcePathSegmentV1>) -> SourceExprSiteV1 {
    SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(segments))
}

fn before_site() -> SourceExprSiteV1 {
    site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Initializer(0),
    ])
}

fn condition_site() -> SourceExprSiteV1 {
    site(vec![
        SourcePathSegmentV1::Body(1),
        SourcePathSegmentV1::LoopCondition,
    ])
}

fn outer_site() -> SourceExprSiteV1 {
    site(vec![
        SourcePathSegmentV1::Body(1),
        SourcePathSegmentV1::LoopBody(0),
        SourcePathSegmentV1::Initializer(0),
    ])
}

fn inner_site() -> SourceExprSiteV1 {
    site(vec![
        SourcePathSegmentV1::Body(1),
        SourcePathSegmentV1::LoopBody(0),
        SourcePathSegmentV1::Initializer(0),
        SourcePathSegmentV1::Argument(0),
    ])
}

fn after_site() -> SourceExprSiteV1 {
    site(vec![
        SourcePathSegmentV1::Body(2),
        SourcePathSegmentV1::Value,
    ])
}

fn loop_root_site(index: u32) -> SourceStmtSiteV1 {
    SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
        SourcePathSegmentV1::Body(index),
    ]))
}

fn loop_condition_site(index: u32) -> SourceExprSiteV1 {
    site(vec![
        SourcePathSegmentV1::Body(index),
        SourcePathSegmentV1::LoopCondition,
    ])
}

fn loop_body_initializer_site(index: u32) -> SourceExprSiteV1 {
    site(vec![
        SourcePathSegmentV1::Body(index),
        SourcePathSegmentV1::LoopBody(0),
        SourcePathSegmentV1::Initializer(0),
    ])
}

fn seal_plan(source: &str) -> VerifiedCallableResultActivationPlanV1 {
    let root = NyashParser::parse_from_string(source).expect("located Loop fixture must parse");
    let declarations = Box::new(
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root)
            .expect("located Loop declarations must seal"),
    );
    let imports = VerifiedStaticImportAliasViewV1::seal(&declarations, Vec::new())
        .expect("empty import view must seal");
    let targets =
        VerifiedSourceStaticCallTargetCatalogV1::seal_qualified(&imports, std::iter::empty())
            .expect("empty selected target catalog must seal");
    let results = VerifiedSameModuleCallableResultCatalogV1::verify(&declarations, &targets)
        .expect("callable result catalog must seal");
    let rows = VerifiedCallableResultActivationRowsV1::verify(&declarations, &targets, &results)
        .expect("activation rows must seal");
    drop(results);
    drop(targets);
    drop(imports);
    VerifiedCallableResultActivationPlanV1::seal(declarations, rows)
        .expect("activation plan must seal")
}

fn caller(
    plan: &VerifiedCallableResultActivationPlanV1,
    owner: &str,
) -> CanonicalSameModuleCallableKeyV1 {
    plan.declaration_catalog()
        .declaration_for(
            SameModuleCallableNamespaceV1::InstanceBoxMethod,
            owner,
            "parse",
            2,
        )
        .unwrap_or_else(|| panic!("missing {owner}.parse/2"))
        .key()
        .clone()
}

fn statement<'plan>(
    plan: &'plan VerifiedCallableResultActivationPlanV1,
    caller: &CanonicalSameModuleCallableKeyV1,
    index: usize,
) -> LegacyStmtInputV1<'plan> {
    let view = VerifiedCallableResultLegacySourceViewV1::verify(plan, caller)
        .expect("located source view");
    view.body_stmt(&view.root_body(), index)
        .expect("located root statement")
}

fn call(source: SourceExprSiteV1, ordinal: u32) -> CoreEffectPlan {
    CoreEffectPlan::GlobalCall {
        dst: Some(ValueId(100 + ordinal)),
        func: format!("fixture.call/{ordinal}"),
        args: vec![ValueId(200 + ordinal)],
        source: CoreCallSourceV1::LocatedMethodCall(source),
    }
}

fn loop_plan(
    body_sites_in_traversal_order: &[SourceExprSiteV1],
    condition_sites_in_traversal_order: &[SourceExprSiteV1],
) -> CorePlan {
    let preheader = BasicBlockId(10);
    let header = BasicBlockId(11);
    let body = BasicBlockId(12);
    let step = BasicBlockId(13);
    let after = BasicBlockId(14);
    CorePlan::Loop(CoreLoopPlan {
        preheader_bb: preheader,
        preheader_is_fresh: false,
        header_bb: header,
        body_bb: body,
        step_bb: step,
        continue_target: step,
        after_bb: after,
        found_bb: after,
        body: body_sites_in_traversal_order
            .iter()
            .enumerate()
            .map(|(ordinal, source)| CorePlan::Effect(call(source.clone(), ordinal as u32)))
            .collect(),
        cond_loop: ValueId(1),
        cond_match: ValueId(2),
        block_effects: vec![
            (preheader, vec![]),
            (
                header,
                condition_sites_in_traversal_order
                    .iter()
                    .enumerate()
                    .map(|(ordinal, source)| call(source.clone(), 50 + ordinal as u32))
                    .collect(),
            ),
            (body, vec![]),
            (step, vec![]),
        ],
        phis: vec![],
        frag: Frag::new(header),
        final_values: vec![],
        step_mode: LoopStepMode::ExtractToStepBb,
        has_explicit_step: false,
    })
}

fn complete_plan() -> CorePlan {
    // CorePlan traversal is body first, and expression evaluation is inner
    // before outer. The source schedule remains condition, outer, inner.
    loop_plan(&[inner_site(), outer_site()], &[condition_site()])
}

fn unlocated_call() -> CoreEffectPlan {
    CoreEffectPlan::GlobalCall {
        dst: Some(ValueId(900)),
        func: "fixture.synthetic/0".to_owned(),
        args: vec![],
        source: CoreCallSourceV1::Unlocated,
    }
}

#[test]
fn source_schedule_is_independent_from_core_plan_traversal_order() {
    let activation = seal_plan(SOURCE);
    let parser = caller(&activation, "ParserBox");
    let loop_statement = statement(&activation, &parser, 1);

    let located = VerifiedLocatedCoreLoopPlanV1::verify(
        complete_plan(),
        &activation,
        &parser,
        loop_statement,
    )
    .expect("complete final Loop plan must seal");

    assert!(located.plan_is_loop());
    assert!(located.schedule().is_branded_by(&activation));
    assert_eq!(located.schedule().caller(), &parser);
    assert_eq!(located.schedule().loop_root(), &loop_root_site(1));
    assert_eq!(
        located
            .schedule()
            .sites_in_source_order()
            .cloned()
            .collect::<Vec<_>>(),
        vec![condition_site(), outer_site(), inner_site()],
    );
}

#[test]
fn malformed_core_loop_rejects_before_location_sealing() {
    let activation = seal_plan(SOURCE);
    let parser = caller(&activation, "ParserBox");
    let mut malformed = complete_plan();
    let CorePlan::Loop(loop_plan) = &mut malformed else {
        unreachable!("complete fixture is a Loop plan")
    };
    loop_plan.frag.entry = BasicBlockId(999);

    let error = VerifiedLocatedCoreLoopPlanV1::verify(
        malformed,
        &activation,
        &parser,
        statement(&activation, &parser, 1),
    )
    .unwrap_err();

    assert!(matches!(
        error,
        LocatedCoreLoopPlanErrorV1::PlanVerification(message)
            if message.contains("[V8]") && message.contains("loop_frag_entry_mismatch")
    ));
}

#[test]
fn unrelated_unlocated_effect_does_not_fabricate_a_source_occurrence() {
    let activation = seal_plan(SOURCE);
    let parser = caller(&activation, "ParserBox");
    let mut plan = complete_plan();
    let CorePlan::Loop(loop_plan) = &mut plan else {
        unreachable!("complete fixture is a Loop plan")
    };
    loop_plan.body.push(CorePlan::Effect(unlocated_call()));

    let located = VerifiedLocatedCoreLoopPlanV1::verify(
        plan,
        &activation,
        &parser,
        statement(&activation, &parser, 1),
    )
    .expect("synthetic Unlocated effect must stay outside source occurrence truth");

    assert_eq!(located.schedule().len(), 3);
}

#[test]
fn required_source_site_stamped_unlocated_is_missing() {
    let activation = seal_plan(SOURCE);
    let parser = caller(&activation, "ParserBox");
    let mut plan = complete_plan();
    let CorePlan::Loop(loop_plan) = &mut plan else {
        unreachable!("complete fixture is a Loop plan")
    };
    loop_plan.body[0] = CorePlan::Effect(unlocated_call());

    let error = VerifiedLocatedCoreLoopPlanV1::verify(
        plan,
        &activation,
        &parser,
        statement(&activation, &parser, 1),
    )
    .unwrap_err();

    assert_eq!(
        error,
        LocatedCoreLoopPlanErrorV1::MissingLocatedOccurrence(inner_site()),
    );
}

#[test]
fn missing_duplicate_and_unexpected_occurrences_are_distinct() {
    let activation = seal_plan(SOURCE);
    let parser = caller(&activation, "ParserBox");

    let missing = VerifiedLocatedCoreLoopPlanV1::verify(
        loop_plan(&[outer_site()], &[condition_site()]),
        &activation,
        &parser,
        statement(&activation, &parser, 1),
    )
    .unwrap_err();
    assert_eq!(
        missing,
        LocatedCoreLoopPlanErrorV1::MissingLocatedOccurrence(inner_site()),
    );

    let duplicate = VerifiedLocatedCoreLoopPlanV1::verify(
        loop_plan(
            &[inner_site(), outer_site(), outer_site()],
            &[condition_site()],
        ),
        &activation,
        &parser,
        statement(&activation, &parser, 1),
    )
    .unwrap_err();
    assert_eq!(
        duplicate,
        LocatedCoreLoopPlanErrorV1::DuplicateLocatedOccurrence(outer_site()),
    );

    let unexpected = VerifiedLocatedCoreLoopPlanV1::verify(
        loop_plan(
            &[inner_site(), outer_site(), after_site()],
            &[condition_site()],
        ),
        &activation,
        &parser,
        statement(&activation, &parser, 1),
    )
    .unwrap_err();
    assert_eq!(
        unexpected,
        LocatedCoreLoopPlanErrorV1::UnexpectedLocatedOccurrence(after_site()),
    );
}

#[test]
fn non_loop_and_foreign_plan_or_caller_pairings_reject() {
    let activation = seal_plan(SOURCE);
    let foreign = seal_plan(SOURCE);
    let parser = caller(&activation, "ParserBox");
    let foreign_parser = caller(&foreign, "ParserBox");
    let other = caller(&activation, "OtherBox");

    let non_loop_plan = VerifiedLocatedCoreLoopPlanV1::verify(
        CorePlan::Seq(vec![]),
        &activation,
        &parser,
        statement(&activation, &parser, 1),
    )
    .unwrap_err();
    assert_eq!(non_loop_plan, LocatedCoreLoopPlanErrorV1::ExpectedLoopPlan,);

    let non_loop_statement = VerifiedLocatedCoreLoopPlanV1::verify(
        complete_plan(),
        &activation,
        &parser,
        statement(&activation, &parser, 0),
    )
    .unwrap_err();
    assert_eq!(
        non_loop_statement,
        LocatedCoreLoopPlanErrorV1::ClaimSchedule(
            CallableResultLoopClaimScheduleErrorV1::ExpectedLocatedLoop,
        ),
    );

    let foreign_plan = VerifiedLocatedCoreLoopPlanV1::verify(
        complete_plan(),
        &activation,
        &parser,
        statement(&foreign, &foreign_parser, 1),
    )
    .unwrap_err();
    assert_eq!(
        foreign_plan,
        LocatedCoreLoopPlanErrorV1::ClaimSchedule(
            CallableResultLoopClaimScheduleErrorV1::ForeignPlan,
        ),
    );

    let foreign_caller = VerifiedLocatedCoreLoopPlanV1::verify(
        complete_plan(),
        &activation,
        &other,
        statement(&activation, &parser, 1),
    )
    .unwrap_err();
    assert!(matches!(
        foreign_caller,
        LocatedCoreLoopPlanErrorV1::ClaimSchedule(
            CallableResultLoopClaimScheduleErrorV1::ForeignCaller { .. },
        ),
    ));
}

#[test]
fn caller_absent_from_activation_plan_rejects_before_domain_lookup() {
    let activation = seal_plan(EMPTY_LOOP_SOURCE);
    let parser = caller(&activation, "ParserBox");
    let foreign = seal_plan(SOURCE);
    let unknown = caller(&foreign, "OtherBox");

    let error = VerifiedLocatedCoreLoopPlanV1::verify(
        loop_plan(&[], &[]),
        &activation,
        &unknown,
        statement(&activation, &parser, 1),
    )
    .unwrap_err();

    assert_eq!(
        error,
        LocatedCoreLoopPlanErrorV1::ClaimSchedule(
            CallableResultLoopClaimScheduleErrorV1::UnknownCaller(unknown),
        ),
    );
}

#[test]
fn same_caller_loop_roots_keep_disjoint_activation_domains() {
    let activation = seal_plan(TWO_LOOP_SOURCE);
    let parser = caller(&activation, "ParserBox");

    let first = VerifiedLocatedCoreLoopPlanV1::verify(
        loop_plan(&[loop_body_initializer_site(0)], &[loop_condition_site(0)]),
        &activation,
        &parser,
        statement(&activation, &parser, 0),
    )
    .expect("first Loop domain must seal independently");
    assert_eq!(first.schedule().loop_root(), &loop_root_site(0));
    assert_eq!(
        first
            .schedule()
            .sites_in_source_order()
            .cloned()
            .collect::<Vec<_>>(),
        vec![loop_condition_site(0), loop_body_initializer_site(0)],
    );

    let second = VerifiedLocatedCoreLoopPlanV1::verify(
        loop_plan(&[loop_body_initializer_site(1)], &[loop_condition_site(1)]),
        &activation,
        &parser,
        statement(&activation, &parser, 1),
    )
    .expect("second Loop domain must seal independently");
    assert_eq!(second.schedule().loop_root(), &loop_root_site(1));
    assert_eq!(
        second
            .schedule()
            .sites_in_source_order()
            .cloned()
            .collect::<Vec<_>>(),
        vec![loop_condition_site(1), loop_body_initializer_site(1)],
    );
}

#[test]
fn empty_loop_domain_rejects_without_borrowing_sibling_rows() {
    let activation = seal_plan(EMPTY_LOOP_SOURCE);
    let parser = caller(&activation, "ParserBox");
    let error = VerifiedLocatedCoreLoopPlanV1::verify(
        loop_plan(&[], &[]),
        &activation,
        &parser,
        statement(&activation, &parser, 1),
    )
    .unwrap_err();

    assert!(matches!(
        error,
        LocatedCoreLoopPlanErrorV1::ClaimSchedule(
            CallableResultLoopClaimScheduleErrorV1::NoActivationRowsUnderLoop(_),
        ),
    ));
    assert_ne!(before_site(), after_site());
}
