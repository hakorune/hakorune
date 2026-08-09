use crate::mir::loop_recipe_contract::{
    LoopBindingKeyV1, LoopJoinEdgeRoleV1, LoopJoinPortV1, LoopNodeKeyV1, LoopValueClassV2,
    LoopValueKeyV1,
};

use super::issue_dynamic_full_loop_semantic_program_v2;
use crate::mir::compiler::dynamic_full_body_recipe::coseal::{
    issue_dynamic_full_loop_source_recipe_envelope_v2, tests::fixture,
};

#[test]
fn exact_envelope_issues_one_atomic_dynamic_semantic_program() {
    let fixture = fixture(true);
    let envelope =
        issue_dynamic_full_loop_source_recipe_envelope_v2(fixture.candidate, &fixture.catalog)
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
}

#[test]
fn missing_dynamic_envelopes_reject_before_semantic_program_issuance() {
    let fixture = fixture(false);
    assert!(
        issue_dynamic_full_loop_source_recipe_envelope_v2(fixture.candidate, &fixture.catalog)
            .is_err()
    );
}

#[test]
fn semantic_program_surface_has_one_input_and_no_split_or_physical_escape() {
    let semantic_source = include_str!("mod.rs");
    assert!(semantic_source
        .contains("issue_dynamic_full_loop_semantic_program_v2<'env, 'decl>(\n    envelope:"));
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
}
