//! ConditionOnly Variable Emitter - 毎イテレーション再計算される派生値
//!
//! # Phase 93 P0: ConditionOnly（Derived Slot）アーキテクチャ
//!
//! ## 問題
//! Trim patternの`is_ch_match`は：
//! - LoopState carrier（PHI対象）ではない（ConditionOnly）
//! - でも初回計算値がConditionBindingで運ばれてしまい、毎イテレーション同じ値がコピーされる
//!
//! ## 解決策
//! ConditionOnlyをConditionBindingで運ばず、「再計算レシピ」として扱う：
//! 1. 初回計算値をConditionBindingに入れない
//! 2. 代わりにDerived slotとして、毎イテレーションで再計算する
//! 3. インライン whitespace check を毎周回呼ぶ
//!
//! ## アーキテクチャ
//! ```
//! ループ前:
//!   is_ch_match0 = (s.substring(0, 1) == "b")  ← 初期化のみ（使われない）
//!
//! ループbody（毎イテレーション）:
//!   ch = s.substring(i, i+1)                   ← body-local init
//!   is_ch_match = (ch == "b")                  ← Derived slot再計算 ★ここ！
//!   if is_ch_match { break }                   ← break条件で使用
//! ```
//!
//! ## 配置理由
//! `src/mir/join_ir/lowering/common/` に置く：
//! - body-local変数とConditionOnly変数は密接に関連
//! - 両方とも「ループbody内でのみ有効」な派生値

use crate::ast::ASTNode;
use crate::mir::join_ir::lowering::condition_env::ConditionEnv;
use crate::mir::join_ir::lowering::loop_body_local_env::LoopBodyLocalEnv;
use crate::mir::join_ir::JoinInst;
use crate::mir::loop_route_detection::support::trim::TrimLoopHelper;
use crate::mir::ValueId;

/// Break semantics for ConditionOnly patterns
///
/// Phase 93 Refactoring: Explicit break condition semantics
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BreakSemantics {
    /// Break when condition is TRUE (e.g., find-first: break on ch == "b")
    WhenMatch,
    /// Break when condition is FALSE (e.g., trim: break on ch != whitespace)
    WhenNotMatch,
}

/// ConditionOnly変数の再計算レシピ
///
/// Phase 93 P0: Trim patternの`is_ch_match`など、毎イテレーション再計算される変数
/// Phase 93 Refactoring: Break semantics明確化
#[derive(Debug, Clone)]
pub struct ConditionOnlyRecipe {
    /// 変数名（例: "is_ch_match"）
    pub name: String,
    /// 元の変数名（例: "ch"）
    pub original_var: String,
    /// Trim pattern用のホワイトスペース文字リスト
    pub whitespace_chars: Vec<String>,
    /// Break semantics (WhenMatch or WhenNotMatch)
    pub break_semantics: BreakSemantics,
}

impl ConditionOnlyRecipe {
    /// Trim patternからレシピを作成（ConditionOnly pattern用）
    ///
    /// Phase 93 P0: ConditionOnly pattern（WhenMatch semantics）
    /// 例: find-first pattern（ch == "b"のときにbreak）
    pub fn from_trim_helper_condition_only(trim_helper: &TrimLoopHelper) -> Self {
        Self {
            name: trim_helper.carrier_name.clone(),
            original_var: trim_helper.original_var.clone(),
            whitespace_chars: trim_helper.whitespace_chars.clone(),
            break_semantics: BreakSemantics::WhenMatch,
        }
    }

    /// Trim patternからレシピを作成（Normal Trim pattern用）
    ///
    /// Phase 93 Refactoring: Normal Trim pattern（WhenNotMatch semantics）
    /// 例: str.trim()（ch != whitespaceのときにbreak）
    pub fn from_trim_helper_normal_trim(trim_helper: &TrimLoopHelper) -> Self {
        Self {
            name: trim_helper.carrier_name.clone(),
            original_var: trim_helper.original_var.clone(),
            whitespace_chars: trim_helper.whitespace_chars.clone(),
            break_semantics: BreakSemantics::WhenNotMatch,
        }
    }

    /// Trim patternからレシピを作成（後方互換性）
    ///
    /// Phase 93 Refactoring: WhenMatchをデフォルトとして使用
    #[allow(dead_code)]
    pub fn from_trim_helper(trim_helper: &TrimLoopHelper) -> Self {
        Self::from_trim_helper_condition_only(trim_helper)
    }

