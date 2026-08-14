use super::*;
use crate::mir::a_prime_i64_physical_receipt::{
    APrimeI64BackendFamilyV1, APrimeI64CallArgumentReceiptV1, APrimeI64CallEdgeReceiptV1,
    APrimeI64ParameterReceiptV1, APrimeI64ReturnReceiptV1, A_PRIME_I64_FORMAL_PARAMETER_COUNT,
};
use crate::mir::checked_callout::{
    CheckedCallOutAdmittedSiteInputV1, CheckedCallOutLeaseSlotIdV1, CheckedCallOutNormalShapeV1,
    CheckedCallOutPlanTableV1, CheckedCallOutSitePlanPairV1,
};
use crate::mir::core_method_op::CoreMethodOp;
use crate::mir::generated::core_method_contract_rows::CORE_METHOD_CONTRACT_RESULT_ROWS_V1;
use crate::mir::module_invocation_identity::ModuleInvocationBrandV1;
use crate::mir::{
    BasicBlock, BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirInstruction, MirType,
    ValueId,
};

#[test]
fn projection_keeps_exact_two_typed_sites_and_stamp() {
    let admission = admission();
    let receipt = receipt();
    let site_plans = site_plans();
    let projection = project(
        &admission,
        &receipt,
        &site_plans,
        &function(),
        formal_parameters(),
    )
    .expect("valid admission/receipt/function projection");
    assert_eq!(projection.calls().len(), 2);
    assert_eq!(projection.calls()[0].role().as_str(), "substring");
    assert_eq!(projection.calls()[1].role().as_str(), "index_of");
    assert_eq!(
        projection.calls()[0].entry().entry(),
        TextScanAotEntryIdV1::Substring
    );
    assert_eq!(
        projection.calls()[1].entry().entry(),
        TextScanAotEntryIdV1::IndexOf
    );
    assert_eq!(projection.calls()[0].site_id().as_u32(), 0);
    assert_eq!(projection.calls()[1].site_id().as_u32(), 1);
    assert_eq!(projection.registry_generation(), 7);
    assert_eq!(
        projection.plan_stamp(),
        ModuleInvocationBrandV1::test_with_ordinal(7)
    );
    assert_eq!(projection.return_lane(), APrimeI64LaneV1::ImmediateI64);
    assert_eq!(projection.function_effects(), EffectMask::READ);
    assert_eq!(
        projection.formal_parameters()[1].role(),
        DynamicV2AotFormalRoleV1::Pos
    );
    assert_eq!(
        projection.formal_parameters()[1].value_id(),
        ValueId::new(1)
    );
    assert_eq!(projection.calls()[0].normal_result_dst(), ValueId::new(20));
    assert_eq!(projection.calls()[1].normal_result_dst(), ValueId::new(21));
}

#[test]
fn projection_rejects_signature_and_formal_drift() {
    let admission = admission();
    let receipt = receipt();
    let site_plans = site_plans();
    let mut invalid_function = function();
    invalid_function.signature.return_type = MirType::Void;
    assert!(matches!(
        project(
            &admission,
            &receipt,
            &site_plans,
            &invalid_function,
            formal_parameters()
        ),
        Err(DynamicV2AotCallMetadataRejectV1::FunctionSignatureMismatch)
    ));

    let mut formals = formal_parameters();
    formals.swap(0, 1);
    assert!(matches!(
        project(&admission, &receipt, &site_plans, &function(), formals),
        Err(DynamicV2AotCallMetadataRejectV1::FormalLaneMismatch)
    ));
}

#[test]
fn projection_rejects_missing_or_duplicate_normal_result() {
    let admission = admission();
    let receipt = receipt();
    let site_plans = site_plans();
    let mut missing = function();
    missing
        .get_block_mut(BasicBlockId::new(1))
        .expect("normal block")
        .instructions
        .clear();
    assert!(matches!(
        site_plans.verify_function(&missing),
        Err(crate::mir::checked_callout::CheckedCallOutFunctionRejectV1::OrphanPlan(_))
            | Err(crate::mir::checked_callout::CheckedCallOutFunctionRejectV1::OrphanProjection(_))
    ));

    let mut duplicate = function();
    duplicate
        .get_block_mut(BasicBlockId::new(1))
        .expect("normal block")
        .add_instruction(MirInstruction::CheckedCallOutNormalResult {
            site_id: CheckedCallOutSiteIdV1::from_test(0),
            dst: ValueId::new(22),
        });
    assert!(matches!(
        site_plans.verify_function(&duplicate),
        Err(crate::mir::checked_callout::CheckedCallOutFunctionRejectV1::DuplicateProjection(_))
    ));
}

