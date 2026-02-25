//! JoinIR — 関数正規化 IR（Phase 26-H）
//!
//! 目的: Hakorune の制御構造を **関数呼び出し＋継続だけに正規化** する IR 層。
//! - φ ノード = 関数の引数
//! - merge ブロック = join 関数
//! - ループ = 再帰関数（loop_step）＋ exit 継続（k_exit）
//! - break / continue = 適切な関数呼び出し
//!
//! 位置づけ:
//! ```text
//! AST  →  MIR（+LoopForm v2）  →  JoinIR  →  VM / LLVM
//! ```
//!
//! Phase 26-H スコープ:
//! - 型定義のみ（変換ロジックは次フェーズ）
//! - 最小限の命令セット
//! - Debug 出力で妥当性確認
//!
//! Phase 27.9: Modular Structure
//! - Type definitions and common utilities in this file
//! - Lowering functions in `lowering/` submodule

use std::collections::BTreeMap;

use crate::mir::ValueId;
// Phase 63-3: 型ヒント用
use crate::mir::MirType;

// Phase 27.9: Lowering submodule
pub mod lowering;

// Phase 29 L-5.2: Progress verification
pub mod verify;

// Phase 30.x: JSON serialization (jsonir v0)
pub mod json;

// Phase 26-H.B: Normalized JoinIR (テスト専用ミニ)
pub mod normalized;

// Phase 34-1: Frontend (AST→JoinIR) — skeleton only
pub mod frontend;

// Phase 56: Ownership analysis (reads/writes → owned/relay/capture)
pub mod ownership;

// Phase 72: PHI reserved region verifier
#[cfg(debug_assertions)]
pub mod verify_phi_reserved;

// Re-export lowering functions for backward compatibility
pub use lowering::{
    lower_funcscanner_trim_to_joinir, lower_min_loop_to_joinir, lower_skip_ws_to_joinir,
};

// Re-export verification functions
pub use normalized::{
    normalize_pattern1_minimal, normalize_pattern2_minimal, normalized_pattern1_to_structured,
    normalized_pattern2_to_structured, NormalizedModule,
};
pub use verify::verify_progress_for_skip_ws;

// Phase 200-3: Contract verification functions are in merge/mod.rs (private module access)

/// JoinIR 関数ID（MIR 関数とは別 ID でもよい）
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JoinFuncId(pub u32);

impl JoinFuncId {
    pub fn new(id: u32) -> Self {
        JoinFuncId(id)
    }

    /// JoinFuncId を JoinContId に変換
    ///
    /// # Use Case
    /// Jump 命令で関数を continuation として使う場合
    /// ```rust
    /// let func_id = JoinFuncId(42);
    /// let cont_id = func_id.as_cont();
    /// Jump { cont: cont_id, args: vec![], cond: None }
    /// ```
    ///
    /// # Phase 34-7 Note
    /// JoinFuncId と JoinContId は別の newtype だが、内部的には同じ u32 ID を共有する。
    /// この変換は型レベルでの役割の明示（関数 vs 継続）を可能にする。
    pub fn as_cont(self) -> JoinContId {
        JoinContId(self.0)
    }
}

/// 継続（join / ループ step / exit continuation）を識別するID
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JoinContId(pub u32);

impl JoinContId {
    pub fn new(id: u32) -> Self {
        JoinContId(id)
    }

    /// JoinContId を JoinFuncId に変換
    ///
    /// # Use Case
    /// 継続 ID を関数 ID として参照する場合（JoinModule の functions map でルックアップ時など）
    /// ```rust
    /// let cont_id = JoinContId(42);
    /// let func = join_module.functions.get(&cont_id.as_func())?;
    /// ```
    ///
    /// # Phase 34-7 Note
    /// JoinIR では継続も関数として実装されるため、この変換が必要になる。
    pub fn as_func(self) -> JoinFuncId {
        JoinFuncId(self.0)
    }
}

/// 変数ID（Phase 26-H では MIR の ValueId を再利用）
pub type VarId = ValueId;