    /// Break条件AST生成（semanticsに基づく）
    ///
    /// Phase 93 Refactoring: Break semanticsに基づいて適切な条件を生成
    ///
    /// # Returns
    ///
    /// - `WhenMatch`: carrier変数そのまま（TRUE時にbreak）
    /// - `WhenNotMatch`: !carrier（FALSE時にbreak）
    pub fn generate_break_condition(&self) -> ASTNode {
        use crate::ast::{Span, UnaryOperator};

        let carrier_var = ASTNode::Variable {
            name: self.name.clone(),
            span: Span::unknown(),
        };

        match self.break_semantics {
            BreakSemantics::WhenMatch => carrier_var,
            BreakSemantics::WhenNotMatch => ASTNode::UnaryOp {
                operator: UnaryOperator::Not,
                operand: Box::new(carrier_var),
                span: Span::unknown(),
            },
        }
    }
}

/// ConditionOnly Variable Emitter - 毎イテレーション再計算
///
/// # 責務
/// - ConditionOnly変数（`is_ch_match`など）を毎イテレーション再計算
/// - body-local変数（`ch`）から派生値を生成
/// - ConditionEnvに再計算後の値を登録
///
/// # Phase 93 P0設計
/// - ConditionBindingでは運ばない（初回値を固定しない）
/// - 代わりに毎周回、body-local initの後に再計算
pub struct ConditionOnlyEmitter;

impl ConditionOnlyEmitter {
    /// ConditionOnly変数を再計算してemit
    ///
    /// # Phase 93 P0: Trim pattern専用
    ///
    /// ## 処理フロー
    /// 1. body-local env から元変数（`ch`）のValueIdを取得
    /// 2. Trim patternの条件（`ch == "b"`など）を再評価
    /// 3. 結果をConditionEnvに登録
    ///
    /// ## 呼び出しタイミング
    /// loop_break の `emit_body_local_init()` の直後
    ///
    /// # Arguments
    /// - `recipe`: 再計算レシピ（Trim pattern情報）
    /// - `body_local_env`: body-local変数の環境（`ch`の値を取得）
    /// - `condition_env`: 条件式環境（再計算後の`is_ch_match`を登録）
    /// - `alloc_value`: ValueId割り当て関数
    /// - `instructions`: 出力先JoinIR命令列
    ///
    /// # Returns
    /// - Ok(condition_value_id): 再計算後のConditionOnly変数のValueId
    /// - Err(msg): エラーメッセージ
    pub fn emit_condition_only_recalc(
        recipe: &ConditionOnlyRecipe,
        body_local_env: &LoopBodyLocalEnv,
        condition_env: &mut ConditionEnv,
        alloc_value: &mut dyn FnMut() -> ValueId,
        instructions: &mut Vec<JoinInst>,
    ) -> Result<ValueId, String> {
        // Step 1: body-local envから元変数（例: "ch"）のValueIdを取得
        let source_value = body_local_env.get(&recipe.original_var).ok_or_else(|| {
            format!(
                "[ConditionOnlyEmitter] Original variable '{}' not found in LoopBodyLocalEnv for '{}'",
                recipe.original_var, recipe.name
            )
        })?;

        // Step 2: Trim patternの条件を再評価
        let condition_value = Self::emit_whitespace_check_inline(
            source_value,
            &recipe.whitespace_chars,
            alloc_value,
            instructions,
        )?;

        // Step 3: ConditionEnvに再計算後の値を登録
        condition_env.insert(recipe.name.clone(), condition_value);

        Ok(condition_value)
    }

