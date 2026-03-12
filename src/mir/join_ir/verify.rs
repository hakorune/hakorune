//! JoinIR Progress Carrier Verification (Phase 29 L-5.2)
//!
//! # Purpose
//!
//! ループの「進捗キャリア」が backedge で必ず更新されていることを検証する。
//! ゼロ進捗 backedge（progress carrier が変化しない再帰呼び出し）を検出し、
//! 無限ループの可能性を早期に警告する。
//!
//! # Phase 29 Strategy
//!
//! - まずは minimal skip_ws JoinIR のみを対象に、dev モードで警告を出す。
//! - 将来（Phase 30）では Verifier エラーに格上げする予定。
//!
//! # Usage
//!
//! ```ignore
//! // NYASH_JOINIR_EXPERIMENT=1 のときのみ有効
//! if let Err(e) = verify_progress_for_skip_ws(&join_module) {
//!     eprintln!("[joinir/progress] warning: {:?}", e);
//! }
//! ```

use crate::mir::join_ir::{BinOpKind, JoinFuncId, JoinInst, JoinModule, MirLikeInst};
use crate::mir::ValueId;
use crate::runtime::get_global_ring0;

/// Progress verification error
#[derive(Debug, Clone)]
pub enum ProgressError {
    /// Progress carrier is not updated before backedge (recursive call)
    ZeroProgressBackedge {
        /// Progress variable name (for diagnostics)
        progress_var: String,
        /// Loop function ID
        loop_func_id: JoinFuncId,
        /// The recursive call that doesn't update progress
        call_index: usize,
    },
    /// No recursive call found in loop function
    NoRecursiveCall { loop_func_id: JoinFuncId },
    /// Progress carrier not found
    ProgressCarrierNotFound {
        expected_param_index: usize,
        loop_func_id: JoinFuncId,
    },
}

/// Verify progress carrier for skip_ws JoinIR (Phase 29 minimal version)
///
/// This function checks that:
/// 1. The loop_step function has a recursive call
/// 2. The progress carrier (i, typically the 3rd parameter) is updated via BinOp::Add
///    before the recursive call
///
/// # Arguments
///
/// * `join_module` - The JoinModule containing skip_ws functions
///
/// # Returns
///
/// Ok(()) if progress carrier is properly updated, Err(ProgressError) otherwise.
///
/// # Skip_ws Structure Expected
///
/// ```text
/// fn loop_step(s: Str, n: Int, i: Int) -> Int {
///     if i >= n { return i }
///     let ch = s.substring(i, i+1)
///     if ch != " " { return i }
///     let next_i = i + 1     // ← progress update
///     loop_step(s, n, next_i) // ← recursive call with updated progress
/// }
/// ```
pub fn verify_progress_for_skip_ws(join_module: &JoinModule) -> Result<(), ProgressError> {
    // skip_ws では loop_step が JoinFuncId(1)
    let loop_func_id = JoinFuncId::new(1);

    let loop_func = match join_module.functions.get(&loop_func_id) {
        Some(f) => f,
        None => {
            // loop_step 関数が無ければ検証スキップ（エラーではない）
            return Ok(());
        }
    };

    // skip_ws の progress carrier は 3番目のパラメータ (i)
    // params = [s, n, i] → index 2
    let progress_param_index = 2;
    let progress_param = match loop_func.params.get(progress_param_index) {
        Some(&p) => p,
        None => {
            return Err(ProgressError::ProgressCarrierNotFound {
                expected_param_index: progress_param_index,
                loop_func_id,
            });
        }
    };

    // 1. 再帰呼び出し（loop_step への Call）を探す
    let mut recursive_call_indices: Vec<usize> = Vec::new();
    for (idx, inst) in loop_func.body.iter().enumerate() {
        if let JoinInst::Call { func, .. } = inst {
            if *func == loop_func_id {
                recursive_call_indices.push(idx);
            }
        }
    }

    if recursive_call_indices.is_empty() {
        return Err(ProgressError::NoRecursiveCall { loop_func_id });
    }

    // 2. 再帰呼び出しの前に progress carrier が更新されているか確認
    //    「BinOp::Add で progress_param を使った演算がある」ことを保守的にチェック
    for &call_idx in &recursive_call_indices {
        let has_progress_update =
            check_progress_update_before(&loop_func.body[..call_idx], progress_param);

        if !has_progress_update {
            return Err(ProgressError::ZeroProgressBackedge {
                progress_var: format!("param[{}]", progress_param_index),
                loop_func_id,
                call_index: call_idx,
            });
        }
    }

    Ok(())
}

