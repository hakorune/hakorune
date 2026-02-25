use super::{bridge_joinir_to_mir, join_func_name, JoinIrVmBridgeError};
use crate::backend::{MirInterpreter, VMValue};
use crate::mir::join_ir::JoinFuncId;
use crate::mir::join_ir::JoinModule;
use crate::mir::join_ir_ops::JoinValue;

/// Phase 27-shortterm S-4.3: JoinIR → VM 実行のエントリーポイント
///
/// ## Arguments
/// - `join_module`: JoinIR モジュール（正規化済み）
/// - `entry_func`: エントリーポイント関数ID
/// - `args`: 初期引数（JoinValue 形式）
///
/// ## Returns
/// - `Ok(JoinValue)`: 実行結果
/// - `Err(JoinIrVmBridgeError)`: 変換エラーまたは実行エラー
///
/// ## Example
/// ```ignore
/// let join_module = lower_skip_ws_to_joinir(&mir_module)?;
/// let result = run_joinir_via_vm(
///     &join_module,
///     JoinFuncId::new(0),
///     &[JoinValue::Str("  hello".to_string()), JoinValue::Int(7)]
/// )?;
/// assert_eq!(result, JoinValue::Int(2));
/// ```
pub fn run_joinir_via_vm(
    join_module: &JoinModule,
    entry_func: JoinFuncId,
    args: &[JoinValue],
) -> Result<JoinValue, JoinIrVmBridgeError> {
    debug_log!("[joinir_vm_bridge] Phase 27-shortterm S-4.3");
    debug_log!("[joinir_vm_bridge] Converting JoinIR to MIR for VM execution");

    // Step 1: JoinIR → MIR 変換
    let mir_module = bridge_joinir_to_mir(join_module)?;

    debug_log!(
        "[joinir_vm_bridge] Converted {} JoinIR functions to MIR",
        join_module.functions.len()
    );

    // Step 2: VM 実行
    let mut vm = MirInterpreter::new();

    debug_log!(
        "[joinir_vm_bridge] Executing via VM with {} arguments",
        args.len()
    );

    // Convert JoinValue → VMValue (BoxRef 含む)
    let vm_args: Vec<VMValue> = args.iter().cloned().map(|v| v.into_vm_value()).collect();

    // Phase 256 P1.7+: Prefer the actual JoinFunction name as the MIR function key.
    // Some bridge paths use `join_func_name()` ("join_func_N"), others use JoinFunction.name.
    let entry_name_actual = join_module
        .functions
        .get(&entry_func)
        .map(|f| f.name.clone());
    let entry_name_fallback = join_func_name(entry_func);
    let entry_name = if let Some(name) = entry_name_actual {
        if mir_module.functions.contains_key(&name) {
            name
        } else {
            entry_name_fallback
        }
    } else {
        entry_name_fallback
    };
    let result = vm.execute_function_with_args(&mir_module, &entry_name, &vm_args)?;

    // Step 3: VMValue → JoinValue 変換
    let join_result = JoinValue::from_vm_value(&result)
        .map_err(|e| JoinIrVmBridgeError::new(format!("Result conversion error: {}", e.message)))?;

    debug_log!("[joinir_vm_bridge] Execution succeeded: {:?}", join_result);

    Ok(join_result)
}
