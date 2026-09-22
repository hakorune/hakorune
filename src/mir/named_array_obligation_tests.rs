//! Physical mutation tests are rejection evidence, never source admission.
use super::*;
use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirType};

fn fixture() -> (MirFunction, NamedArrayWriteMarkerV1) {
    let marker = NamedArrayWriteMarkerV1 {
        allocation: ValueId::new(1),
        receiver: ValueId::new(1),
        argument: ValueId::new(2),
        write: ArrayWriteSiteId(0),
    };
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "probe".into(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let block = function.blocks.get_mut(&function.entry_block).unwrap();
    block.instructions.push(MirInstruction::NewBox {
        dst: marker.allocation,
        target: ConstructionTarget::Named("ArrayBox".into()),
        args: vec![],
    });
    block.instructions.push(MirInstruction::ArrayElementWrite {
        site_id: marker.write,
        dst: None,
        kind: ArrayElementWriteKind::Push,
        producer: ArrayWriteProducerKind::MethodCall,
        receiver: marker.receiver,
        index: None,
        value: marker.argument,
    });
    (function, marker)
}

#[test]
fn allocation_and_write_drift_cannot_match_retained_observation() {
    let (function, marker) = fixture();
    validate_physical_marker(&function, &marker).unwrap();
    for mutation in 0..7 {
        let mut changed = function.clone();
        let instructions = &mut changed
            .blocks
            .get_mut(&changed.entry_block)
            .unwrap()
            .instructions;
        match mutation {
            0 => {
                instructions.remove(0);
            }
            1 => {
                instructions.push(instructions[0].clone());
            }
            2 => {
                if let MirInstruction::NewBox { target, .. } = &mut instructions[0] {
                    *target = ConstructionTarget::Named("OtherBox".into());
                }
            }
            3 => {
                instructions.pop();
            }
            4 => {
                instructions.push(instructions[1].clone());
            }
            5 => {
                if let MirInstruction::ArrayElementWrite { value, .. } = &mut instructions[1] {
                    *value = ValueId::new(9);
                }
            }
            6 => {
                if let MirInstruction::ArrayElementWrite { dst, .. } = &mut instructions[1] {
                    *dst = Some(ValueId::new(9));
                }
            }
            _ => unreachable!(),
        }
        assert!(
            validate_physical_marker(&changed, &marker).is_err(),
            "mutation {mutation}"
        );
    }
}

#[test]
fn cloned_marker_cannot_replace_retained_source_authority() {
    let (mut function, marker) = fixture();
    function.metadata.named_array_write_obligations.push(marker);
    let mut module = MirModule::new("probe".into());
    module.functions.insert("probe".into(), function);
    for module in [&module, &module.clone()] {
        assert_eq!(
            reject_unretained_module(module).unwrap_err(),
            fault("retained-source-required")
        );
        assert!(
            crate::mir::normal_callable_semantic_package::validate_named_array_coverage(
                module,
                &[]
            )
            .is_err()
        );
        assert_eq!(
            crate::mir::backend_capability::enforce_mir_backend_supported(module, "ny-llvmc-obj")
                .unwrap_err(),
            fault("retained-source-required")
        );
    }
}
