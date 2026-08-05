//! D3-S1-S1 source-backed V1-only lexical-local evidence.
//!
//! This is a cfg(test)-only witness.  It observes the parser/resolver/facts/
//! frame boundary and stops before eligibility, selection, or production.
//! The lexical `tmp` local is deliberately distinct from the router's
//! dedicated LoopBreakBodyLocalFacts `has_body_local` flag.

use super::route_id::LoopRouteId;
use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::builder::control_flow::joinir::route_entry::router::{
    test_issue_live_preflight_frame, LoopRouteContext,
};
use crate::mir::builder::control_flow::lower::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::plan::facts::GenericLoopCarrierObservationV1;
use crate::mir::builder::control_flow::plan::single_planner::try_build_outcome;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::located::LocatedStmtV1;
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::loop_structural_facts::{
    bind_resolved_loop_source_forest_v1, VerifiedLoopSourceForestBindingV1,
};
use crate::mir::resolved_semantics::{
    BindingRefV1, BodyChildRoleV1, ExprChildRoleV1, FunctionOriginV1, FunctionOwnerIdV1,
    LoopExecutionFrameKeyV1, ResolvedAssignmentTargetV1, ResolvedLexicalRefV1,
    SemanticOwnerSourceKindV1, SourceExprSiteV1,
};
use crate::parser::NyashParser;

const V1_ONLY_LOCAL_SOURCE: &str = r#"
function generic_v1_only_local(i) {
    loop(i < 3) {
        local tmp = 0
        i = i + 1
    }
    return i
}
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum V1OnlyLocalModeV1 {
    Release,
    Strict,
}

