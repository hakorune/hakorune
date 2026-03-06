//! Phase P3: LoopFrontendBinding - 関数名 → LoopRoute 変換層
//!
//! ## 責務（1行で表現）
//! **関数名から適切な LoopRoute を決定し、loop_routes 層にディスパッチする**
//!
//! ## この層の責務
//!
//! - 関数名（"simple", "filter" 等）から `LoopRoute` enum を決定
//! - Break/Continue の有無を検出して route を調整
//! - `loop_routes::lower_loop_with_route()` に委譲
//!
//! ## やらないこと
//!
//! - JSON body の詳細解析（それは loop_routes 層）
//! - Box 名・メソッド名での判定（それは Bridge/VM 層）
//!
//! ## 設計原則
//!
//! ChatGPT 推奨の責務分離:
//! ```
//! LoopFrontendBinding → 関数名ベースの判定
//! loop_routes         → LoopRoute enum での lowering
//! Bridge/VM           → Box 名・メソッド名での最適化
//! ```

use super::loop_routes::{self, LoopRoute};
use super::{AstToJoinIrLowerer, JoinModule};

/// 関数名から LoopRoute を検出
///
/// # Arguments
/// * `func_name` - 関数名
/// * `loop_body` - Loop body の JSON（Break/Continue 検出用）
///
/// # Returns
/// 検出された LoopRoute
pub fn detect_loop_route(
    func_name: &str,
    loop_body: Option<&[serde_json::Value]>,
) -> LoopRoute {
    // Phase P3: 関数名ベースの判定
    // 将来的には Box 名やアノテーションも考慮可能
    match func_name {
        // Phase 55: PrintTokens route
        "print_tokens" => LoopRoute::PrintTokens,

        // Phase 56: Filter route
        "filter" => LoopRoute::Filter,

        // Phase 57+: Map route（未実装）
        "map" => LoopRoute::Map,

        // Phase 58+: Reduce route（未実装）
        "reduce" | "fold" => LoopRoute::Reduce,

        // デフォルト: Simple route
        // ただし Break/Continue/Return があれば別 route
        _ => {
            if let Some(body) = loop_body {
                let has_break = AstToJoinIrLowerer::has_break_in_loop_body(body);
                let has_continue = AstToJoinIrLowerer::has_continue_in_loop_body(body);
                let has_return = AstToJoinIrLowerer::has_return_in_loop_body(body);

                // Phase 89: Continue + Return の複合 route
                if has_continue && has_return {
                    LoopRoute::ContinueReturn
                } else if has_break {
                    LoopRoute::Break
                } else if has_continue {
                    LoopRoute::Continue
                } else {
                    LoopRoute::Simple
                }
            } else {
                LoopRoute::Simple
            }
        }
    }
}

/// ループ route に基づいて lowering を実行
///
/// 関数名から LoopRoute を決定し、適切な lowering を実行する。
///
/// # Arguments
/// * `lowerer` - AstToJoinIrLowerer インスタンス
/// * `program_json` - Program(JSON v0)
///
/// # Returns
/// JoinModule または panic（未対応 route）
pub fn lower_loop_by_function_name(
    lowerer: &mut AstToJoinIrLowerer,
    program_json: &serde_json::Value,
) -> JoinModule {
    // 1. 関数名を取得
    let defs = program_json["defs"]
        .as_array()
        .expect("Program(JSON v0) must have 'defs' array");

    let func_def = defs
        .get(0)
        .expect("At least one function definition required");

    let func_name = func_def["name"]
        .as_str()
        .expect("Function must have 'name'");

    // 2. Loop body を取得（Break/Continue 検出用）
    let body = &func_def["body"]["body"];
    let stmts = body.as_array().expect("Function body must be array");

    let loop_body: Option<&[serde_json::Value]> = stmts
        .iter()
        .find(|stmt| stmt["type"].as_str() == Some("Loop"))
        .and_then(|loop_node| loop_node["body"].as_array())
        .map(|v| v.as_slice());

    // 3. LoopRoute を決定（Break/Continue も新モジュールで処理）
    let route = detect_loop_route(func_name, loop_body);

    // 4. loop_routes 層に委譲
    match loop_routes::lower_loop_with_route(route.clone(), lowerer, program_json) {
        Ok(module) => module,
        Err(e) => panic!("LoopFrontendBinding error: {:?}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_loop_route_by_name() {
        assert_eq!(detect_loop_route("filter", None), LoopRoute::Filter);
        assert_eq!(
            detect_loop_route("print_tokens", None),
            LoopRoute::PrintTokens
        );
        assert_eq!(detect_loop_route("map", None), LoopRoute::Map);
        assert_eq!(detect_loop_route("reduce", None), LoopRoute::Reduce);
        assert_eq!(detect_loop_route("simple", None), LoopRoute::Simple);
        assert_eq!(detect_loop_route("unknown", None), LoopRoute::Simple);
    }
}
