//! Phase P2: Loop Patterns - JoinIR Frontend ループパターン処理層
//!
//! ## 責務
//! JSON v0 のループ body を、LoopFrontendBinding が渡してきた LoopPattern に従って
//! JoinIR 命令列に変換する。
//!
//! ## やらないこと
//! - 関数名ベースの判定（それは LoopFrontendBinding 層）
//! - Box 名・メソッド名で意味論を変える（それは Bridge/VM 層）
//!
//! ## 設計原則
//! - パターン = 1箱 = 1責務
//! - 共通処理は common.rs に集約
//! - エラーは Err(UnimplementedPattern) で返す

use super::{AstToJoinIrLowerer, JoinModule};

pub mod break_pattern;
pub mod common;
pub mod continue_pattern;
pub mod continue_return_pattern;
pub mod filter;
pub mod param_guess;
pub mod print_tokens;
pub mod simple;
pub mod step_calculator;

/// ループパターンの分類（ユースケースベース + 制御構造）
///
/// Phase 55/56 で実装済みのパターンと、今後実装予定のパターンを含む。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoopPattern {
    /// PrintTokens パターン（Phase 55）
    /// 責務: token を順番に取り出して print するループを Jump/Call/MethodCall に落とす
    PrintTokens,

    /// Filter パターン（Phase 56）
    /// 責務: pred が true のときだけ push するループを ConditionalMethodCall に落とす
    Filter,

    /// Map パターン（Phase 57+ 予定）
    /// 責務: 各要素を変換して新配列作成
    Map,

    /// Reduce パターン（Phase 58+ 予定）
    /// 責務: 累積計算（fold）
    Reduce,

    /// Simple パターン（汎用ループ）
    /// 責務: 上記以外の汎用的なループ処理
    Simple,

    /// Break パターン（Phase P4）
    /// 責務: if break 条件で早期 return するループを Jump(k_exit, cond) に落とす
    Break,

    /// Continue パターン（Phase P4）
    /// 責務: if continue 条件で処理をスキップするループを Select に落とす
    Continue,

    /// ContinueReturn パターン（Phase 89）
    /// 責務: continue + early return 両方を持つループを複合的に処理
    /// - continue: Select で carrier 切り替え
    /// - early return: 条件付き Jump で k_exit へ早期脱出
    ContinueReturn,

}

/// ループパターン lowering エラー
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum LoweringError {
    /// 未実装のパターン
    UnimplementedPattern {
        pattern: LoopPattern,
        reason: String,
    },
    /// JSON パースエラー
    JsonParseError { message: String },
    /// ループ body が不正
    InvalidLoopBody { message: String },
}

/// LoopPattern lowering の統一インターフェース
///
/// 各パターンの lowering モジュールはこの trait を実装する。
#[allow(dead_code)]
pub trait LoopPatternLowerer {
    /// LoopPattern を JoinModule に変換
    ///
    /// # Arguments
    /// * `lowerer` - AstToJoinIrLowerer インスタンス
    /// * `program_json` - Program(JSON v0)
    ///
    /// # Returns
    /// JoinModule または LoweringError
    fn lower(
        lowerer: &mut AstToJoinIrLowerer,
        program_json: &serde_json::Value,
    ) -> Result<JoinModule, LoweringError>;
}

/// LoopPattern に応じた lowering を実行（ディスパッチ箱）
///
/// Phase P2: この関数は薄いディスパッチのみを行い、実際の lowering は
/// 各パターンのモジュール（filter.rs, print_tokens.rs 等）に委譲する。
///
/// # Arguments
/// * `pattern` - LoopPattern enum
/// * `lowerer` - AstToJoinIrLowerer インスタンス
/// * `program_json` - Program(JSON v0)
pub fn lower_loop_with_pattern(
    pattern: LoopPattern,
    lowerer: &mut AstToJoinIrLowerer,
    program_json: &serde_json::Value,
) -> Result<JoinModule, LoweringError> {
    match pattern {
        LoopPattern::PrintTokens => print_tokens::lower(lowerer, program_json),
        LoopPattern::Filter => filter::lower(lowerer, program_json),
        LoopPattern::Map => Err(LoweringError::UnimplementedPattern {
            pattern: LoopPattern::Map,
            reason: "Map pattern is not yet implemented (Phase 57+)".to_string(),
        }),
        LoopPattern::Reduce => Err(LoweringError::UnimplementedPattern {
            pattern: LoopPattern::Reduce,
            reason: "Reduce pattern is not yet implemented (Phase 58+)".to_string(),
        }),
        LoopPattern::Simple => simple::lower(lowerer, program_json),
        LoopPattern::Break => break_pattern::lower(lowerer, program_json),
        LoopPattern::Continue => continue_pattern::lower(lowerer, program_json),
        LoopPattern::ContinueReturn => continue_return_pattern::lower(lowerer, program_json),
    }
}
