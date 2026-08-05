//! D3-S0 source-backed eligibility protocol evidence, test-only.
//!
//! This row consumes the existing resolver/source/facts/preflight products and
//! seals them into one private witness. It deliberately stops before a neutral
//! production issuer, selector, Builder, or MIR caller exists.

use super::generic_resolved_carrier_projector_tests::{
    input_and_root, issue_projector_handoff_for_test, unit, ProjectorRejectV1, NESTED_IF_SOURCE,
};
use super::route_id::LoopRouteId;
use crate::ast::ASTNode;
use crate::mir::builder::control_flow::joinir::route_entry::router::{
    test_issue_live_preflight_frame, LoopRouteContext,
};
use crate::mir::builder::control_flow::lower::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::plan::facts::GenericLoopCarrierObservationV1;
use crate::mir::builder::control_flow::plan::single_planner::try_build_outcome;
use crate::mir::loop_structural_facts::{
    bind_resolved_loop_source_forest_v1, VerifiedLoopSourceForestBindingV1,
};
use crate::mir::resolved_semantics::{
    BindingRefV1, BodyChildRoleV1, ExprChildRoleV1, FunctionOriginV1, FunctionOwnerIdV1,
    LoopExecutionFrameKeyV1, ResolvedAssignmentTargetV1, ResolvedLexicalRefV1,
    SemanticOwnerSourceKindV1, SourceExprSiteV1,
};

const INDEX_SOURCE: &str = r#"
function generic_both_nested_index(i, j, items) {
    loop(i < 3) {
        loop(j < 3) {
            if i < 2 {
                items[j] = i
            }
            j = j + 1
        }
        i = i + 1
    }
    return j
}
"#;

const SHADOWING_SOURCE: &str = r#"
function generic_both_nested_shadow(i, j) {
    loop(i < 3) {
        loop(j < 3) {
            local j = 0
            j = j + 1
        }
        i = i + 1
    }
    return j
}
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EligibilityModeV1 {
    Release,
    Strict,
    PlannerRequired,
}

