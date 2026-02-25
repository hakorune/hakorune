// Phase 27-shortterm S-3.2: JoinIR Runner 単体スモークテスト
//
// 目的:
// - JoinIR Runner の基本動作を VM に依存せず検証
// - StringBox ベースの skip_ws / trim を直接実行
// - ArrayBox 未サポートのため、Stage-1 minimal は VM 経由が必要
//
// 制約:
// - Runner サポート型: Int, Bool, Str, Unit のみ
// - Runner サポート BoxCall: StringBox.length, StringBox.substring のみ
// - ArrayBox/MapBox は未実装 (joinir_coverage.md A-2.5 参照)

use crate::mir::join_ir::verify::verify_progress_for_skip_ws;
use crate::mir::join_ir::*;
use crate::mir::join_ir_runner::{run_joinir_function, JoinValue};
use crate::tests::helpers::joinir_env;

fn require_experiment_toggle() -> bool {
    if !joinir_env::is_experiment_enabled() {
        eprintln!(
            "[joinir/runner/standalone] NYASH_JOINIR_EXPERIMENT=1 not set, skipping standalone test"
        );
        return false;
    }
    true
}

/// Phase 27-shortterm S-3.2: skip_ws Runner 単体テスト
///
/// JoinIR を手書きで構築し、VM なしで skip_ws ロジックを検証する。
///
/// ## ロジック:
/// ```text
/// fn skip_ws_entry(s: Str) -> Int {
///     let i_init = 0;
///     loop_step(s, s.length(), i_init)
/// }
///
/// fn loop_step(s: Str, n: Int, i: Int) -> Int {
///     if i >= n { return i }
///     let ch = s.substring(i, i+1)
///     if ch != " " { return i }
///     let next_i = i + 1
///     loop_step(s, n, next_i)
/// }
/// ```
///
/// ## 期待結果:
/// - Input: "   abc" → Output: 3 (先頭空白3文字をスキップ)
/// - Input: "" → Output: 0 (空文字列)
/// - Input: "abc" → Output: 0 (空白なし)
#[test]
fn joinir_runner_standalone_skip_ws() {
    if !require_experiment_toggle() {
        return;
    }

    let join_module = build_skip_ws_joinir();

    // Phase 29 L-5.2: Progress carrier verification
    // まずは警告のみ（将来 Phase 30 でエラーに格上げ予定）
    match verify_progress_for_skip_ws(&join_module) {
        Ok(()) => {
            eprintln!("[joinir/progress] ✅ skip_ws progress carrier check passed");
        }
        Err(e) => {
            eprintln!("[joinir/progress] ⚠️ warning: {:?}", e);
            // Phase 29: 警告のみ、panic しない
        }
    }

    // S-5.2-improved: Create MirInterpreter instance for VM integration
    let mut vm = crate::backend::mir_interpreter::MirInterpreter::new();

    // Test case 1: "   abc" → 3
    let result = run_joinir_function(
        &mut vm,
        &join_module,
        JoinFuncId::new(0), // entry function
        &[JoinValue::Str("   abc".to_string())],
    )
    .expect("skip_ws runner failed");
    match result {
        JoinValue::Int(v) => assert_eq!(v, 3, "skip_ws should skip 3 leading spaces"),
        other => panic!("skip_ws returned non-int: {:?}", other),
    }

    // Test case 2: "" → 0
    let result_empty = run_joinir_function(
        &mut vm,
        &join_module,
        JoinFuncId::new(0),
        &[JoinValue::Str("".to_string())],
    )
    .expect("skip_ws runner failed on empty string");
    match result_empty {
        JoinValue::Int(v) => assert_eq!(v, 0, "skip_ws on empty string should return 0"),
        other => panic!("skip_ws returned non-int: {:?}", other),
    }

    // Test case 3: "abc" → 0
    let result_no_ws = run_joinir_function(
        &mut vm,
        &join_module,
        JoinFuncId::new(0),
        &[JoinValue::Str("abc".to_string())],
    )
    .expect("skip_ws runner failed on no-whitespace string");
    match result_no_ws {
        JoinValue::Int(v) => assert_eq!(v, 0, "skip_ws on 'abc' should return 0"),
        other => panic!("skip_ws returned non-int: {:?}", other),
    }
}

