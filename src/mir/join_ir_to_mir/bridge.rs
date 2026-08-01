use super::{join_func_name, module_converter::convert_join_module_to_mir, JoinIrToMirError};
use crate::mir::join_ir::JoinModule;
use crate::mir::MirModule;

fn ensure_joinir_function_aliases(mir_module: &mut MirModule, join_module: &JoinModule) {
    for (func_id, join_func) in &join_module.functions {
        let generated_name = join_func_name(*func_id);
        let function = mir_module
            .functions
            .get(&join_func.name)
            .or_else(|| mir_module.functions.get(&generated_name))
            .cloned();

        if let Some(function) = function {
            if !mir_module.functions.contains_key(&join_func.name) {
                mir_module
                    .functions
                    .insert(join_func.name.clone(), function.clone());
            }

            if !mir_module.functions.contains_key(&generated_name) {
                mir_module
                    .functions
                    .insert(generated_name.clone(), function.clone());
            }

            let actual_arity = format!("{}/{}", join_func.name, join_func.params.len());
            if !mir_module.functions.contains_key(&actual_arity) {
                mir_module.functions.insert(actual_arity, function.clone());
            }

            let generated_arity = format!("{}/{}", generated_name, join_func.params.len());
            if !mir_module.functions.contains_key(&generated_arity) {
                mir_module.functions.insert(generated_arity, function);
            }
        }
    }
}

/// Structured JoinIR → MIR（既存経路）の明示エントリ。
pub(crate) fn lower_structured_joinir_to_mir(
    module: &JoinModule,
) -> Result<MirModule, JoinIrToMirError> {
    if !module.is_structured() {
        return Err(JoinIrToMirError::new(
            "[joinir/to-mir] expected Structured JoinIR module",
        ));
    }

    let mut mir = convert_join_module_to_mir(module)?;
    ensure_joinir_function_aliases(&mut mir, module);
    Ok(mir)
}
