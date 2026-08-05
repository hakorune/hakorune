//! D2-S5-S1 source-backed flat NoRecursive evidence.
//!
//! This is a single cfg(test)-only source witness. It observes the exact
//! parser/resolver/facts/frame boundary and stops before eligibility or
//! selection. The one-member shape is intentionally out of the current
//! recursive-carrier capability target.

use super::route_id::LoopRouteId;
use crate::ast::{ASTNode, BinaryOperator};
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

const NORECURSIVE_SOURCE: &str = r#"
function generic_both_no_recursive(j, m, n) {
    loop(j + m < n) {
        j = j + 1
    }
    return j
}
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NoRecursiveModeV1 {
    Release,
    Strict,
}

impl NoRecursiveModeV1 {
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
enum NoRecursiveRejectV1 {
    Parse,
    Resolve,
    Owner,
    Forest,
    RootBody,
    RootShape,
    LoopShape,
    AssignmentShape,
    TargetSite,
    ReturnSite,
    ReturnValue,
    Target,
    Binding,
    Identity,
    Facts,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NoRecursiveDispositionV1 {
    UnresolvedStopNonRecursiveOutOfTarget,
}

#[derive(Debug, PartialEq, Eq)]
struct NoRecursiveObservationV1 {
    forest_binding: VerifiedLoopSourceForestBindingV1,
    function_origin: FunctionOriginV1,
    function_owner: FunctionOwnerIdV1,
    source_kind: SemanticOwnerSourceKindV1,
    frame_key: LoopExecutionFrameKeyV1,
    write_binding: BindingRefV1,
    read_binding: BindingRefV1,
    carrier: GenericLoopCarrierObservationV1,
    strict_or_dev: bool,
    planner_required: bool,
    raw_schedule: Box<[LoopRouteId]>,
}

fn parse_unit(source: &str) -> Result<VerifiedResolvedSourceUnitV1, NoRecursiveRejectV1> {
    let root = NyashParser::parse_from_string(source).map_err(|_| NoRecursiveRejectV1::Parse)?;
    let ASTNode::Program { statements, .. } = root else {
        return Err(NoRecursiveRejectV1::Parse);
    };
    let function = statements
        .into_iter()
        .find(|node| matches!(node, ASTNode::FunctionDeclaration { .. }))
        .ok_or(NoRecursiveRejectV1::Parse)?;
    VerifiedResolvedSourceUnitV1::resolve_function(function)
        .map_err(|_| NoRecursiveRejectV1::Resolve)
}

fn exact_sites(
    input: ResolvedFunctionLoweringInputV1<'_>,
    root: &LocatedStmtV1<'_>,
) -> Result<(SourceExprSiteV1, SourceExprSiteV1), NoRecursiveRejectV1> {
    let source = input.source();
    let root_body = source
        .root_body()
        .map_err(|_| NoRecursiveRejectV1::RootBody)?;
    source
        .body_stmt(&root_body, 1)
        .map_err(|_| NoRecursiveRejectV1::RootShape)?;
    if source.body_stmt(&root_body, 2).is_ok() {
        return Err(NoRecursiveRejectV1::RootShape);
    }
    let loop_body = source
        .child_body_from_stmt(root, BodyChildRoleV1::LoopBody)
        .map_err(|_| NoRecursiveRejectV1::LoopShape)?;
    if source.body_stmt(&loop_body, 1).is_ok() {
        return Err(NoRecursiveRejectV1::LoopShape);
    }
    let assignment = source
        .body_stmt(&loop_body, 0)
        .map_err(|_| NoRecursiveRejectV1::AssignmentShape)?;
    let ASTNode::Assignment { target, value, .. } = assignment.node() else {
        return Err(NoRecursiveRejectV1::AssignmentShape);
    };
    let ASTNode::Variable { name, .. } = target.as_ref() else {
        return Err(NoRecursiveRejectV1::TargetSite);
    };
    if name != "j" {
        return Err(NoRecursiveRejectV1::TargetSite);
    }
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left,
        right,
        ..
    } = value.as_ref()
    else {
        return Err(NoRecursiveRejectV1::AssignmentShape);
    };
    if !matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == "j")
        || !matches!(right.as_ref(), ASTNode::Literal { .. })
    {
        return Err(NoRecursiveRejectV1::AssignmentShape);
    }
    let write = source
        .child_expr_from_stmt(&assignment, ExprChildRoleV1::AssignmentTarget)
        .map_err(|_| NoRecursiveRejectV1::TargetSite)?
        .site()
        .clone();
    let return_stmt = source
        .body_stmt(&root_body, 1)
        .map_err(|_| NoRecursiveRejectV1::ReturnSite)?;
    let read = source
        .child_expr_from_stmt(&return_stmt, ExprChildRoleV1::ReturnValue)
        .map_err(|_| NoRecursiveRejectV1::ReturnValue)?
        .site()
        .clone();
    Ok((write, read))
}

