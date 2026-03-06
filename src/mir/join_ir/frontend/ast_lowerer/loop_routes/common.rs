//! Phase P2: Loop Patterns 共通処理
//!
//! ## 責務
//! 各 LoopRoute lowering で共通する処理を提供する。
//!
//! ## 提供する機能
//! - `ParsedProgram`: Program(JSON) のパース結果を保持
//! - `LoopContext`: 3関数構造（entry/loop_step/k_exit）のコンテキスト
//! - `parse_program_json()`: Program(JSON) からパース結果を抽出
//! - `process_local_inits()`: Loop 前の Local 初期化を処理
//! - `collect_external_refs()`: Phase 56 external_refs 収集
//! - `create_entry_function()`: entry 関数生成
//! - `create_k_exit_function()`: k_exit 関数生成

use super::super::stmt_handlers::StatementEffect;
use super::{AstToJoinIrLowerer, JoinModule};
use crate::mir::join_ir::JoinIrPhase;
use crate::mir::join_ir::{JoinFuncId, JoinFunction, JoinInst};
use crate::mir::ValueId;
use std::collections::BTreeMap;

use super::super::context::ExtractCtx;

/// Program(JSON) のパース結果
pub struct ParsedProgram {
    /// 関数名
    pub func_name: String,
    /// パラメータ名リスト
    pub param_names: Vec<String>,
    /// 関数 body の statements
    pub stmts: Vec<serde_json::Value>,
    /// Loop ノードのインデックス
    pub loop_node_idx: usize,
}

/// 3関数構造のコンテキスト
pub struct LoopContext {
    /// entry 関数 ID
    pub entry_id: JoinFuncId,
    /// loop_step 関数 ID
    pub loop_step_id: JoinFuncId,
    /// k_exit 関数 ID
    pub k_exit_id: JoinFuncId,
    /// "me" パラメータが存在するか
    pub has_me: bool,
    /// external_refs（Phase 56: arr, pred など）
    pub external_refs: Vec<(String, ValueId)>,
}

/// Program(JSON) をパースして ParsedProgram を返す
///
/// # Arguments
/// * `program_json` - Program(JSON v0)
///
/// # Returns
/// パース結果
pub fn parse_program_json(program_json: &serde_json::Value) -> ParsedProgram {
    // 1. defs 配列を取得
    let defs = program_json["defs"]
        .as_array()
        .expect("Program(JSON v0) must have 'defs' array");

    // 2. 最初の関数定義を取得
    let func_def = defs
        .get(0)
        .expect("At least one function definition required");

    let func_name = func_def["name"]
        .as_str()
        .expect("Function must have 'name'")
        .to_string();

    let params = func_def["params"]
        .as_array()
        .expect("Function must have 'params' array");

    let param_names: Vec<String> = params
        .iter()
        .map(|p| p.as_str().expect("Parameter must be string").to_string())
        .collect();

    // 3. body を取得
    let body = &func_def["body"]["body"];
    let stmts: Vec<serde_json::Value> = body
        .as_array()
        .expect("Function body must be array")
        .clone();

    // 4. Loop ノードのインデックスを探す
    let loop_node_idx = stmts
        .iter()
        .position(|stmt| stmt["type"].as_str() == Some("Loop"))
        .expect("Loop node not found");

    ParsedProgram {
        func_name,
        param_names,
        stmts,
        loop_node_idx,
    }
}

/// Loop 前の Local 初期化を処理
///
/// # Arguments
/// * `lowerer` - AstToJoinIrLowerer インスタンス
/// * `parsed` - パース済み Program
/// * `ctx` - ExtractCtx
///
/// # Returns
/// 初期化命令列
pub fn process_local_inits(
    lowerer: &mut AstToJoinIrLowerer,
    parsed: &ParsedProgram,
    ctx: &mut ExtractCtx,
) -> Vec<JoinInst> {
    let mut init_insts = Vec::new();

    for stmt in &parsed.stmts[..parsed.loop_node_idx] {
        let stmt_type = stmt["type"].as_str().expect("Statement must have type");

        match stmt_type {
            "Local" | "Assignment" | "If" => {
                let (insts, effect) = lowerer.lower_statement(stmt, ctx);
                init_insts.extend(insts);

                if let StatementEffect::VarUpdate { name, value_id } = effect {
                    ctx.register_param(name, value_id);
                } else if matches!(effect, StatementEffect::SideEffect) {
                    panic!(
                        "Unexpected side-effecting statement before Loop: {}",
                        stmt_type
                    );
                }
            }
            _ => panic!("Unexpected statement type before Loop: {}", stmt_type),
        }
    }

    init_insts
}