/// 環境変数フラグが "1" かチェックするヘルパー（JoinIR 実験経路用）
/// Phase 72-C Step 2: SSOT統一 - すべてのリードを config::env::joinir_dev 経由に
pub(crate) fn env_flag_is_1(name: &str) -> bool {
    use crate::config::env::joinir_dev;
    match name {
        "NYASH_JOINIR_LOWER_GENERIC" => joinir_dev::lower_generic_enabled(),
        "NYASH_JOINIR_MAINLINE_DEBUG" => joinir_dev::mainline_debug_enabled(),
        "NYASH_JOINIR_IF_MERGE" => joinir_dev::if_merge_enabled(),
        "NYASH_JOINIR_DEBUG" => joinir_dev::debug_enabled(),
        "NYASH_JOINIR_VM_BRIDGE" => joinir_dev::vm_bridge_enabled(),
        "NYASH_JOINIR_STRICT" => joinir_dev::strict_enabled(),
        "NYASH_JOINIR_SNAPSHOT_GENERATE" => joinir_dev::snapshot_generate_enabled(),
        "NYASH_JOINIR_SNAPSHOT_TEST" => joinir_dev::snapshot_test_enabled(),
        "NYASH_JOINIR_LOWER_FROM_MIR" => joinir_dev::lower_from_mir_enabled(),
        "NYASH_JOINIR_LLVM_EXPERIMENT" => joinir_dev::llvm_experiment_enabled(),
        "HAKO_JOINIR_IF_TOPLEVEL" => joinir_dev::if_toplevel_enabled(),
        "HAKO_JOINIR_IF_TOPLEVEL_TRACE" => joinir_dev::if_toplevel_trace_enabled(),
        "HAKO_JOINIR_IF_IN_LOOP_TRACE" => joinir_dev::if_in_loop_trace_enabled(),
        "HAKO_JOINIR_NESTED_IF" => joinir_dev::nested_if_enabled(),
        "HAKO_JOINIR_PRINT_TOKENS_MAIN" => joinir_dev::print_tokens_main_enabled(),
        "HAKO_JOINIR_ARRAY_FILTER_MAIN" => joinir_dev::array_filter_main_enabled(),
        "HAKO_JOINIR_READ_QUOTED" => joinir_dev::read_quoted_enabled(),
        "HAKO_JOINIR_READ_QUOTED_IFMERGE" => joinir_dev::read_quoted_ifmerge_enabled(),
        // Fallback for unknown flags (shouldn't happen in normal operation)
        // NYASH_JOINIR_EXPERIMENT is handled by test helpers, not body code
        _ => std::env::var(name).ok().as_deref() == Some("1"),
    }
}

/// Phase 27.4-A: ループ header φ の意味を表す構造（Pinned/Carrier 分類）
///
/// HeaderPhiBuilder が生成していた「ループ変数の合流」を JoinIR の loop_step 引数として表現するためのヘルパー。
///
/// 用語:
/// - **Pinned**: ループ中で値が変わらない変数（例: skip_ws の s, n / trim の str, b）
/// - **Carrier**: ループで更新される変数（例: skip_ws の i / trim の e）
///
/// Phase 27.4 では minimal/trim 用に手動で構成するが、将来は LoopScopeShape から自動導出する。
#[derive(Debug, Clone)]
#[allow(dead_code)] // Phase 27.4-C で実際に使用予定（現在は設計の雛形）
pub(crate) struct LoopHeaderShape {
    /// Pinned: ループ中で不変の変数リスト（初期値がそのまま使われる）
    pinned: Vec<ValueId>,
    /// Carrier: ループで更新される変数リスト（φ ノードで合流が必要）
    carriers: Vec<ValueId>,
}

#[allow(dead_code)] // Phase 27.4-C で実際に使用予定
impl LoopHeaderShape {
    /// Phase 27.4-A: 手動で Pinned/Carrier を指定して構築
    pub(crate) fn new_manual(pinned: Vec<ValueId>, carriers: Vec<ValueId>) -> Self {
        LoopHeaderShape { pinned, carriers }
    }

