//! Phase P4 副産物: Step increment 抽出・計算ユーティリティ
//!
//! ## 責務（1行で表現）
//! **ループ step increment の線形パターン（i = i + const）を検出・計算する**
//!
//! ## 使用例
//! ```rust
//! // Continue パターンで通常/continue パスの step 差分を計算
//! let base_k = StepCalculator::extract_linear_increment(i_expr, "i")?;
//! let then_k = StepCalculator::extract_linear_increment(&then_i_expr, "i")?;
//! let delta = StepCalculator::calculate_step_difference(then_k, base_k);
//! ```

use serde_json::Value;

/// Step increment 計算器（純関数集合）
pub struct StepCalculator;

impl StepCalculator {
    /// `expr` が `i + const` または `const + i` 形式なら K を返す
    ///
    /// # Arguments
    /// * `expr` - JSON v0 形式の式（Binary op想定）
    /// * `var_name` - ターゲット変数名（通常は "i"）
    ///
    /// # Returns
    /// * `Some(K)`: 線形インクリメント定数
    /// * `None`: パターン不一致
    ///
    /// # 対応形式
    /// - `i + 1`, `i + 2`, `i + K`
    /// - `1 + i`, `2 + i`, `K + i`
    ///
    /// # 非対応形式
    /// - `i - K` (減算は未対応)
    /// - `i * K` (乗算は未対応)
    /// - `i + j` (変数同士の加算)
    pub fn extract_linear_increment(expr: &Value, var_name: &str) -> Option<i64> {
        // Binary + 演算子チェック
        if expr["type"].as_str()? != "Binary" || expr["op"].as_str()? != "+" {
            return None;
        }

        let lhs = &expr["lhs"];
        let rhs = &expr["rhs"];

        // パターン1: var + const
        if Self::is_var_with_name(lhs, var_name) {
            return Self::extract_int_const(rhs);
        }

        // パターン2: const + var
        if Self::is_var_with_name(rhs, var_name) {
            return Self::extract_int_const(lhs);
        }

        None
    }

    /// Step 差分を計算
    ///
    /// # Arguments
    /// * `then_step`: Continue パスの step 値（例: i = i + 2）
    /// * `normal_step`: 通常パスの step 値（例: i = i + 1）
    ///
    /// # Returns
    /// * `delta`: Continue パスの追加 step（例: 2 - 1 = 1）
    ///
    /// # 使用例
    /// ```rust
    /// let delta = StepCalculator::calculate_step_difference(3, 1); // → 2
    /// // i_next_continue = i_next + delta
    /// ```
    pub fn calculate_step_difference(then_step: i64, normal_step: i64) -> i64 {
        then_step - normal_step
    }

    /// 変数が指定名かチェック
    fn is_var_with_name(expr: &Value, var_name: &str) -> bool {
        expr["type"].as_str() == Some("Var") && expr["name"].as_str() == Some(var_name)
    }

    /// 整数定数を抽出
    fn extract_int_const(expr: &Value) -> Option<i64> {
        if expr["type"].as_str()? == "Int" {
            expr["value"].as_i64()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_extract_linear_increment_i_plus_const() {
        let expr = json!({
            "type": "Binary",
            "op": "+",
            "lhs": {"type": "Var", "name": "i"},
            "rhs": {"type": "Int", "value": 2}
        });
        assert_eq!(
            StepCalculator::extract_linear_increment(&expr, "i"),
            Some(2)
        );
    }

    #[test]
    fn test_extract_linear_increment_const_plus_i() {
        let expr = json!({
            "type": "Binary",
            "op": "+",
            "lhs": {"type": "Int", "value": 3},
            "rhs": {"type": "Var", "name": "i"}
        });
        assert_eq!(
            StepCalculator::extract_linear_increment(&expr, "i"),
            Some(3)
        );
    }

    #[test]
    fn test_extract_linear_increment_non_addition() {
        let expr = json!({
            "type": "Binary",
            "op": "*",
            "lhs": {"type": "Var", "name": "i"},
            "rhs": {"type": "Int", "value": 2}
        });
        assert_eq!(StepCalculator::extract_linear_increment(&expr, "i"), None);
    }

    #[test]
    fn test_extract_linear_increment_wrong_var_name() {
        let expr = json!({
            "type": "Binary",
            "op": "+",
            "lhs": {"type": "Var", "name": "j"},
            "rhs": {"type": "Int", "value": 1}
        });
        assert_eq!(StepCalculator::extract_linear_increment(&expr, "i"), None);
    }

    #[test]
    fn test_extract_linear_increment_var_plus_var() {
        let expr = json!({
            "type": "Binary",
            "op": "+",
            "lhs": {"type": "Var", "name": "i"},
            "rhs": {"type": "Var", "name": "j"}
        });
        assert_eq!(StepCalculator::extract_linear_increment(&expr, "i"), None);
    }

    #[test]
    fn test_calculate_step_difference() {
        assert_eq!(StepCalculator::calculate_step_difference(3, 1), 2);
        assert_eq!(StepCalculator::calculate_step_difference(1, 1), 0);
        assert_eq!(StepCalculator::calculate_step_difference(5, 2), 3);
        assert_eq!(StepCalculator::calculate_step_difference(2, 5), -3);
    }
}
