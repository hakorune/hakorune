use super::*;
use crate::mir::module_invocation_identity::ModuleInvocationBrandV1;
use crate::mir::{
    BasicBlock, BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirInstruction, MirType,
    ValueId,
};

fn test_function_with_site(with_projection: bool) -> (MirFunction, CheckedCallOutPlanTableV1) {
    let source = BasicBlockId::new(0);
    let normal = BasicBlockId::new(1);
    let fault = BasicBlockId::new(2);
    let site = CheckedCallOutSiteIdV1::from_test(6);
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "checked/0".to_owned(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::READ,
        },
        source,
    );
    function.add_block(BasicBlock::new(normal));
    function.add_block(BasicBlock::new(fault));
    function
        .get_block_mut(source)
        .unwrap()
        .set_terminator(MirInstruction::CheckedCallOut {
            site_id: site,
            receiver: ValueId::new(0),
            arguments: vec![],
            normal_landing: normal,
            fault_landing: fault,
            effects: EffectMask::READ,
        });
    for landing in [normal, fault] {
        function
            .get_block_mut(landing)
            .unwrap()
            .add_predecessor(source);
    }
    if with_projection {
        function.get_block_mut(normal).unwrap().add_instruction(
            MirInstruction::CheckedCallOutNormalResult {
                site_id: site,
                dst: ValueId::new(1),
            },
        );
    }
    let mut plans = CheckedCallOutPlanTableV1::default();
    plans
        .admit(CheckedCallOutSitePlanV1::from_test(
            site,
            CheckedCallOutEntryIdV1::from_test(17),
            CheckedCallOutNormalShapeV1::ImmediateI64,
            EffectMask::READ,
            ModuleInvocationBrandV1::legacy_test(),
        ))
        .unwrap();
    (function, plans)
}
#[test]
fn plan_json_roundtrip_preserves_site_shape_and_stamp() {
    let plan = CheckedCallOutSitePlanV1::from_test(
        CheckedCallOutSiteIdV1::from_test(6),
        CheckedCallOutEntryIdV1::from_test(17),
        CheckedCallOutNormalShapeV1::EndAuthorizedHandle {
            lease_slot: CheckedCallOutLeaseSlotIdV1::from_test(1),
        },
        EffectMask::READ,
        ModuleInvocationBrandV1::legacy_test(),
    );
    let json = plan.to_json_for_test();
    let roundtrip =
        CheckedCallOutSitePlanV1::from_json_for_test(&json).expect("test-only JSON roundtrip");
    assert_eq!(roundtrip, plan);
    assert_eq!(json["site_id"], 6);
    assert_eq!(json["normal_shape"]["kind"], "end_authorized_handle");
    assert_eq!(json["plan_stamp"]["compiler_domain"], 1);
    assert_eq!(json["plan_stamp"]["invocation_ordinal"], 1);
}
#[test]
fn plan_json_roundtrip_preserves_foreign_domain_and_rejects_zero_parts() {
    let plan = CheckedCallOutSitePlanV1::from_test(
        CheckedCallOutSiteIdV1::from_test(8),
        CheckedCallOutEntryIdV1::from_test(19),
        CheckedCallOutNormalShapeV1::ImmediateI64,
        EffectMask::READ,
        ModuleInvocationBrandV1::test_with_parts(7, 3),
    );
    let json = plan.to_json_for_test();
    let roundtrip =
        CheckedCallOutSitePlanV1::from_json_for_test(&json).expect("foreign domain roundtrip");
    assert_eq!(roundtrip, plan);
    let reject_zero = |field: &str| {
        let mut invalid = json.clone();
        invalid["plan_stamp"][field] = serde_json::json!(0);
        CheckedCallOutSitePlanV1::from_json_for_test(&invalid).unwrap_err()
    };
    assert!(reject_zero("compiler_domain").contains("non-zero test domain"));
    assert!(reject_zero("invocation_ordinal").contains("non-zero test ordinal"));
}
#[test]
fn duplicate_site_and_wrong_effect_are_rejected() {
    let plan = CheckedCallOutSitePlanV1::from_test(
        CheckedCallOutSiteIdV1::from_test(7),
        CheckedCallOutEntryIdV1::from_test(18),
        CheckedCallOutNormalShapeV1::ImmediateI64,
        EffectMask::READ,
        ModuleInvocationBrandV1::legacy_test(),
    );
    let mut table = CheckedCallOutPlanTableV1::default();
    table.admit(plan.clone()).expect("first site");
    assert!(matches!(
        table.admit(plan),
        Err(CheckedCallOutPlanRejectV1::DuplicateSite(_))
    ));
    assert!(matches!(
        table
            .get(CheckedCallOutSiteIdV1::from_test(7))
            .unwrap()
            .validate_instruction(
                CheckedCallOutSiteIdV1::from_test(7),
                BasicBlockId::new(1),
                BasicBlockId::new(2),
                EffectMask::WRITE,
            ),
        Err(CheckedCallOutPlanRejectV1::EffectCacheMismatch)
    ));
}

