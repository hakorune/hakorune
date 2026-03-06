//! AST/CFG → JoinIR Lowering
//!
//! このモジュールは AST/CFG ノードを JoinIR 命令に変換する。
//!
//! ## 責務
//!
//! - **If 文→Select/IfMerge 変換**: 条件分岐を JoinIR の継続渡しスタイルに変換
//! - **Loop 文→loop_step/k_exit 変換**: ループを関数呼び出しと継続に正規化
//! - **Break/Continue/Return→k_* 変換**: 制御フローを継続 ID として表現
//!
//! ## Phase 34-2 での実装スコープ
//!
//! 最初は `IfSelectTest.*` 相当の tiny ケースのみ対応：
//! - simple if-return shape: `if cond { return 1 } else { return 2 }`
//!
//! ## 設計原則
//!
//! - **JoinIR = PHI 生成器**: 既存 PHI の変換器にはしない（Phase 33-10 原則）
//! - **段階的移行**: 既存 MIR Builder 経路は保持、新経路はデフォルト OFF
//! - **A/B テスト可能**: 既存経路と新経路の両方で実行して比較検証

pub(crate) use crate::mir::join_ir::{
    BinOpKind, CompareOp, ConstValue, JoinFuncId, JoinFunction, JoinInst, JoinModule, MergePair,
    VarId,
};
pub(crate) use std::collections::{BTreeMap, HashSet};

mod analysis;
mod context;
mod expr;
mod if_in_loop;
mod if_return;
mod loop_frontend_binding;
mod loop_routes;
// Obsolete legacy dispatcher removed; active loop lowering now lives in loop_routes/.
mod nested_if;
mod read_quoted;
pub(crate) mod route;
mod stmt_handlers;

#[cfg(test)]
mod tests;

pub(crate) use context::ExtractCtx;
pub(crate) use route::{resolve_function_route, FunctionRoute};
pub(crate) use stmt_handlers::StatementEffect;

/// AST/CFG → JoinIR 変換器
///
/// Phase 34-2: Program(JSON v0) から tiny IfSelect ケースを JoinIR に変換
pub struct AstToJoinIrLowerer {
    pub(crate) next_func_id: u32,
    #[allow(dead_code)]
    pub(crate) next_var_id: u32,
}

impl AstToJoinIrLowerer {
    /// 新しい lowerer を作成
    pub fn new() -> Self {
        Self {
            next_func_id: 0,
            next_var_id: 0,
        }
    }

    /// Program(JSON v0) → JoinModule
    ///
    /// Phase 34-2/34-3/34-4: simple/local/json_shape pattern に対応
    /// Phase 34-5: extract_value 統一化（Int/Var/Method 構造まで）
    ///
    /// # Panics
    ///
    /// - 想定 route/shape に合わない Program(JSON) が来た場合（Phase 34 は tiny テスト専用）
    /// - ループ・複数変数・副作用付き if（Phase 34-6 以降で対応予定）
    pub fn lower_program_json(&mut self, program_json: &serde_json::Value) -> JoinModule {
        // 1. Program(JSON) から defs を取得
        let defs = program_json["defs"]
            .as_array()
            .expect("Program(JSON v0) must have 'defs' array");

        // 2. 最初の関数定義を取得
        let func_def = defs
            .get(0)
            .expect("At least one function definition required");

        let func_name = func_def["name"]
            .as_str()
            .expect("Function must have 'name'");

        let route = resolve_function_route(func_name).unwrap_or_else(|msg| panic!("{msg}"));

        match route {
            FunctionRoute::IfReturn => self.lower_if_return_pattern(program_json),
            FunctionRoute::LoopFrontend => {
                loop_frontend_binding::lower_loop_by_function_name(self, program_json)
            }
            FunctionRoute::NestedIf => self.lower_nested_if_pattern(program_json),
            FunctionRoute::ReadQuoted => self.lower_read_quoted_pattern(program_json),
        }
    }

    /// 次の関数 ID を生成
    pub(crate) fn next_func_id(&mut self) -> JoinFuncId {
        let id = JoinFuncId::new(self.next_func_id);
        self.next_func_id += 1;
        id
    }
}

impl Default for AstToJoinIrLowerer {
    fn default() -> Self {
        Self::new()
    }
}
