use crate::mir::compiler::dynamic_full_body_recipe::coseal::{
    issue_dynamic_carrier_ingress_lifecycle_program_v1,
    issue_dynamic_full_loop_semantic_program_v2, issue_dynamic_full_loop_source_recipe_envelope_v2,
    issue_dynamic_invocation_carrier_lifecycle_program_v1,
    issue_dynamic_operator_carrier_lifecycle_program_v1, tests::fixture,
};
use crate::mir::loop_recipe_contract::{LoopBindingKeyV1, LoopItemKeyV1, LoopValueKeyV1};
use crate::mir::resolved_semantics::HomeDemandV1;

use super::{
    issue_dynamic_carrier_rebind_transaction_program_v1, DynamicCarrierCurrentDispositionV1,
    DynamicCarrierRebindTransactionRejectV1,
};

fn exact_rebind_program() -> super::VerifiedDynamicCarrierRebindTransactionProgramV1 {
    let fixture = fixture(true);
    let parameter_binding = fixture
        .candidate
        .source
        .bindings
        .iter()
        .find(|row| {
            row.role()
                == crate::mir::compiler::dynamic_full_body_source::DynamicFullBodyBindingRoleV1::Pos
        })
        .expect("pos binding")
        .binding();
    let envelope =
        issue_dynamic_full_loop_source_recipe_envelope_v2(fixture.candidate, fixture.calls)
            .expect("exact source/Recipe envelope");
    let semantic = issue_dynamic_full_loop_semantic_program_v2(envelope).expect("semantic program");
    let invocation = issue_dynamic_invocation_carrier_lifecycle_program_v1(semantic)
        .expect("invocation lifecycle");
    let operator = issue_dynamic_operator_carrier_lifecycle_program_v1(invocation)
        .expect("operator lifecycle");
    let ingress = issue_dynamic_carrier_ingress_lifecycle_program_v1(
        operator,
        1,
        parameter_binding,
        HomeDemandV1::Handle,
    )
    .expect("borrowed ingress");
    issue_dynamic_carrier_rebind_transaction_program_v1(ingress)
        .expect("rebind transaction relation")
}

#[test]
fn exact_program_seals_borrowed_current_read_replace_and_backedge() {
    let program = exact_rebind_program();
    assert_eq!(
        program.current(),
        DynamicCarrierCurrentDispositionV1::BorrowedIngressNoEnd
    );
    assert_eq!(program.read().item, LoopItemKeyV1::new(13));
    assert_eq!(program.read().binding, LoopBindingKeyV1::new(0));
    assert_eq!(program.read().result, LoopValueKeyV1::new(15));
    assert_eq!(program.commit().write, LoopItemKeyV1::new(16));
    assert_eq!(program.commit().result, LoopValueKeyV1::new(17));
    assert_eq!(program.commit().binding, LoopBindingKeyV1::new(0));
    assert_eq!(
        program.commit().backedge_loop,
        crate::mir::loop_recipe_contract::LoopNodeKeyV1::new(0)
    );
    assert_eq!(
        program.fault(),
        crate::mir::dynamic_operator_contract::DynamicOperatorFaultV1::TypeErrorBeforeResultNoOperandMutationNoRebind
    );
}

#[test]
fn missing_parameter_demand_rejects_before_rebind_program_exists() {
    let fixture = fixture(true);
    let parameter_binding = fixture
        .candidate
        .source
        .bindings
        .iter()
        .find(|row| {
            row.role()
                == crate::mir::compiler::dynamic_full_body_source::DynamicFullBodyBindingRoleV1::Pos
        })
        .expect("pos binding")
        .binding();
    let envelope =
        issue_dynamic_full_loop_source_recipe_envelope_v2(fixture.candidate, fixture.calls)
            .expect("exact source/Recipe envelope");
    let semantic = issue_dynamic_full_loop_semantic_program_v2(envelope).expect("semantic program");
    let invocation = issue_dynamic_invocation_carrier_lifecycle_program_v1(semantic)
        .expect("invocation lifecycle");
    let operator = issue_dynamic_operator_carrier_lifecycle_program_v1(invocation)
        .expect("operator lifecycle");
    assert!(matches!(
        issue_dynamic_carrier_ingress_lifecycle_program_v1(
            operator,
            0,
            parameter_binding,
            HomeDemandV1::Handle,
        ),
        Err(super::DynamicCarrierIngressLifecycleProgramRejectV1::ParameterDemand)
    ));
}

#[test]
fn semantic_rebind_module_does_not_execute_or_split_physical_state() {
    let source = include_str!("carrier_rebind.rs");
    for forbidden in [
        "into_parts",
        "install_new",
        "take_old",
        "ValueId",
        "MirBuilder",
        "ReleaseStrong",
        "Completion",
        "fallback",
        "retry",
    ] {
        assert!(
            !source.contains(forbidden),
            "forbidden term in rebind issuer: {forbidden}"
        );
    }
    let _ = DynamicCarrierRebindTransactionRejectV1::Ingress;
}
