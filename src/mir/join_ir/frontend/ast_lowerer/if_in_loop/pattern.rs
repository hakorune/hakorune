//! Phase P1: If in Loop パターン分類
//!
//! ループ内の If ステートメントを 5 つのパターンに分類し、
//! 適切な lowering 戦略を選択する。

use serde_json::Value;

/// ループ内 If ステートメントのパターン
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IfInLoopPattern {
    /// ケース 1: 空の If（条件チェックのみ）
    /// then/else 両方が空
    Empty,

    /// ケース 2: then のみ単一変数更新、else は空
    /// `if cond { x = expr }` → `x = cond ? expr : x`
    SingleVarThen { var_name: String },

    /// ケース 3: then/else 両方が同じ変数への単一更新
    /// `if cond { x = a } else { x = b }` → `x = cond ? a : b`
    SingleVarBoth { var_name: String },

    /// ケース 4: 条件付き側効果（filter パターン）
    /// `if pred(v) { acc.push(v) }` → ConditionalMethodCall
    ConditionalEffect {
        receiver_name: String,
        method_name: String,
    },

    /// ケース 5: 複雑なケース（未対応）
    /// 複数ステートメント、異なる変数更新など
    Unsupported {
        then_count: usize,
        else_count: usize,
    },
}

impl IfInLoopPattern {
    /// If ステートメントから適切なパターンを検出
    ///
    /// # Arguments
    /// * `then_stmts` - then 分岐のステートメント配列
    /// * `else_stmts` - else 分岐のステートメント配列
    pub fn detect(then_stmts: &[Value], else_stmts: &[Value]) -> Self {
        // ケース 1: 空の If
        if then_stmts.is_empty() && else_stmts.is_empty() {
            return Self::Empty;
        }

        // ケース 2: then のみ単一変数更新
        if let (Some((then_name, _)), None) = (
            Self::extract_single_var_update(then_stmts),
            Self::extract_single_var_update(else_stmts),
        ) {
            return Self::SingleVarThen {
                var_name: then_name,
            };
        }

        // ケース 3: 両方に単一変数更新（同じ変数）
        if let (Some((then_name, _)), Some((else_name, _))) = (
            Self::extract_single_var_update(then_stmts),
            Self::extract_single_var_update(else_stmts),
        ) {
            if then_name == else_name {
                return Self::SingleVarBoth {
                    var_name: then_name,
                };
            }
        }

        // ケース 4: 条件付き側効果パターン
        if then_stmts.len() == 1 && else_stmts.is_empty() {
            let stmt = &then_stmts[0];
            let stmt_type = stmt["type"].as_str();

            if matches!(stmt_type, Some("Method") | Some("MethodCall")) {
                let receiver_expr = stmt.get("receiver").or_else(|| stmt.get("object"));
                let method_name = stmt["method"].as_str();

                if let (Some(receiver_expr), Some(method_name)) = (receiver_expr, method_name) {
                    let receiver_name = receiver_expr["name"].as_str().unwrap_or("acc").to_string();

                    return Self::ConditionalEffect {
                        receiver_name,
                        method_name: method_name.to_string(),
                    };
                }
            }
        }

        // ケース 5: 複雑なケース（未対応）
        Self::Unsupported {
            then_count: then_stmts.len(),
            else_count: else_stmts.len(),
        }
    }

    /// ステートメント配列から単一の変数更新を抽出
    ///
    /// Returns: Some((変数名, 式)) if 単一の Local/Assignment
    fn extract_single_var_update(stmts: &[Value]) -> Option<(String, &Value)> {
        if stmts.len() != 1 {
            return None;
        }

        let stmt = &stmts[0];
        let stmt_type = stmt["type"].as_str()?;

        match stmt_type {
            "Local" => {
                let name = stmt["name"].as_str()?.to_string();
                let expr = &stmt["expr"];
                Some((name, expr))
            }
            "Assignment" => {
                let name = stmt["target"].as_str()?.to_string();
                let expr = &stmt["expr"];
                Some((name, expr))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_detect_empty() {
        let pattern = IfInLoopPattern::detect(&[], &[]);
        assert_eq!(pattern, IfInLoopPattern::Empty);
    }

    #[test]
    fn test_detect_single_var_then() {
        let then_stmts = vec![json!({
            "type": "Assignment",
            "target": "x",
            "expr": json!({"type": "Int", "value": 42})
        })];
        let pattern = IfInLoopPattern::detect(&then_stmts, &[]);
        assert_eq!(
            pattern,
            IfInLoopPattern::SingleVarThen {
                var_name: "x".to_string()
            }
        );
    }

    #[test]
    fn test_detect_single_var_both() {
        let then_stmts = vec![json!({
            "type": "Assignment",
            "target": "x",
            "expr": json!({"type": "Int", "value": 1})
        })];
        let else_stmts = vec![json!({
            "type": "Assignment",
            "target": "x",
            "expr": json!({"type": "Int", "value": 2})
        })];
        let pattern = IfInLoopPattern::detect(&then_stmts, &else_stmts);
        assert_eq!(
            pattern,
            IfInLoopPattern::SingleVarBoth {
                var_name: "x".to_string()
            }
        );
    }

    #[test]
    fn test_detect_conditional_effect() {
        let then_stmts = vec![json!({
            "type": "Method",
            "receiver": json!({"name": "acc"}),
            "method": "push",
            "args": [json!({"type": "Var", "name": "v"})]
        })];
        let pattern = IfInLoopPattern::detect(&then_stmts, &[]);
        assert_eq!(
            pattern,
            IfInLoopPattern::ConditionalEffect {
                receiver_name: "acc".to_string(),
                method_name: "push".to_string()
            }
        );
    }

    #[test]
    fn test_detect_unsupported() {
        let then_stmts = vec![
            json!({"type": "Assignment", "target": "x", "expr": json!(1)}),
            json!({"type": "Assignment", "target": "y", "expr": json!(2)}),
        ];
        let pattern = IfInLoopPattern::detect(&then_stmts, &[]);
        assert_eq!(
            pattern,
            IfInLoopPattern::Unsupported {
                then_count: 2,
                else_count: 0
            }
        );
    }
}