/// Phase 56: external_refs を収集
///
/// パラメータのうち、ループ制御変数（me, i, acc, n）以外を収集する。
/// filter(arr, pred) などで使用。
///
/// # Arguments
/// * `param_names` - パラメータ名リスト
///
/// # Returns
/// external_refs のリスト（名前, ValueId）
pub fn collect_external_refs(param_names: &[String]) -> Vec<(String, ValueId)> {
    const RESERVED_VARS: [&str; 4] = ["me", "i", "acc", "n"];

    param_names
        .iter()
        .enumerate()
        .filter_map(|(idx, name)| {
            if !RESERVED_VARS.contains(&name.as_str()) {
                Some((name.clone(), ValueId(idx as u32)))
            } else {
                None
            }
        })
        .collect()
}

/// LoopContext と entry 用 ExtractCtx を作成
///
/// 3関数構造の ID 生成と ExtractCtx 初期化を行う。
///
/// # Arguments
/// * `lowerer` - AstToJoinIrLowerer インスタンス
/// * `parsed` - パース済み Program
///
/// # Returns
/// (LoopContext, ExtractCtx)
pub fn create_loop_context(
    lowerer: &mut AstToJoinIrLowerer,
    parsed: &ParsedProgram,
) -> (LoopContext, ExtractCtx) {
    // 3関数 ID 生成
    let entry_id = lowerer.next_func_id();
    let loop_step_id = lowerer.next_func_id();
    let k_exit_id = lowerer.next_func_id();

    // ExtractCtx 作成とパラメータ登録
    let mut entry_ctx = ExtractCtx::new(parsed.param_names.len() as u32);
    for (i, name) in parsed.param_names.iter().enumerate() {
        entry_ctx.register_param(name.clone(), ValueId(i as u32));
    }

    // me パラメータ確認
    let has_me = parsed.param_names.iter().any(|n| n == "me");

    // external_refs 収集
    let external_refs = collect_external_refs(&parsed.param_names);

    let ctx = LoopContext {
        entry_id,
        loop_step_id,
        k_exit_id,
        has_me,
        external_refs,
    };

    (ctx, entry_ctx)
}

/// entry 関数を生成
///
/// # Arguments
/// * `ctx` - LoopContext
/// * `parsed` - パース済み Program
/// * `init_insts` - 初期化命令列
///
/// # Returns
/// entry JoinFunction
pub fn create_entry_function(
    ctx: &LoopContext,
    parsed: &ParsedProgram,
    init_insts: Vec<JoinInst>,
    entry_ctx: &mut ExtractCtx,
) -> JoinFunction {
    // i, acc, n を取得
    let i_init = entry_ctx.get_var("i").expect("i must be initialized");
    let acc_init = entry_ctx.get_var("acc").expect("acc must be initialized");
    let n_param = entry_ctx.get_var("n").expect("n must be parameter");

    // me パラメータ取得
    let me_param = entry_ctx.get_var("me");

    let loop_result = entry_ctx.alloc_var();

    // Call args 構築（me?, i, acc, n, ...external_refs）
    let mut call_args = if let Some(me_id) = me_param {
        vec![me_id, i_init, acc_init, n_param]
    } else {
        vec![i_init, acc_init, n_param]
    };

    // external_refs を追加
    for (name, _) in &ctx.external_refs {
        if let Some(var_id) = entry_ctx.get_var(name) {
            call_args.push(var_id);
        }
    }

    let mut body = init_insts;
    body.push(JoinInst::Call {
        func: ctx.loop_step_id,
        args: call_args,
        k_next: None,
        dst: Some(loop_result),
    });
    body.push(JoinInst::Ret {
        value: Some(loop_result),
    });

    JoinFunction {
        id: ctx.entry_id,
        name: parsed.func_name.clone(),
        params: (0..parsed.param_names.len())
            .map(|i| ValueId(i as u32))
            .collect(),
        body,
        exit_cont: None,
    }
}

