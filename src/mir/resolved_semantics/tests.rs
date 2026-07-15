use std::collections::{BTreeMap, BTreeSet};

use hakorune_mir_core::BindingId;

use super::ids::{BindingRefV1, FunctionOwnerIdV1, FunctionOwnerIssuerV1, RegionId, ScopeId};
use super::product::{ResolvedFunctionDataV1, ResolvedFunctionDraftV1};
use super::records::{
    BindingKindV1, BindingOriginV1, RegionKindV1, RegionOriginV1, ResolvedAssignmentTargetV1,
    ResolvedBindingRecordV1, ResolvedControlTransferV1, ResolvedExitOriginV1, ResolvedExitRecordV1,
    ResolvedRegionRecordV1, ResolvedScopeRecordV1, ScopeKindV1, ScopeOriginV1,
};
use super::source_site::{
    FunctionOriginV1, OwnedExprSiteV1, ResolvedExitSiteV1, SourceBindingSiteV1, SourceExprSiteV1,
    SourceNodeSiteV1, SourcePathSegmentV1, SourceStmtSiteV1,
};
use super::VerifiedResolvedFunctionV1;

fn owner() -> FunctionOwnerIdV1 {
    FunctionOwnerIssuerV1::new_for_compilation()
        .unwrap()
        .issue()
        .unwrap()
}

fn seal(data: ResolvedFunctionDataV1) -> VerifiedResolvedFunctionV1 {
    ResolvedFunctionDraftV1 { data }.seal().unwrap()
}

fn node(segments: Vec<SourcePathSegmentV1>) -> SourceNodeSiteV1 {
    SourceNodeSiteV1::from_segments(segments)
}

fn stmt(index: u32) -> SourceStmtSiteV1 {
    SourceStmtSiteV1::from_node(node(vec![SourcePathSegmentV1::Body(index)]))
}

fn expr(index: u32, role: SourcePathSegmentV1) -> SourceExprSiteV1 {
    SourceExprSiteV1::from_node(node(vec![SourcePathSegmentV1::Body(index), role]))
}

fn sample_data(owner: FunctionOwnerIdV1, binding: BindingId) -> ResolvedFunctionDataV1 {
    let function_origin = FunctionOriginV1::new(0, 3);
    let binding_ref = BindingRefV1::new(owner, binding);
    let scope = ScopeId::new(owner, 0);
    let region = RegionId::new(owner, 0);
    let body_scope = ScopeId::new(owner, 1000);
    let body_region = RegionId::new(owner, 1000);
    let body_origin = node(vec![SourcePathSegmentV1::FunctionBody]);
    let declaration = SourceBindingSiteV1::Local {
        statement: stmt(0),
        ordinal: 0,
    };
    let use_site = expr(1, SourcePathSegmentV1::Value);
    let assignment_site = expr(2, SourcePathSegmentV1::Target);
    let exit_site = stmt(3);

    ResolvedFunctionDataV1 {
        owner,
        function_origin,
        function_scope: scope,
        function_region: region,
        bindings: BTreeMap::from([(
            binding,
            ResolvedBindingRecordV1::new(
                "x",
                BindingKindV1::Local { ordinal: 0 },
                scope,
                BindingOriginV1::Source(declaration.clone()),
            ),
        )]),
        scopes: BTreeMap::from([
            (
                scope,
                ResolvedScopeRecordV1::new(
                    ScopeKindV1::Function,
                    None,
                    region,
                    vec![binding_ref],
                    ScopeOriginV1::Function(function_origin),
                ),
            ),
            (
                body_scope,
                ResolvedScopeRecordV1::new(
                    ScopeKindV1::LexicalBlock,
                    Some(scope),
                    body_region,
                    Vec::new(),
                    ScopeOriginV1::Source(body_origin.clone()),
                ),
            ),
        ]),
        regions: BTreeMap::from([
            (
                region,
                ResolvedRegionRecordV1::new(
                    RegionKindV1::Function,
                    None,
                    Some(scope),
                    RegionOriginV1::Function(function_origin),
                ),
            ),
            (
                body_region,
                ResolvedRegionRecordV1::new(
                    RegionKindV1::Sequence,
                    Some(region),
                    Some(body_scope),
                    RegionOriginV1::Source(body_origin),
                ),
            ),
        ]),
        declarations: BTreeMap::from([(declaration.clone(), binding_ref)]),
        variable_uses: BTreeMap::from([(
            use_site,
            super::ResolvedLexicalRefV1::Local(binding_ref),
        )]),
        assignment_targets: BTreeMap::from([(
            assignment_site,
            ResolvedAssignmentTargetV1::BindingRebind(binding_ref),
        )]),
        direct_call_targets: BTreeMap::new(),
        resolved_exits: BTreeMap::from([(
            ResolvedExitSiteV1::Statement(exit_site),
            ResolvedExitRecordV1::new(
                body_region,
                ResolvedExitOriginV1::ExplicitReturn,
                ResolvedControlTransferV1::Return {
                    target_function: region,
                },
            ),
        )]),
    }
}

