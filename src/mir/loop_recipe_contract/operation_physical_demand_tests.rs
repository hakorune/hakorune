use crate::mir::loop_recipe_contract::generic_g0::generic_operation_demand_parts_for_test;
use crate::mir::resolved_semantics::FunctionOwnerIssuerV1;

use super::{
    LoopOperationPhysicalDemandRejectV1, VerifiedLoopOperationPhysicalDemandV1,
    VerifiedLoopSemanticContextV1,
};

fn foreign_owner() -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
    let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().expect("owner issuer");
    issuer.issue().expect("foreign owner")
}

#[test]
fn generic_g0_full_demand_prepares_all_fifteen_operations_without_builder_effect() {
    let (operation_effect, context, continuation) = generic_operation_demand_parts_for_test();
    let demand =
        VerifiedLoopOperationPhysicalDemandV1::issue(context, operation_effect, continuation)
            .expect("Generic G0 demand");
    let prepared = demand.prepare_all().expect("Generic G0 prepare");
    assert_eq!(prepared.schedule().len(), 15);
    assert_eq!(prepared.coverage().operation_count(), 15);
    assert_eq!(prepared.demand().operation_effect().evidence().len(), 15);
    assert!(prepared.schedule().iter().any(|row| row.item().raw() == 3));
    assert_eq!(prepared.ledger().operation_rows().len(), 15);
    assert_eq!(prepared.ledger().read_binding_rows().len(), 4);
    assert_eq!(prepared.ledger().derived_carrier_seed_rows().len(), 1);
    assert_eq!(prepared.ledger().write_binding_rows().len(), 2);
}

#[test]
fn demand_rejects_foreign_context_owner_before_prepare() {
    let (operation_effect, context, continuation) = generic_operation_demand_parts_for_test();
    let owner = foreign_owner();
    let foreign_context = VerifiedLoopSemanticContextV1::from_parts(
        owner,
        context.origin(),
        context.source_kind(),
        context.loop_site().clone(),
        context.frame().clone(),
        context.scope_region(),
    );
    assert!(matches!(
        VerifiedLoopOperationPhysicalDemandV1::issue(
            foreign_context,
            operation_effect,
            continuation,
        ),
        Err(LoopOperationPhysicalDemandRejectV1::ContextOwnerMismatch)
    ));
}

#[test]
fn demand_moves_context_and_continuation_into_prepared_program() {
    let (operation_effect, context, continuation) = generic_operation_demand_parts_for_test();
    let owner = context.owner();
    let demand =
        VerifiedLoopOperationPhysicalDemandV1::issue(context, operation_effect, continuation)
            .expect("demand");
    let prepared = demand.prepare_all().expect("prepare");
    assert_eq!(prepared.demand().context().owner(), owner);
    assert_eq!(
        prepared.demand().continuation().owner(),
        prepared.demand().context().owner()
    );
}

#[test]
fn generic_write_projection_is_full_program_and_source_bound() {
    let (operation_effect, context, continuation) = generic_operation_demand_parts_for_test();
    let demand =
        VerifiedLoopOperationPhysicalDemandV1::issue(context, operation_effect, continuation)
            .expect("demand");
    let prepared = demand.prepare_all().expect("prepare");
    let rows = prepared.write_binding_rows().expect("write projection");
    let items: Vec<_> = rows.iter().map(|row| row.item().raw()).collect();
    assert_eq!(items, [11, 15]);
    for row in rows {
        assert_eq!(row.source_binding().owner(), prepared.demand().context().owner());
    }
}
