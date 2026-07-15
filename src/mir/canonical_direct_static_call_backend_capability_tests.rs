use crate::mir::canonical_direct_static_call_capability::CanonicalDirectStaticCallCapabilityV1;
use crate::mir::{
    BasicBlockId, Callee, EffectMask, FunctionSignature, MirFunction, MirInstruction, MirType,
    ValueId,
};

use super::*;

fn function() -> MirFunction {
    MirFunction::new(
        FunctionSignature {
            name: "countdown/1".to_string(),
            params: vec![MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    )
}

fn module_with_function(function: MirFunction) -> MirModule {
    let mut module = MirModule::new("canonical-direct-call-capability".to_string());
    module.add_function(function);
    module
}

#[test]
fn empty_metadata_keeps_existing_backend_behavior() {
    let module = MirModule::new("plain".to_string());
    for backend in ["mir-interpreter", "wasm", "ny-llvmc-exe"] {
        assert!(enforce(&module, backend).is_ok());
    }
}

#[test]
fn explicit_witness_is_vm_only_in_the_first_slice() {
    let mut function = function();
    CanonicalDirectStaticCallCapabilityV1::install_for_function(
        &mut function.metadata.canonical_direct_static_call_capabilities,
        true,
    )
    .unwrap();
    let module = module_with_function(function);
    assert_eq!(inspect(&module).capability_rows, 1);
    assert!(enforce(&module, "mir-interpreter").is_ok());
    for backend in [
        "pyvm-harness",
        "ny-llvmc-exe",
        "llvmlite-obj",
        "wasm",
        "wasm-v2",
    ] {
        let error = enforce(&module, backend).unwrap_err();
        assert!(error.contains(CANONICAL_DIRECT_STATIC_CALL_BACKEND_UNSUPPORTED_TAG));
        assert!(error.contains("silent_fallback_allowed=false"));
    }
}

#[test]
fn legacy_generic_call_does_not_create_the_capability() {
    let mut function = function();
    function
        .get_block_mut(function.entry_block)
        .unwrap()
        .add_instruction(MirInstruction::Call {
            dst: Some(ValueId::new(1)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("legacy/0".to_string())),
            args: Vec::new(),
            effects: EffectMask::IO,
        });
    let module = module_with_function(function);
    assert_eq!(inspect(&module).capability_rows, 0);
    assert!(enforce(&module, "wasm").is_ok());
}
