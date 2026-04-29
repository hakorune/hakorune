//! Phase P2: Loop Routes - JoinIR Frontend ループ route 処理層
//!
//! ## 責務
//! JSON v0 のループ body を、LoopFrontendBinding が渡してきた LoopRoute に従って
//! JoinIR 命令列に変換する。
//!
//! ## やらないこと
//! - 関数名ベースの判定（それは LoopFrontendBinding 層）
//! - Box 名・メソッド名で意味論を変える（それは Bridge/VM 層）
//!
//! ## 設計原則
//! - route = 1箱 = 1責務
//! - 共通処理は common.rs に集約
//! - エラーは Err(UnimplementedRoute) で返す

use super::{AstToJoinIrLowerer, JoinModule};
use std::fmt;

pub mod break_route;
pub mod common;
pub mod continue_return_route;
pub mod continue_route;
pub mod filter;
pub mod param_guess;
pub mod print_tokens;
pub mod simple;
pub mod step_calculator;

/// ループ route の分類（ユースケースベース + 制御構造）
///
/// Phase 55/56 で実装済みの route と、今後実装予定の route を含む。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoopRoute {
    /// PrintTokens route（Phase 55）
    /// 責務: token を順番に取り出して print するループを Jump/Call/MethodCall に落とす
    PrintTokens,

    /// Filter route（Phase 56）
    /// 責務: pred が true のときだけ push するループを ConditionalMethodCall に落とす
    Filter,

    /// Map route（Phase 57+ 予定）
    /// 責務: 各要素を変換して新配列作成
    Map,

    /// Reduce route（Phase 58+ 予定）
    /// 責務: 累積計算（fold）
    Reduce,

    /// Simple route（汎用ループ）
    /// 責務: 上記以外の汎用的なループ処理
    Simple,

    /// Break route（Phase P4）
    /// 責務: if break 条件で早期 return するループを Jump(k_exit, cond) に落とす
    Break,

    /// Continue route（Phase P4）
    /// 責務: if continue 条件で処理をスキップするループを Select に落とす
    Continue,

    /// ContinueReturn route（Phase 89）
    /// 責務: continue + early return 両方を持つループを複合的に処理
    /// - continue: Select で carrier 切り替え
    /// - early return: 条件付き Jump で k_exit へ早期脱出
    ContinueReturn,
}

/// ループ route lowering エラー
#[derive(Debug, Clone)]
pub enum LoweringError {
    /// 未実装の route
    UnimplementedRoute { route: LoopRoute, reason: String },
    /// ループ body が不正
    InvalidLoopBody { message: String },
}

impl fmt::Display for LoweringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoweringError::UnimplementedRoute { route, reason } => {
                write!(f, "unimplemented loop route {:?}: {}", route, reason)
            }
            LoweringError::InvalidLoopBody { message } => {
                write!(f, "invalid loop body: {}", message)
            }
        }
    }
}

impl std::error::Error for LoweringError {}

/// LoopRoute に応じた lowering を実行（ディスパッチ箱）
///
/// Phase P2: この関数は薄いディスパッチのみを行い、実際の lowering は
/// 各 route モジュール（filter.rs, print_tokens.rs 等）に委譲する。
///
/// # Arguments
/// * `route` - LoopRoute enum
/// * `lowerer` - AstToJoinIrLowerer インスタンス
/// * `program_json` - Program(JSON v0)
pub fn lower_loop_with_route(
    route: LoopRoute,
    lowerer: &mut AstToJoinIrLowerer,
    program_json: &serde_json::Value,
) -> Result<JoinModule, LoweringError> {
    match route {
        LoopRoute::PrintTokens => print_tokens::lower(lowerer, program_json),
        LoopRoute::Filter => filter::lower(lowerer, program_json),
        LoopRoute::Map => Err(LoweringError::UnimplementedRoute {
            route: LoopRoute::Map,
            reason: "Map route is not yet implemented (Phase 57+)".to_string(),
        }),
        LoopRoute::Reduce => Err(LoweringError::UnimplementedRoute {
            route: LoopRoute::Reduce,
            reason: "Reduce route is not yet implemented (Phase 58+)".to_string(),
        }),
        LoopRoute::Simple => simple::lower(lowerer, program_json),
        LoopRoute::Break => break_route::lower(lowerer, program_json),
        LoopRoute::Continue => continue_route::lower(lowerer, program_json),
        LoopRoute::ContinueReturn => continue_return_route::lower(lowerer, program_json),
    }
}