pub(super) fn sample_verified_for_owner_forest(
    owner: FunctionOwnerIdV1,
    binding: BindingId,
) -> VerifiedResolvedFunctionV1 {
    seal(sample_data(owner, binding))
}

fn two_binding_data(
    owner: FunctionOwnerIdV1,
    first: BindingId,
    second: BindingId,
    reverse_scope_order: bool,
) -> ResolvedFunctionDataV1 {
    let mut data = sample_data(owner, first);
    let scope = data.function_scope;
    let region = data.function_region;
    let second_site = SourceBindingSiteV1::Local {
        statement: stmt(4),
        ordinal: 0,
    };
    let second_ref = BindingRefV1::new(owner, second);
    data.bindings.insert(
        second,
        ResolvedBindingRecordV1::new(
            "y",
            BindingKindV1::Local { ordinal: 0 },
            scope,
            BindingOriginV1::Source(second_site.clone()),
        ),
    );
    data.declarations.insert(second_site, second_ref);
    let first_ref = data.declarations.values().copied().next().unwrap();
    let declarations = if reverse_scope_order {
        vec![second_ref, first_ref]
    } else {
        vec![first_ref, second_ref]
    };
    data.scopes.insert(
        scope,
        ResolvedScopeRecordV1::new(
            ScopeKindV1::Function,
            None,
            region,
            declarations,
            ScopeOriginV1::Function(data.function_origin),
        ),
    );
    data
}