    /// loop_step 関数の引数リストを生成（pinned → carrier の順）
    pub(crate) fn to_loop_step_params(&self) -> Vec<ValueId> {
        let mut params = self.pinned.clone();
        params.extend(self.carriers.clone());
        params
    }
}

/// Phase 27.5: ループ exit φ の意味を表す構造
///
/// ExitPhiBuilder が生成していた「ループ脱出時の変数合流」を JoinIR の k_exit 引数として表現するためのヘルパー。
///
/// 用語:
/// - **exit_args**: ループから脱出する際に k_exit に渡す値のリスト
///
/// 例:
/// - **minimal_ssa_skip_ws**: exit_args = [i]
///   - ループから抜ける時、現在の i の値を返す
/// - **FuncScanner.trim**: exit_args = [e] (Option A)
///   - ループから抜ける時、現在の e の値を返す（後続で substring(b, e) を呼ぶ）
///
/// Phase 27.5 では minimal/trim 用に手動で構成するが、将来は ExitPhiBuilder の分析から自動導出する。
#[derive(Debug, Clone)]
#[allow(dead_code)] // Phase 27.6 で Exit φ 統合の実装フェーズで使用予定（現在は設計の雛形）
pub(crate) struct LoopExitShape {
    /// Exit 時に k_exit に渡したい値（JoinIR 引数）
    exit_args: Vec<ValueId>,
}

#[allow(dead_code)] // Phase 27.6 で実際に使用予定
impl LoopExitShape {
    /// Phase 27.5: 手動で exit_args を指定して構築
    pub(crate) fn new_manual(exit_args: Vec<ValueId>) -> Self {
        LoopExitShape { exit_args }
    }
}

/// JoinIR フェーズメタデータ。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoinIrPhase {
    /// Lowering 直後の構造化 JoinIR（Pattern1–5 / CarrierInfo / Boundary/ExitLine）
    Structured,
    /// 将来導入予定の正規化済み JoinIR（関数＋継続＋Env、TailCall-only）
    Normalized,
}

/// JoinIR 関数
#[derive(Debug, Clone)]
pub struct JoinFunction {
    /// 関数ID
    pub id: JoinFuncId,

    /// 関数名（デバッグ用）
    pub name: String,

    /// 引数（φ に相当）
    pub params: Vec<VarId>,

    /// 命令列（現在は直列、将来的にはブロック構造も可）
    pub body: Vec<JoinInst>,

    /// 呼び出し元に返す継続（ルートは None）
    pub exit_cont: Option<JoinContId>,
}

impl JoinFunction {
    pub fn new(id: JoinFuncId, name: String, params: Vec<VarId>) -> Self {
        Self {
            id,
            name,
            params,
            body: Vec::new(),
            exit_cont: None,
        }
    }
}

/// Phase 33-6: 複数変数を merge する if/else のペア
#[derive(Debug, Clone)]
pub struct MergePair {
    /// merge 先の変数
    pub dst: VarId,
    /// then 分岐での値
    pub then_val: VarId,
    /// else 分岐での値
    pub else_val: VarId,
    /// Phase 63-3: 結果型ヒント（MIR PHI 生成時の型推論を回避）
    pub type_hint: Option<MirType>,
}

