use crate::mir::loop_recipe_contract::{
    LoopBindingKeyV1, LoopItemKeyV1, LoopJoinEdgeRoleV1, LoopJoinPortV1, LoopNodeKeyV1,
    LoopValueClassV2, LoopValueKeyV1,
};

use super::{
    fault_cut_points::{
        verify_recipe_fault_cut_points_for_test_v2, DynamicFullLoopFaultCutPointRejectV2,
    },
    invocation_carrier_lifecycle::{
        verify_recipe_invocation_lifecycle_for_test_v1, DynamicInvocationCarrierLifecycleRejectV1,
    },
    issue_dynamic_full_loop_semantic_program_v2,
    issue_dynamic_invocation_carrier_lifecycle_program_v1, DynamicFullLoopFaultFamilyV2,
    DynamicInvocationCarrierDestinationRefV1, DynamicInvocationCarrierPublicationV1,
};
use crate::mir::compiler::dynamic_full_body_recipe::coseal::{
    issue_dynamic_full_loop_source_recipe_envelope_v2, tests::fixture,
};

#[test]
fn exact_envelope_issues_one_atomic_dynamic_semantic_program() {
    let fixture = fixture(true);
    let envelope =
        issue_dynamic_full_loop_source_recipe_envelope_v2(fixture.candidate, &fixture.calls)
            .expect("exact source/Recipe/envelope");
    let program = issue_dynamic_full_loop_semantic_program_v2(envelope)
        .expect("atomic Dynamic semantic program");

    let after = program.after();
    assert_eq!(after.loop_key(), LoopNodeKeyV1::new(0));
    assert_eq!(after.binding(), LoopBindingKeyV1::new(0));
    assert_eq!(after.class(), LoopValueClassV2::Dynamic);

    let signature = program.join_sig().as_sig();
    assert_eq!(signature.loops.len(), 1);
    assert_eq!(signature.loops[0].edges.len(), 5);
    assert_eq!(signature.branches.len(), 1);
    assert_eq!(signature.port_bindings.len(), 2);
    assert_eq!(
        signature
            .loops
            .iter()
            .flat_map(|row| row.edges.iter())
            .filter(|edge| edge.role == LoopJoinEdgeRoleV1::Return)
            .count(),
        1
    );
    assert!(signature.port_bindings.iter().all(|row| {
        row.loop_key == LoopNodeKeyV1::new(0)
            && row.binding == LoopBindingKeyV1::new(0)
            && row.class == LoopValueClassV2::Dynamic
            && matches!(row.port, LoopJoinPortV1::Header | LoopJoinPortV1::After)
    }));
    assert!(signature
        .loops
        .iter()
        .flat_map(|row| row.edges.iter())
        .flat_map(|edge| edge.payload.iter())
        .all(|row| row.value != LoopValueKeyV1::new(10) && row.value != LoopValueKeyV1::new(14)));

    let local = program.iteration_local();
    assert_eq!(local.value(), LoopValueKeyV1::new(10));
    assert_eq!(
        local.producer(),
        crate::mir::loop_recipe_contract::LoopItemKeyV1::new(6)
    );
    assert_eq!(
        local.consumer(),
        crate::mir::loop_recipe_contract::LoopItemKeyV1::new(7)
    );
    let local_binding = local.binding();
    let local_scope = local.scope_region();
    let local_declaration = local.declaration().clone();
    let local_declaration_statement = local.declaration_statement().clone();
    let local_read = local.read().clone();

    let fault_rows = program.fault_cut_points();
    assert_eq!(
        fault_rows
            .rows()
            .iter()
            .map(|row| (row.item(), row.family(), row.normal_result()))
            .collect::<Vec<_>>(),
        vec![
            (
                LoopItemKeyV1::new(1),
                DynamicFullLoopFaultFamilyV2::DynamicLess,
                LoopValueKeyV1::new(5),
            ),
            (
                LoopItemKeyV1::new(5),
                DynamicFullLoopFaultFamilyV2::DynamicAdd,
                LoopValueKeyV1::new(9),
            ),
            (
                LoopItemKeyV1::new(6),
                DynamicFullLoopFaultFamilyV2::DynamicInvocation,
                LoopValueKeyV1::new(10),
            ),
            (
                LoopItemKeyV1::new(7),
                DynamicFullLoopFaultFamilyV2::DynamicInvocation,
                LoopValueKeyV1::new(11),
            ),
            (
                LoopItemKeyV1::new(9),
                DynamicFullLoopFaultFamilyV2::DynamicLess,
                LoopValueKeyV1::new(13),
            ),
            (
                LoopItemKeyV1::new(15),
                DynamicFullLoopFaultFamilyV2::DynamicAdd,
                LoopValueKeyV1::new(17),
            ),
        ]
    );

    let lifecycle = issue_dynamic_invocation_carrier_lifecycle_program_v1(program)
        .expect("complete Dynamic invocation-result lifecycle");
    let rows = lifecycle.invocation_lifecycle();
    let rows = rows.rows().collect::<Vec<_>>();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].producer(), LoopItemKeyV1::new(6));
    assert_eq!(rows[0].result(), LoopValueKeyV1::new(10));
    assert_eq!(
        rows[0].publication(),
        DynamicInvocationCarrierPublicationV1::OnNormalResultPublication
    );
    match rows[0].destination() {
        DynamicInvocationCarrierDestinationRefV1::LoopBodyLocal {
            declaration,
            declaration_statement,
            binding,
            scope_region,
            read,
            borrowed_by,
            input_contract,
        } => {
            assert_eq!(declaration, &local_declaration);
            assert_eq!(declaration_statement, &local_declaration_statement);
            assert_eq!(binding, local_binding);
            assert_eq!(scope_region, local_scope);
            assert_eq!(read, &local_read);
            assert_eq!(borrowed_by, LoopItemKeyV1::new(7));
            assert_eq!(
                input_contract,
                crate::mir::dynamic_invocation_contract::DynamicInvocationInputHomeV1::BorrowedNoEscapeForInvocation
            );
        }
        other => panic!("unexpected local destination: {other:?}"),
    }
    assert_eq!(rows[1].producer(), LoopItemKeyV1::new(7));
    assert_eq!(rows[1].result(), LoopValueKeyV1::new(11));
    assert_eq!(
        rows[1].publication(),
        DynamicInvocationCarrierPublicationV1::OnNormalResultPublication
    );
    match rows[1].destination() {
        DynamicInvocationCarrierDestinationRefV1::FullExpressionTemporary {
            boundary_source,
            boundary_item,
        } => {
            assert_eq!(boundary_item, LoopItemKeyV1::new(9));
            assert_ne!(boundary_source, rows[1].producer_source());
        }
        other => panic!("unexpected temporary destination: {other:?}"),
    }
    assert!(rows.iter().all(|row| {
        row.lifecycle()
            == crate::mir::dynamic_carrier_contract::DynamicCarrierLifecycleObligationV1::EndExactlyOnceUnlessForwarded
    }));
    assert_eq!(lifecycle.after().loop_key(), LoopNodeKeyV1::new(0));
    assert_eq!(lifecycle.fault_cut_points().rows().len(), 6);
}

