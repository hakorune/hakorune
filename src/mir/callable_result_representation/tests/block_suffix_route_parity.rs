use crate::mir::builder::{
    run_block_suffix_parity_reference_v1, BlockSuffixParityInputV1,
    CanonicalSameModuleCallableKeyV1, StatementDescentReferenceV1,
};
use crate::mir::resolved_semantics::{SourceExprSiteV1, SourcePathSegmentV1};

use super::super::{
    CallableResultBodySuffixDecisionV1, LegacyBodyInputV1, VerifiedCallableResultActivationPlanV1,
    VerifiedCallableResultActivationRowsV1, VerifiedCallableResultCallerLedgerV1,
    VerifiedCallableResultLegacySourceViewV1,
};
use super::support::{
    declarations, instance_key, qualified_targets, seal_with_targets, site, CallSiteSpecV1,
};

const INACTIVE_SOURCE: &str = r#"
    box ParserBox {
        parse() {
            loop(true) {
                break
            }
            return 7
        }
    }
"#;

const ACTIVE_SOURCE: &str = r#"
    box ParserBox {
        parse() {
            loop(true) {
                break
            }
            return Helpers.step(7)
        }
    }
    static box Helpers { step(value) { return value } }
"#;

fn seal_plan(
    source: &str,
    active_sites: Vec<SourceExprSiteV1>,
) -> VerifiedCallableResultActivationPlanV1 {
    let declarations = Box::new(declarations(source));
    let specs = active_sites
        .into_iter()
        .map(|site| CallSiteSpecV1 {
            caller_owner: "ParserBox",
            caller_name: "parse",
            caller_arity: 0,
            site,
        })
        .collect::<Vec<_>>();
    let targets = qualified_targets(declarations.as_ref(), &[], &specs);
    let results = seal_with_targets(declarations.as_ref(), &targets);
    let rows =
        VerifiedCallableResultActivationRowsV1::verify(declarations.as_ref(), &targets, &results)
            .expect("route parity rows");
    drop(results);
    drop(targets);
    VerifiedCallableResultActivationPlanV1::seal(declarations, rows)
        .expect("route parity activation plan")
}

fn caller(plan: &VerifiedCallableResultActivationPlanV1) -> CanonicalSameModuleCallableKeyV1 {
    instance_key(plan.declaration_catalog(), "ParserBox", "parse", 0)
}

fn classified_suffixes<'plan>(
    plan: &'plan VerifiedCallableResultActivationPlanV1,
) -> (
    LegacyBodyInputV1<'plan>,
    Vec<CallableResultBodySuffixDecisionV1<'plan>>,
) {
    let caller = caller(plan);
    let view = VerifiedCallableResultLegacySourceViewV1::verify(plan, &caller)
        .expect("route parity source view");
    let body = view.root_body();
    let ledger =
        VerifiedCallableResultCallerLedgerV1::verify(plan, &caller).expect("route parity ledger");
    let decisions = (0..body.statements().len())
        .map(|index| {
            ledger
                .classify_body_suffix(view.body_suffix(&body, index).unwrap())
                .unwrap()
        })
        .collect();
    (body, decisions)
}

fn with_router_env(f: impl FnOnce()) {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::tests::helpers::joinir_env::with_joinir_env_lock(|| {
        crate::test_support::with_env_vars(
            &[
                ("NYASH_JOINIR_DEV", Some("1")),
                ("HAKO_JOINIR_PLANNER_REQUIRED", None),
                ("HAKO_JOINIR_STRICT", Some("1")),
                ("NYASH_JOINIR_STRICT", None),
            ],
            f,
        )
    })
}