/// JoinIR 命令セット（最小版）
#[derive(Debug, Clone)]
pub enum JoinInst {
    /// 通常の関数呼び出し（末尾再帰）: f(args..., k_next)
    ///
    /// # Semantics
    /// - 他の JoinIR 関数を呼び出す（MIR の Call に変換）
    /// - ループでは末尾再帰として使うのが典型的
    ///
    /// # MIR 変換
    /// - `MirInstruction::Call { func, args, ... }` を生成
    ///
    /// # Constraints (Phase 31/34 時点)
    /// - **k_next は常に None にすること！**
    ///   JoinIR→MIR bridge が `k_next: Some(...)` 未対応
    ///   → エラー: "Call with k_next is not yet supported"
    /// - 典型的な使い方: `Call { func, args, k_next: None, dst: Some(...) }`
    ///
    /// # Loop Pattern での使い方 (Phase 34-7)
    /// ```rust
    /// // ✅ 正解: 末尾再帰
    /// Call {
    ///     func: loop_step_id,
    ///     args: vec![i_next, acc_next, n],
    ///     k_next: None,  // ⚠️ 必須: None にすること
    ///     dst: Some(result),
    /// }
    /// ```
    Call {
        func: JoinFuncId,
        args: Vec<VarId>,
        k_next: Option<JoinContId>,
        /// 呼び出し結果を書き込む変数（None の場合は末尾呼び出しとして扱う）
        dst: Option<VarId>,
    },

    /// 継続呼び出し（早期 return / exit 継続）
    ///
    /// # Semantics
    /// - **「早期 return」（条件付き関数脱出）として使う！**
    /// - cond=Some(v): v が true なら cont に Jump、false なら次の命令へ
    /// - cond=None: 無条件 Jump
    ///
    /// # MIR 変換
    /// - cond=Some(v): `Branch(v, exit_block[Return], continue_block)` を生成
    ///   - exit_block: cont 関数を Call して Return
    ///   - continue_block: 次の JoinInst に続く
    /// - cond=None: 無条件に cont を Call して Return
    ///
    /// # Loop Pattern での使い方 (Phase 34-7)
    /// ```rust
    /// // ✅ 正解: 条件付き早期 return
    /// Jump {
    ///     cont: k_exit_id.as_cont(),
    ///     args: vec![acc],
    ///     cond: Some(exit_cond),  // exit_cond が true なら k_exit へ
    /// }
    /// // ↑ exit_cond が false なら次の命令（body 処理）へ進む
    ///
    /// // ❌ 間違い: Call で条件分岐しようとする
    /// Call {
    ///     func: k_exit_id,
    ///     cond: Some(exit_cond),  // こんなフィールドはない！
    /// }
    /// ```
    ///
    /// # 典型的なパターン
    /// ```text
    /// loop_step(i, acc, n):
    ///   exit_cond = !(i < n)
    ///   Jump(k_exit, [acc], cond=exit_cond)  // 🔑 早期 return
    ///   // ↓ Jump で抜けなかった場合のみ実行
    ///   acc_next = acc + 1
    ///   i_next = i + 1
    ///   Call(loop_step, [i_next, acc_next, n])  // 🔑 末尾再帰
    /// ```
    Jump {
        cont: JoinContId,
        args: Vec<VarId>,
        /// None のときは無条件ジャンプ、Some(var) のときは var が truthy のときだけ実行
        cond: Option<VarId>,
    },

    /// ルート関数 or 上位への戻り
    Ret { value: Option<VarId> },

    /// Phase 33: If/Else の単純な値選択（単一値）
    /// cond が true なら then_val、false なら else_val を dst に代入
    ///
    /// Phase 63-3: type_hint で結果型を伝播（infer_type_from_phi 削減用）
    Select {
        dst: VarId,
        cond: VarId,
        then_val: VarId,
        else_val: VarId,
        /// Phase 63-3: 結果型ヒント（MIR PHI 生成時の型推論を回避）
        type_hint: Option<MirType>,
    },

    /// Phase 33-6: If/Else の複数変数 merge
    /// cond が true なら各 dst に then_val を、false なら else_val を代入
    /// 複数の PHI ノードを一括で表現する
    IfMerge {
        cond: VarId,
        merges: Vec<MergePair>,
        k_next: Option<JoinContId>,
    },

    /// Phase 34-6: メソッド呼び出し構造
    /// receiver.method(args...) の構造を JoinIR で表現
    /// 意味論（BoxCall/Call への変換）は JoinIR→MIR ブリッジで実装
    ///
    /// Phase 65-2-A: type_hint で戻り値型を伝播（infer_type_from_phi 削減用）
    MethodCall {
        dst: VarId,
        receiver: VarId,
        method: String,
        args: Vec<VarId>,
        /// Phase 65-2-A: 戻り値型ヒント（P3-A StringBox メソッド対応）
        type_hint: Option<MirType>,
    },

