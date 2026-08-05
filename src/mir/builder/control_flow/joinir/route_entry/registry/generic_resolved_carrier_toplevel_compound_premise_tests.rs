//! D2-S4 source-backed top-level CompoundAssignment premise evidence.
//!
//! This is deliberately a cfg(test)-only observation. It keeps the result
//! open because the collector has a depth-sensitive CompoundAssignment arm.
//! No selector, eligibility, Builder, or MIR authority is introduced here.

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

const TOPLEVEL_COMPOUND_SOURCE: &str = r#"
function generic_top_level_compound(i, j) {
    loop(i < 3) {
        j += 1
        i = i + 1
    }
    return j
}
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CompoundPremiseModeV1 {
    Release,
    Strict,
}

impl CompoundPremiseModeV1 {
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
enum CompoundPremiseRejectV1 {
    Parse,
    Resolve,
    Owner,
    Forest,
    Body,
    Compound,
    CompoundTarget,
    Return,
    ReturnValue,
    SyntaxShape,
    Target,
    Binding,
    Identity,
    Facts,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CompoundPremiseDispositionV1 {
    CompleteNoRecursive,
    UnavailableCompound,
    Ambiguous,
    NoStandaloneRow,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum CompoundPremiseCarrierV1 {
    Observed(GenericLoopCarrierObservationV1),
    NoStandaloneRow,
}

#[derive(Debug, PartialEq, Eq)]
struct CompoundPremiseObservationV1 {
    forest_binding: VerifiedLoopSourceForestBindingV1,
    function_origin: FunctionOriginV1,
    function_owner: FunctionOwnerIdV1,
    source_kind: SemanticOwnerSourceKindV1,
    frame_key: LoopExecutionFrameKeyV1,
    write_binding: BindingRefV1,
    read_binding: BindingRefV1,
    carrier: CompoundPremiseCarrierV1,
    strict_or_dev: bool,
    planner_required: bool,
    raw_schedule: Box<[LoopRouteId]>,
}

fn parse_unit(source: &str) -> Result<VerifiedResolvedSourceUnitV1, CompoundPremiseRejectV1> {
    let root = NyashParser::parse_from_string(source).map_err(|_| CompoundPremiseRejectV1::Parse)?;
    let ASTNode::Program { statements, .. } = root else {
        return Err(CompoundPremiseRejectV1::Parse);
    };
    let function = statements
        .into_iter()
        .find(|node| matches!(node, ASTNode::FunctionDeclaration { .. }))
        .ok_or(CompoundPremiseRejectV1::Parse)?;
    VerifiedResolvedSourceUnitV1::resolve_function(function)
        .map_err(|_| CompoundPremiseRejectV1::Resolve)
}

fn compound_sites(
    input: ResolvedFunctionLoweringInputV1<'_>,
    root: &LocatedStmtV1<'_>,
) -> Result<(SourceExprSiteV1, SourceExprSiteV1), CompoundPremiseRejectV1> {
    let source = input.source();
    let loop_body = source
        .child_body_from_stmt(root, BodyChildRoleV1::LoopBody)
        .map_err(|_| CompoundPremiseRejectV1::Body)?;
    let compound = source
        .body_stmt(&loop_body, 0)
        .map_err(|_| CompoundPremiseRejectV1::Compound)?;
    let ASTNode::CompoundAssignment {
        operator: BinaryOperator::Add,
        ..
    } = compound.node()
    else {
        return Err(CompoundPremiseRejectV1::SyntaxShape);
    };
    let write = source
        .child_expr_from_stmt(&compound, ExprChildRoleV1::CompoundAssignmentTarget)
        .map_err(|_| CompoundPremiseRejectV1::CompoundTarget)?
        .site()
        .clone();
    let function_body = source
        .root_body()
        .map_err(|_| CompoundPremiseRejectV1::Body)?;
    let return_stmt = source
        .body_stmt(&function_body, 1)
        .map_err(|_| CompoundPremiseRejectV1::Return)?;
    let read = source
        .child_expr_from_stmt(&return_stmt, ExprChildRoleV1::ReturnValue)
        .map_err(|_| CompoundPremiseRejectV1::ReturnValue)?
        .site()
        .clone();
    Ok((write, read))
}

fn observe() -> Result<CompoundPremiseObservationV1, CompoundPremiseRejectV1> {
    let unit = parse_unit(TOPLEVEL_COMPOUND_SOURCE)?;
    let input = unit
        .root_function_input()
        .map_err(|_| CompoundPremiseRejectV1::Resolve)?;
    let source = input.source();
    let root_body = source
        .root_body()
        .map_err(|_| CompoundPremiseRejectV1::Body)?;
    let root = source
        .body_stmt(&root_body, 0)
        .map_err(|_| CompoundPremiseRejectV1::Body)?;
    if input.owner() != root.owner() {
        return Err(CompoundPremiseRejectV1::Owner);
    }
    let function = input.function();
    let forest = function
        .resolved_loop_source_forest(root.site())
        .map_err(|_| CompoundPremiseRejectV1::Forest)?;
    if forest.members().len() != 1
        || forest.members()[0].parent_index().is_some()
        || forest.members()[0].source().site() != root.site()
    {
        return Err(CompoundPremiseRejectV1::Forest);
    }
    let frame_key = forest.members()[0].source().frame_key().clone();
    let function_origin = function.function_origin();
    let forest_binding =
        bind_resolved_loop_source_forest_v1(forest).map_err(|_| CompoundPremiseRejectV1::Forest)?;
    let (write_site, read_site) = compound_sites(input, &root)?;
    let write_binding = match function.assignment_target(&write_site) {
        Some(ResolvedAssignmentTargetV1::BindingRebind(binding)) => *binding,
        _ => return Err(CompoundPremiseRejectV1::Target),
    };
    let read_binding = match function.variable_ref(&read_site) {
        Some(ResolvedLexicalRefV1::Local(binding)) => binding,
        _ => return Err(CompoundPremiseRejectV1::Binding),
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
        return Err(CompoundPremiseRejectV1::Identity);
    }
    let ASTNode::Loop {
        condition, body, ..
    } = root.node()
    else {
        return Err(CompoundPremiseRejectV1::Body);
    };
    let ctx = LoopRouteContext::new(condition, body, "generic_toplevel_compound/0", false, false);
    let outcome = try_build_outcome(&ctx).map_err(|_| CompoundPremiseRejectV1::Facts)?;
    let carrier = outcome
        .facts
        .as_ref()
        .and_then(|facts: &CanonicalLoopFacts| facts.facts.generic_loop_v1())
        .map(|generic| CompoundPremiseCarrierV1::Observed(generic.carrier_observation.clone()))
        .unwrap_or(CompoundPremiseCarrierV1::NoStandaloneRow);
    let strict_or_dev = crate::config::env::joinir_dev::strict_enabled();
    let planner_required = crate::config::env::joinir_dev::planner_required_enabled();
    let frame = test_issue_live_preflight_frame(&ctx, &outcome, strict_or_dev, planner_required);
    let frame_env = frame.test_env();
    Ok(CompoundPremiseObservationV1 {
        forest_binding,
        function_origin,
        function_owner: function.owner(),
        source_kind: function.source_kind(),
        frame_key,
        write_binding,
        read_binding,
        carrier,
        strict_or_dev: frame_env.strict_or_dev,
        planner_required: frame_env.planner_required,
        raw_schedule: frame.test_raw_schedule().to_vec().into_boxed_slice(),
    })
}

fn observe_in_mode(
    mode: CompoundPremiseModeV1,
) -> Result<CompoundPremiseObservationV1, CompoundPremiseRejectV1> {
    let _config = mode.config();
    observe()
}

fn classify(observation: &CompoundPremiseObservationV1) -> CompoundPremiseDispositionV1 {
    match &observation.carrier {
        CompoundPremiseCarrierV1::Observed(
            GenericLoopCarrierObservationV1::CompleteNoRecursiveCarrier,
        ) => {
            CompoundPremiseDispositionV1::CompleteNoRecursive
        }
        CompoundPremiseCarrierV1::Observed(GenericLoopCarrierObservationV1::Unavailable(reason))
            if reason == "CompoundAssignment" =>
        {
            CompoundPremiseDispositionV1::UnavailableCompound
        }
        CompoundPremiseCarrierV1::Observed(GenericLoopCarrierObservationV1::Ambiguous(_)) => {
            CompoundPremiseDispositionV1::Ambiguous
        }
        _ => CompoundPremiseDispositionV1::NoStandaloneRow,
    }
}

#[test]
fn top_level_compound_preserves_observed_facts_before_effects() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    for mode in [CompoundPremiseModeV1::Release, CompoundPremiseModeV1::Strict] {
        let observation = observe_in_mode(mode).expect("parsed top-level compound witness");
        assert_eq!(observation.function_origin.function_ordinal(), 0);
        assert_eq!(
            observation.source_kind,
            SemanticOwnerSourceKindV1::DeclaredFunction
        );
        assert_eq!(observation.function_owner, observation.write_binding.owner());
        assert_eq!(observation.function_owner, observation.read_binding.owner());
        assert_eq!(observation.write_binding, observation.read_binding);
        assert!(!observation.planner_required);
        assert_eq!(observation.strict_or_dev, mode == CompoundPremiseModeV1::Strict);
        assert!(observation.raw_schedule.is_empty());
        assert_eq!(
            classify(&observation),
            CompoundPremiseDispositionV1::NoStandaloneRow
        );
    }
}

#[test]
fn top_level_compound_repeat_keeps_identity_shape_and_schedule() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    for mode in [CompoundPremiseModeV1::Release, CompoundPremiseModeV1::Strict] {
        let first = observe_in_mode(mode).expect("first parsed top-level compound witness");
        let second = observe_in_mode(mode).expect("second parsed top-level compound witness");
        assert_eq!(first.forest_binding.members(), second.forest_binding.members());
        assert_ne!(first.function_owner, second.function_owner);
        assert_eq!(first.function_origin, second.function_origin);
        assert_eq!(first.source_kind, second.source_kind);
        assert_eq!(first.frame_key, second.frame_key);
        assert_eq!(first.write_binding.binding(), second.write_binding.binding());
        assert_eq!(first.read_binding.binding(), second.read_binding.binding());
        assert_eq!(first.carrier, second.carrier);
        assert_eq!(first.strict_or_dev, second.strict_or_dev);
        assert_eq!(first.planner_required, second.planner_required);
        assert_eq!(first.raw_schedule, second.raw_schedule);
        assert_eq!(classify(&first), classify(&second));
    }
}
