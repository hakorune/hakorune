//! Test-only resolved Generic source-projector evidence.
//!
//! This module consumes existing resolver/source-view products and emits a
//! private AST-free witness. It deliberately has no registry, Builder, MIR, or
//! runtime caller; production projector activation is a later design step.

use crate::ast::ASTNode;
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
    BindingRefV1, FunctionOriginV1, LoopExecutionFrameKeyV1, ResolvedAssignmentTargetV1,
    ResolvedLexicalRefV1, SemanticOwnerSourceKindV1, SourceExprSiteV1, SourceNodeSiteV1,
    SourcePathSegmentV1, SourceStmtSiteV1,
};
use crate::parser::NyashParser;

const SOURCE: &str = r#"
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProjectorRejectV1 {
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
}

#[derive(Debug, PartialEq, Eq)]
struct ProjectorSealV1 {
    function_origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    outer_site: SourceStmtSiteV1,
    inner_site: SourceStmtSiteV1,
    write_binding: BindingRefV1,
    read_binding: BindingRefV1,
    frame_key: LoopExecutionFrameKeyV1,
}

/// Private, non-Clone output. The facts and source capability are only
/// available through this one receipt, so they cannot be re-paired later.
#[derive(Debug)]
struct ResolvedGenericProjectorReceiptV1 {
    forest_binding: VerifiedLoopSourceForestBindingV1,
    facts: CanonicalLoopFacts,
    seal: ProjectorSealV1,
}

impl ResolvedGenericProjectorReceiptV1 {
    fn identity_is_stable(&self) -> bool {
        self.seal.source_kind == SemanticOwnerSourceKindV1::DeclaredFunction
            && self.seal.write_binding == self.seal.read_binding
            && self.seal.frame_key.matches(&self.seal.frame_key)
            && self
                .forest_binding
                .members()
                .first()
                .map(|member| member.parent_index().is_none())
                .unwrap_or(false)
    }
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

fn unit(source: &str) -> VerifiedResolvedSourceUnitV1 {
    VerifiedResolvedSourceUnitV1::resolve_function(parse_function(source))
        .expect("projector fixture resolves")
}

fn input_and_root(
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

fn expr_site(segments: &[SourcePathSegmentV1]) -> SourceExprSiteV1 {
    SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(segments.to_vec()))
}

fn write_site(shadowing: bool) -> SourceExprSiteV1 {
    expr_site(&[
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::LoopBody(0),
        SourcePathSegmentV1::LoopBody(if shadowing { 1 } else { 0 }),
        SourcePathSegmentV1::Target,
    ])
}

fn read_site() -> SourceExprSiteV1 {
    expr_site(&[SourcePathSegmentV1::Body(1), SourcePathSegmentV1::Value])
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
    let facts = try_build_outcome(&LoopRouteContext::new(
        condition,
        body,
        "generic_projector/0",
        false,
        false,
    ))
    .map_err(|_| ProjectorRejectV1::FactsAbsent)?
    .facts
    .ok_or(ProjectorRejectV1::FactsAbsent)?;
    let shadowing = body
        .first()
        .and_then(|statement| match statement {
            ASTNode::Loop { body, .. } => body.first(),
            _ => None,
        })
        .map(|statement| matches!(statement, ASTNode::Local { .. }))
        .unwrap_or(false);
    let write = write_site(shadowing);
    let read = read_site();
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
    let recursive = facts
        .facts
        .generic_loop_v1()
        .map(|facts| {
            matches!(
                facts.carrier_observation,
                crate::mir::builder::control_flow::plan::facts::GenericLoopCarrierObservationV1::CompleteRecursiveCarrier(_)
            )
        })
        .unwrap_or(false);
    if !recursive {
        return Err(ProjectorRejectV1::UnsupportedCarrier);
    }
    Ok(ResolvedGenericProjectorReceiptV1 {
        forest_binding,
        facts,
        seal: ProjectorSealV1 {
            function_origin: function.function_origin(),
            source_kind: function.source_kind(),
            outer_site,
            inner_site,
            write_binding,
            read_binding,
            frame_key,
        },
    })
}

#[test]
fn generic_resolved_projector_co_seals_forest_bindings_and_facts() {
    let unit = unit(SOURCE);
    let (input, root) = input_and_root(&unit);
    let receipt = issue_projector(input, &root).expect("positive projector witness");
    assert_eq!(receipt.forest_binding.members().len(), 2);
    assert!(receipt.identity_is_stable());
    assert!(receipt
        .facts
        .facts
        .generic_loop_v1()
        .map(|facts| {
            matches!(
                facts.carrier_observation,
                crate::mir::builder::control_flow::plan::facts::GenericLoopCarrierObservationV1::CompleteRecursiveCarrier(_)
            )
        })
        .unwrap_or(false));
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
