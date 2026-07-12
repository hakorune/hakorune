//! Backend preflight for the internal generic strict-JSON tree accessors.

use crate::mir::{extern_call_route_plan::ExternCallRouteKind, MirModule};

pub(crate) const STRICT_JSON_TREE_BACKEND_UNSUPPORTED_TAG: &str =
    "[analysis/strict_json_tree_v0_backend_unsupported]";

fn is_strict_json_tree_route(kind: ExternCallRouteKind) -> bool {
    matches!(
        kind,
        ExternCallRouteKind::HakoAnalysisStrictJsonTreeKindV0
            | ExternCallRouteKind::HakoAnalysisStrictJsonTreeObjectLenV0
            | ExternCallRouteKind::HakoAnalysisStrictJsonTreeObjectKeyAtV0
            | ExternCallRouteKind::HakoAnalysisStrictJsonTreeObjectValueAtV0
            | ExternCallRouteKind::HakoAnalysisStrictJsonTreeArrayLenV0
            | ExternCallRouteKind::HakoAnalysisStrictJsonTreeArrayAtV0
            | ExternCallRouteKind::HakoAnalysisStrictJsonTreeStringValueV0
            | ExternCallRouteKind::HakoAnalysisStrictJsonTreeBoolValueV0
            | ExternCallRouteKind::HakoAnalysisStrictJsonTreeI64ValueV0
            | ExternCallRouteKind::HakoAnalysisStrictJsonTreeU64FitsI64V0
            | ExternCallRouteKind::HakoAnalysisStrictJsonTreeU64AsI64V0
    )
}

pub(crate) fn enforce_strict_json_tree_backend_supported(
    module: &MirModule,
    backend: &str,
) -> Result<(), String> {
    let rows = module
        .functions
        .values()
        .flat_map(|function| &function.metadata.extern_call_routes)
        .filter(|route| is_strict_json_tree_route(route.kind()))
        .count();
    if rows == 0 || backend == "mir-interpreter" {
        return Ok(());
    }
    Err(format!(
        "{} backend={} contract_rows={} require=strict_json_tree_v0",
        STRICT_JSON_TREE_BACKEND_UNSUPPORTED_TAG, backend, rows
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{
        BasicBlockId, Callee, ConstValue, EffectMask, FunctionSignature, MirFunction,
        MirInstruction, MirType, ValueId,
    };

    fn module_with_capability() -> MirModule {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "Main.kind/0".to_string(),
                params: vec![],
                return_type: MirType::Box("StringBox".to_string()),
                effects: EffectMask::IO,
            },
            BasicBlockId::new(0),
        );
        let block = function.entry_block;
        function.get_block_mut(block).unwrap().instructions.extend([
            MirInstruction::Const {
                dst: ValueId::new(1),
                value: ConstValue::Integer(1),
            },
            MirInstruction::Const {
                dst: ValueId::new(2),
                value: ConstValue::Integer(0),
            },
            MirInstruction::Call {
                dst: Some(ValueId::new(3)),
                func: ValueId::INVALID,
                callee: Some(Callee::Extern(
                    "hako.analysis.strict_json_tree_v0.kind".to_string(),
                )),
                args: vec![ValueId::new(1), ValueId::new(2)],
                effects: EffectMask::IO,
            },
        ]);
        let mut module = MirModule::new("strict-json-tree-capability".to_string());
        module.add_function(function);
        crate::mir::extern_call_route_plan::refresh_module_extern_call_routes(&mut module);
        module
    }

    #[test]
    fn strict_json_tree_is_reference_interpreter_only() {
        let module = module_with_capability();
        assert!(enforce_strict_json_tree_backend_supported(&module, "mir-interpreter").is_ok());
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
            let error = enforce_strict_json_tree_backend_supported(&module, backend)
                .expect_err("unsupported backend");
            assert!(error.contains(STRICT_JSON_TREE_BACKEND_UNSUPPORTED_TAG));
            assert!(error.contains(&format!("backend={backend}")));
        }
    }
}