#[test]
fn non_aot_backends_reject_checked_callout_by_name() {
    let term = MirInstruction::CheckedCallOut {
        site_id: CheckedCallOutSiteIdV1::from_test(1),
        receiver: ValueId::new(0),
        arguments: vec![],
        normal_landing: BasicBlockId::new(1),
        fault_landing: BasicBlockId::new(2),
        effects: EffectMask::READ,
    };
    assert_eq!(
        crate::mir::contracts::backend_core_ops::instruction_tag(&term),
        "CheckedCallOut"
    );
    assert!(crate::mir::contracts::backend_core_ops::is_supported_mir_json_terminator(&term));
    assert!(!crate::mir::contracts::backend_core_ops::is_supported_vm_terminator(&term));
    assert!(
        crate::mir::contracts::backend_core_ops::llvm_json_ops_for_instruction(&term).is_empty()
    );
}

#[test]
fn function_census_accepts_exact_plan_terminator_projection_triplet() {
    let (function, plans) = test_function_with_site(true);
    let verified = plans.verify_function(&function).expect("exact triplet");
    assert_eq!(verified.site_count(), 1);
}

#[test]
fn function_census_rejects_orphan_projection_and_late_predecessor() {
    let (function, plans) = test_function_with_site(false);
    assert!(matches!(
        plans.verify_function(&function),
        Err(CheckedCallOutFunctionRejectV1::OrphanProjection(site))
            if site.as_u32() == 6
    ));

    let (mut function, plans) = test_function_with_site(true);
    function
        .get_block_mut(BasicBlockId::new(1))
        .unwrap()
        .add_predecessor(BasicBlockId::new(9));
    assert!(matches!(
        plans.verify_function(&function),
        Err(CheckedCallOutFunctionRejectV1::LandingPredecessorMismatch(site))
            if site.as_u32() == 6
    ));
}

#[test]
fn admitted_text_scan_pair_is_typed_and_move_only() {
    let pair = CheckedCallOutSitePlanPairV1::from_admitted(
        CheckedCallOutAdmittedSiteInputV1 {
            entry: CheckedCallOutEntryIdV1::from_test(1),
            call_abi_revision: 1,
            wire_revision: 2,
            normal_shape: CheckedCallOutNormalShapeV1::EndAuthorizedHandle {
                lease_slot: CheckedCallOutLeaseSlotIdV1::from_test(0),
            },
            effects: EffectMask::READ,
        },
        CheckedCallOutAdmittedSiteInputV1 {
            entry: CheckedCallOutEntryIdV1::from_test(2),
            call_abi_revision: 1,
            wire_revision: 2,
            normal_shape: CheckedCallOutNormalShapeV1::ImmediateI64,
            effects: EffectMask::READ,
        },
        ModuleInvocationBrandV1::legacy_test(),
    )
    .expect("exact TextScan pair");
    pair.consume(|i6, i7| {
        assert_eq!(i6.site_id(), CheckedCallOutSiteIdV1::from_test(0));
        assert_eq!(i7.site_id(), CheckedCallOutSiteIdV1::from_test(1));
        assert!(matches!(
            i6.normal_shape(),
            CheckedCallOutNormalShapeV1::EndAuthorizedHandle { .. }
        ));
        assert!(matches!(
            i7.normal_shape(),
            CheckedCallOutNormalShapeV1::ImmediateI64
        ));
    });
}

#[test]
fn admitted_text_scan_pair_rejects_wrong_i7_shape() {
    let error = CheckedCallOutSitePlanPairV1::from_admitted(
        CheckedCallOutAdmittedSiteInputV1 {
            entry: CheckedCallOutEntryIdV1::from_test(1),
            call_abi_revision: 1,
            wire_revision: 2,
            normal_shape: CheckedCallOutNormalShapeV1::EndAuthorizedHandle {
                lease_slot: CheckedCallOutLeaseSlotIdV1::from_test(0),
            },
            effects: EffectMask::READ,
        },
        CheckedCallOutAdmittedSiteInputV1 {
            entry: CheckedCallOutEntryIdV1::from_test(2),
            call_abi_revision: 1,
            wire_revision: 2,
            normal_shape: CheckedCallOutNormalShapeV1::EndAuthorizedHandle {
                lease_slot: CheckedCallOutLeaseSlotIdV1::from_test(1),
            },
            effects: EffectMask::READ,
        },
        ModuleInvocationBrandV1::legacy_test(),
    );
    assert!(matches!(
        error,
        Err(CheckedCallOutSitePlanPairRejectV1::InvalidI7)
    ));
}