#[test]
fn sealed_product_exposes_read_only_owner_scoped_records() {
    let owner = owner();
    let binding = BindingId::new(11);
    let verified = seal(sample_data(owner, binding));
    let binding_ref = verified.binding_ref(binding).unwrap();

    assert_eq!(verified.owner(), owner);
    assert_eq!(verified.function_origin().function_ordinal(), 3);
    assert_eq!(verified.binding_count(), 1);
    assert_eq!(verified.scope_count(), 2);
    assert_eq!(verified.region_count(), 2);
    assert_eq!(
        verified.binding(binding_ref).unwrap().diagnostic_name(),
        "x"
    );
    assert_eq!(verified.function_scope(), ScopeId::new(owner, 0));
    assert_eq!(verified.function_region(), RegionId::new(owner, 0));

    let declaration = SourceBindingSiteV1::Local {
        statement: stmt(0),
        ordinal: 0,
    };
    let use_site = expr(1, SourcePathSegmentV1::Value);
    let assignment_site = expr(2, SourcePathSegmentV1::Target);
    let exit_site = stmt(3);

    assert_eq!(
        verified.declaration_binding(&declaration),
        Some(binding_ref)
    );
    assert_eq!(
        verified.variable_ref(&use_site),
        Some(super::ResolvedLexicalRefV1::Local(binding_ref))
    );
    assert_eq!(
        verified.assignment_target(&assignment_site),
        Some(&ResolvedAssignmentTargetV1::BindingRebind(binding_ref))
    );
    let exit = verified
        .resolved_exit(&ResolvedExitSiteV1::Statement(exit_site))
        .unwrap();
    assert_eq!(exit.source_region(), RegionId::new(owner, 1000));
    assert_eq!(exit.origin(), ResolvedExitOriginV1::ExplicitReturn);
    assert_eq!(
        exit.transfer(),
        ResolvedControlTransferV1::Return {
            target_function: RegionId::new(owner, 0),
        }
    );

    let binding_record = verified.binding(binding_ref).unwrap();
    assert_eq!(binding_record.kind(), BindingKindV1::Local { ordinal: 0 });
    assert_eq!(binding_record.owner_scope(), ScopeId::new(owner, 0));
    assert!(matches!(
        binding_record.origin(),
        BindingOriginV1::Source(_)
    ));

    let scope_record = verified.scope(verified.function_scope()).unwrap();
    assert_eq!(scope_record.kind(), ScopeKindV1::Function);
    assert_eq!(scope_record.parent(), None);
    assert_eq!(scope_record.declarations(), &[binding_ref]);
    assert!(matches!(scope_record.origin(), ScopeOriginV1::Function(_)));

    let region_record = verified.region(verified.function_region()).unwrap();
    assert_eq!(region_record.kind(), RegionKindV1::Function);
    assert_eq!(region_record.parent(), None);
    assert_eq!(
        region_record.lexical_scope(),
        Some(verified.function_scope())
    );
    assert!(matches!(
        region_record.origin(),
        RegionOriginV1::Function(_)
    ));
}

#[test]
fn lookup_rejects_handles_from_another_function_owner() {
    let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
    let owner = issuer.issue().unwrap();
    let other = issuer.issue().unwrap();
    let binding = BindingId::new(11);
    let verified = seal(sample_data(owner, binding));

    assert!(verified
        .binding(BindingRefV1::new(other, binding))
        .is_none());
    assert!(verified.scope(ScopeId::new(other, 0)).is_none());
    assert!(verified.region(RegionId::new(other, 0)).is_none());
}

#[test]
fn binding_arena_does_not_assume_dense_or_zero_based_canonical_ids() {
    let owner = owner();
    let binding = BindingId::new(42);
    let verified = seal(sample_data(owner, binding));
    let binding_ref = verified.binding_ref(binding).unwrap();

    assert_eq!(
        verified.binding(binding_ref).unwrap().diagnostic_name(),
        "x"
    );
    assert!(verified.binding_ref(BindingId::new(0)).is_none());
}

#[test]
fn structural_sites_distinguish_identical_roles_at_different_positions() {
    let first = expr(1, SourcePathSegmentV1::Value);
    let second = expr(2, SourcePathSegmentV1::Value);
    assert_ne!(first, second);
}

#[test]
fn binding_rebind_and_heap_writes_are_distinct_vocabulary() {
    let owner = owner();
    let binding = BindingRefV1::new(owner, BindingId::new(0));
    let receiver = expr(1, SourcePathSegmentV1::Receiver);

    assert_ne!(
        ResolvedAssignmentTargetV1::BindingRebind(binding),
        ResolvedAssignmentTargetV1::FieldWrite {
            receiver: receiver.clone()
        }
    );
    assert_ne!(
        ResolvedAssignmentTargetV1::FieldWrite {
            receiver: receiver.clone()
        },
        ResolvedAssignmentTargetV1::IndexWrite { receiver }
    );
}

#[test]
fn compilation_owner_issuer_never_reuses_a_brand() {
    let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
    assert_ne!(issuer.issue().unwrap(), issuer.issue().unwrap());
}

#[test]
fn independent_compilation_issuers_never_collide() {
    let first = owner();
    let second = owner();
    assert_ne!(first, second);
}