impl V1OnlyLocalModeV1 {
    fn config(self) -> crate::test_support::ScopedTestConfig {
        crate::test_support::ScopedTestConfig::apply(&[
            (
                "HAKO_JOINIR_STRICT",
                match self {
                    Self::Release => None,
                    Self::Strict => Some("1"),
                },
            ),
            ("HAKO_JOINIR_PLANNER_REQUIRED", None),
            ("NYASH_JOINIR_STRICT", None),
            ("NYASH_SYNTAX_SUGAR_LEVEL", Some("basic")),
        ])
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum V1OnlyLocalRejectV1 {
    Parse,
    Resolve,
    Owner,
    Forest,
    RootBody,
    RootShape,
    LoopShape,
    LocalShape,
    LocalInitializer,
    AssignmentShape,
    TargetSite,
    ReturnSite,
    ReturnValue,
    Binding,
    Identity,
    Facts,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum V1OnlyLocalDispositionV1 {
    UnresolvedStopV1OnlyNonRecursive,
}

#[derive(Debug, PartialEq, Eq)]
struct V1OnlyLocalObservationV1 {
    forest_binding: VerifiedLoopSourceForestBindingV1,
    function_origin: FunctionOriginV1,
    function_owner: FunctionOwnerIdV1,
    source_kind: SemanticOwnerSourceKindV1,
    frame_key: LoopExecutionFrameKeyV1,
    write_binding: BindingRefV1,
    read_binding: BindingRefV1,
    carrier: GenericLoopCarrierObservationV1,
    v0_facts_present: bool,
    v1_facts_present: bool,
    source_has_lexical_local: bool,
    loop_break_body_local_fact: bool,
    strict_or_dev: bool,
    planner_required: bool,
    recipe_contract_present: bool,
    recipe_first_allowed: bool,
    has_body_local: bool,
    raw_schedule: Box<[LoopRouteId]>,
}

fn parse_unit(source: &str) -> Result<VerifiedResolvedSourceUnitV1, V1OnlyLocalRejectV1> {
    let root = NyashParser::parse_from_string(source).map_err(|_| V1OnlyLocalRejectV1::Parse)?;
    let ASTNode::Program { statements, .. } = root else {
        return Err(V1OnlyLocalRejectV1::Parse);
    };
    let function = statements
        .into_iter()
        .find(|node| matches!(node, ASTNode::FunctionDeclaration { .. }))
        .ok_or(V1OnlyLocalRejectV1::Parse)?;
    VerifiedResolvedSourceUnitV1::resolve_function(function)
        .map_err(|_| V1OnlyLocalRejectV1::Resolve)
}

fn exact_sites(
    input: ResolvedFunctionLoweringInputV1<'_>,
    root: &LocatedStmtV1<'_>,
) -> Result<(SourceExprSiteV1, SourceExprSiteV1), V1OnlyLocalRejectV1> {
    let source = input.source();
    let root_body = source
        .root_body()
        .map_err(|_| V1OnlyLocalRejectV1::RootBody)?;
    source
        .body_stmt(&root_body, 1)
        .map_err(|_| V1OnlyLocalRejectV1::RootShape)?;
    if source.body_stmt(&root_body, 2).is_ok() {
        return Err(V1OnlyLocalRejectV1::RootShape);
    }
    let loop_body = source
        .child_body_from_stmt(root, BodyChildRoleV1::LoopBody)
        .map_err(|_| V1OnlyLocalRejectV1::LoopShape)?;
    source
        .body_stmt(&loop_body, 1)
        .map_err(|_| V1OnlyLocalRejectV1::LoopShape)?;
    if source.body_stmt(&loop_body, 2).is_ok() {
        return Err(V1OnlyLocalRejectV1::LoopShape);
    }
    let local = source
        .body_stmt(&loop_body, 0)
        .map_err(|_| V1OnlyLocalRejectV1::LocalShape)?;
    let ASTNode::Local {
        variables,
        initial_values,
        ..
    } = local.node()
    else {
        return Err(V1OnlyLocalRejectV1::LocalShape);
    };
    if variables.as_slice() != ["tmp"]
        || initial_values.len() != 1
        || !matches!(
            initial_values[0].as_deref(),
            Some(ASTNode::Literal {
                value: LiteralValue::Integer(0),
                ..
            })
        )
    {
        return Err(V1OnlyLocalRejectV1::LocalShape);
    }
    source
        .child_expr_from_stmt(&local, ExprChildRoleV1::LocalInitializer(0))
        .map_err(|_| V1OnlyLocalRejectV1::LocalInitializer)?;

    let assignment = source
        .body_stmt(&loop_body, 1)
        .map_err(|_| V1OnlyLocalRejectV1::AssignmentShape)?;
    let ASTNode::Assignment { target, value, .. } = assignment.node() else {
        return Err(V1OnlyLocalRejectV1::AssignmentShape);
    };
    if !matches!(target.as_ref(), ASTNode::Variable { name, .. } if name == "i")
        || !matches!(
            value.as_ref(),
            ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left,
                right,
                ..
            } if matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == "i")
                && matches!(right.as_ref(), ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    ..
                })
        )
    {
        return Err(V1OnlyLocalRejectV1::AssignmentShape);
    }
    let write = source
        .child_expr_from_stmt(&assignment, ExprChildRoleV1::AssignmentTarget)
        .map_err(|_| V1OnlyLocalRejectV1::TargetSite)?
        .site()
        .clone();
    let return_stmt = source
        .body_stmt(&root_body, 1)
        .map_err(|_| V1OnlyLocalRejectV1::ReturnSite)?;
    let read = source
        .child_expr_from_stmt(&return_stmt, ExprChildRoleV1::ReturnValue)
        .map_err(|_| V1OnlyLocalRejectV1::ReturnValue)?;
    if !matches!(read.node(), ASTNode::Variable { name, .. } if name == "i") {
        return Err(V1OnlyLocalRejectV1::ReturnValue);
    }
    Ok((write, read.site().clone()))
}

