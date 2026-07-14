use crate::mir::{MirInstruction, MirModule};

pub(crate) const OWNERSHIP_BACKEND_MISSING_TAG: &str =
    "[backend-missing-capability:owned-value-lifecycle-v1]";

pub(crate) fn enforce(module: &MirModule, backend: &str) -> Result<(), String> {
    for function in module.functions.values() {
        let has_ownership_ops = function.blocks.values().any(|block| {
            block.instructions.iter().any(|instruction| {
                matches!(
                    instruction,
                    MirInstruction::CopyOwned { .. } | MirInstruction::DestroyOwned { .. }
                )
            })
        });
        if !has_ownership_ops {
            continue;
        }
        if backend != "llvmlite-obj" {
            return Err(format!(
                "{OWNERSHIP_BACKEND_MISSING_TAG} backend={backend} function={}",
                function.signature.name
            ));
        }
        let Some(witness) = function.metadata.ownership_ssa_v1.as_ref() else {
            return Err(format!(
                "[freeze:contract][ownership-backend:missing-witness] backend={backend} function={}",
                function.signature.name
            ));
        };
        if !witness.matches_function(function) {
            return Err(format!(
                "[freeze:contract][ownership-backend:stale-witness] backend={backend} function={}",
                function.signature.name
            ));
        }
        if witness.owner().as_u64() == u64::MAX {
            return Err(format!(
                "[freeze:contract][ownership-backend:invalid-owner] function={}",
                function.signature.name
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::ownership_ssa::{
        verify_ownership_ssa_v1, FunctionResultOwnershipV1, MirOwnershipKindV1,
        OwnershipFunctionAbiV1, OwnershipFunctionOwnerV1,
    };
    use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirType, ValueId};

    fn module_with_owned_copy(install_witness: bool) -> MirModule {
        let owner = OwnershipFunctionOwnerV1::new(7);
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "ownership/1".into(),
                params: vec![MirType::Box("OwnedTestBox".into())],
                return_type: MirType::Void,
                effects: EffectMask::WRITE,
            },
            BasicBlockId::new(0),
        );
        function
            .get_block_mut(BasicBlockId::new(0))
            .unwrap()
            .add_instruction(MirInstruction::CopyOwned {
                dst: ValueId::new(1),
                src: ValueId::new(0),
            });
        function
            .get_block_mut(BasicBlockId::new(0))
            .unwrap()
            .add_instruction(MirInstruction::DestroyOwned {
                value: ValueId::new(1),
            });
        function
            .get_block_mut(BasicBlockId::new(0))
            .unwrap()
            .set_terminator(MirInstruction::Return { value: None });
        if install_witness {
            let abi = OwnershipFunctionAbiV1::new(
                owner,
                vec![MirOwnershipKindV1::Borrowed],
                FunctionResultOwnershipV1::None,
            );
            function.metadata.ownership_ssa_v1 =
                Some(verify_ownership_ssa_v1(&function, &abi).unwrap());
        }
        let mut module = MirModule::new("ownership".into());
        module.add_function(function);
        module
    }

    #[test]
    fn only_witnessed_llvmlite_object_lane_is_accepted() {
        let module = module_with_owned_copy(true);
        assert!(enforce(&module, "llvmlite-obj").is_ok());
        assert!(
            crate::mir::backend_capability::enforce_mir_backend_supported(&module, "llvmlite-obj")
                .is_ok()
        );
        for backend in ["wasm", "wasm-v2", "pyvm-harness", "ny-llvmc-obj"] {
            assert!(enforce(&module, backend)
                .unwrap_err()
                .contains(OWNERSHIP_BACKEND_MISSING_TAG));
        }
        assert!(
            crate::mir::backend_capability::enforce_mir_backend_supported(&module, "wasm")
                .unwrap_err()
                .contains(OWNERSHIP_BACKEND_MISSING_TAG)
        );
    }

    #[test]
    fn missing_sealed_witness_is_rejected() {
        assert!(enforce(&module_with_owned_copy(false), "llvmlite-obj")
            .unwrap_err()
            .contains("ownership-backend:missing-witness"));
    }

    #[test]
    fn stale_witness_is_rejected_after_control_lifetime_changes() {
        let mut module = module_with_owned_copy(true);
        module
            .get_function_mut("ownership/1")
            .unwrap()
            .get_block_mut(BasicBlockId::new(0))
            .unwrap()
            .set_terminator(MirInstruction::Return {
                value: Some(ValueId::new(0)),
            });
        assert!(enforce(&module, "llvmlite-obj")
            .unwrap_err()
            .contains("ownership-backend:stale-witness"));
    }
}