fn observe() -> Result<NoRecursiveObservationV1, NoRecursiveRejectV1> {
    let unit = parse_unit(NORECURSIVE_SOURCE)?;
    let input = unit
        .root_function_input()
        .map_err(|_| NoRecursiveRejectV1::Resolve)?;
    let source = input.source();
    let root_body = source
        .root_body()
        .map_err(|_| NoRecursiveRejectV1::RootBody)?;
    let root = source
        .body_stmt(&root_body, 0)
        .map_err(|_| NoRecursiveRejectV1::RootShape)?;
    if input.owner() != root.owner() {
        return Err(NoRecursiveRejectV1::Owner);
    }
    let function = input.function();
    let forest = function
        .resolved_loop_source_forest(root.site())
        .map_err(|_| NoRecursiveRejectV1::Forest)?;
    if forest.members().len() != 1
        || forest.members()[0].parent_index().is_some()
        || forest.members()[0].source().site() != root.site()
    {
        return Err(NoRecursiveRejectV1::Forest);
    }
    let frame_key = forest.members()[0].source().frame_key().clone();
    let function_origin = function.function_origin();
    let forest_binding =
        bind_resolved_loop_source_forest_v1(forest).map_err(|_| NoRecursiveRejectV1::Forest)?;
    let (write_site, read_site) = exact_sites(input, &root)?;
    let write_binding = match function.assignment_target(&write_site) {
        Some(ResolvedAssignmentTargetV1::BindingRebind(binding)) => *binding,
        _ => return Err(NoRecursiveRejectV1::TargetSite),
    };
    let read_binding = match function.variable_ref(&read_site) {
        Some(ResolvedLexicalRefV1::Local(binding)) => binding,
        _ => return Err(NoRecursiveRejectV1::Binding),
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
        return Err(NoRecursiveRejectV1::Identity);
    }
    let ASTNode::Loop {
        condition, body, ..
    } = root.node()
    else {
        return Err(NoRecursiveRejectV1::LoopShape);
    };
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left,
        right,
        ..
    } = condition.as_ref()
    else {
        return Err(NoRecursiveRejectV1::LoopShape);
    };
    if !matches!(
        left.as_ref(),
        ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            ..
        }
    ) || !matches!(right.as_ref(), ASTNode::Variable { name, .. } if name == "n")
    {
        return Err(NoRecursiveRejectV1::LoopShape);
    }
    let ctx = LoopRouteContext::new(condition, body, "generic_both_no_recursive/0", false, false);
    let outcome = try_build_outcome(&ctx).map_err(|_| NoRecursiveRejectV1::Facts)?;
    let facts: &CanonicalLoopFacts = outcome.facts.as_ref().ok_or(NoRecursiveRejectV1::Facts)?;
    let generic = facts
        .facts
        .generic_loop_v1()
        .ok_or(NoRecursiveRejectV1::Facts)?;
    if !matches!(
        generic.carrier_observation,
        GenericLoopCarrierObservationV1::CompleteNoRecursiveCarrier
    ) {
        return Err(NoRecursiveRejectV1::Facts);
    }
    let strict_or_dev = crate::config::env::joinir_dev::strict_enabled();
    let planner_required = crate::config::env::joinir_dev::planner_required_enabled();
    let frame = test_issue_live_preflight_frame(&ctx, &outcome, strict_or_dev, planner_required);
    let frame_env = frame.test_env();
    Ok(NoRecursiveObservationV1 {
        forest_binding,
        function_origin,
        function_owner: function.owner(),
        source_kind: function.source_kind(),
        frame_key,
        write_binding,
        read_binding,
        carrier: generic.carrier_observation.clone(),
        strict_or_dev: frame_env.strict_or_dev,
        planner_required: frame_env.planner_required,
        raw_schedule: frame.test_raw_schedule().to_vec().into_boxed_slice(),
    })
}

fn observe_in_mode(
    mode: NoRecursiveModeV1,
) -> Result<NoRecursiveObservationV1, NoRecursiveRejectV1> {
    let _config = mode.config();
    observe()
}

fn disposition(observation: &NoRecursiveObservationV1) -> NoRecursiveDispositionV1 {
    assert_eq!(
        observation.carrier,
        GenericLoopCarrierObservationV1::CompleteNoRecursiveCarrier
    );
    assert_eq!(
        observation.raw_schedule.as_ref(),
        &[LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1]
    );
    assert!(!observation.planner_required);
    NoRecursiveDispositionV1::UnresolvedStopNonRecursiveOutOfTarget
}

#[test]
fn flat_norecursive_source_preserves_both_facts_pre_effect() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    for mode in [NoRecursiveModeV1::Release, NoRecursiveModeV1::Strict] {
        let observation = observe_in_mode(mode).expect("parsed flat NoRecursive source witness");
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
        assert!(!observation.planner_required);
        assert_eq!(observation.strict_or_dev, mode == NoRecursiveModeV1::Strict);
        assert_eq!(
            observation.raw_schedule.as_ref(),
            &[LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1]
        );
        assert_eq!(
            disposition(&observation),
            NoRecursiveDispositionV1::UnresolvedStopNonRecursiveOutOfTarget
        );
    }
}

#[test]
fn flat_norecursive_source_repeat_keeps_identity_shape_and_schedule() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    for mode in [NoRecursiveModeV1::Release, NoRecursiveModeV1::Strict] {
        let first = observe_in_mode(mode).expect("first parsed flat NoRecursive witness");
        let second = observe_in_mode(mode).expect("second parsed flat NoRecursive witness");
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
        assert_eq!(first.strict_or_dev, second.strict_or_dev);
        assert_eq!(first.planner_required, second.planner_required);
        assert_eq!(first.raw_schedule, second.raw_schedule);
        assert_eq!(disposition(&first), disposition(&second));
    }
}