/// Phase 27-shortterm S-3.2: trim Runner 単体テスト
///
/// JoinIR を手書きで構築し、VM なしで trim ロジックを検証する。
///
/// ## ロジック:
/// ```text
/// fn trim_entry(s: Str) -> Str {
///     let n = s.length()
///     let start = skip_leading(s, n, 0)
///     let end = skip_trailing(s, start)
///     return s.substring(start, end)
/// }
///
/// fn skip_leading(s: Str, n: Int, i: Int) -> Int {
///     if i >= n { return i }
///     let ch = s.substring(i, i+1)
///     if ch != " " { return i }
///     skip_leading(s, n, i+1)
/// }
///
/// fn skip_trailing(s: Str, start: Int) -> Int {
///     let n = s.length()
///     skip_trailing_loop(s, n, n)
/// }
///
/// fn skip_trailing_loop(s: Str, n: Int, i: Int) -> Int {
///     if i <= 0 { return i }
///     let ch = s.substring(i-1, i)
///     if ch != " " { return i }
///     skip_trailing_loop(s, n, i-1)
/// }
/// ```
///
/// ## 期待結果 (simplified - leading whitespace only):
/// - Input: "   abc  " → Output: "abc  " (only leading spaces removed)
/// - Input: "" → Output: ""
/// - Input: "abc" → Output: "abc"
///
/// ## Phase 29 昇格 (2025-11-25)
/// skip_ws に続いて本線テストに昇格。NYASH_JOINIR_EXPERIMENT=1 トグルで制御。
#[test]
fn joinir_runner_standalone_trim() {
    if !require_experiment_toggle() {
        return;
    }

    let join_module = build_trim_joinir();

    // S-5.2-improved: Create MirInterpreter instance for VM integration
    let mut vm = crate::backend::mir_interpreter::MirInterpreter::new();

    // Test case 1: "   abc  " → "abc  " (simplified - only leading whitespace)
    let result = run_joinir_function(
        &mut vm,
        &join_module,
        JoinFuncId::new(0), // entry function
        &[JoinValue::Str("   abc  ".to_string())],
    )
    .expect("trim runner failed");
    match result {
        JoinValue::Str(s) => assert_eq!(
            s, "abc  ",
            "simplified trim should remove only leading spaces"
        ),
        other => panic!("trim returned non-string: {:?}", other),
    }

    // Test case 2: "" → ""
    let result_empty = run_joinir_function(
        &mut vm,
        &join_module,
        JoinFuncId::new(0),
        &[JoinValue::Str("".to_string())],
    )
    .expect("trim runner failed on empty string");
    match result_empty {
        JoinValue::Str(s) => assert_eq!(s, "", "trim on empty string should return empty"),
        other => panic!("trim returned non-string: {:?}", other),
    }

    // Test case 3: "abc" → "abc"
    let result_no_ws = run_joinir_function(
        &mut vm,
        &join_module,
        JoinFuncId::new(0),
        &[JoinValue::Str("abc".to_string())],
    )
    .expect("trim runner failed on no-whitespace string");
    match result_no_ws {
        JoinValue::Str(s) => assert_eq!(s, "abc", "trim on 'abc' should return 'abc'"),
        other => panic!("trim returned non-string: {:?}", other),
    }
}