#[test]
fn seal_rejects_foreign_scope_identity() {
    let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
    let owner = issuer.issue().unwrap();
    let foreign = issuer.issue().unwrap();
    let mut data = sample_data(owner, BindingId::new(0));
    let record = data.scopes.remove(&data.function_scope).unwrap();
    data.scopes.insert(ScopeId::new(foreign, 0), record);

    assert!(ResolvedFunctionDraftV1 { data }.seal().is_err());
}

#[test]
fn normalized_graph_ignores_owner_and_raw_binding_numbers() {
    let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
    let first = seal(sample_data(issuer.issue().unwrap(), BindingId::new(7)));
    let second = seal(sample_data(issuer.issue().unwrap(), BindingId::new(91)));

    assert_eq!(first.normalized_graph(), second.normalized_graph());
}

#[test]
fn normalized_graph_ignores_scope_declaration_storage_order() {
    let first = seal(two_binding_data(
        owner(),
        BindingId::new(1),
        BindingId::new(2),
        false,
    ));
    let second = seal(two_binding_data(
        owner(),
        BindingId::new(9),
        BindingId::new(4),
        true,
    ));
    assert_eq!(first.normalized_graph(), second.normalized_graph());
}

#[test]
fn seal_rejects_missing_source_declaration_index() {
    let mut data = sample_data(owner(), BindingId::new(0));
    data.declarations.clear();

    assert!(ResolvedFunctionDraftV1 { data }.seal().is_err());
}

#[test]
fn seal_rejects_sibling_loop_as_control_target() {
    let owner = owner();
    let mut data = sample_data(owner, BindingId::new(0));
    data.resolved_exits.clear();
    let first_loop = RegionId::new(owner, 1);
    let sibling_loop = RegionId::new(owner, 2);
    data.regions.insert(
        first_loop,
        ResolvedRegionRecordV1::new(
            RegionKindV1::Loop,
            Some(data.function_region),
            None,
            RegionOriginV1::Source(stmt(4).node().clone()),
        ),
    );
    data.regions.insert(
        sibling_loop,
        ResolvedRegionRecordV1::new(
            RegionKindV1::Loop,
            Some(data.function_region),
            None,
            RegionOriginV1::Source(stmt(5).node().clone()),
        ),
    );
    let exit_site = SourceStmtSiteV1::from_node(node(vec![
        SourcePathSegmentV1::Body(4),
        SourcePathSegmentV1::LoopBody(0),
    ]));
    data.resolved_exits.insert(
        ResolvedExitSiteV1::Statement(exit_site),
        ResolvedExitRecordV1::new(
            first_loop,
            ResolvedExitOriginV1::ExplicitBreak,
            ResolvedControlTransferV1::Break {
                target_loop: sibling_loop,
            },
        ),
    );

    assert!(ResolvedFunctionDraftV1 { data }.seal().is_err());
}

#[test]
fn seal_rejects_falsified_outer_owner_for_inner_loop_exit() {
    let owner = owner();
    let mut data = sample_data(owner, BindingId::new(0));
    data.resolved_exits.clear();
    let outer_loop = RegionId::new(owner, 1);
    let inner_loop = RegionId::new(owner, 2);
    let outer_site = node(vec![SourcePathSegmentV1::Body(4)]);
    let inner_site = node(vec![
        SourcePathSegmentV1::Body(4),
        SourcePathSegmentV1::LoopBody(0),
    ]);
    data.regions.insert(
        outer_loop,
        ResolvedRegionRecordV1::new(
            RegionKindV1::Loop,
            Some(data.function_region),
            None,
            RegionOriginV1::Source(outer_site),
        ),
    );
    data.regions.insert(
        inner_loop,
        ResolvedRegionRecordV1::new(
            RegionKindV1::Loop,
            Some(outer_loop),
            None,
            RegionOriginV1::Source(inner_site.clone()),
        ),
    );
    let exit_site = SourceStmtSiteV1::from_node(node(vec![
        SourcePathSegmentV1::Body(4),
        SourcePathSegmentV1::LoopBody(0),
        SourcePathSegmentV1::LoopBody(0),
    ]));
    data.resolved_exits.insert(
        ResolvedExitSiteV1::Statement(exit_site),
        ResolvedExitRecordV1::new(
            outer_loop,
            ResolvedExitOriginV1::ExplicitBreak,
            ResolvedControlTransferV1::Break {
                target_loop: outer_loop,
            },
        ),
    );

    assert!(ResolvedFunctionDraftV1 { data }.seal().is_err());
}