/// k_exit 関数を生成
///
/// # Arguments
/// * `ctx` - LoopContext
/// * `func_name` - 関数名
///
/// # Returns
/// k_exit JoinFunction
pub fn create_k_exit_function(ctx: &LoopContext, func_name: &str) -> JoinFunction {
    let k_exit_acc = ValueId(0);

    JoinFunction {
        id: ctx.k_exit_id,
        name: format!("{}_k_exit", func_name),
        params: vec![k_exit_acc],
        body: vec![JoinInst::Ret {
            value: Some(k_exit_acc),
        }],
        exit_cont: None,
    }
}

/// loop_step 用の ExtractCtx を作成
///
/// # Arguments
/// * `ctx` - LoopContext
///
/// # Returns
/// loop_step 用 ExtractCtx
pub fn create_step_ctx(ctx: &LoopContext) -> ExtractCtx {
    // パラメータ数: me?, i, acc, n, ...external_refs
    let base_params: u32 = if ctx.has_me { 4 } else { 3 };
    let num_params = base_params + ctx.external_refs.len() as u32;

    let mut step_ctx = ExtractCtx::new(num_params);

    // ValueId 割り当て
    let (step_i, step_acc, step_n) = if ctx.has_me {
        step_ctx.register_param("me".to_string(), ValueId(0));
        (ValueId(1), ValueId(2), ValueId(3))
    } else {
        (ValueId(0), ValueId(1), ValueId(2))
    };

    step_ctx.register_param("i".to_string(), step_i);
    step_ctx.register_param("acc".to_string(), step_acc);
    step_ctx.register_param("n".to_string(), step_n);

    // external_refs 登録
    let ext_offset = base_params;
    for (i, (name, _)) in ctx.external_refs.iter().enumerate() {
        step_ctx.register_param(name.clone(), ValueId(ext_offset + i as u32));
    }

    step_ctx
}

/// loop_step 関数のパラメータリストを構築
///
/// # Arguments
/// * `ctx` - LoopContext
///
/// # Returns
/// パラメータ ValueId リスト
pub fn build_step_params(ctx: &LoopContext) -> Vec<ValueId> {
    let base_params: u32 = if ctx.has_me { 4 } else { 3 };

    let mut params = if ctx.has_me {
        vec![ValueId(0), ValueId(1), ValueId(2), ValueId(3)]
    } else {
        vec![ValueId(0), ValueId(1), ValueId(2)]
    };

    // external_refs を追加
    for (i, _) in ctx.external_refs.iter().enumerate() {
        params.push(ValueId(base_params + i as u32));
    }

    params
}

/// 再帰呼び出し args を構築
///
/// # Arguments
/// * `ctx` - LoopContext
/// * `step_ctx` - loop_step 用 ExtractCtx（更新後の i, acc を持つ）
///
/// # Returns
/// 再帰呼び出し args
pub fn build_recurse_args(ctx: &LoopContext, step_ctx: &ExtractCtx) -> Vec<ValueId> {
    let i_next = step_ctx.get_var("i").expect("i must exist");
    let acc_next = step_ctx.get_var("acc").expect("acc must exist");
    let step_n = step_ctx.get_var("n").expect("n must exist");

    let mut args = if ctx.has_me {
        let me_id = step_ctx.get_var("me").expect("me must exist");
        vec![me_id, i_next, acc_next, step_n]
    } else {
        vec![i_next, acc_next, step_n]
    };

    // external_refs を追加
    for (name, _) in &ctx.external_refs {
        if let Some(var_id) = step_ctx.get_var(name) {
            args.push(var_id);
        }
    }

    args
}

/// JoinModule を構築
///
/// # Arguments
/// * `entry_func` - entry 関数
/// * `loop_step_func` - loop_step 関数
/// * `k_exit_func` - k_exit 関数
///
/// # Returns
/// JoinModule
pub fn build_join_module(
    entry_func: JoinFunction,
    loop_step_func: JoinFunction,
    k_exit_func: JoinFunction,
) -> JoinModule {
    let entry_id = entry_func.id;

    let mut functions = BTreeMap::new();
    functions.insert(entry_func.id, entry_func);
    functions.insert(loop_step_func.id, loop_step_func);
    functions.insert(k_exit_func.id, k_exit_func);

    JoinModule {
        functions,
        entry: Some(entry_id),
        phase: JoinIrPhase::Structured,
    }
}
