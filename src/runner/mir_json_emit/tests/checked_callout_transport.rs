use super::super::emit_mir_json_string_for_harness_bin;
use crate::mir::{
    BasicBlock, BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirInstruction,
    MirModule, MirType, ValueId,
};

#[test]
fn checked_callout_transport_round_trips_without_backend_activation() {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "checked_callout_transport".to_owned(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::READ,
        },
        BasicBlockId::new(0),
    );
    let mut normal = BasicBlock::new(BasicBlockId::new(1));
    normal.add_instruction(MirInstruction::CheckedCallOutNormalResult {
        site_id: crate::mir::checked_callout::CheckedCallOutSiteIdV1::from_test(0),
        dst: ValueId::new(3),
    });
    normal.add_instruction(MirInstruction::CheckedCallOutEnd {
        site_id: crate::mir::checked_callout::CheckedCallOutSiteIdV1::from_test(0),
        lease_slot: crate::mir::checked_callout::CheckedCallOutLeaseSlotIdV1::from_test(0),
    });
    normal.set_terminator(MirInstruction::Return { value: None });
    let mut fault = BasicBlock::new(BasicBlockId::new(2));
    fault.set_terminator(MirInstruction::CheckedCallOutFault {
        site_id: crate::mir::checked_callout::CheckedCallOutSiteIdV1::from_test(0),
    });
    function.blocks.insert(BasicBlockId::new(1), normal);
    function.blocks.insert(BasicBlockId::new(2), fault);
    function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .set_terminator(MirInstruction::CheckedCallOut {
            site_id: crate::mir::checked_callout::CheckedCallOutSiteIdV1::from_test(0),
            receiver: ValueId::new(1),
            arguments: vec![ValueId::new(2)],
            normal_landing: BasicBlockId::new(1),
            fault_landing: BasicBlockId::new(2),
            effects: EffectMask::READ,
        });

    let mut module = MirModule::new("checked_callout_transport".to_owned());
    module.add_function(function);
    let json = emit_mir_json_string_for_harness_bin(&module).expect("emit transport JSON");
    assert!(json.contains("checked_callout"));
    assert!(json.contains("checked_callout_normal_result"));
    assert!(json.contains("checked_callout_end"));
    assert!(json.contains("checked_callout_fault"));

    let reparsed = crate::runner::mir_json_v0::parse_mir_v0_to_module(&json)
        .expect("reparse checked callout transport");
    let reparsed_fn = reparsed
        .get_function("checked_callout_transport")
        .expect("reparsed function");
    assert!(matches!(
        reparsed_fn
            .get_block(BasicBlockId::new(0))
            .and_then(|block| block.terminator.as_ref()),
        Some(MirInstruction::CheckedCallOut {
            site_id,
            receiver,
            arguments,
            normal_landing,
            fault_landing,
            effects,
        }) if site_id.as_u32() == 0
            && *receiver == ValueId::new(1)
            && arguments == &vec![ValueId::new(2)]
            && *normal_landing == BasicBlockId::new(1)
            && *fault_landing == BasicBlockId::new(2)
            && *effects == EffectMask::READ
    ));
    let normal = reparsed_fn.get_block(BasicBlockId::new(1)).unwrap();
    assert!(matches!(
        normal.instructions.as_slice(),
        [
            MirInstruction::CheckedCallOutNormalResult { dst, .. },
            MirInstruction::CheckedCallOutEnd { lease_slot, .. },
        ] if *dst == ValueId::new(3) && lease_slot.as_u32() == 0
    ));
    assert!(matches!(
        reparsed_fn
            .get_block(BasicBlockId::new(2))
            .and_then(|block| block.terminator.as_ref()),
        Some(MirInstruction::CheckedCallOutFault { site_id }) if site_id.as_u32() == 0
    ));
}