/// Check if there's a BinOp::Add that uses the progress parameter before the call
///
/// This is a conservative check: we look for any BinOp::Add where progress_param
/// is the LHS. The result should be used as an argument to the recursive call.
fn check_progress_update_before(instructions: &[JoinInst], progress_param: ValueId) -> bool {
    for inst in instructions {
        if let JoinInst::Compute(MirLikeInst::BinOp {
            op: BinOpKind::Add,
            lhs,
            ..
        }) = inst
        {
            if *lhs == progress_param {
                return true;
            }
        }
    }
    false
}

/// Verify progress carrier for a generic loop (future extension)
///
/// This is a placeholder for Phase 30 where we'll support arbitrary loops.
#[allow(dead_code)]
pub fn verify_progress_generic(
    _join_module: &JoinModule,
    _loop_func_id: JoinFuncId,
    _progress_param_index: usize,
) -> Result<(), ProgressError> {
    // Phase 30: 汎用ループの progress チェック
    // 現在は未実装（skip_ws 専用版のみ）
    Ok(())
}

// ============================================================================
// Phase 33-3.2: Select Minimal Invariant Verification
// ============================================================================

/// JoinIR Verify Error for Select verification
#[derive(Debug)]
pub struct JoinIrVerifyError {
    message: String,
}

impl JoinIrVerifyError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl std::fmt::Display for JoinIrVerifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "JoinIR Verify Error: {}", self.message)
    }
}

impl std::error::Error for JoinIrVerifyError {}