#[test]
fn malformed_receipt_is_rejected_before_projection() {
    let admission = admission();
    let error = APrimeI64PhysicalReceiptV1::seal_for_test(
        APrimeI64BackendFamilyV1::Llvm,
        A_PRIME_I64_FORMAL_PARAMETER_COUNT,
        vec![
            APrimeI64ParameterReceiptV1 {
                role: "pos".into(),
                formal_parameter_index: 0,
                value_id: ValueId::new(2),
                lane: APrimeI64LaneV1::ImmediateI64,
            },
            APrimeI64ParameterReceiptV1 {
                role: "end".into(),
                formal_parameter_index: 2,
                value_id: ValueId::new(3),
                lane: APrimeI64LaneV1::ImmediateI64,
            },
        ],
        vec![],
        vec![],
    )
    .expect_err("wrong formal lane must reject");
    assert!(matches!(
        error,
        APrimeI64PhysicalReceiptRejectV1::ParameterRoleIndexMismatch
    ));
    let _ = admission;
}

fn admission() -> PreparedAotExecutableAdmissionV1 {
    let substring = CORE_METHOD_CONTRACT_RESULT_ROWS_V1
        .iter()
        .find(|row| row.op == CoreMethodOp::StringSubstring)
        .expect("substring core row");
    let index_of = CORE_METHOD_CONTRACT_RESULT_ROWS_V1
        .iter()
        .find(|row| row.op == CoreMethodOp::StringIndexOf)
        .expect("indexOf core row");
    let aliases =
        super::super::seal::TextScanAliasProjectionV1::from_type_registry().expect("type aliases");
    super::super::seal::ProviderAdmissionSealV1::consume_text_scan(
        substring,
        index_of,
        aliases,
        ModuleInvocationBrandV1::test_with_ordinal(7),
    )
    .expect("TextScan admission")
}

fn site_plans() -> CheckedCallOutPlanTableV1 {
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
        ModuleInvocationBrandV1::test_with_ordinal(7),
    )
    .expect("valid CheckedCallOut site pair");
    pair.into_plan_table_for_test()
}

fn formal_parameters() -> [DynamicV2AotFormalProjectionV1; 4] {
    [
        DynamicV2AotFormalProjectionV1::new(
            DynamicV2AotFormalRoleV1::Src,
            ValueId::new(0),
            APrimeI64LaneV1::OpaqueHandle,
        ),
        DynamicV2AotFormalProjectionV1::new(
            DynamicV2AotFormalRoleV1::Pos,
            ValueId::new(1),
            APrimeI64LaneV1::ImmediateI64,
        ),
        DynamicV2AotFormalProjectionV1::new(
            DynamicV2AotFormalRoleV1::End,
            ValueId::new(2),
            APrimeI64LaneV1::ImmediateI64,
        ),
        DynamicV2AotFormalProjectionV1::new(
            DynamicV2AotFormalRoleV1::PredChars,
            ValueId::new(3),
            APrimeI64LaneV1::OpaqueHandle,
        ),
    ]
}

fn project(
    admission: &PreparedAotExecutableAdmissionV1,
    receipt: &APrimeI64PhysicalReceiptV1,
    site_plans: &CheckedCallOutPlanTableV1,
    function: &MirFunction,
    formal_parameters: [DynamicV2AotFormalProjectionV1; 4],
) -> Result<DynamicV2AotCallMetadataProjectionV1, DynamicV2AotCallMetadataRejectV1> {
    let census = site_plans
        .verify_function(function)
        .expect("test function has a canonical callout census");
    project_dynamic_v2_aot_call_metadata(
        admission,
        receipt,
        site_plans,
        function,
        formal_parameters,
        EffectMask::READ,
        &census,
    )
}