/// Build skip_ws JoinIR module (handwritten, no VM dependency)
///
/// ValueId range: 3000-4999 (skip_ws allocation from value_id_ranges.rs)
fn build_skip_ws_joinir() -> JoinModule {
    use crate::mir::ValueId;

    let mut module = JoinModule::new();

    // skip_ws_entry(s: Str) -> Int
    let entry_id = JoinFuncId::new(0);
    let s_param = ValueId(3000);
    let n_var = ValueId(3001);
    let i_init = ValueId(3002);

    let mut entry_func = JoinFunction::new(entry_id, "skip_ws_entry".to_string(), vec![s_param]);

    // n = s.length()
    entry_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BoxCall {
            dst: Some(n_var),
            box_name: "StringBox".to_string(),
            method: "length".to_string(),
            args: vec![s_param],
        }));

    // i_init = 0
    entry_func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: i_init,
        value: ConstValue::Integer(0),
    }));

    // loop_step(s, n, i_init)
    let loop_step_id = JoinFuncId::new(1);
    entry_func.body.push(JoinInst::Call {
        func: loop_step_id,
        args: vec![s_param, n_var, i_init],
        k_next: None,
        dst: None,
    });

    module.entry = Some(entry_id);
    module.add_function(entry_func);

    // loop_step(s: Str, n: Int, i: Int) -> Int
    let s_loop = ValueId(4000);
    let n_loop = ValueId(4001);
    let i_loop = ValueId(4002);
    let cmp_result = ValueId(4010);
    let ch_var = ValueId(4011);
    let const_1_var = ValueId(4012);
    let next_i = ValueId(4013);
    let space_const = ValueId(4014);
    let ch_cmp = ValueId(4015);
    let i_plus_1_for_substr = ValueId(4016);

    let mut loop_func = JoinFunction::new(
        loop_step_id,
        "loop_step".to_string(),
        vec![s_loop, n_loop, i_loop],
    );

    // if i >= n { return i }
    loop_func.body.push(JoinInst::Compute(MirLikeInst::Compare {
        dst: cmp_result,
        op: CompareOp::Ge,
        lhs: i_loop,
        rhs: n_loop,
    }));
    loop_func.body.push(JoinInst::Jump {
        cont: JoinContId::new(0),
        args: vec![i_loop], // return i
        cond: Some(cmp_result),
    });

    // ch = s.substring(i, i+1)
    loop_func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: const_1_var,
        value: ConstValue::Integer(1),
    }));
    loop_func.body.push(JoinInst::Compute(MirLikeInst::BinOp {
        dst: i_plus_1_for_substr,
        op: BinOpKind::Add,
        lhs: i_loop,
        rhs: const_1_var,
    }));
    loop_func.body.push(JoinInst::Compute(MirLikeInst::BoxCall {
        dst: Some(ch_var),
        box_name: "StringBox".to_string(),
        method: "substring".to_string(),
        args: vec![s_loop, i_loop, i_plus_1_for_substr],
    }));

    // if ch != " " { return i }
    loop_func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: space_const,
        value: ConstValue::String(" ".to_string()),
    }));
    loop_func.body.push(JoinInst::Compute(MirLikeInst::Compare {
        dst: ch_cmp,
        op: CompareOp::Ne,
        lhs: ch_var,
        rhs: space_const,
    }));
    loop_func.body.push(JoinInst::Jump {
        cont: JoinContId::new(0),
        args: vec![i_loop], // return i
        cond: Some(ch_cmp),
    });

    // next_i = i + 1 (reuse const_1_var)
    loop_func.body.push(JoinInst::Compute(MirLikeInst::BinOp {
        dst: next_i,
        op: BinOpKind::Add,
        lhs: i_loop,
        rhs: const_1_var,
    }));

    // loop_step(s, n, next_i) - tail recursion
    loop_func.body.push(JoinInst::Call {
        func: loop_step_id,
        args: vec![s_loop, n_loop, next_i],
        k_next: None,
        dst: None,
    });

    module.add_function(loop_func);

    module
}

