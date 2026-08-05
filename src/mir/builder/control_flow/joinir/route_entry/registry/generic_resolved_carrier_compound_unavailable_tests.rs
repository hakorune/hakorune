//! D2-S3 source-backed CompoundAssignment/Unavailable evidence.
//!
//! This is deliberately a cfg(test)-only source row. It observes the parser,
//! resolver, facts collector, and preflight schedule as one witness, then
//! stops before eligibility, selection, Builder, or MIR effects.

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

const COMPOUND_SOURCE: &str = r#"
function generic_both_nested_compound(i, j) {
    loop(i < 3) {
        loop(j < 3) {
            if i < 2 {
                j += 1
            }
            j = j + 1
        }
        i = i + 1
    }
    return j
}
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CompoundModeV1 {
    Release,
    Strict,
}

impl CompoundModeV1 {
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
enum CompoundRejectV1 {
    Parse,
    Resolve,
    Owner,
    Forest,
    OuterBodyNavigation,
    InnerNavigation,
    InnerBodyNavigation,
    IfNavigation,
    ThenBodyNavigation,
    CompoundNavigation,
    CompoundTargetNavigation,
    FunctionBodyNavigation,
    ReturnNavigation,
    ReturnValueNavigation,
    SyntaxShape,
    Target,
    Binding,
    Identity,
    Facts,
    Carrier,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CompoundDispositionV1 {
    UnresolvedStopCompoundUnavailable,
}

#[derive(Debug, PartialEq, Eq)]
struct CompoundObservationV1 {
    forest_binding: VerifiedLoopSourceForestBindingV1,
    function_origin: FunctionOriginV1,
    function_owner: FunctionOwnerIdV1,
    source_kind: SemanticOwnerSourceKindV1,
    frame_key: LoopExecutionFrameKeyV1,
    write_binding: BindingRefV1,
    read_binding: BindingRefV1,
    carrier_reason: Box<str>,
    strict_or_dev: bool,
    planner_required: bool,
    raw_schedule: Box<[LoopRouteId]>,
}

fn parse_unit(source: &str) -> Result<VerifiedResolvedSourceUnitV1, CompoundRejectV1> {
    let root = NyashParser::parse_from_string(source).map_err(|_| CompoundRejectV1::Parse)?;
    let ASTNode::Program { statements, .. } = root else {
        return Err(CompoundRejectV1::Parse);
    };
    let function = statements
        .into_iter()
        .find(|node| matches!(node, ASTNode::FunctionDeclaration { .. }))
        .ok_or(CompoundRejectV1::Parse)?;
    VerifiedResolvedSourceUnitV1::resolve_function(function).map_err(|_| CompoundRejectV1::Resolve)
}

fn compound_sites(
    input: ResolvedFunctionLoweringInputV1<'_>,
    root: &LocatedStmtV1<'_>,
) -> Result<(SourceExprSiteV1, SourceExprSiteV1), CompoundRejectV1> {
    let source = input.source();
    let outer_body = source
        .child_body_from_stmt(root, BodyChildRoleV1::LoopBody)
        .map_err(|_| CompoundRejectV1::OuterBodyNavigation)?;
    let inner = source
        .body_stmt(&outer_body, 0)
        .map_err(|_| CompoundRejectV1::InnerNavigation)?;
    let inner_body = source
        .child_body_from_stmt(&inner, BodyChildRoleV1::LoopBody)
        .map_err(|_| CompoundRejectV1::InnerBodyNavigation)?;
    let if_stmt = source
        .body_stmt(&inner_body, 0)
        .map_err(|_| CompoundRejectV1::IfNavigation)?;
    let then_body = source
        .child_body_from_stmt(&if_stmt, BodyChildRoleV1::IfThen)
        .map_err(|_| CompoundRejectV1::ThenBodyNavigation)?;
    let compound = source
        .body_stmt(&then_body, 0)
        .map_err(|_| CompoundRejectV1::CompoundNavigation)?;
    let ASTNode::CompoundAssignment {
        operator: BinaryOperator::Add,
        ..
    } = compound.node()
    else {
        return Err(CompoundRejectV1::SyntaxShape);
    };
    let write = source
        .child_expr_from_stmt(&compound, ExprChildRoleV1::CompoundAssignmentTarget)
        .map_err(|_| CompoundRejectV1::CompoundTargetNavigation)?
        .site()
        .clone();
    let function_body = source
        .root_body()
        .map_err(|_| CompoundRejectV1::FunctionBodyNavigation)?;
    let return_stmt = source
        .body_stmt(&function_body, 1)
        .map_err(|_| CompoundRejectV1::ReturnNavigation)?;
    let read = source
        .child_expr_from_stmt(&return_stmt, ExprChildRoleV1::ReturnValue)
        .map_err(|_| CompoundRejectV1::ReturnValueNavigation)?
        .site()
        .clone();
    Ok((write, read))
}

fn strict_ancestor(
    function: &crate::mir::resolved_semantics::VerifiedResolvedFunctionV1,
    binding: BindingRefV1,
    site: &SourceExprSiteV1,
) -> bool {
    let Some(owner_scope) = function.binding(binding).map(|record| record.owner_scope()) else {
        return false;
    };
    let Some(mut current) = function.exact_scope_containing(site.node()) else {
        return false;
    };
    while let Some(parent) = function.scope(current).and_then(|scope| scope.parent()) {
        if parent == owner_scope {
            return true;
        }
        current = parent;
    }
    false
}

fn observe() -> Result<CompoundObservationV1, CompoundRejectV1> {
    let unit = parse_unit(COMPOUND_SOURCE)?;
    let input = unit
        .root_function_input()
        .map_err(|_| CompoundRejectV1::Resolve)?;
    let source = input.source();
    let root_body = source
        .root_body()
        .map_err(|_| CompoundRejectV1::FunctionBodyNavigation)?;
    let root = source
        .body_stmt(&root_body, 0)
        .map_err(|_| CompoundRejectV1::OuterBodyNavigation)?;
    if input.owner() != root.owner() {
        return Err(CompoundRejectV1::Owner);
    }
    let function = input.function();
    let forest = function
        .resolved_loop_source_forest(root.site())
        .map_err(|_| CompoundRejectV1::Forest)?;
    if forest.members().len() != 2
        || forest.members()[0].parent_index().is_some()
        || forest.members()[1].parent_index() != Some(0)
        || forest.members()[0].source().site() != root.site()
    {
        return Err(CompoundRejectV1::Forest);
    }
    let frame_key = forest.members()[0].source().frame_key().clone();
    let function_origin = function.function_origin();
    let forest_binding =
        bind_resolved_loop_source_forest_v1(forest).map_err(|_| CompoundRejectV1::Forest)?;
    let (write_site, read_site) = compound_sites(input, &root)?;
    let write_binding = match function.assignment_target(&write_site) {
        Some(ResolvedAssignmentTargetV1::BindingRebind(binding)) => *binding,
        _ => return Err(CompoundRejectV1::Target),
    };
    let read_binding = match function.variable_ref(&read_site) {
        Some(ResolvedLexicalRefV1::Local(binding)) => binding,
        _ => return Err(CompoundRejectV1::Binding),
    };
    if function.source_kind() != SemanticOwnerSourceKindV1::DeclaredFunction
        || write_binding.owner() != function.owner()
        || read_binding.owner() != function.owner()
        || write_binding != read_binding
        || !strict_ancestor(function, write_binding, &write_site)
        || forest_binding.owner()
            != (crate::mir::loop_recipe_contract::LoopRecipeSourceOwnerV1::FunctionBody {
                compilation_unit_ordinal: function_origin.compilation_unit_ordinal(),
                function_ordinal: function_origin.function_ordinal(),
            })
    {
        return Err(CompoundRejectV1::Identity);
    }
    let ASTNode::Loop {
        condition, body, ..
    } = root.node()
    else {
        return Err(CompoundRejectV1::FunctionBodyNavigation);
    };
    let ctx = LoopRouteContext::new(condition, body, "generic_compound/0", false, false);
    let outcome = try_build_outcome(&ctx).map_err(|_| CompoundRejectV1::Facts)?;
    let facts: &CanonicalLoopFacts = outcome.facts.as_ref().ok_or(CompoundRejectV1::Facts)?;
    let generic = facts
        .facts
        .generic_loop_v1()
        .ok_or(CompoundRejectV1::Facts)?;
    let carrier_reason = match &generic.carrier_observation {
        GenericLoopCarrierObservationV1::Unavailable(reason) if reason == "CompoundAssignment" => {
            reason.clone().into_boxed_str()
        }
        _ => return Err(CompoundRejectV1::Carrier),
    };
    let strict_or_dev = crate::config::env::joinir_dev::strict_enabled();
    let planner_required = crate::config::env::joinir_dev::planner_required_enabled();
    let frame = test_issue_live_preflight_frame(&ctx, &outcome, strict_or_dev, planner_required);
    let frame_env = frame.test_env();
    Ok(CompoundObservationV1 {
        forest_binding,
        function_origin,
        function_owner: function.owner(),
        source_kind: function.source_kind(),
        frame_key,
        write_binding,
        read_binding,
        carrier_reason,
        strict_or_dev: frame_env.strict_or_dev,
        planner_required: frame_env.planner_required,
        raw_schedule: frame.test_raw_schedule().to_vec().into_boxed_slice(),
    })
}

fn observe_in_mode(mode: CompoundModeV1) -> Result<CompoundObservationV1, CompoundRejectV1> {
    let _config = mode.config();
    observe()
}

fn classify(observation: &CompoundObservationV1) -> CompoundDispositionV1 {
    assert_eq!(observation.carrier_reason.as_ref(), "CompoundAssignment");
    assert_eq!(observation.write_binding, observation.read_binding);
    assert!(!observation.planner_required);
    CompoundDispositionV1::UnresolvedStopCompoundUnavailable
}

#[test]
fn compound_source_preserves_exact_unavailable_pre_effect() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    for mode in [CompoundModeV1::Release, CompoundModeV1::Strict] {
        let observation = observe_in_mode(mode).expect("parsed nested compound witness");
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
        assert_eq!(observation.carrier_reason.as_ref(), "CompoundAssignment");
        assert!(!observation.planner_required);
        assert_eq!(observation.strict_or_dev, mode == CompoundModeV1::Strict);
        assert_eq!(
            observation.raw_schedule.as_ref(),
            &[LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1]
        );
        assert_eq!(
            classify(&observation),
            CompoundDispositionV1::UnresolvedStopCompoundUnavailable
        );
    }
}

#[test]
fn compound_source_repeat_keeps_identity_and_measured_schedule() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    for mode in [CompoundModeV1::Release, CompoundModeV1::Strict] {
        let first = observe_in_mode(mode).expect("first parsed nested compound witness");
        let second = observe_in_mode(mode).expect("second parsed nested compound witness");
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
        assert_eq!(first.carrier_reason, second.carrier_reason);
        assert_eq!(first.raw_schedule, second.raw_schedule);
        assert_eq!(classify(&first), classify(&second));
    }
}