#[test]
fn wrong_invocation_membership_rejects_the_private_catalog() {
    let fixture = fixture(true);
    let recipe = fixture.candidate.artifact.recipe().as_recipe();
    assert_eq!(
        verify_recipe_fault_cut_points_for_test_v2(
            recipe,
            [LoopItemKeyV1::new(6), LoopItemKeyV1::new(8)],
        ),
        Err(DynamicFullLoopFaultCutPointRejectV2::UnexpectedDynamicInvocation)
    );
}

#[test]
fn missing_dynamic_envelopes_reject_before_semantic_program_issuance() {
    let fixture = fixture(false);
    assert!(
        issue_dynamic_full_loop_source_recipe_envelope_v2(fixture.candidate, &fixture.calls)
            .is_err()
    );
}

#[test]
fn invocation_lifecycle_recipe_relations_reject_duplicates_and_wrong_boundaries() {
    let fixture = fixture(true);
    let recipe = fixture.candidate.artifact.recipe().as_recipe();
    assert_eq!(
        verify_recipe_invocation_lifecycle_for_test_v1(
            recipe,
            (LoopItemKeyV1::new(6), LoopValueKeyV1::new(10)),
            (LoopItemKeyV1::new(7), LoopValueKeyV1::new(11)),
            LoopValueKeyV1::new(10),
            LoopItemKeyV1::new(9),
        ),
        Ok(())
    );
    assert_eq!(
        verify_recipe_invocation_lifecycle_for_test_v1(
            recipe,
            (LoopItemKeyV1::new(6), LoopValueKeyV1::new(10)),
            (LoopItemKeyV1::new(6), LoopValueKeyV1::new(11)),
            LoopValueKeyV1::new(10),
            LoopItemKeyV1::new(9),
        ),
        Err(DynamicInvocationCarrierLifecycleRejectV1::InvocationCoverage)
    );
    assert_eq!(
        verify_recipe_invocation_lifecycle_for_test_v1(
            recipe,
            (LoopItemKeyV1::new(6), LoopValueKeyV1::new(10)),
            (LoopItemKeyV1::new(7), LoopValueKeyV1::new(12)),
            LoopValueKeyV1::new(10),
            LoopItemKeyV1::new(9),
        ),
        Err(DynamicInvocationCarrierLifecycleRejectV1::RecipeRelation)
    );
    assert_eq!(
        verify_recipe_invocation_lifecycle_for_test_v1(
            recipe,
            (LoopItemKeyV1::new(6), LoopValueKeyV1::new(10)),
            (LoopItemKeyV1::new(7), LoopValueKeyV1::new(11)),
            LoopValueKeyV1::new(10),
            LoopItemKeyV1::new(8),
        ),
        Err(DynamicInvocationCarrierLifecycleRejectV1::TemporaryBoundary)
    );
    assert_eq!(
        verify_recipe_invocation_lifecycle_for_test_v1(
            recipe,
            (LoopItemKeyV1::new(6), LoopValueKeyV1::new(10)),
            (LoopItemKeyV1::new(7), LoopValueKeyV1::new(11)),
            LoopValueKeyV1::new(9),
            LoopItemKeyV1::new(9),
        ),
        Err(DynamicInvocationCarrierLifecycleRejectV1::RecipeRelation)
    );
}