#[test]
fn seal_rejects_binding_kind_origin_drift() {
    let mut data = sample_data(owner(), BindingId::new(0));
    let binding = *data.bindings.keys().next().unwrap();
    let record = data.bindings.get(&binding).unwrap();
    data.bindings.insert(
        binding,
        ResolvedBindingRecordV1::new(
            record.diagnostic_name(),
            BindingKindV1::Parameter { index: 0 },
            record.owner_scope(),
            record.origin().clone(),
        ),
    );
    assert!(ResolvedFunctionDraftV1 { data }.seal().is_err());
}

#[test]
fn mutable_draft_is_crate_private_and_distinct_from_verified_product() {
    let draft = ResolvedFunctionDraftV1 {
        data: sample_data(owner(), BindingId::new(0)),
    };
    assert_eq!(draft.data.bindings.len(), 1);
}

#[test]
fn p0_source_roles_have_stable_order_and_debug_vocabulary() {
    let roles = vec![
        SourcePathSegmentV1::LambdaBodyRoot,
        SourcePathSegmentV1::LambdaBody(3),
        SourcePathSegmentV1::QMarkOperand,
        SourcePathSegmentV1::MatchScrutinee,
        SourcePathSegmentV1::MatchArm(3),
        SourcePathSegmentV1::MatchElse,
        SourcePathSegmentV1::EnumMatchScrutinee,
        SourcePathSegmentV1::EnumMatchArm(3),
        SourcePathSegmentV1::EnumMatchElse,
        SourcePathSegmentV1::BlockExprPreludeRoot,
        SourcePathSegmentV1::BlockExprPrelude(3),
        SourcePathSegmentV1::BlockExprTail,
        SourcePathSegmentV1::TryBodyRoot,
        SourcePathSegmentV1::TryBody(3),
        SourcePathSegmentV1::CatchClause(3),
        SourcePathSegmentV1::CatchBodyRoot,
        SourcePathSegmentV1::CatchBody(3),
        SourcePathSegmentV1::CleanupBodyRoot,
        SourcePathSegmentV1::CleanupBody(3),
    ];
    let expected_debug = [
        "LambdaBodyRoot",
        "LambdaBody(3)",
        "QMarkOperand",
        "MatchScrutinee",
        "MatchArm(3)",
        "MatchElse",
        "EnumMatchScrutinee",
        "EnumMatchArm(3)",
        "EnumMatchElse",
        "BlockExprPreludeRoot",
        "BlockExprPrelude(3)",
        "BlockExprTail",
        "TryBodyRoot",
        "TryBody(3)",
        "CatchClause(3)",
        "CatchBodyRoot",
        "CatchBody(3)",
        "CleanupBodyRoot",
        "CleanupBody(3)",
    ];

    assert!(roles.windows(2).all(|pair| pair[0] < pair[1]));
    assert_eq!(
        roles
            .iter()
            .map(|role| format!("{role:?}"))
            .collect::<Vec<_>>(),
        expected_debug
    );
}