    /// Whitespace check（インライン実装）
    ///
    /// ## 処理
    /// ```mir
    /// %cond1 = icmp Eq %ch, " "
    /// %cond2 = icmp Eq %ch, "\t"
    /// %result = %cond1 Or %cond2  // 複数文字の場合はORチェーン
    /// ```
    fn emit_whitespace_check_inline(
        source_value: ValueId,
        whitespace_chars: &[String],
        alloc_value: &mut dyn FnMut() -> ValueId,
        instructions: &mut Vec<JoinInst>,
    ) -> Result<ValueId, String> {
        use crate::mir::join_ir::{BinOpKind, CompareOp, ConstValue, MirLikeInst};

        if whitespace_chars.is_empty() {
            return Err("[ConditionOnlyEmitter] Whitespace chars list is empty".to_string());
        }

        // 各ホワイトスペース文字との比較を生成
        let mut cond_values = Vec::new();
        for ws_char in whitespace_chars {
            // Const: ホワイトスペース文字
            let const_dst = alloc_value();
            instructions.push(JoinInst::Compute(MirLikeInst::Const {
                dst: const_dst,
                value: ConstValue::String(ws_char.clone()),
            }));

            // Compare: source_value == ws_char
            let cmp_dst = alloc_value();
            instructions.push(JoinInst::Compute(MirLikeInst::Compare {
                dst: cmp_dst,
                op: CompareOp::Eq,
                lhs: source_value,
                rhs: const_dst,
            }));

            cond_values.push(cmp_dst);
        }

        // 複数の条件をORでチェーン
        let mut result = cond_values[0];
        for &cond in &cond_values[1..] {
            let or_dst = alloc_value();
            instructions.push(JoinInst::Compute(MirLikeInst::BinOp {
                dst: or_dst,
                op: BinOpKind::Or,
                lhs: result,
                rhs: cond,
            }));
            result = or_dst;
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::join_ir::lowering::condition_env::ConditionEnv;
    use crate::mir::join_ir::lowering::loop_body_local_env::LoopBodyLocalEnv;
    use crate::mir::ValueId;

    #[test]
    fn test_emit_condition_only_recalc_single_char() {
        // Setup
        let recipe = ConditionOnlyRecipe {
            name: "is_ch_match".to_string(),
            original_var: "ch".to_string(),
            whitespace_chars: vec!["b".to_string()],
            break_semantics: BreakSemantics::WhenMatch,
        };

        let mut body_local_env = LoopBodyLocalEnv::new();
        body_local_env.insert("ch".to_string(), ValueId(100)); // ch = ValueId(100)

        let mut condition_env = ConditionEnv::new();
        let mut value_counter = 200u32;
        let mut alloc_value = || {
            let id = ValueId(value_counter);
            value_counter += 1;
            id
        };
        let mut instructions = Vec::new();

        // Execute
        let result = ConditionOnlyEmitter::emit_condition_only_recalc(
            &recipe,
            &body_local_env,
            &mut condition_env,
            &mut alloc_value,
            &mut instructions,
        );

        // Verify
        assert!(result.is_ok(), "Should succeed");
        let condition_value = result.unwrap();

        // Should generate: Const("b"), Compare(ch == "b")
        assert_eq!(instructions.len(), 2, "Should generate Const + Compare");

        // ConditionEnv should have is_ch_match registered
        assert_eq!(
            condition_env.get("is_ch_match"),
            Some(condition_value),
            "is_ch_match should be registered in ConditionEnv"
        );
    }

    #[test]
    fn test_emit_condition_only_recalc_multiple_chars() {
        let recipe = ConditionOnlyRecipe {
            name: "is_ws".to_string(),
            original_var: "ch".to_string(),
            whitespace_chars: vec![" ".to_string(), "\t".to_string(), "\n".to_string()],
            break_semantics: BreakSemantics::WhenMatch,
        };

        let mut body_local_env = LoopBodyLocalEnv::new();
        body_local_env.insert("ch".to_string(), ValueId(100));

        let mut condition_env = ConditionEnv::new();
        let mut value_counter = 200u32;
        let mut alloc_value = || {
            let id = ValueId(value_counter);
            value_counter += 1;
            id
        };
        let mut instructions = Vec::new();

        let result = ConditionOnlyEmitter::emit_condition_only_recalc(
            &recipe,
            &body_local_env,
            &mut condition_env,
            &mut alloc_value,
            &mut instructions,
        );

        assert!(result.is_ok());

        // Should generate: 3 * (Const + Compare) + 2 * Or = 8 instructions
        // Const(" "), Compare, Const("\t"), Compare, Or, Const("\n"), Compare, Or
        assert_eq!(
            instructions.len(),
            8,
            "Should generate 3 comparisons + 2 ORs"
        );
    }

    #[test]
    fn test_missing_body_local_variable() {
        let recipe = ConditionOnlyRecipe {
            name: "is_ch_match".to_string(),
            original_var: "ch".to_string(),
            whitespace_chars: vec!["b".to_string()],
            break_semantics: BreakSemantics::WhenMatch,
        };

        let body_local_env = LoopBodyLocalEnv::new(); // Empty - no "ch"
        let mut condition_env = ConditionEnv::new();
        let mut value_counter = 200u32;
        let mut alloc_value = || {
            let id = ValueId(value_counter);
            value_counter += 1;
            id
        };
        let mut instructions = Vec::new();

        let result = ConditionOnlyEmitter::emit_condition_only_recalc(
            &recipe,
            &body_local_env,
            &mut condition_env,
            &mut alloc_value,
            &mut instructions,
        );

        assert!(
            result.is_err(),
            "Should fail when body-local variable missing"
        );
        assert!(
            result
                .unwrap_err()
                .contains("not found in LoopBodyLocalEnv"),
            "Error should mention missing variable"
        );
    }
}