#[test]
fn semantic_program_surface_has_one_input_and_no_split_or_physical_escape() {
    let semantic_source = include_str!("mod.rs");
    assert!(semantic_source.contains("issue_dynamic_full_loop_semantic_program_v2(\n    envelope:"));
    assert!(!semantic_source.contains("<'env, 'decl>"));
    for forbidden in [
        "from_after",
        "into_parts",
        "FunctionOwnerIdV1",
        "VerifiedFunctionCompletionV1",
        "MirBuilder",
        "BasicBlockId",
        "ValueId",
    ] {
        assert!(
            !semantic_source.contains(forbidden),
            "semantic program surface must not contain {forbidden}"
        );
    }

    let recipe_facade = include_str!("../../../../loop_recipe_contract/mod.rs");
    assert!(!recipe_facade.contains("LoopJoinSigElaboratorV2"));
    assert!(!recipe_facade.contains("VerifiedLoopAfterBindingV2"));

    let join_v2 = include_str!("../../../../loop_recipe_contract/join_sig/v2.rs");
    assert_eq!(
        join_v2.matches("require_after_binding_internal(").count(),
        1
    );
    assert!(!join_v2.contains("pub(crate) fn require_after"));

    let fault_source = include_str!("fault_cut_points.rs");
    for forbidden in [
        "FaultRecord",
        "VerifiedHome",
        "BasicBlockId",
        "ValueId",
        "into_parts",
    ] {
        assert!(
            !fault_source.contains(forbidden),
            "Fault catalog must not contain {forbidden}"
        );
    }

    let lifecycle_source = include_str!("invocation_carrier_lifecycle.rs");
    for forbidden in [
        "HomeRoot",
        "HomeDemand",
        "HomeResultRelation",
        "BasicBlockId",
        "ValueId",
        "into_parts",
        "runtime tag",
        "selector ==",
    ] {
        assert!(
            !lifecycle_source.contains(forbidden),
            "invocation lifecycle must not contain {forbidden}"
        );
    }
    assert!(!lifecycle_source.contains("struct DynamicInvocationCarrierLifecycleRowV1 {\n    pub"));
    assert!(!lifecycle_source.contains(
        "derive(Debug, Clone, Copy, PartialEq, Eq)]\nstruct DynamicInvocationCarrierLifecycleRowV1"
    ));
    assert!(!lifecycle_source.contains("fn into_parts"));
}