fn function() -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "ParserScanLoopBox.skip_while/4".to_owned(),
            params: vec![
                MirType::Unknown,
                MirType::Integer,
                MirType::Integer,
                MirType::Unknown,
            ],
            return_type: MirType::Integer,
            effects: EffectMask::READ,
        },
        BasicBlockId::new(0),
    );
    for id in 1..6 {
        function.add_block(BasicBlock::new(BasicBlockId::new(id)));
    }
    for (source, site_id, normal, fault, receiver, arguments, result) in [
        (
            0,
            CheckedCallOutSiteIdV1::from_test(0),
            1,
            2,
            ValueId::new(0),
            vec![ValueId::new(1), ValueId::new(2)],
            ValueId::new(20),
        ),
        (
            3,
            CheckedCallOutSiteIdV1::from_test(1),
            4,
            5,
            ValueId::new(3),
            vec![ValueId::new(0)],
            ValueId::new(21),
        ),
    ] {
        let source = BasicBlockId::new(source);
        let normal = BasicBlockId::new(normal);
        let fault = BasicBlockId::new(fault);
        function
            .get_block_mut(source)
            .expect("source block")
            .set_terminator(MirInstruction::CheckedCallOut {
                site_id,
                receiver,
                arguments,
                normal_landing: normal,
                fault_landing: fault,
                effects: EffectMask::READ,
            });
        for landing in [normal, fault] {
            function
                .get_block_mut(landing)
                .expect("landing block")
                .add_predecessor(source);
        }
        function
            .get_block_mut(normal)
            .expect("normal block")
            .add_instruction(MirInstruction::CheckedCallOutNormalResult {
                site_id,
                dst: result,
            });
    }
    for block in [2, 4, 5] {
        function
            .get_block_mut(BasicBlockId::new(block))
            .expect("end block")
            .add_instruction(MirInstruction::CheckedCallOutEnd {
                site_id: CheckedCallOutSiteIdV1::from_test(0),
                lease_slot: crate::mir::checked_callout::CheckedCallOutLeaseSlotIdV1::from_test(0),
            });
    }
    function
}

fn receipt() -> APrimeI64PhysicalReceiptV1 {
    APrimeI64PhysicalReceiptV1::seal_for_test(
        APrimeI64BackendFamilyV1::Llvm,
        A_PRIME_I64_FORMAL_PARAMETER_COUNT,
        vec![
            APrimeI64ParameterReceiptV1 {
                role: "pos".into(),
                formal_parameter_index: 1,
                value_id: ValueId::new(2),
                lane: APrimeI64LaneV1::ImmediateI64,
            },
            APrimeI64ParameterReceiptV1 {
                role: "end".into(),
                formal_parameter_index: 2,
                value_id: ValueId::new(3),
                lane: APrimeI64LaneV1::ImmediateI64,
            },
        ],
        vec![call("substring", 0), call("index_of", 1)],
        vec![
            APrimeI64ReturnReceiptV1 {
                site: "inner".into(),
                block: BasicBlockId::new(8),
                value_id: ValueId::new(30),
                lane: APrimeI64LaneV1::ImmediateI64,
            },
            APrimeI64ReturnReceiptV1 {
                site: "outer".into(),
                block: BasicBlockId::new(9),
                value_id: ValueId::new(31),
                lane: APrimeI64LaneV1::ImmediateI64,
            },
        ],
    )
    .expect("valid receipt")
}

fn call(role: &str, site_id: u32) -> APrimeI64CallEdgeReceiptV1 {
    APrimeI64CallEdgeReceiptV1 {
        site_id: CheckedCallOutSiteIdV1::from_test(site_id),
        role: role.into(),
        target_fingerprint: if role == "substring" {
            "substring/2".into()
        } else {
            "indexOf/1".into()
        },
        receiver_role: if role == "substring" {
            "src"
        } else {
            "pred_chars"
        }
        .into(),
        receiver_value_id: ValueId::new(if role == "substring" { 0 } else { 3 }),
        receiver_lane: APrimeI64LaneV1::OpaqueHandle,
        arguments: if role == "substring" {
            vec![
                APrimeI64CallArgumentReceiptV1 {
                    ordinal: 0,
                    role: "start".into(),
                    value_id: ValueId::new(1),
                    lane: APrimeI64LaneV1::ImmediateI64,
                },
                APrimeI64CallArgumentReceiptV1 {
                    ordinal: 1,
                    role: "end".into(),
                    value_id: ValueId::new(2),
                    lane: APrimeI64LaneV1::ImmediateI64,
                },
            ]
        } else {
            vec![APrimeI64CallArgumentReceiptV1 {
                ordinal: 0,
                role: "ch".into(),
                value_id: ValueId::new(0),
                lane: APrimeI64LaneV1::OpaqueHandle,
            }]
        },
        result_value_id: ValueId::new(if role == "substring" { 20 } else { 21 }),
        result_lane: if role == "substring" {
            APrimeI64LaneV1::OpaqueHandle
        } else {
            APrimeI64LaneV1::ImmediateI64
        },
    }
}
