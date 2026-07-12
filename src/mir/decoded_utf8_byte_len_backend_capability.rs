//! Backend preflight for the SnapshotV0 internal UTF-8 byte-length capability.

use crate::mir::{extern_call_route_plan::ExternCallRouteKind, MirModule};

pub(crate) const DECODED_UTF8_BYTE_LEN_BACKEND_UNSUPPORTED_TAG: &str =
    "[analysis/decoded_utf8_byte_len_v0_backend_unsupported]";

pub(crate) fn enforce_decoded_utf8_byte_len_backend_supported(
    module: &MirModule,
    backend: &str,
) -> Result<(), String> {
    let rows = module
        .functions
        .values()
        .flat_map(|function| &function.metadata.extern_call_routes)
        .filter(|route| route.kind() == ExternCallRouteKind::HakoAnalysisDecodedUtf8ByteLenV0)
        .count();
    if rows == 0 || backend == "mir-interpreter" {
        return Ok(());
    }
    Err(format!(
        "{} backend={} contract_rows={} require=decoded_utf8_byte_len_v0",
        DECODED_UTF8_BYTE_LEN_BACKEND_UNSUPPORTED_TAG, backend, rows
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{
        BasicBlockId, Callee, ConstValue, EffectMask, FunctionSignature, MirFunction,
        MirInstruction, MirModule, MirType, ValueId,
    };

    fn module_with_capability() -> MirModule {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "Main.bytes/0".to_string(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::IO,
            },
            BasicBlockId::new(0),
        );
        let block = function.entry_block;
        function.get_block_mut(block).unwrap().instructions.extend([
            MirInstruction::Const {
                dst: ValueId::new(1),
                value: ConstValue::String("猫".to_string()),
            },
            MirInstruction::Call {
                dst: Some(ValueId::new(2)),
                func: ValueId::INVALID,
                callee: Some(Callee::Extern(
                    "hako.analysis.decoded_utf8_byte_len_v0".to_string(),
                )),
                args: vec![ValueId::new(1)],
                effects: EffectMask::IO,
            },
        ]);
        let mut module = MirModule::new("decoded-utf8-byte-len".to_string());
        module.add_function(function);
        module
    }

    #[test]
    fn only_reference_interpreter_supports_the_internal_byte_capability() {
        let module = module_with_capability();
        assert!(
            crate::mir::backend_capability::enforce_mir_backend_supported(
                &module,
                "mir-interpreter"
            )
            .is_ok()
        );
        for backend in [
            "ny-llvmc-exe",
            "ny-llvmc-obj",
            "llvmlite-obj",
            "llvm-legacy-obj",
            "llvm-mock-fallback",
            "pyvm-harness",
            "wasm",
            "wasm-v2",
        ] {
            let error =
                crate::mir::backend_capability::enforce_mir_backend_supported(&module, backend)
                    .unwrap_err();
            assert!(error.contains(DECODED_UTF8_BYTE_LEN_BACKEND_UNSUPPORTED_TAG));
            assert!(error.contains(&format!("backend={backend}")));
        }
    }

    #[test]
    fn plain_modules_remain_outside_the_internal_byte_capability_gate() {
        let module = MirModule::new("plain".to_string());
        assert!(enforce_decoded_utf8_byte_len_backend_supported(&module, "wasm").is_ok());
    }
}