/// Phase 33-3.2: Select 命令の最小 invariant チェック
/// IfSelectTest.* 専用の軽い検証
///
/// # 検証項目
///
/// 1. **型一貫性**: then_val と else_val の型が一致（将来拡張）
/// 2. **単一 PHI**: 1 つの Select のみが存在
/// 3. **完全性**: then_val と else_val が両方存在
///
/// # phi_invariants.rs からの移譲
///
/// - `ensure_if_values_exist()` のエッセンス: then/else/pre のいずれかに値が必須
///   → Select では then_val/else_val が必須（完全性チェック）
///
/// # conservative.rs からの移譲
///
/// - `ConservativeMerge::analyze()` の最小部分: 単一変数のみ対象
///   → Select は1つの dst のみ（単一 PHI 保証）
///
/// # Arguments
///
/// * `join_func` - 検証対象の JoinFunction
/// * `debug` - デバッグログ出力フラグ
///
/// # Returns
///
/// Ok(()) if verification passes, Err(JoinIrVerifyError) otherwise.
pub fn verify_select_minimal(
    join_func: &crate::mir::join_ir::JoinFunction,
    debug: bool,
) -> Result<(), JoinIrVerifyError> {
    if debug {
        get_global_ring0().log.debug(&format!(
            "[verify_select_minimal] checking {}",
            join_func.name
        ));
    }

    // 1. Select 命令を探す
    let mut select_count = 0;
    let mut select_inst: Option<&JoinInst> = None;

    for inst in &join_func.body {
        if let JoinInst::Select { .. } = inst {
            select_count += 1;
            select_inst = Some(inst);
        }
    }

    // 2. Select が 1 個だけか確認（単一 PHI チェック）
    if select_count != 1 {
        return Err(JoinIrVerifyError::new(format!(
            "verify_select_minimal: expected exactly 1 Select, found {}. \
             This violates the 'single PHI' invariant from conservative.rs",
            select_count
        )));
    }

    let JoinInst::Select {
        dst,
        cond,
        then_val,
        else_val,
        type_hint: _, // Phase 63-3: 検証では未使用
    } = select_inst.unwrap()
    else {
        unreachable!()
    };

    // 3. ValueId の妥当性チェック（基本的な存在確認）
    // Phase 33-3.2 では最小チェックのみ
    // 将来的に型情報があれば型一貫性チェック追加可能

    // 完全性チェック: then_val と else_val が両方存在
    // (ValueId は常に存在するので、ここでは簡易チェックのみ)
    // phi_invariants.rs::ensure_if_values_exist() のエッセンス:
    // - then/else のいずれかに値が必須
    // - Select 構造上、then_val/else_val は常に存在するのでOK

    if debug {
        get_global_ring0().log.debug(&format!(
            "[verify_select_minimal] OK: Select {{ dst: {:?}, cond: {:?}, then: {:?}, else: {:?} }}",
            dst, cond, then_val, else_val
        ));
        get_global_ring0().log.debug(
            "[verify_select_minimal] Invariants verified: single PHI (from conservative.rs), \
             completeness (from phi_invariants.rs)",
        );
    }

    Ok(())
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::join_ir::{ConstValue, JoinFunction, JoinModule};

    fn build_valid_skip_ws_loop() -> JoinModule {
        let mut module = JoinModule::new();

        // loop_step(s, n, i) with proper progress update
        let loop_func_id = JoinFuncId::new(1);
        let s_param = ValueId(4000);
        let n_param = ValueId(4001);
        let i_param = ValueId(4002);
        let const_1 = ValueId(4010);
        let next_i = ValueId(4011);

        let mut loop_func = JoinFunction::new(
            loop_func_id,
            "loop_step".to_string(),
            vec![s_param, n_param, i_param],
        );

        // const 1
        loop_func.body.push(JoinInst::Compute(MirLikeInst::Const {
            dst: const_1,
            value: ConstValue::Integer(1),
        }));

        // next_i = i + 1 (progress update)
        loop_func.body.push(JoinInst::Compute(MirLikeInst::BinOp {
            dst: next_i,
            op: BinOpKind::Add,
            lhs: i_param, // progress carrier as LHS
            rhs: const_1,
        }));

        // recursive call: loop_step(s, n, next_i)
        loop_func.body.push(JoinInst::Call {
            func: loop_func_id,
            args: vec![s_param, n_param, next_i],
            k_next: None,
            dst: None,
        });

        module.add_function(loop_func);
        module
    }

    fn build_invalid_skip_ws_loop() -> JoinModule {
        let mut module = JoinModule::new();

        // loop_step(s, n, i) WITHOUT progress update
        let loop_func_id = JoinFuncId::new(1);
        let s_param = ValueId(4000);
        let n_param = ValueId(4001);
        let i_param = ValueId(4002);

        let mut loop_func = JoinFunction::new(
            loop_func_id,
            "loop_step".to_string(),
            vec![s_param, n_param, i_param],
        );

        // recursive call with SAME i (no progress!)
        loop_func.body.push(JoinInst::Call {
            func: loop_func_id,
            args: vec![s_param, n_param, i_param], // i is not updated!
            k_next: None,
            dst: None,
        });

        module.add_function(loop_func);
        module
    }

    #[test]
    fn test_valid_progress_passes() {
        let module = build_valid_skip_ws_loop();
        let result = verify_progress_for_skip_ws(&module);
        assert!(result.is_ok(), "Valid progress should pass: {:?}", result);
    }

    #[test]
    fn test_zero_progress_detected() {
        let module = build_invalid_skip_ws_loop();
        let result = verify_progress_for_skip_ws(&module);
        assert!(result.is_err(), "Zero progress should be detected");
        match result {
            Err(ProgressError::ZeroProgressBackedge { .. }) => {
                // Expected
            }
            other => panic!("Expected ZeroProgressBackedge, got {:?}", other),
        }
    }

    #[test]
    fn test_empty_module_ok() {
        let module = JoinModule::new();
        let result = verify_progress_for_skip_ws(&module);
        assert!(
            result.is_ok(),
            "Empty module should pass (no loop_step to verify)"
        );
    }
}

// ============================================================================
// Phase 200-3: JoinIR Contract Verification (Loop Header PHI / Exit Line)
// ============================================================================

// Note: Verification functions are moved to merge/mod.rs to avoid circular dependencies
// with private control_flow module. This file only contains progress verification.