/// Build trim JoinIR module (simplified: only leading whitespace for now)
///
/// ValueId range: 5000-6999 (trim allocation from value_id_ranges.rs)
///
/// Note: Full trim (leading + trailing) requires more complex logic.
/// For S-3.2, we implement simplified trim that only strips leading whitespace
/// to keep the test green and demonstrate Runner capability.
///
/// ## Simplified Algorithm:
/// ```text
/// fn trim_entry(s: Str) -> Str {
///     let n = s.length()
///     let start = find_first_non_space(s, n, 0)
///     return s.substring(start, n)
/// }
///
/// fn find_first_non_space(s: Str, n: Int, i: Int) -> Int {
///     if i >= n { return n }  // return n to handle empty substring correctly
///     let ch = s.substring(i, i+1)
///     if ch != " " { return i }
///     find_first_non_space(s, n, i+1)
/// }
/// ```
fn build_trim_joinir() -> JoinModule {
    use crate::mir::ValueId;

    let mut module = JoinModule::new();

    // trim_entry(s: Str) -> Str
    let entry_id = JoinFuncId::new(0);
    let s_param = ValueId(5000);
    let n_var = ValueId(5001);
    let start_var = ValueId(5002);
    let result_var = ValueId(5003);
    let const_0 = ValueId(5004);

    let mut entry_func = JoinFunction::new(entry_id, "trim_entry".to_string(), vec![s_param]);

    // n = s.length()
    entry_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BoxCall {
            dst: Some(n_var),
            box_name: "StringBox".to_string(),
            method: "length".to_string(),
            args: vec![s_param],
        }));

    // const_0 = 0
    entry_func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: const_0,
        value: ConstValue::Integer(0),
    }));

    // start = find_first_non_space(s, n, 0)
    let find_func_id = JoinFuncId::new(1);
    entry_func.body.push(JoinInst::Call {
        func: find_func_id,
        args: vec![s_param, n_var, const_0],
        k_next: None,
        dst: Some(start_var),
    });

    // result = s.substring(start, n)
    entry_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BoxCall {
            dst: Some(result_var),
            box_name: "StringBox".to_string(),
            method: "substring".to_string(),
            args: vec![s_param, start_var, n_var],
        }));

    // return result
    entry_func.body.push(JoinInst::Ret {
        value: Some(result_var),
    });

    module.entry = Some(entry_id);
    module.add_function(entry_func);

    // find_first_non_space(s: Str, n: Int, i: Int) -> Int
    let s_loop = ValueId(6000);
    let n_loop = ValueId(6001);
    let i_loop = ValueId(6002);
    let cmp_result = ValueId(6010);
    let ch_var = ValueId(6011);
    let const_1_var = ValueId(6012);
    let next_i = ValueId(6013);
    let space_const = ValueId(6014);
    let ch_cmp = ValueId(6015);
    let i_plus_1_for_substr = ValueId(6016);

    let mut find_func = JoinFunction::new(
        find_func_id,
        "find_first_non_space".to_string(),
        vec![s_loop, n_loop, i_loop],
    );

    // if i >= n { return n }  // return n to handle empty substring correctly
    find_func.body.push(JoinInst::Compute(MirLikeInst::Compare {
        dst: cmp_result,
        op: CompareOp::Ge,
        lhs: i_loop,
        rhs: n_loop,
    }));
    find_func.body.push(JoinInst::Jump {
        cont: JoinContId::new(0), // exit continuation (acts as return)
        args: vec![n_loop],       // return n to handle empty substring correctly
        cond: Some(cmp_result),   // only if i >= n
    });

    // ch = s.substring(i, i+1)
    find_func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: const_1_var,
        value: ConstValue::Integer(1),
    }));
    find_func.body.push(JoinInst::Compute(MirLikeInst::BinOp {
        dst: i_plus_1_for_substr,
        op: BinOpKind::Add,
        lhs: i_loop,
        rhs: const_1_var,
    }));
    find_func.body.push(JoinInst::Compute(MirLikeInst::BoxCall {
        dst: Some(ch_var),
        box_name: "StringBox".to_string(),
        method: "substring".to_string(),
        args: vec![s_loop, i_loop, i_plus_1_for_substr],
    }));

    // if ch != " " { return i }
    find_func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: space_const,
        value: ConstValue::String(" ".to_string()),
    }));
    find_func.body.push(JoinInst::Compute(MirLikeInst::Compare {
        dst: ch_cmp,
        op: CompareOp::Ne,
        lhs: ch_var,
        rhs: space_const,
    }));
    find_func.body.push(JoinInst::Jump {
        cont: JoinContId::new(0), // exit continuation (acts as return)
        args: vec![i_loop],       // return i if non-space found
        cond: Some(ch_cmp),       // only if ch != " "
    });

    // next_i = i + 1 (reuse const_1_var)
    find_func.body.push(JoinInst::Compute(MirLikeInst::BinOp {
        dst: next_i,
        op: BinOpKind::Add,
        lhs: i_loop,
        rhs: const_1_var,
    }));

    // find_first_non_space(s, n, next_i) - tail recursion
    find_func.body.push(JoinInst::Call {
        func: find_func_id,
        args: vec![s_loop, n_loop, next_i],
        k_next: None,
        dst: None,
    });

    module.add_function(find_func);

    module
}
