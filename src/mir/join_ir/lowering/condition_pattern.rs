//! ConditionPatternBox: ルート判定用の if 条件正規化
//!
//! Phase 219 regression fix: if条件が「単純比較」かどうかを判定
//! Phase 222: 左辺変数・右辺変数の両方をサポートする正規化を追加
//!
//! ## 問題
//!
//! Phase 219で `is_if_sum_pattern()`（legacy 呼称, traceability-only）を
//! AST-basedに変更した結果、
//! `loop_if_phi.hako` のような複雑条件 (`i % 2 == 1`) を
//! IfPhiJoin 経路として誤判定してしまう問題が発生。
//!
//! Phase 221で発見した制約：if条件が `0 < i` や `i > j` のような形式を拒否。
//!
//! ## 解決策
//!
//! ConditionPatternBox を導入し、if条件が「単純比較」かどうかを判定する。
//! ただし「単純/複雑/legacy」の語彙は混線しやすいので、routing 用には
//! `ConditionCapability` を使って「どの経路で扱うか」を明示する。
//!
//! Phase 222: 左右反転（literal on left → var on left）と変数同士の比較をサポート。
//!
//! ## 単純比較の定義（Phase 222拡張版）
//!
//! 以下の形は IfPhiJoin 経路の lowerer で処理可能：
//! - **Phase 219**: `var > literal` (e.g., `i > 0`)
//! - **Phase 222**: `literal < var` → `var > literal` に正規化
//! - **Phase 222**: `var > var` (e.g., `i > j`) - 変数同士の比較
//!
//! ## 複雑条件（別経路へフォールバック）
//!
//! - `i % 2 == 1` (BinaryOp in LHS)
//! - `a && b` (複合条件)
//! - `method_call() > 0` (MethodCall)
//! - その他

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::CompareOp;

/// ConditionCapability: 条件式をどの戦略で扱えるか（routing 用）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConditionCapability {
    /// IfPhiJoin 経路の AST-based lowerer で比較として扱える
    IfPhiJoinComparable,
    /// 上記以外（caller が別経路を選ぶ）
    Unsupported,
}

fn is_if_phi_join_value_expr(expr: &ASTNode) -> bool {
    match expr {
        ASTNode::Variable { .. } | ASTNode::Literal { .. } => true,
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } => {
            matches!(
                operator,
                BinaryOperator::Add
                    | BinaryOperator::Subtract
                    | BinaryOperator::Multiply
                    | BinaryOperator::Divide
                    | BinaryOperator::Modulo
            ) && is_if_phi_join_value_expr(left.as_ref())
                && is_if_phi_join_value_expr(right.as_ref())
        }
        _ => false,
    }
}

/// 条件式の“能力”を判定（routing のための入口）
pub fn analyze_condition_capability(cond: &ASTNode) -> ConditionCapability {
    match cond {
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } => {
            let is_comparison = matches!(
                operator,
                BinaryOperator::Equal
                    | BinaryOperator::NotEqual
                    | BinaryOperator::Less
                    | BinaryOperator::Greater
                    | BinaryOperator::LessEqual
                    | BinaryOperator::GreaterEqual
            );
            if !is_comparison {
                return ConditionCapability::Unsupported;
            }
            if is_if_phi_join_value_expr(left.as_ref()) && is_if_phi_join_value_expr(right.as_ref())
            {
                ConditionCapability::IfPhiJoinComparable
            } else {
                ConditionCapability::Unsupported
            }
        }
        _ => ConditionCapability::Unsupported,
    }
}

/// if条件のパターン種別
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConditionPattern {
    /// 単純比較: var CmpOp literal (e.g., i > 0)
    SimpleComparison,
    /// 複雑条件: BinaryOp, MethodCall, etc.
    Complex,
}

