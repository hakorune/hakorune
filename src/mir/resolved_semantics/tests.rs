use std::collections::BTreeMap;

use hakorune_mir_core::BindingId;

use super::ids::{BindingRefV1, FunctionOwnerIdV1, RegionId, ScopeId};
use super::product::{ResolvedFunctionDataV1, ResolvedFunctionDraftV1};
use super::records::{
    BindingKindV1, BindingOriginV1, RegionKindV1, RegionOriginV1, ResolvedAssignmentTargetV1,
    ResolvedBindingRecordV1, ResolvedControlExitV1, ResolvedRegionRecordV1, ResolvedScopeRecordV1,
    ScopeKindV1, ScopeOriginV1,
};
use super::source_site::{
    FunctionOriginV1, SourceBindingSiteV1, SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1,
    SourceStmtSiteV1,
};
use super::VerifiedResolvedFunctionV1;

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
        scopes: BTreeMap::from([(
            scope,
            ResolvedScopeRecordV1::new(
                ScopeKindV1::Function,
                None,
                region,
                vec![binding_ref],
                ScopeOriginV1::Function(function_origin),
            ),
        )]),
        regions: BTreeMap::from([(
            region,
            ResolvedRegionRecordV1::new(
                RegionKindV1::Function,
                None,
                Some(scope),
                RegionOriginV1::Function(function_origin),
            ),
        )]),
        declarations: BTreeMap::from([(declaration, binding_ref)]),
        variable_uses: BTreeMap::from([(use_site, binding_ref)]),
        assignment_targets: BTreeMap::from([(
            assignment_site,
            ResolvedAssignmentTargetV1::BindingRebind(binding_ref),
        )]),
        control_exits: BTreeMap::from([(
            exit_site,
            ResolvedControlExitV1::Return {
                target_function: region,
            },
        )]),
    }
}

#[test]
fn sealed_product_exposes_read_only_owner_scoped_records() {
    let owner = FunctionOwnerIdV1::from_raw(7);
    let binding = BindingId::new(11);
    let verified = VerifiedResolvedFunctionV1::from_unverified_data_for_schema_test(sample_data(
        owner, binding,
    ));
    let binding_ref = verified.binding_ref(binding).unwrap();

    assert_eq!(verified.owner(), owner);
    assert_eq!(verified.function_origin().function_ordinal(), 3);
    assert_eq!(verified.binding_count(), 1);
    assert_eq!(verified.scope_count(), 1);
    assert_eq!(verified.region_count(), 1);
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
    assert_eq!(verified.variable_binding(&use_site), Some(binding_ref));
    assert_eq!(
        verified.assignment_target(&assignment_site),
        Some(&ResolvedAssignmentTargetV1::BindingRebind(binding_ref))
    );
    assert_eq!(
        verified.control_exit(&exit_site),
        Some(ResolvedControlExitV1::Return {
            target_function: RegionId::new(owner, 0)
        })
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
    let owner = FunctionOwnerIdV1::from_raw(7);
    let other = FunctionOwnerIdV1::from_raw(8);
    let binding = BindingId::new(11);
    let verified = VerifiedResolvedFunctionV1::from_unverified_data_for_schema_test(sample_data(
        owner, binding,
    ));

    assert!(verified
        .binding(BindingRefV1::new(other, binding))
        .is_none());
    assert!(verified.scope(ScopeId::new(other, 0)).is_none());
    assert!(verified.region(RegionId::new(other, 0)).is_none());
}

#[test]
fn binding_arena_does_not_assume_dense_or_zero_based_canonical_ids() {
    let owner = FunctionOwnerIdV1::from_raw(7);
    let binding = BindingId::new(42);
    let verified = VerifiedResolvedFunctionV1::from_unverified_data_for_schema_test(sample_data(
        owner, binding,
    ));
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
    let owner = FunctionOwnerIdV1::from_raw(7);
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
fn mutable_draft_is_crate_private_and_distinct_from_verified_product() {
    let draft = ResolvedFunctionDraftV1 {
        data: sample_data(FunctionOwnerIdV1::from_raw(7), BindingId::new(0)),
    };
    assert_eq!(draft.data.bindings.len(), 1);
}