#[test]
fn inactive_nonempty_loop_suffix_routes_with_raw_parity() {
    with_router_env(|| {
        let plan = seal_plan(INACTIVE_SOURCE, Vec::new());
        let (body, decisions) = classified_suffixes(&plan);
        assert!(matches!(
            decisions[0],
            CallableResultBodySuffixDecisionV1::Inactive(_)
        ));

        let raw = run_block_suffix_parity_reference_v1(
            body.statements(),
            BlockSuffixParityInputV1::Raw,
            StatementDescentReferenceV1::Actual,
            "suffix_raw/0",
        );
        let selected = run_block_suffix_parity_reference_v1(
            body.statements(),
            BlockSuffixParityInputV1::Classified(decisions),
            StatementDescentReferenceV1::Actual,
            "suffix_raw/0",
        );

        assert_eq!(selected.output, raw.output);
        assert_eq!(selected, raw);
        assert_eq!(selected.route_demand_indices, vec![0]);
        assert_eq!(selected.lowered_indices, vec![1]);
    });
}

#[test]
fn active_suffix_supplies_no_router_input_and_continues_statement_descent() {
    with_router_env(|| {
        let plan = seal_plan(
            ACTIVE_SOURCE,
            vec![site(vec![
                SourcePathSegmentV1::Body(1),
                SourcePathSegmentV1::Value,
            ])],
        );
        let (body, decisions) = classified_suffixes(&plan);
        let first = match &decisions[0] {
            CallableResultBodySuffixDecisionV1::Active { first } => first.node().clone(),
            CallableResultBodySuffixDecisionV1::Inactive(_) => panic!("root suffix must be active"),
        };
        assert_eq!(
            first.segments(),
            &[SourcePathSegmentV1::Body(1), SourcePathSegmentV1::Value]
        );

        let selected = run_block_suffix_parity_reference_v1(
            body.statements(),
            BlockSuffixParityInputV1::Classified(decisions),
            StatementDescentReferenceV1::RecordOnly,
            "suffix_active/0",
        );

        assert!(selected.output.is_ok());
        assert_eq!(selected.route_demand_indices, vec![0, 1]);
        assert_eq!(selected.lowered_indices, vec![0, 1]);
    });
}

#[test]
fn always_none_is_explicitly_not_located_route_parity() {
    with_router_env(|| {
        let plan = seal_plan(INACTIVE_SOURCE, Vec::new());
        let (body, decisions) = classified_suffixes(&plan);
        let selected = run_block_suffix_parity_reference_v1(
            body.statements(),
            BlockSuffixParityInputV1::Classified(decisions),
            StatementDescentReferenceV1::Actual,
            "suffix_selected/0",
        );
        let always_none = run_block_suffix_parity_reference_v1(
            body.statements(),
            BlockSuffixParityInputV1::AlwaysNone,
            StatementDescentReferenceV1::Actual,
            "suffix_selected/0",
        );

        assert_eq!(selected.lowered_indices, vec![1]);
        assert_eq!(always_none.lowered_indices, vec![0, 1]);
        assert_ne!(
            selected.lowered_indices,
            always_none.lowered_indices,
            "always-none routing is not driver-route parity even when legacy lowering emits equivalent MIR"
        );
    });
}

#[test]
fn suffix_selector_failure_stops_before_router_and_statement_descent() {
    with_router_env(|| {
        let statements = vec![crate::ast::ASTNode::Literal {
            value: crate::ast::LiteralValue::Integer(1),
            span: crate::ast::Span::unknown(),
        }];
        let rejected = run_block_suffix_parity_reference_v1(
            &statements,
            BlockSuffixParityInputV1::RejectAt {
                index: 0,
                message: "suffix-reference/reject",
            },
            StatementDescentReferenceV1::RecordOnly,
            "suffix_reject/0",
        );

        assert_eq!(rejected.output, Err("suffix-reference/reject".to_string()));
        assert_eq!(rejected.route_demand_indices, vec![0]);
        assert!(rejected.lowered_indices.is_empty());
        assert_eq!(rejected.instruction_count, 0);
        assert_eq!(rejected.lexical_scope_depth, 0);

        let valid = run_block_suffix_parity_reference_v1(
            &statements,
            BlockSuffixParityInputV1::AlwaysNone,
            StatementDescentReferenceV1::RecordOnly,
            "suffix_after_reject/0",
        );
        assert!(valid.output.is_ok());
        assert_eq!(valid.lowered_indices, vec![0]);
    });
}