/// if条件ASTを分析してパターンを判定
///
/// # Arguments
///
/// * `cond` - if条件のASTノード
///
/// # Returns
///
/// - `ConditionPattern::SimpleComparison` - 単純比較（AST-based lowerer で処理可能）
/// - `ConditionPattern::Complex` - 複雑条件（別経路へフォールバック）
///
/// # Examples
///
/// ```rust
/// // Simple: i > 0
/// let simple = ASTNode::BinaryOp {
///     operator: BinaryOperator::Greater,
///     left: Box::new(ASTNode::Variable { name: "i".to_string(), span: Span::unknown() }),
///     right: Box::new(ASTNode::Literal { value: LiteralValue::Integer(0), span: Span::unknown() }),
///     span: Span::unknown(),
/// };
/// assert_eq!(analyze_condition_pattern(&simple), ConditionPattern::SimpleComparison);
///
/// // Phase 242-EX-A: Now Simple: i % 2 == 1 (BinaryOp in LHS)
/// let complex = ASTNode::BinaryOp {
///     operator: BinaryOperator::Equal,
///     left: Box::new(ASTNode::BinaryOp { ... }), // BinaryOp in LHS
///     right: Box::new(ASTNode::Literal { value: LiteralValue::Integer(1), span: Span::unknown() }),
///     span: Span::unknown(),
/// };
/// assert_eq!(analyze_condition_pattern(&complex), ConditionPattern::SimpleComparison);
/// ```
pub fn analyze_condition_pattern(cond: &ASTNode) -> ConditionPattern {
    match cond {
        // Comparison operators: ==, !=, <, >, <=, >=
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } => {
            // Check if operator is a comparison
            let is_comparison = matches!(
                operator,
                BinaryOperator::Equal
                    | BinaryOperator::NotEqual
                    | BinaryOperator::Less
                    | BinaryOperator::Greater
                    | BinaryOperator::LessEqual
                    | BinaryOperator::GreaterEqual
            );

            if !is_comparison {
                // Not a comparison (e.g., And, Or) → Complex
                return ConditionPattern::Complex;
            }

            // Phase 242-EX-A: Accept any expr CmpOp expr pattern.
            // The IfPhiJoin-route lowerer (legacy file name contains if_sum,
            // traceability-only) handles BinaryOp via lower_value_expression.

            // Check LHS/RHS patterns
            let left_is_var = matches!(left.as_ref(), ASTNode::Variable { .. });
            let left_is_literal = matches!(left.as_ref(), ASTNode::Literal { .. });
            let left_is_binop = matches!(left.as_ref(), ASTNode::BinaryOp { .. });
            let right_is_var = matches!(right.as_ref(), ASTNode::Variable { .. });
            let right_is_literal = matches!(right.as_ref(), ASTNode::Literal { .. });
            let right_is_binop = matches!(right.as_ref(), ASTNode::BinaryOp { .. });

            // Phase 219: var CmpOp literal (e.g., i > 0)
            if left_is_var && right_is_literal {
                return ConditionPattern::SimpleComparison;
            }

            // Phase 222: literal CmpOp var (e.g., 0 < i)
            if left_is_literal && right_is_var {
                return ConditionPattern::SimpleComparison;
            }

            // Phase 222: var CmpOp var (e.g., i > j)
            if left_is_var && right_is_var {
                return ConditionPattern::SimpleComparison;
            }

            // Phase 242-EX-A: BinaryOp CmpOp literal (e.g., i % 2 == 1)
            if left_is_binop && right_is_literal {
                return ConditionPattern::SimpleComparison;
            }

            // Phase 242-EX-A: BinaryOp CmpOp var (e.g., i + j > k)
            if left_is_binop && right_is_var {
                return ConditionPattern::SimpleComparison;
            }

            // Phase 242-EX-A: var CmpOp BinaryOp (e.g., i > j + 1)
            if left_is_var && right_is_binop {
                return ConditionPattern::SimpleComparison;
            }

            // Phase 242-EX-A: literal CmpOp BinaryOp (e.g., 0 < i + 1)
            if left_is_literal && right_is_binop {
                return ConditionPattern::SimpleComparison;
            }

            // Phase 242-EX-A: BinaryOp CmpOp BinaryOp (e.g., a + b > c + d)
            if left_is_binop && right_is_binop {
                return ConditionPattern::SimpleComparison;
            }

            // Complex LHS/RHS (e.g., method_call() > 0) - MethodCall not yet supported
            ConditionPattern::Complex
        }
        // Any other node type → Complex
        _ => ConditionPattern::Complex,
    }
}