#[test]
fn owned_expression_site_distinguishes_equal_relative_paths_across_owners() {
    let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
    let first_owner = issuer.issue().unwrap();
    let second_owner = issuer.issue().unwrap();
    let relative = SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
        SourcePathSegmentV1::LambdaBody(0),
        SourcePathSegmentV1::QMarkOperand,
    ]));
    let first = OwnedExprSiteV1::new(first_owner, relative.clone());
    let second = OwnedExprSiteV1::new(second_owner, relative.clone());

    assert_eq!(first.owner(), first_owner);
    assert_eq!(first.site(), &relative);
    assert_ne!(first, second);
    assert_eq!(BTreeSet::from([first, second]).len(), 2);
}

#[test]
fn exit_site_preserves_statement_and_expression_source_families() {
    let statement = stmt(7);
    let expression = SourceExprSiteV1::from_node(statement.node().clone());

    let statement_exit = ResolvedExitSiteV1::Statement(statement.clone());
    let expression_exit = ResolvedExitSiteV1::Expression(expression);

    assert_eq!(statement_exit.node(), statement.node());
    assert_eq!(expression_exit.node(), statement.node());
    assert_ne!(statement_exit, expression_exit);
}

#[test]
fn seal_rejects_exit_origin_transfer_mismatch() {
    let owner = owner();
    let mut data = sample_data(owner, BindingId::new(0));
    let site = data.resolved_exits.keys().next().unwrap().clone();
    let source_region = data.resolved_exits[&site].source_region();
    data.resolved_exits.insert(
        site,
        ResolvedExitRecordV1::new(
            source_region,
            ResolvedExitOriginV1::ExplicitBreak,
            ResolvedControlTransferV1::Return {
                target_function: data.function_region,
            },
        ),
    );

    assert!(matches!(
        ResolvedFunctionDraftV1 { data }.seal(),
        Err(super::ResolvedFunctionVerificationErrorV1::ExitOriginTransferMismatch(_))
    ));
}

#[test]
fn seal_rejects_explicit_statement_origin_at_expression_site() {
    let owner = owner();
    let mut data = sample_data(owner, BindingId::new(0));
    let statement = stmt(3);
    let expression = SourceExprSiteV1::from_node(statement.node().clone());
    let record = data.resolved_exits.values().next().copied().unwrap();
    data.resolved_exits.clear();
    data.resolved_exits
        .insert(ResolvedExitSiteV1::Expression(expression), record);

    assert!(matches!(
        ResolvedFunctionDraftV1 { data }.seal(),
        Err(super::ResolvedFunctionVerificationErrorV1::UnsupportedExitSiteKind(_))
    ));
}

#[test]
fn source_region_containment_uses_closed_root_member_roles() {
    use super::verifier::source_region_contains_site_v1;

    let cases = [
        (
            RegionKindV1::Sequence,
            vec![SourcePathSegmentV1::FunctionBody],
            vec![SourcePathSegmentV1::Body(0)],
        ),
        (
            RegionKindV1::LexicalScope,
            vec![
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::ScopeBodyRoot,
            ],
            vec![
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::ScopeBody(0),
            ],
        ),
        (
            RegionKindV1::LexicalScope,
            vec![
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::TaskScopeBodyRoot,
            ],
            vec![
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::TaskScopeBody(0),
            ],
        ),
        (
            RegionKindV1::LexicalScope,
            vec![
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::FastMemBodyRoot,
            ],
            vec![
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::FastMemBody(0),
            ],
        ),
        (
            RegionKindV1::IfThen,
            vec![
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::IfThenBody,
            ],
            vec![SourcePathSegmentV1::Body(0), SourcePathSegmentV1::IfThen(0)],
        ),
        (
            RegionKindV1::IfElse,
            vec![
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::IfElseBody,
            ],
            vec![SourcePathSegmentV1::Body(0), SourcePathSegmentV1::IfElse(0)],
        ),
        (
            RegionKindV1::Loop,
            vec![SourcePathSegmentV1::Body(0)],
            vec![
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::LoopBody(0),
            ],
        ),
    ];

    for (kind, origin, site) in cases {
        assert!(source_region_contains_site_v1(
            kind,
            &RegionOriginV1::Source(node(origin)),
            &node(site),
        ));
    }
}