    /// Phase 56: 条件付きメソッド呼び出し（filter パターン用）
    /// cond が true の場合のみ receiver.method(args) を実行
    /// cond が false の場合は dst = receiver（変更なし）
    ///
    /// 使用例: `if pred(v) { acc.push(v) }` → ConditionalMethodCall
    ConditionalMethodCall {
        cond: VarId,
        dst: VarId,
        receiver: VarId,
        method: String,
        args: Vec<VarId>,
    },

    /// Phase 51: フィールドアクセス
    /// object.field の構造を JoinIR で表現
    /// MIR 変換時に Load 命令に変換
    FieldAccess {
        dst: VarId,
        object: VarId,
        field: String,
    },

    /// Phase 51: Box インスタンス生成
    /// new BoxName(args...) の構造を JoinIR で表現
    /// MIR 変換時に NewBox 命令に変換
    ///
    /// Phase 65-2-B: type_hint で生成される Box 型を伝播（infer_type_from_phi 削減用）
    NewBox {
        dst: VarId,
        box_name: String,
        args: Vec<VarId>,
        /// Phase 65-2-B: 生成される Box の型ヒント（P3-B Box コンストラクタ対応）
        type_hint: Option<MirType>,
    },

    /// Phase 41-4: 深いネスト if の複数変数 merge（else なし）
    ///
    /// # Pattern
    /// ```text
    /// if cond1 {
    ///   if cond2 {
    ///     if cond3 {
    ///       x = new_val  // modifications only at deepest level
    ///     }
    ///   }
    /// }
    /// // merge: x = phi(new_val if all conds true, original otherwise)
    /// ```
    ///
    /// # Semantics
    /// - `conds`: 外側から内側への条件リスト
    /// - `merges`: 最深レベルでの変数更新
    ///   - then_val: 全ての conds が true の場合の値
    ///   - else_val: いずれかの cond が false の場合の値（元の値）
    /// - MIR 変換時に多段 Branch + PHI を生成
    ///
    /// # Target
    /// ParserControlBox.parse_loop() の 4 レベルネスト if パターン
    NestedIfMerge {
        /// 条件リスト（外側から内側へ）
        conds: Vec<VarId>,
        /// 変数更新（全条件 true 時 → then_val、いずれか false 時 → else_val）
        merges: Vec<MergePair>,
        /// merge 後の継続
        k_next: Option<JoinContId>,
    },

    /// それ以外の演算は、現行 MIR の算術/比較/boxcall を再利用
    Compute(MirLikeInst),
}

/// MIR からの算術・比較命令のラッパー（Phase 26-H では最小限）
#[derive(Debug, Clone)]
pub enum MirLikeInst {
    /// 定数代入
    Const { dst: VarId, value: ConstValue },

    /// 二項演算
    BinOp {
        dst: VarId,
        op: BinOpKind,
        lhs: VarId,
        rhs: VarId,
    },

    /// 比較演算
    Compare {
        dst: VarId,
        op: CompareOp,
        lhs: VarId,
        rhs: VarId,
    },

    /// Box呼び出し（将来的には統一 Call に統合予定）
    BoxCall {
        dst: Option<VarId>,
        box_name: String,
        method: String,
        args: Vec<VarId>,
    },

    /// Phase 56: 単項演算（not, 負号）
    UnaryOp {
        dst: VarId,
        op: UnaryOp,
        operand: VarId,
    },

    /// Phase 188: Print 文（コンソール出力）
    /// print(value) の構造を JoinIR で表現
    /// MIR 変換時に Print 命令に変換
    Print { value: VarId },