/// if条件が単純比較かどうか
///
/// # Arguments
///
/// * `cond` - if条件のASTノード
///
/// # Returns
///
/// `true` if 単純比較（AST-based lowerer で処理可能）
///
/// # Examples
///
/// ```rust
/// // i > 0 → true
/// assert!(is_simple_comparison(&simple_condition));
///
/// // i % 2 == 1 → true（Phase 242-EX-A で比較のオペランドに算術式を許可）
/// assert!(is_simple_comparison(&complex_condition));
/// ```
pub fn is_simple_comparison(cond: &ASTNode) -> bool {
    analyze_condition_pattern(cond) == ConditionPattern::SimpleComparison
}

// ============================================================================
// Phase 222: Condition Normalization
// ============================================================================

/// 条件式の右辺値（変数 or リテラル）
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConditionValue {
    /// 変数
    Variable(String),
    /// 整数リテラル
    Literal(i64),
}

/// 正規化された条件式
///
/// 常に左辺が変数の形に正規化される:
/// - `i > 0` → `i > 0` (そのまま)
/// - `0 < i` → `i > 0` (左右反転)
/// - `i > j` → `i > j` (変数同士、そのまま)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedCondition {
    /// 左辺変数名
    pub left_var: String,
    /// 比較演算子
    pub op: CompareOp,
    /// 右辺（変数 or リテラル）
    pub right: ConditionValue,
}

/// 比較演算子を左右反転
///
/// 左辺がリテラル、右辺が変数の場合に使用。
///
/// # Examples
///
/// ```
/// // 0 < i → i > 0
/// assert_eq!(flip_compare_op(CompareOp::Lt), CompareOp::Gt);
///
/// // len > i → i < len
/// assert_eq!(flip_compare_op(CompareOp::Gt), CompareOp::Lt);
///
/// // 5 == i → i == 5 (不変)
/// assert_eq!(flip_compare_op(CompareOp::Eq), CompareOp::Eq);
/// ```
fn flip_compare_op(op: CompareOp) -> CompareOp {
    match op {
        CompareOp::Lt => CompareOp::Gt, // < → >
        CompareOp::Gt => CompareOp::Lt, // > → <
        CompareOp::Le => CompareOp::Ge, // <= → >=
        CompareOp::Ge => CompareOp::Le, // >= → <=
        CompareOp::Eq => CompareOp::Eq, // == → == (不変)
        CompareOp::Ne => CompareOp::Ne, // != → != (不変)
    }
}

/// BinaryOperator を CompareOp に変換
fn binary_op_to_compare_op(op: &BinaryOperator) -> Option<CompareOp> {
    match op {
        BinaryOperator::Less => Some(CompareOp::Lt),
        BinaryOperator::Greater => Some(CompareOp::Gt),
        BinaryOperator::LessEqual => Some(CompareOp::Le),
        BinaryOperator::GreaterEqual => Some(CompareOp::Ge),
        BinaryOperator::Equal => Some(CompareOp::Eq),
        BinaryOperator::NotEqual => Some(CompareOp::Ne),
        _ => None,
    }
}

