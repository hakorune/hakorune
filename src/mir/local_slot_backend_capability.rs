use crate::mir::type_contracts::local_slot::validate_local_slot_contracts;
use crate::mir::MirModule;

pub(crate) const LOCAL_SLOT_BACKEND_CAPABILITY_MISSING_TAG: &str =
    "[type/backend_local_contract_capability_missing]";

pub(crate) fn enforce_local_slot_backend_supported(
    module: &MirModule,
    backend: &str,
) -> Result<(), String> {
    let mut rows = 0usize;
    for function in module.functions.values() {
        validate_local_slot_contracts(function)?;
        rows += function.metadata.local_slot_contracts.len();
    }
    if rows == 0 || backend == "mir-interpreter" {
        return Ok(());
    }
    Err(format!(
        "{} backend={} contract_rows={} require=local_slot_exact_numeric",
        LOCAL_SLOT_BACKEND_CAPABILITY_MISSING_TAG, backend, rows
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::function::{LocalContractWriteKind, LocalSlotContract};
    use crate::mir::{
        BasicBlockId, EffectMask, FunctionSignature, LocalSlotId, MirFunction, MirInstruction,
        MirType, ValueId,
    };

    fn module_with_contract() -> MirModule {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "Main.local/0".to_string(),
                params: Vec::new(),
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let slot = LocalSlotId::from(crate::mir::BindingId::new(0));
        function
            .metadata
            .local_slot_contracts
            .push(LocalSlotContract {
                contract_id: "local-slot:0".to_string(),
                local_slot_id: slot,
                diagnostic_source_name: "x".to_string(),
                declared_type_name: "u8".to_string(),
                runtime_check_required: true,
                proof_elision_allowed: false,
                backend_capability_required: "local_slot_exact_numeric".to_string(),
            });
        function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .unwrap()
            .instructions
            .push(MirInstruction::LocalContractWrite {
                dst: ValueId::new(1),
                src: ValueId::new(0),
                local_slot_id: slot,
                write_kind: LocalContractWriteKind::Init,
            });
        let mut module = MirModule::new("local-contract".to_string());
        module.add_function(function);
        module
    }

    #[test]
    fn only_reference_interpreter_supports_first_slice() {
        let module = module_with_contract();
        assert!(enforce_local_slot_backend_supported(&module, "mir-interpreter").is_ok());
        for backend in ["pyvm-harness", "ny-llvmc-exe", "llvmlite-obj", "wasm"] {
            let error = enforce_local_slot_backend_supported(&module, backend).unwrap_err();
            assert!(error.contains(LOCAL_SLOT_BACKEND_CAPABILITY_MISSING_TAG));
        }
    }
}