impl EligibilityModeV1 {
    fn config(self) -> crate::test_support::ScopedTestConfig {
        crate::test_support::ScopedTestConfig::apply(&[
            (
                "HAKO_JOINIR_STRICT",
                match self {
                    Self::Release => None,
                    Self::Strict | Self::PlannerRequired => Some("1"),
                },
            ),
            (
                "HAKO_JOINIR_PLANNER_REQUIRED",
                match self {
                    Self::PlannerRequired => Some("1"),
                    Self::Release | Self::Strict => None,
                },
            ),
            ("NYASH_JOINIR_STRICT", None),
        ])
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EligibilityTargetV1 {
    BindingRebind,
    IndexWrite,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum EligibilityCarrierV1 {
    CompleteRecursive(Box<[String]>),
    Ambiguous(Box<str>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EligibilityObservationRejectV1 {
    Owner,
    Forest,
    Navigation,
    Target,
    Binding,
    StrictAncestor,
    Facts,
    Carrier,
    Planner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EligibilityDispositionV1 {
    EligibleCompleteRecursive,
    UnresolvedStopIndexWriteAmbiguous,
    UnresolvedStopPlannerRequired,
    UnresolvedStopShadowing,
    UnresolvedStopMissingCapability,
    UnresolvedStopFactsIdentityMismatch,
}

#[derive(Debug, PartialEq, Eq)]
struct EligibilityObservationV1 {
    forest_binding: VerifiedLoopSourceForestBindingV1,
    function_origin: FunctionOriginV1,
    function_owner: FunctionOwnerIdV1,
    source_kind: SemanticOwnerSourceKindV1,
    frame_key: LoopExecutionFrameKeyV1,
    write_binding: BindingRefV1,
    read_binding: BindingRefV1,
    subscript_binding: Option<BindingRefV1>,
    target: EligibilityTargetV1,
    carrier: EligibilityCarrierV1,
    source_identity_stable: bool,
    strict_or_dev: bool,
    planner_required: bool,
    raw_schedule: Box<[LoopRouteId]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EligibilitySourceV1 {
    Natural,
    Index,
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

fn write_and_read_sites(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    root: &crate::mir::compiler::located::LocatedStmtV1<'_>,
) -> Result<
    (SourceExprSiteV1, SourceExprSiteV1, Option<SourceExprSiteV1>),
    EligibilityObservationRejectV1,
> {
    let source = input.source();
    let outer_body = source
        .child_body_from_stmt(root, BodyChildRoleV1::LoopBody)
        .map_err(|_| EligibilityObservationRejectV1::Navigation)?;
    let inner = source
        .body_stmt(&outer_body, 0)
        .map_err(|_| EligibilityObservationRejectV1::Navigation)?;
    let inner_body = source
        .child_body_from_stmt(&inner, BodyChildRoleV1::LoopBody)
        .map_err(|_| EligibilityObservationRejectV1::Navigation)?;
    let if_stmt = source
        .body_stmt(&inner_body, 0)
        .map_err(|_| EligibilityObservationRejectV1::Navigation)?;
    let write_stmt = match if_stmt.node() {
        ASTNode::If { .. } => {
            let then_body = source
                .child_body_from_stmt(&if_stmt, BodyChildRoleV1::IfThen)
                .map_err(|_| EligibilityObservationRejectV1::Navigation)?;
            source
                .body_stmt(&then_body, 0)
                .map_err(|_| EligibilityObservationRejectV1::Navigation)?
        }
        ASTNode::Local { .. } => source
            .body_stmt(&inner_body, 1)
            .map_err(|_| EligibilityObservationRejectV1::Navigation)?,
        ASTNode::Assignment { .. } => if_stmt,
        _ => return Err(EligibilityObservationRejectV1::Navigation),
    };
    let target = source
        .child_expr_from_stmt(&write_stmt, ExprChildRoleV1::AssignmentTarget)
        .map_err(|_| EligibilityObservationRejectV1::Navigation)?;
    let subscript = source
        .child_expr_from_expr(&target, ExprChildRoleV1::IndexSubscript)
        .ok()
        .map(|expr| expr.site().clone());
    let function_body = source
        .root_body()
        .map_err(|_| EligibilityObservationRejectV1::Navigation)?;
    let return_stmt = source
        .body_stmt(&function_body, 1)
        .map_err(|_| EligibilityObservationRejectV1::Navigation)?;
    let read = source
        .child_expr_from_stmt(&return_stmt, ExprChildRoleV1::ReturnValue)
        .map_err(|_| EligibilityObservationRejectV1::Navigation)?
        .site()
        .clone();
    Ok((target.site().clone(), read, subscript))
}

fn observe(
    source_text: &str,
    source_kind: EligibilitySourceV1,
) -> Result<EligibilityObservationV1, EligibilityObservationRejectV1> {
    let source_unit = unit(source_text);
    let (input, root) = input_and_root(&source_unit);
    if input.owner() != root.owner() {
        return Err(EligibilityObservationRejectV1::Owner);
    }
    let function = input.function();
    let forest = function
        .resolved_loop_source_forest(root.site())
        .map_err(|_| EligibilityObservationRejectV1::Forest)?;
    if forest.members().len() != 2
        || forest.members()[0].parent_index().is_some()
        || forest.members()[1].parent_index() != Some(0)
        || forest.members()[0].source().site() != root.site()
    {
        return Err(EligibilityObservationRejectV1::Forest);
    }
    let outer_site = forest.members()[0].source().site().clone();
    let inner_site = forest.members()[1].source().site().clone();
    let outer_frame_key = forest.members()[0].source().frame_key();
    let frame_key = outer_frame_key.clone();
    let forest_binding = bind_resolved_loop_source_forest_v1(forest)
        .map_err(|_| EligibilityObservationRejectV1::Forest)?;
    let (target_site, read_site, subscript_site) = write_and_read_sites(input, &root)?;
    let target = function
        .assignment_target(&target_site)
        .ok_or(EligibilityObservationRejectV1::Target)?;
    let (target_kind, write_binding) = match target {
        ResolvedAssignmentTargetV1::BindingRebind(binding) => {
            (EligibilityTargetV1::BindingRebind, *binding)
        }
        ResolvedAssignmentTargetV1::IndexWrite { receiver } => {
            let binding = match function.variable_ref(receiver) {
                Some(ResolvedLexicalRefV1::Local(binding)) => binding,
                _ => return Err(EligibilityObservationRejectV1::Binding),
            };
            (EligibilityTargetV1::IndexWrite, binding)
        }
        _ => return Err(EligibilityObservationRejectV1::Target),
    };
    let read_binding = match function.variable_ref(&read_site) {
        Some(ResolvedLexicalRefV1::Local(binding)) => binding,
        _ => return Err(EligibilityObservationRejectV1::Binding),
    };
    let subscript_binding =
        subscript_site
            .as_ref()
            .and_then(|site| match function.variable_ref(site) {
                Some(ResolvedLexicalRefV1::Local(binding)) => Some(binding),
                _ => None,
            });
    let source_identity_stable = function.source_kind()
        == SemanticOwnerSourceKindV1::DeclaredFunction
        && outer_site == *root.site()
        && inner_site != *root.site()
        && outer_frame_key.matches(&frame_key)
        && write_binding.owner() == function.owner()
        && read_binding.owner() == function.owner()
        && (target_kind == EligibilityTargetV1::BindingRebind && write_binding == read_binding
            || target_kind == EligibilityTargetV1::IndexWrite
                && subscript_binding == Some(read_binding));
    if !source_identity_stable {
        return Err(EligibilityObservationRejectV1::StrictAncestor);
    }
    if target_kind == EligibilityTargetV1::BindingRebind
        && !strict_ancestor(function, write_binding, &target_site)
    {
        return Err(EligibilityObservationRejectV1::StrictAncestor);
    }

    let ASTNode::Loop {
        condition, body, ..
    } = root.node()
    else {
        return Err(EligibilityObservationRejectV1::Navigation);
    };
    let ctx = LoopRouteContext::new(condition, body, "generic_eligibility/0", false, false);
    let outcome = try_build_outcome(&ctx).map_err(|_| EligibilityObservationRejectV1::Facts)?;
    let facts: &CanonicalLoopFacts = outcome
        .facts
        .as_ref()
        .ok_or(EligibilityObservationRejectV1::Facts)?;
    let generic = facts
        .facts
        .generic_loop_v1()
        .ok_or(EligibilityObservationRejectV1::Facts)?;
    let carrier = match (&source_kind, &generic.carrier_observation) {
        (
            EligibilitySourceV1::Natural,
            GenericLoopCarrierObservationV1::CompleteRecursiveCarrier(carriers),
        ) if !carriers.is_empty() => {
            EligibilityCarrierV1::CompleteRecursive(carriers.clone().into_boxed_slice())
        }
        (EligibilitySourceV1::Index, GenericLoopCarrierObservationV1::Ambiguous(reason))
            if reason == "assignment target" =>
        {
            EligibilityCarrierV1::Ambiguous(reason.clone().into_boxed_str())
        }
        _ => return Err(EligibilityObservationRejectV1::Carrier),
    };
    let strict_or_dev = crate::config::env::joinir_dev::strict_enabled();
    let planner_required = crate::config::env::joinir_dev::planner_required_enabled();
    let frame = test_issue_live_preflight_frame(&ctx, &outcome, strict_or_dev, planner_required);
    Ok(EligibilityObservationV1 {
        forest_binding,
        function_origin: function.function_origin(),
        function_owner: function.owner(),
        source_kind: function.source_kind(),
        frame_key,
        write_binding,
        read_binding,
        subscript_binding,
        target: target_kind,
        carrier,
        source_identity_stable,
        strict_or_dev,
        planner_required,
        raw_schedule: frame.test_raw_schedule().to_vec().into_boxed_slice(),
    })
}

fn classify(observation: Option<&EligibilityObservationV1>) -> EligibilityDispositionV1 {
    let Some(observation) = observation else {
        return EligibilityDispositionV1::UnresolvedStopMissingCapability;
    };
    assert!(observation.source_identity_stable);
    assert_eq!(observation.forest_binding.members().len(), 2);
    if observation.planner_required {
        return EligibilityDispositionV1::UnresolvedStopPlannerRequired;
    }
    if observation.target == EligibilityTargetV1::IndexWrite
        && matches!(observation.carrier, EligibilityCarrierV1::Ambiguous(_))
    {
        return EligibilityDispositionV1::UnresolvedStopIndexWriteAmbiguous;
    }
    if observation.target == EligibilityTargetV1::BindingRebind
        && matches!(
            observation.carrier,
            EligibilityCarrierV1::CompleteRecursive(_)
        )
        && observation.write_binding == observation.read_binding
        && observation.raw_schedule.as_ref()
            == [LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1]
    {
        return EligibilityDispositionV1::EligibleCompleteRecursive;
    }
    EligibilityDispositionV1::UnresolvedStopShadowing
}

fn classify_projector_pair(result: Result<(), ProjectorRejectV1>) -> EligibilityDispositionV1 {
    match result {
        Err(ProjectorRejectV1::FactsIdentityMismatch) => {
            EligibilityDispositionV1::UnresolvedStopFactsIdentityMismatch
        }
        _ => panic!("unexpected projector pair result"),
    }
}

fn observe_in_mode(
    mode: EligibilityModeV1,
    source_text: &str,
    source_kind: EligibilitySourceV1,
) -> Result<EligibilityObservationV1, EligibilityObservationRejectV1> {
    let _config = mode.config();
    observe(source_text, source_kind)
}

#[test]
fn source_backed_eligibility_accepts_natural_both_release_and_strict() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    for mode in [EligibilityModeV1::Release, EligibilityModeV1::Strict] {
        let observation = observe_in_mode(mode, NESTED_IF_SOURCE, EligibilitySourceV1::Natural)
            .expect("natural source eligibility witness");
        assert_eq!(observation.function_origin.function_ordinal(), 0);
        assert!(observation.source_identity_stable);
        assert_eq!(
            observation.function_owner,
            observation.write_binding.owner()
        );
        assert_eq!(
            observation.source_kind,
            SemanticOwnerSourceKindV1::DeclaredFunction
        );
        assert!(!observation.planner_required);
        assert_eq!(
            observation.strict_or_dev,
            mode != EligibilityModeV1::Release
        );
        assert_eq!(
            classify(Some(&observation)),
            EligibilityDispositionV1::EligibleCompleteRecursive
        );
    }
}

#[test]
fn source_backed_eligibility_keeps_negative_rows_unresolved() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let index = observe_in_mode(
        EligibilityModeV1::Release,
        INDEX_SOURCE,
        EligibilitySourceV1::Index,
    )
    .expect("Index/Ambiguous source witness");
    assert_eq!(
        classify(Some(&index)),
        EligibilityDispositionV1::UnresolvedStopIndexWriteAmbiguous
    );

    let planner = observe_in_mode(
        EligibilityModeV1::PlannerRequired,
        NESTED_IF_SOURCE,
        EligibilitySourceV1::Natural,
    )
    .expect("planner source witness");
    assert_eq!(
        classify(Some(&planner)),
        EligibilityDispositionV1::UnresolvedStopPlannerRequired
    );

    assert_eq!(
        classify(None),
        EligibilityDispositionV1::UnresolvedStopMissingCapability
    );
}

#[test]
fn source_backed_eligibility_shadowing_and_repeat_are_stable() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    assert!(matches!(
        observe_in_mode(
            EligibilityModeV1::Release,
            SHADOWING_SOURCE,
            EligibilitySourceV1::Natural,
        ),
        Err(EligibilityObservationRejectV1::StrictAncestor)
    ));

    let first = observe_in_mode(
        EligibilityModeV1::Release,
        NESTED_IF_SOURCE,
        EligibilitySourceV1::Natural,
    )
    .expect("first natural source witness");
    let second = observe_in_mode(
        EligibilityModeV1::Release,
        NESTED_IF_SOURCE,
        EligibilitySourceV1::Natural,
    )
    .expect("second natural source witness");
    assert_eq!(
        first.forest_binding.members(),
        second.forest_binding.members()
    );
    assert_eq!(first.frame_key, second.frame_key);
    assert_eq!(
        first.write_binding.binding(),
        second.write_binding.binding()
    );
    assert_eq!(first.read_binding.binding(), second.read_binding.binding());
    assert_eq!(first.carrier, second.carrier);
    assert_eq!(first.raw_schedule, second.raw_schedule);
    assert_eq!(first.planner_required, second.planner_required);
    assert_eq!(classify(Some(&first)), classify(Some(&second)));
}

#[test]
fn source_backed_eligibility_rejects_cross_invocation_pairing() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let first =
        issue_projector_handoff_for_test(NESTED_IF_SOURCE).expect("first source projector witness");
    let second = issue_projector_handoff_for_test(NESTED_IF_SOURCE)
        .expect("second source projector witness");
    assert_eq!(
        first.co_sealed_with(&second),
        Err(ProjectorRejectV1::FactsIdentityMismatch)
    );
    assert_eq!(
        classify_projector_pair(first.co_sealed_with(&second)),
        EligibilityDispositionV1::UnresolvedStopFactsIdentityMismatch
    );
}