/// 条件式を正規化（左辺=変数 の形に統一）
///
/// # Arguments
///
/// * `cond` - 条件式ASTノード
///
/// # Returns
///
/// - `Some(NormalizedCondition)` - 正規化成功
/// - `None` - 正規化失敗（複雑条件 or サポート外）
///
/// # Examples
///
/// ```rust
/// // i > 0 → NormalizedCondition { left_var: "i", op: Gt, right: Literal(0) }
/// let norm = normalize_comparison(&i_gt_0).unwrap();
/// assert_eq!(norm.left_var, "i");
/// assert_eq!(norm.op, CompareOp::Gt);
/// assert_eq!(norm.right, ConditionValue::Literal(0));
///
/// // 0 < i → NormalizedCondition { left_var: "i", op: Gt, right: Literal(0) }
/// let norm = normalize_comparison(&zero_lt_i).unwrap();
/// assert_eq!(norm.left_var, "i");
/// assert_eq!(norm.op, CompareOp::Gt); // 左右反転により Gt
///
/// // i > j → NormalizedCondition { left_var: "i", op: Gt, right: Variable("j") }
/// let norm = normalize_comparison(&i_gt_j).unwrap();
/// assert_eq!(norm.right, ConditionValue::Variable("j".to_string()));
/// ```
pub fn normalize_comparison(cond: &ASTNode) -> Option<NormalizedCondition> {
    match cond {
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } => {
            // Comparison operator のみ受理
            let compare_op = binary_op_to_compare_op(operator)?;

            // Case 1: var CmpOp literal (e.g., i > 0)
            if let (
                ASTNode::Variable { name: left_var, .. },
                ASTNode::Literal {
                    value: LiteralValue::Integer(right_val),
                    ..
                },
            ) = (left.as_ref(), right.as_ref())
            {
                return Some(NormalizedCondition {
                    left_var: left_var.clone(),
                    op: compare_op,
                    right: ConditionValue::Literal(*right_val),
                });
            }

            // Case 2: literal CmpOp var (e.g., 0 < i) → 左右反転
            if let (
                ASTNode::Literal {
                    value: LiteralValue::Integer(left_val),
                    ..
                },
                ASTNode::Variable {
                    name: right_var, ..
                },
            ) = (left.as_ref(), right.as_ref())
            {
                return Some(NormalizedCondition {
                    left_var: right_var.clone(),
                    op: flip_compare_op(compare_op), // 演算子を反転
                    right: ConditionValue::Literal(*left_val),
                });
            }

            // Case 3: var CmpOp var (e.g., i > j)
            if let (
                ASTNode::Variable { name: left_var, .. },
                ASTNode::Variable {
                    name: right_var, ..
                },
            ) = (left.as_ref(), right.as_ref())
            {
                return Some(NormalizedCondition {
                    left_var: left_var.clone(),
                    op: compare_op,
                    right: ConditionValue::Variable(right_var.clone()),
                });
            }

            // その他（BinaryOp, MethodCall等）→ 正規化失敗
            None
        }
        _ => None, // 非 BinaryOp → 正規化失敗
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{LiteralValue, Span};

    // Helper: Create a simple variable node
    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    // Helper: Create an integer literal node
    fn int_lit(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    // Helper: Create a BinaryOp node
    fn binop(op: BinaryOperator, left: ASTNode, right: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator: op,
            left: Box::new(left),
            right: Box::new(right),
            span: Span::unknown(),
        }
    }

    #[test]
    fn test_simple_comparison_greater() {
        // i > 0
        let cond = binop(BinaryOperator::Greater, var("i"), int_lit(0));
        assert_eq!(
            analyze_condition_pattern(&cond),
            ConditionPattern::SimpleComparison
        );
        assert!(is_simple_comparison(&cond));
    }

    #[test]
    fn test_simple_comparison_less() {
        // i < 10
        let cond = binop(BinaryOperator::Less, var("i"), int_lit(10));
        assert_eq!(
            analyze_condition_pattern(&cond),
            ConditionPattern::SimpleComparison
        );
        assert!(is_simple_comparison(&cond));
    }

    #[test]
    fn test_simple_comparison_equal() {
        // i == 5
        let cond = binop(BinaryOperator::Equal, var("i"), int_lit(5));
        assert_eq!(
            analyze_condition_pattern(&cond),
            ConditionPattern::SimpleComparison
        );
        assert!(is_simple_comparison(&cond));
    }

    #[test]
    fn test_simple_comparison_not_equal() {
        // i != 0
        let cond = binop(BinaryOperator::NotEqual, var("i"), int_lit(0));
        assert_eq!(
            analyze_condition_pattern(&cond),
            ConditionPattern::SimpleComparison
        );
        assert!(is_simple_comparison(&cond));
    }

    #[test]
    fn test_simple_binop_in_lhs() {
        // Phase 242-EX-A: i % 2 == 1 (BinaryOp in LHS) is now SimpleComparison
        let lhs = binop(BinaryOperator::Modulo, var("i"), int_lit(2));
        let cond = binop(BinaryOperator::Equal, lhs, int_lit(1));
        assert_eq!(
            analyze_condition_pattern(&cond),
            ConditionPattern::SimpleComparison
        );
        assert!(is_simple_comparison(&cond));
    }

    #[test]
    fn test_simple_binop_in_rhs() {
        // Phase 242-EX-A: i == a + b (BinaryOp in RHS) is now SimpleComparison
        let rhs = binop(BinaryOperator::Add, var("a"), var("b"));
        let cond = binop(BinaryOperator::Equal, var("i"), rhs);
        assert_eq!(
            analyze_condition_pattern(&cond),
            ConditionPattern::SimpleComparison
        );
        assert!(is_simple_comparison(&cond));
    }

    #[test]
    fn if_phi_join_and_loop_continue_only_cond_i_mod_2_eq_1_is_recognized() {
        // IfPhiJoin / LoopContinueOnly で使う典型的なフィルタ条件: i % 2 == 1
        let lhs = binop(BinaryOperator::Modulo, var("i"), int_lit(2));
        let cond = binop(BinaryOperator::Equal, lhs, int_lit(1));
        assert_eq!(
            analyze_condition_pattern(&cond),
            ConditionPattern::SimpleComparison
        );
        assert!(is_simple_comparison(&cond));
    }

    #[test]
    fn test_complex_logical_and() {
        // a && b (logical And)
        let cond = binop(BinaryOperator::And, var("a"), var("b"));
        assert_eq!(analyze_condition_pattern(&cond), ConditionPattern::Complex);
        assert!(!is_simple_comparison(&cond));
    }

    #[test]
    fn test_complex_logical_or() {
        // a || b (logical Or)
        let cond = binop(BinaryOperator::Or, var("a"), var("b"));
        assert_eq!(analyze_condition_pattern(&cond), ConditionPattern::Complex);
        assert!(!is_simple_comparison(&cond));
    }

    #[test]
    fn test_complex_method_call() {
        // method_call() > 0 (MethodCall in LHS)
        let method_call = ASTNode::MethodCall {
            object: Box::new(var("obj")),
            method: "get".to_string(),
            arguments: vec![],
            span: Span::unknown(),
        };
        let cond = binop(BinaryOperator::Greater, method_call, int_lit(0));
        assert_eq!(analyze_condition_pattern(&cond), ConditionPattern::Complex);
        assert!(!is_simple_comparison(&cond));
    }

    #[test]
    fn unsupported_nested_mod_condition_is_rejected() {
        // (i % 2 == 1) && (j > 0) → 複合条件として Complex 扱い
        let mod_eq = {
            let lhs = binop(BinaryOperator::Modulo, var("i"), int_lit(2));
            binop(BinaryOperator::Equal, lhs, int_lit(1))
        };
        let gt_zero = binop(BinaryOperator::Greater, var("j"), int_lit(0));
        let cond = binop(BinaryOperator::And, mod_eq, gt_zero);
        assert_eq!(analyze_condition_pattern(&cond), ConditionPattern::Complex);
        assert!(!is_simple_comparison(&cond));
    }

    #[test]
    fn test_complex_non_binary_op() {
        // Just a variable (not a comparison)
        let cond = var("condition");
        assert_eq!(analyze_condition_pattern(&cond), ConditionPattern::Complex);
        assert!(!is_simple_comparison(&cond));
    }

    // ========================================================================
    // Phase 222: Normalization Tests
    // ========================================================================

    #[test]
    fn test_normalize_var_cmp_literal() {
        // i > 0 → そのまま
        let cond = binop(BinaryOperator::Greater, var("i"), int_lit(0));
        let norm = normalize_comparison(&cond).unwrap();
        assert_eq!(norm.left_var, "i");
        assert_eq!(norm.op, CompareOp::Gt);
        assert_eq!(norm.right, ConditionValue::Literal(0));
    }

    #[test]
    fn test_normalize_literal_cmp_var() {
        // 0 < i → i > 0 (左右反転)
        let cond = binop(BinaryOperator::Less, int_lit(0), var("i"));
        let norm = normalize_comparison(&cond).unwrap();
        assert_eq!(norm.left_var, "i");
        assert_eq!(norm.op, CompareOp::Gt); // 反転により Gt
        assert_eq!(norm.right, ConditionValue::Literal(0));
    }

    #[test]
    fn test_normalize_literal_gt_var() {
        // len > i → i < len (左右反転)
        let cond = binop(BinaryOperator::Greater, int_lit(10), var("i"));
        let norm = normalize_comparison(&cond).unwrap();
        assert_eq!(norm.left_var, "i");
        assert_eq!(norm.op, CompareOp::Lt); // 反転により Lt
        assert_eq!(norm.right, ConditionValue::Literal(10));
    }

    #[test]
    fn test_normalize_literal_eq_var() {
        // 5 == i → i == 5 (反転だが == は不変)
        let cond = binop(BinaryOperator::Equal, int_lit(5), var("i"));
        let norm = normalize_comparison(&cond).unwrap();
        assert_eq!(norm.left_var, "i");
        assert_eq!(norm.op, CompareOp::Eq); // == は不変
        assert_eq!(norm.right, ConditionValue::Literal(5));
    }

    #[test]
    fn test_normalize_var_cmp_var() {
        // i > j → そのまま（変数同士）
        let cond = binop(BinaryOperator::Greater, var("i"), var("j"));
        let norm = normalize_comparison(&cond).unwrap();
        assert_eq!(norm.left_var, "i");
        assert_eq!(norm.op, CompareOp::Gt);
        assert_eq!(norm.right, ConditionValue::Variable("j".to_string()));
    }

    #[test]
    fn test_normalize_var_lt_var() {
        // i < end → そのまま（変数同士）
        let cond = binop(BinaryOperator::Less, var("i"), var("end"));
        let norm = normalize_comparison(&cond).unwrap();
        assert_eq!(norm.left_var, "i");
        assert_eq!(norm.op, CompareOp::Lt);
        assert_eq!(norm.right, ConditionValue::Variable("end".to_string()));
    }

    #[test]
    fn test_normalize_fails_on_binop() {
        // Phase 242-EX-A: i % 2 == 1 → 正規化失敗（BinaryOp in LHS）
        // Note: This is OK - normalization is only for simple cases.
        // The IfPhiJoin-route lowerer will use lower_value_expression() instead.
        let lhs = binop(BinaryOperator::Modulo, var("i"), int_lit(2));
        let cond = binop(BinaryOperator::Equal, lhs, int_lit(1));
        assert_eq!(normalize_comparison(&cond), None);
    }

    #[test]
    fn test_analyze_route_literal_cmp_var() {
        // Phase 222: 0 < i → SimpleComparison
        let cond = binop(BinaryOperator::Less, int_lit(0), var("i"));
        assert_eq!(
            analyze_condition_pattern(&cond),
            ConditionPattern::SimpleComparison
        );
        assert!(is_simple_comparison(&cond));
    }

    #[test]
    fn test_analyze_route_var_cmp_var() {
        // Phase 222: i > j → SimpleComparison
        let cond = binop(BinaryOperator::Greater, var("i"), var("j"));
        assert_eq!(
            analyze_condition_pattern(&cond),
            ConditionPattern::SimpleComparison
        );
        assert!(is_simple_comparison(&cond));
    }

    // ========================================================================
    // ConditionCapability (routing) Tests
    // ========================================================================

    #[test]
    fn test_capability_if_phi_join_comparable_simple() {
        let cond = binop(BinaryOperator::Greater, var("i"), int_lit(0));
        assert_eq!(
            analyze_condition_capability(&cond),
            ConditionCapability::IfPhiJoinComparable
        );
    }

    #[test]
    fn test_capability_if_phi_join_comparable_binop_operand() {
        let lhs = binop(BinaryOperator::Modulo, var("i"), int_lit(2));
        let cond = binop(BinaryOperator::Equal, lhs, int_lit(1));
        assert_eq!(
            analyze_condition_capability(&cond),
            ConditionCapability::IfPhiJoinComparable
        );
    }

    #[test]
    fn test_capability_rejects_logical_and() {
        let cond = binop(BinaryOperator::And, var("a"), var("b"));
        assert_eq!(
            analyze_condition_capability(&cond),
            ConditionCapability::Unsupported
        );
    }

    #[test]
    fn test_capability_rejects_method_call_operand() {
        let method_call = ASTNode::MethodCall {
            object: Box::new(var("obj")),
            method: "get".to_string(),
            arguments: vec![],
            span: Span::unknown(),
        };
        let cond = binop(BinaryOperator::Greater, method_call, int_lit(0));
        assert_eq!(
            analyze_condition_capability(&cond),
            ConditionCapability::Unsupported
        );
    }
}