fn observe() -> Result<V1OnlyLocalObservationV1, V1OnlyLocalRejectV1> {
    let unit = parse_unit(V1_ONLY_LOCAL_SOURCE)?;
    let input = unit
        .root_function_input()
        .map_err(|_| V1OnlyLocalRejectV1::Resolve)?;
    let source = input.source();
    let root_body = source
        .root_body()
        .map_err(|_| V1OnlyLocalRejectV1::RootBody)?;
    let root = source
        .body_stmt(&root_body, 0)
        .map_err(|_| V1OnlyLocalRejectV1::RootShape)?;
    if input.owner() != root.owner() {
        return Err(V1OnlyLocalRejectV1::Owner);
    }
    let function = input.function();
    let forest = function
        .resolved_loop_source_forest(root.site())
        .map_err(|_| V1OnlyLocalRejectV1::Forest)?;
    if forest.members().len() != 1
        || forest.members()[0].parent_index().is_some()
        || forest.members()[0].source().site() != root.site()
    {
        return Err(V1OnlyLocalRejectV1::Forest);
    }
    let frame_key = forest.members()[0].source().frame_key().clone();
    let function_origin = function.function_origin();
    let forest_binding =
        bind_resolved_loop_source_forest_v1(forest).map_err(|_| V1OnlyLocalRejectV1::Forest)?;
    let (write_site, read_site) = exact_sites(input, &root)?;
    let write_binding = match function.assignment_target(&write_site) {
        Some(ResolvedAssignmentTargetV1::BindingRebind(binding)) => *binding,
        _ => return Err(V1OnlyLocalRejectV1::TargetSite),
    };
    let read_binding = match function.variable_ref(&read_site) {
        Some(ResolvedLexicalRefV1::Local(binding)) => binding,
        _ => return Err(V1OnlyLocalRejectV1::Binding),
    };
    if function.source_kind() != SemanticOwnerSourceKindV1::DeclaredFunction
        || write_binding.owner() != function.owner()
        || read_binding.owner() != function.owner()
        || write_binding != read_binding
        || forest_binding.owner()
            != (crate::mir::loop_recipe_contract::LoopRecipeSourceOwnerV1::FunctionBody {
                compilation_unit_ordinal: function_origin.compilation_unit_ordinal(),
                function_ordinal: function_origin.function_ordinal(),
            })
    {
        return Err(V1OnlyLocalRejectV1::Identity);
    }
    let ASTNode::Loop {
        condition, body, ..
    } = root.node()
    else {
        return Err(V1OnlyLocalRejectV1::LoopShape);
    };
    if !matches!(
        condition.as_ref(),
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left,
            right,
            ..
        } if matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == "i")
            && matches!(right.as_ref(), ASTNode::Literal {
                value: LiteralValue::Integer(3),
                ..
            })
    ) {
        return Err(V1OnlyLocalRejectV1::LoopShape);
    }
    let ctx = LoopRouteContext::new(condition, body, "generic_v1_only_local/0", false, false);
    let outcome = try_build_outcome(&ctx).map_err(|_| V1OnlyLocalRejectV1::Facts)?;
    let facts: &CanonicalLoopFacts = outcome.facts.as_ref().ok_or(V1OnlyLocalRejectV1::Facts)?;
    let generic = facts
        .facts
        .generic_loop_v1()
        .ok_or(V1OnlyLocalRejectV1::Facts)?;
    if facts.facts.generic_loop_v0().is_some()
        || !matches!(
            generic.carrier_observation,
            GenericLoopCarrierObservationV1::CompleteNoRecursiveCarrier
        )
    {
        return Err(V1OnlyLocalRejectV1::Facts);
    }
    let strict_or_dev = crate::config::env::joinir_dev::strict_enabled();
    let planner_required = crate::config::env::joinir_dev::planner_required_enabled();
    let frame = test_issue_live_preflight_frame(&ctx, &outcome, strict_or_dev, planner_required);
    let frame_env = frame.test_env();
    Ok(V1OnlyLocalObservationV1 {
        forest_binding,
        function_origin,
        function_owner: function.owner(),
        source_kind: function.source_kind(),
        frame_key,
        write_binding,
        read_binding,
        carrier: generic.carrier_observation.clone(),
        v0_facts_present: facts.facts.generic_loop_v0().is_some(),
        v1_facts_present: facts.facts.generic_loop_v1().is_some(),
        source_has_lexical_local: true,
        loop_break_body_local_fact: facts.facts.loop_break_body_local().is_some(),
        strict_or_dev: frame_env.strict_or_dev,
        planner_required: frame_env.planner_required,
        recipe_contract_present: frame.test_recipe_contract_present(),
        recipe_first_allowed: frame.test_recipe_first_allowed(),
        has_body_local: frame_env.has_body_local,
        raw_schedule: frame.test_raw_schedule().to_vec().into_boxed_slice(),
    })
}

