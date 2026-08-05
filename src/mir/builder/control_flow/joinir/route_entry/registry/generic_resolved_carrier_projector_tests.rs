//! Test-only resolved Generic source-projector evidence.
//!
//! This module consumes existing resolver/source-view products and emits a
//! private AST-free witness. It deliberately has no registry, Builder, MIR, or
//! runtime caller; production projector activation is a later design step.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::joinir::route_entry::router::test_issue_live_preflight_frame;
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::lower::normalize::CanonicalLoopFacts;
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
    SemanticOwnerSourceKindV1, SourceExprSiteV1, SourceStmtSiteV1,
};
use crate::parser::NyashParser;

pub(super) const SOURCE: &str = r#"
function generic_both(i, j) {
    loop(i < 3) {
        loop(j < 3) {
            j = j + 1
        }
        i = i + 1
    }
    return j
}
"#;

const SHADOWING_SOURCE: &str = r#"
function generic_both_shadowing(i, j) {
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

pub(super) const NESTED_IF_SOURCE: &str = r#"
function generic_both_nested_if(i, j) {
    loop(i < 3) {
        loop(j < 3) {
            if i < 2 {
                j = j + 1
            }
            j = j + 1
        }
        i = i + 1
    }
    return j
}
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ProjectorRejectV1 {
    ForeignOwner,
    SourceLookup,
    ForestShape,
    SourceNavigation,
    FactsAbsent,
    MissingBinding,
    NonBindingTarget,
    BindingOwnerMismatch,
    StrictAncestorMismatch,
    BindingMismatch,
    UnsupportedCarrier,
    FactsIdentityMismatch,
}

#[derive(Debug, PartialEq, Eq)]
struct ProjectorFactsIdentityV1 {
    loop_var: String,
    recursive_carriers: Vec<String>,
}

impl ProjectorFactsIdentityV1 {
    fn from_facts(facts: &CanonicalLoopFacts) -> Option<Self> {
        let generic = facts.facts.generic_loop_v1()?;
        let crate::mir::builder::control_flow::plan::facts::GenericLoopCarrierObservationV1::CompleteRecursiveCarrier(carriers) = &generic.carrier_observation else {
            return None;
        };
        Some(Self {
            loop_var: generic.loop_var.clone(),
            recursive_carriers: carriers.clone(),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ProjectorFactsObservationV1 {
    function_owner: FunctionOwnerIdV1,
    function_origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    loop_site: SourceStmtSiteV1,
    frame_key: LoopExecutionFrameKeyV1,
    identity: ProjectorFactsIdentityV1,
}

#[derive(Debug, PartialEq, Eq)]
struct ProjectorSealV1 {
    function_origin: FunctionOriginV1,
    function_owner: FunctionOwnerIdV1,
    source_kind: SemanticOwnerSourceKindV1,
    outer_site: SourceStmtSiteV1,
    inner_site: SourceStmtSiteV1,
    write_binding: BindingRefV1,
    read_binding: BindingRefV1,
    frame_key: LoopExecutionFrameKeyV1,
    facts_observation: ProjectorFactsObservationV1,
}

/// Private, non-Clone output. The facts and source capability are only
/// available through this one receipt, so they cannot be re-paired later.
#[derive(Debug)]
struct ResolvedGenericProjectorReceiptV1 {
    forest_binding: VerifiedLoopSourceForestBindingV1,
    seal: ProjectorSealV1,
    raw_schedule: Box<[super::route_id::LoopRouteId]>,
    mode_strict_or_dev: bool,
    mode_planner_required: bool,
    frame_recipe_first_allowed: bool,
    frame_contract_present: bool,
    v0_facts_present: bool,
    v1_facts_present: bool,
}

impl ResolvedGenericProjectorReceiptV1 {
    fn identity_is_stable(&self) -> bool {
        let forest_owner = self.forest_binding.owner();
        self.seal.source_kind == SemanticOwnerSourceKindV1::DeclaredFunction
            && self.seal.function_owner == self.seal.facts_observation.function_owner
            && self.seal.write_binding == self.seal.read_binding
            && forest_owner
                == crate::mir::loop_recipe_contract::LoopRecipeSourceOwnerV1::FunctionBody {
                    compilation_unit_ordinal: self.seal.function_origin.compilation_unit_ordinal(),
                    function_ordinal: self.seal.function_origin.function_ordinal(),
                }
            && self.seal.outer_site != self.seal.inner_site
            && self.seal.facts_observation.function_origin == self.seal.function_origin
            && self.seal.facts_observation.source_kind == self.seal.source_kind
            && self.seal.facts_observation.loop_site == self.seal.outer_site
            && self
                .seal
                .facts_observation
                .frame_key
                .matches(&self.seal.frame_key)
            && !self.seal.facts_observation.identity.loop_var.is_empty()
            && !self
                .seal
                .facts_observation
                .identity
                .recursive_carriers
                .is_empty()
            && self
                .forest_binding
                .members()
                .first()
                .map(|member| member.parent_index().is_none())
                .unwrap_or(false)
    }
}

/// Test-only source-backed handoff witness. It keeps the resolver-issued
/// receipt private and exposes only immutable observations to the sibling
/// protocol test. It is intentionally not Clone and never enters production.
#[derive(Debug)]
pub(super) struct ProjectorHandoffObservationV1 {
    receipt: ResolvedGenericProjectorReceiptV1,
}

impl ProjectorHandoffObservationV1 {
    pub(super) fn raw_schedule(&self) -> &[super::route_id::LoopRouteId] {
        &self.receipt.raw_schedule
    }

    pub(super) fn mode_flags(&self) -> (bool, bool) {
        (
            self.receipt.mode_strict_or_dev,
            self.receipt.mode_planner_required,
        )
    }

    pub(super) fn preflight_flags(&self) -> (bool, bool) {
        (
            self.receipt.frame_recipe_first_allowed,
            self.receipt.frame_contract_present,
        )
    }

    pub(super) fn facts_flags(&self) -> (bool, bool) {
        (self.receipt.v0_facts_present, self.receipt.v1_facts_present)
    }

    pub(super) fn source_identity_is_stable(&self) -> bool {
        self.receipt.identity_is_stable()
    }

    pub(super) fn recursive_carrier_count(&self) -> usize {
        self.receipt
            .seal
            .facts_observation
            .identity
            .recursive_carriers
            .len()
    }

    pub(super) fn source_forest_len(&self) -> usize {
        self.receipt.forest_binding.members().len()
    }

    pub(super) fn co_sealed_with(&self, other: &Self) -> Result<(), ProjectorRejectV1> {
        verify_facts_pair(
            &self.receipt.seal.facts_observation,
            &other.receipt.seal.facts_observation,
        )
    }

    pub(super) fn is_natural_both(&self) -> bool {
        self.receipt.identity_is_stable()
            && self.receipt.raw_schedule.as_ref()
                == [
                    super::route_id::LoopRouteId::GenericLoopV0,
                    super::route_id::LoopRouteId::GenericLoopV1,
                ]
    }
}

pub(super) fn issue_projector_handoff_for_test(
    source: &str,
) -> Result<ProjectorHandoffObservationV1, ProjectorRejectV1> {
    let unit = unit(source);
    let (input, root) = input_and_root(&unit);
    Ok(ProjectorHandoffObservationV1 {
        receipt: issue_projector(input, &root)?,
    })
}

fn verify_facts_pair(
    expected: &ProjectorFactsObservationV1,
    observed: &ProjectorFactsObservationV1,
) -> Result<(), ProjectorRejectV1> {
    if expected.function_owner != observed.function_owner
        || expected.function_origin != observed.function_origin
        || expected.source_kind != observed.source_kind
        || expected.loop_site != observed.loop_site
        || !expected.frame_key.matches(&observed.frame_key)
        || expected.identity != observed.identity
    {
        return Err(ProjectorRejectV1::FactsIdentityMismatch);
    }
    Ok(())
}

fn parse_function(source: &str) -> ASTNode {
    let root = NyashParser::parse_from_string(source).expect("projector fixture parses");
    let ASTNode::Program { statements, .. } = root else {
        panic!("projector fixture must parse to Program")
    };
    statements
        .into_iter()
        .find(|node| matches!(node, ASTNode::FunctionDeclaration { .. }))
        .expect("projector fixture must contain a function")
}

pub(super) fn unit(source: &str) -> VerifiedResolvedSourceUnitV1 {
    VerifiedResolvedSourceUnitV1::resolve_function(parse_function(source))
        .expect("projector fixture resolves")
}

pub(super) fn input_and_root(
    unit: &VerifiedResolvedSourceUnitV1,
) -> (ResolvedFunctionLoweringInputV1<'_>, LocatedStmtV1<'_>) {
    let input = unit.root_function_input().expect("root function input");
    let body = input.source().root_body().expect("function body");
    let root = input
        .source()
        .body_stmt(&body, 0)
        .expect("outer loop statement");
    (input, root)
}

struct ProjectorSitesV1 {
    write: SourceExprSiteV1,
    read: SourceExprSiteV1,
}

fn projector_sites(
    input: ResolvedFunctionLoweringInputV1<'_>,
    root: &LocatedStmtV1<'_>,
) -> Result<ProjectorSitesV1, ProjectorRejectV1> {
    let source = input.source();
    let outer_body = source
        .child_body_from_stmt(root, BodyChildRoleV1::LoopBody)
        .map_err(|_| ProjectorRejectV1::SourceNavigation)?;
    let inner = source
        .body_stmt(&outer_body, 0)
        .map_err(|_| ProjectorRejectV1::SourceNavigation)?;
    let inner_body = source
        .child_body_from_stmt(&inner, BodyChildRoleV1::LoopBody)
        .map_err(|_| ProjectorRejectV1::SourceNavigation)?;
    let write_stmt = match inner_body.statements().first() {
        Some(ASTNode::If { .. }) => {
            let if_stmt = source
                .body_stmt(&inner_body, 0)
                .map_err(|_| ProjectorRejectV1::SourceNavigation)?;
            let then_body = source
                .child_body_from_stmt(&if_stmt, BodyChildRoleV1::IfThen)
                .map_err(|_| ProjectorRejectV1::SourceNavigation)?;
            source
                .body_stmt(&then_body, 0)
                .map_err(|_| ProjectorRejectV1::SourceNavigation)?
        }
        Some(ASTNode::Local { .. }) => source
            .body_stmt(&inner_body, 1)
            .map_err(|_| ProjectorRejectV1::SourceNavigation)?,
        Some(ASTNode::Assignment { .. }) => source
            .body_stmt(&inner_body, 0)
            .map_err(|_| ProjectorRejectV1::SourceNavigation)?,
        _ => return Err(ProjectorRejectV1::SourceNavigation),
    };
    let write = source
        .child_expr_from_stmt(&write_stmt, ExprChildRoleV1::AssignmentTarget)
        .map_err(|_| ProjectorRejectV1::SourceNavigation)?
        .site()
        .clone();
    let function_body = source
        .root_body()
        .map_err(|_| ProjectorRejectV1::SourceNavigation)?;
    let return_stmt = source
        .body_stmt(&function_body, 1)
        .map_err(|_| ProjectorRejectV1::SourceNavigation)?;
    let read = source
        .child_expr_from_stmt(&return_stmt, ExprChildRoleV1::ReturnValue)
        .map_err(|_| ProjectorRejectV1::SourceNavigation)?
        .site()
        .clone();
    Ok(ProjectorSitesV1 { write, read })
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

fn issue_projector(
    input: ResolvedFunctionLoweringInputV1<'_>,
    root: &LocatedStmtV1<'_>,
) -> Result<ResolvedGenericProjectorReceiptV1, ProjectorRejectV1> {
    if input.owner() != root.owner() {
        return Err(ProjectorRejectV1::ForeignOwner);
    }
    let function = input.function();
    let forest = function
        .resolved_loop_source_forest(root.site())
        .map_err(|_| ProjectorRejectV1::SourceLookup)?;
    if forest.members().len() != 2
        || forest.members()[0].parent_index().is_some()
        || forest.members()[1].parent_index() != Some(0)
    {
        return Err(ProjectorRejectV1::ForestShape);
    }
    let outer_source = forest.members()[0].source();
    let inner_source = forest.members()[1].source();
    let outer_site = outer_source.site().clone();
    let inner_site = inner_source.site().clone();
    let frame_key = outer_source.frame_key();
    let forest_binding =
        bind_resolved_loop_source_forest_v1(forest).map_err(|_| ProjectorRejectV1::ForestShape)?;
    let ASTNode::Loop {
        condition, body, ..
    } = root.node()
    else {
        return Err(ProjectorRejectV1::SourceNavigation);
    };
    let ctx = LoopRouteContext::new(condition, body, "generic_projector/0", false, false);
    let outcome = try_build_outcome(&ctx).map_err(|_| ProjectorRejectV1::FactsAbsent)?;
    let facts = outcome
        .facts
        .as_ref()
        .ok_or(ProjectorRejectV1::FactsAbsent)?;
    let strict_or_dev = crate::config::env::joinir_dev::strict_enabled();
    let planner_required = crate::config::env::joinir_dev::planner_required_enabled();
    let frame = test_issue_live_preflight_frame(&ctx, &outcome, strict_or_dev, planner_required);
    let frame_env = frame.test_env();
    let raw_schedule = frame.test_raw_schedule().to_vec().into_boxed_slice();
    let v0_facts_present = facts.facts.generic_loop_v0().is_some();
    let v1_facts_present = facts.facts.generic_loop_v1().is_some();
    let sites = projector_sites(input, root)?;
    let write = sites.write;
    let read = sites.read;
    let write_binding = match function.assignment_target(&write) {
        Some(ResolvedAssignmentTargetV1::BindingRebind(binding)) => *binding,
        Some(_) => return Err(ProjectorRejectV1::NonBindingTarget),
        None => return Err(ProjectorRejectV1::MissingBinding),
    };
    let read_binding = match function.variable_ref(&read) {
        Some(ResolvedLexicalRefV1::Local(binding)) => binding,
        Some(_) => return Err(ProjectorRejectV1::MissingBinding),
        None => return Err(ProjectorRejectV1::MissingBinding),
    };
    if write_binding.owner() != function.owner() || read_binding.owner() != function.owner() {
        return Err(ProjectorRejectV1::BindingOwnerMismatch);
    }
    if !strict_ancestor(function, write_binding, &write) {
        return Err(ProjectorRejectV1::StrictAncestorMismatch);
    }
    if write_binding != read_binding {
        return Err(ProjectorRejectV1::BindingMismatch);
    }
    let facts_identity = ProjectorFactsIdentityV1::from_facts(&facts)
        .ok_or(ProjectorRejectV1::UnsupportedCarrier)?;
    let function_origin = function.function_origin();
    let function_owner = function.owner();
    let source_kind = function.source_kind();
    Ok(ResolvedGenericProjectorReceiptV1 {
        forest_binding,
        seal: ProjectorSealV1 {
            function_origin,
            function_owner,
            source_kind,
            outer_site: outer_site.clone(),
            inner_site,
            write_binding,
            read_binding,
            frame_key: frame_key.clone(),
            facts_observation: ProjectorFactsObservationV1 {
                function_owner,
                function_origin,
                source_kind,
                loop_site: outer_site.clone(),
                frame_key,
                identity: facts_identity,
            },
        },
        raw_schedule,
        mode_strict_or_dev: frame_env.strict_or_dev,
        mode_planner_required: frame_env.planner_required,
        frame_recipe_first_allowed: frame.test_recipe_first_allowed(),
        frame_contract_present: frame.test_recipe_contract_present(),
        v0_facts_present,
        v1_facts_present,
    })
}

#[test]
fn generic_resolved_projector_co_seals_forest_bindings_and_facts() {
    let unit = unit(SOURCE);
    let (input, root) = input_and_root(&unit);
    let receipt = issue_projector(input, &root).expect("positive projector witness");
    assert_eq!(receipt.forest_binding.members().len(), 2);
    assert!(receipt.identity_is_stable());
    assert!(!receipt
        .seal
        .facts_observation
        .identity
        .recursive_carriers
        .is_empty());
}

#[test]
fn generic_resolved_projector_rejects_shadowing_before_effects() {
    let unit = unit(SHADOWING_SOURCE);
    let (input, root) = input_and_root(&unit);
    assert!(matches!(
        issue_projector(input, &root),
        Err(ProjectorRejectV1::StrictAncestorMismatch)
    ));
}

#[test]
fn generic_resolved_projector_rejects_foreign_located_root() {
    let first = unit(SOURCE);
    let second = unit(SOURCE);
    let (input, _) = input_and_root(&first);
    let (_, foreign_root) = input_and_root(&second);
    assert!(matches!(
        issue_projector(input, &foreign_root),
        Err(ProjectorRejectV1::ForeignOwner)
    ));
}

#[test]
fn generic_resolved_projector_co_seals_parsed_nested_if() {
    let unit = unit(NESTED_IF_SOURCE);
    let (input, root) = input_and_root(&unit);
    let receipt = issue_projector(input, &root).expect("nested If projector witness");
    assert_eq!(receipt.forest_binding.members().len(), 2);
    assert_eq!(receipt.seal.facts_observation.identity.loop_var, "i");
    assert!(receipt
        .seal
        .facts_observation
        .identity
        .recursive_carriers
        .iter()
        .any(|carrier| carrier == "j"));
    assert!(receipt.identity_is_stable());
}

#[test]
fn generic_resolved_projector_keeps_facts_identity_private_to_receipt() {
    let first = unit(SOURCE);
    let second = unit(NESTED_IF_SOURCE);
    let (first_input, first_root) = input_and_root(&first);
    let (second_input, second_root) = input_and_root(&second);
    let first_receipt = issue_projector(first_input, &first_root).expect("first witness");
    let second_receipt = issue_projector(second_input, &second_root).expect("second witness");
    assert_eq!(
        first_receipt.seal.facts_observation.identity,
        second_receipt.seal.facts_observation.identity,
        "same carrier facts may share a shape"
    );
    assert_ne!(
        first_receipt.seal.facts_observation.function_owner,
        second_receipt.seal.facts_observation.function_owner,
        "facts observations remain invocation-owner sealed"
    );
    assert!(matches!(
        verify_facts_pair(
            &first_receipt.seal.facts_observation,
            &second_receipt.seal.facts_observation,
        ),
        Err(ProjectorRejectV1::FactsIdentityMismatch)
    ));
}