    /// Phase 188-Impl-3: 条件付き値選択（三項演算子）
    /// cond が true なら then_val を、false なら else_val を dst に代入
    /// JoinIR の Select 命令と同じ semantics
    Select {
        dst: VarId,
        cond: VarId,
        then_val: VarId,
        else_val: VarId,
    },
}

/// Phase 56: 単項演算種別
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    /// 論理否定
    Not,
    /// 算術否定（負号）
    Neg,
}

/// 定数値（MIR の ConstValue を簡略化）
#[derive(Debug, Clone)]
pub enum ConstValue {
    Integer(i64),
    Bool(bool),
    String(String),
    Null,
}

/// 二項演算種別
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
    Mod, // Phase 188-Impl-3: 剰余演算 (a % b)
    Or,  // Phase 27.1: 論理OR (bool || bool)
    And, // Phase 27.1: 論理AND (bool && bool)
}

/// 比較演算種別
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompareOp {
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
}

/// JoinIR モジュール（複数の関数を保持）
#[derive(Debug, Clone)]
pub struct JoinModule {
    /// 関数マップ
    pub functions: BTreeMap<JoinFuncId, JoinFunction>,

    /// エントリーポイント関数ID
    pub entry: Option<JoinFuncId>,

    /// JoinIR のフェーズ（構造化 / 正規化）
    pub phase: JoinIrPhase,
}

impl JoinModule {
    pub fn new() -> Self {
        Self {
            functions: BTreeMap::new(),
            entry: None,
            phase: JoinIrPhase::Structured,
        }
    }

    pub fn add_function(&mut self, func: JoinFunction) {
        self.functions.insert(func.id, func);
    }

    pub fn is_structured(&self) -> bool {
        self.phase == JoinIrPhase::Structured
    }

    pub fn is_normalized(&self) -> bool {
        self.phase == JoinIrPhase::Normalized
    }

    pub fn mark_normalized(&mut self) {
        self.phase = JoinIrPhase::Normalized;
    }

    // Phase 132-Post: Box-First principle - encapsulate function search logic
    /// Find function by name
    pub fn get_function_by_name(&self, name: &str) -> Option<&JoinFunction> {
        self.functions.values().find(|f| f.name == name)
    }

    /// Find function by name or panic with descriptive message
    pub fn require_function(&self, name: &str, context: &str) -> &JoinFunction {
        self.get_function_by_name(name)
            .unwrap_or_else(|| panic!("{}: missing required function '{}'", context, name))
    }
}

impl Default for JoinModule {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_join_function_creation() {
        let func_id = JoinFuncId::new(0);
        let func = JoinFunction::new(
            func_id,
            "test_func".to_string(),
            vec![ValueId(1), ValueId(2)],
        );

        assert_eq!(func.id, func_id);
        assert_eq!(func.name, "test_func");
        assert_eq!(func.params.len(), 2);
        assert_eq!(func.body.len(), 0);
        assert_eq!(func.exit_cont, None);
    }

    #[test]
    fn test_join_module() {
        let mut module = JoinModule::new();
        let func = JoinFunction::new(JoinFuncId::new(0), "main".to_string(), vec![]);
        module.add_function(func);

        assert_eq!(module.functions.len(), 1);
        assert!(module.functions.contains_key(&JoinFuncId::new(0)));
        assert_eq!(module.phase, JoinIrPhase::Structured);
        assert!(module.is_structured());
    }

    #[test]
    fn test_mark_normalized() {
        let mut module = JoinModule::new();
        assert!(module.is_structured());
        module.mark_normalized();
        assert!(module.is_normalized());
    }

    #[test]
    fn loop_header_shape_params_order_is_pinned_then_carrier() {
        // Phase 27.4-A: to_loop_step_params() が pinned→carriers の順を返すことを保証
        let v1 = ValueId(1);
        let v2 = ValueId(2);
        let v3 = ValueId(3);
        let shape = LoopHeaderShape::new_manual(vec![v1, v2], vec![v3]);
        let params = shape.to_loop_step_params();
        assert_eq!(params, vec![v1, v2, v3]);
    }
}