fn observe_in_mode(
    mode: V1OnlyLocalModeV1,
) -> Result<V1OnlyLocalObservationV1, V1OnlyLocalRejectV1> {
    let _config = mode.config();
    observe()
}

fn disposition(observation: &V1OnlyLocalObservationV1) -> V1OnlyLocalDispositionV1 {
    assert!(observation.source_has_lexical_local);
    assert!(!observation.loop_break_body_local_fact);
    assert!(!observation.has_body_local);
    assert!(!observation.v0_facts_present);
    assert!(observation.v1_facts_present);
    assert_eq!(
        observation.carrier,
        GenericLoopCarrierObservationV1::CompleteNoRecursiveCarrier
    );
    assert_eq!(
        observation.raw_schedule.as_ref(),
        &[LoopRouteId::GenericLoopV1]
    );
    assert!(!observation.planner_required);
    assert!(!observation.recipe_contract_present);
    assert!(observation.recipe_first_allowed);
    V1OnlyLocalDispositionV1::UnresolvedStopV1OnlyNonRecursive
}

#[test]
fn v1_only_local_source_co_seals_facts_frame_and_raw_schedule() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    for mode in [V1OnlyLocalModeV1::Release, V1OnlyLocalModeV1::Strict] {
        let observation = observe_in_mode(mode).expect("parsed V1-only local source witness");
        assert_eq!(observation.function_origin.function_ordinal(), 0);
        assert_eq!(
            observation.source_kind,
            SemanticOwnerSourceKindV1::DeclaredFunction
        );
        assert_eq!(
            observation.function_owner,
            observation.write_binding.owner()
        );
        assert_eq!(observation.function_owner, observation.read_binding.owner());
        assert_eq!(observation.write_binding, observation.read_binding);
        assert_eq!(observation.strict_or_dev, mode == V1OnlyLocalModeV1::Strict);
        assert!(!observation.planner_required);
        assert!(!observation.recipe_contract_present);
        assert!(observation.recipe_first_allowed);
        assert!(!observation.has_body_local);
        assert_eq!(
            observation.raw_schedule.as_ref(),
            &[LoopRouteId::GenericLoopV1]
        );
        assert_eq!(
            disposition(&observation),
            V1OnlyLocalDispositionV1::UnresolvedStopV1OnlyNonRecursive
        );
    }
}

#[test]
fn v1_only_local_source_repeat_keeps_identity_shape_and_schedule() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    for mode in [V1OnlyLocalModeV1::Release, V1OnlyLocalModeV1::Strict] {
        let first = observe_in_mode(mode).expect("first parsed V1-only local witness");
        let second = observe_in_mode(mode).expect("second parsed V1-only local witness");
        assert_eq!(
            first.forest_binding.members(),
            second.forest_binding.members()
        );
        assert_ne!(first.function_owner, second.function_owner);
        assert_eq!(first.function_origin, second.function_origin);
        assert_eq!(first.source_kind, second.source_kind);
        assert_eq!(first.frame_key, second.frame_key);
        assert_eq!(
            first.write_binding.binding(),
            second.write_binding.binding()
        );
        assert_eq!(first.read_binding.binding(), second.read_binding.binding());
        assert_eq!(first.carrier, second.carrier);
        assert_eq!(first.v0_facts_present, second.v0_facts_present);
        assert_eq!(first.v1_facts_present, second.v1_facts_present);
        assert_eq!(
            first.source_has_lexical_local,
            second.source_has_lexical_local
        );
        assert_eq!(
            first.loop_break_body_local_fact,
            second.loop_break_body_local_fact
        );
        assert_eq!(first.strict_or_dev, second.strict_or_dev);
        assert_eq!(first.planner_required, second.planner_required);
        assert_eq!(
            first.recipe_contract_present,
            second.recipe_contract_present
        );
        assert_eq!(first.recipe_first_allowed, second.recipe_first_allowed);
        assert_eq!(first.has_body_local, second.has_body_local);
        assert_eq!(first.raw_schedule, second.raw_schedule);
        assert_eq!(disposition(&first), disposition(&second));
    }
}
