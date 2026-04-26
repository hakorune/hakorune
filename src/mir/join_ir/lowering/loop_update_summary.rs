//! Phase 170-C-2: LoopUpdateSummary - ループ更新パターン解析
//!
//! キャリア変数の更新パターン（CounterLike / AccumulationLike）を判定し、
//! CaseALoweringShape の検出精度を向上させる。
//!
//! ## 設計思想
//!
//! - 責務: AST のループ body から「各キャリアがどう更新されているか」を判定
//! - 差し替え可能: AST 解析 → MIR 解析と段階的に精度向上
//! - LoopFeatures / CaseALoweringShape から独立したモジュール
//! - No body observation means no update summary; carrier names alone are not
//!   update-kind proof.
//!
//! ## 使用例
//!
//! ```ignore
//! let summary = analyze_loop_updates_from_ast(&carrier_names, loop_body);
//! if summary.has_single_counter() {
//!     // StringExamination 系ルート候補
//! }
//! ```

/// キャリア変数の更新パターン
///
/// Phase 170-C-2: 3種類のパターンを区別
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateKind {
    /// カウンタ系: i = i + 1, i = i - 1, i += 1
    ///
    /// 典型的な skip/trim パターン。進捗変数として使われる。
    CounterLike,

    /// 蓄積系: result = result + x, arr.push(x), list.append(x)
    ///
    /// 典型的な collect/filter パターン。結果を蓄積する変数。
    AccumulationLike,

    /// 判定不能
    ///
    /// 複雑な更新パターン、または解析できなかった場合。
    Other,
}

impl UpdateKind {
    /// デバッグ用の名前を返す
    pub fn name(&self) -> &'static str {
        match self {
            UpdateKind::CounterLike => "CounterLike",
            UpdateKind::AccumulationLike => "AccumulationLike",
            UpdateKind::Other => "Other",
        }
    }
}

/// 単一キャリアの更新情報
#[derive(Debug, Clone)]
pub struct CarrierUpdateInfo {
    /// キャリア変数名
    pub name: String,

    /// 更新パターン
    pub kind: UpdateKind,

    /// Phase 213: Then branch update expression (for IfPhiJoin route)
    /// (legacy "if-sum" wording is traceability-only)
    /// e.g., for "if (cond) { sum = sum + 1 }", then_expr is "sum + 1"
    #[allow(dead_code)]
    pub then_expr: Option<crate::ast::ASTNode>,

    /// Phase 213: Else branch update expression (for IfPhiJoin route)
    /// (legacy "if-sum" wording is traceability-only)
    /// e.g., for "else { sum = sum + 0 }", else_expr is "sum + 0"
    /// If no else branch, this can be the identity update (e.g., "sum")
    #[allow(dead_code)]
    pub else_expr: Option<crate::ast::ASTNode>,
}

/// ループ全体の更新サマリ
///
/// Phase 170-C-2: CaseALoweringShape の判定入力として使用
#[derive(Debug, Clone, Default)]
pub struct LoopUpdateSummary {
    /// 各キャリアの更新情報
    pub carriers: Vec<CarrierUpdateInfo>,
}

impl LoopUpdateSummary {
    /// 空のサマリを作成
    pub fn empty() -> Self {
        Self { carriers: vec![] }
    }

    /// 単一 CounterLike キャリアを持つか
    ///
    /// StringExamination パターンの判定に使用
    pub fn has_single_counter(&self) -> bool {
        self.carriers.len() == 1 && self.carriers[0].kind == UpdateKind::CounterLike
    }

    /// AccumulationLike キャリアを含むか
    ///
    /// ArrayAccumulation パターンの判定に使用
    pub fn has_accumulation(&self) -> bool {
        self.carriers
            .iter()
            .any(|c| c.kind == UpdateKind::AccumulationLike)
    }

    /// CounterLike キャリアの数
    pub fn counter_count(&self) -> usize {
        self.carriers
            .iter()
            .filter(|c| c.kind == UpdateKind::CounterLike)
            .count()
    }

    /// AccumulationLike キャリアの数
    pub fn accumulation_count(&self) -> usize {
        self.carriers
            .iter()
            .filter(|c| c.kind == UpdateKind::AccumulationLike)
            .count()
    }

    /// Phase 213: Check if this matches the minimal IfPhiJoin signature
    /// (API name keeps legacy "if_sum" for compatibility)
    ///
    /// Minimal IfPhiJoin signature:
    /// - Has exactly 1 CounterLike carrier (loop index, e.g., "i")
    /// - Has exactly 1 AccumulationLike carrier (accumulator, e.g., "sum")
    /// - Optionally has additional accumulators (e.g., "count")
    ///
    /// Examples:
    /// - `loop(i < len) { if cond { sum = sum + 1 } i = i + 1 }` ✅
    /// - `loop(i < len) { if cond { sum = sum + 1; count = count + 1 } i = i + 1 }` ✅
    /// - `loop(i < len) { result = result + data[i]; i = i + 1 }` ❌ (no if statement)
    pub fn is_simple_if_sum_pattern(&self) -> bool {
        let counter_count = self.counter_count();
        let accumulation_count = self.accumulation_count();

        // Must have exactly 1 counter (loop index)
        if counter_count != 1 {
            return false;
        }
        // Must have at least 1 accumulator (sum)
        if accumulation_count < 1 {
            return false;
        }
        // For now, only support up to 2 accumulators (sum, count)
        // This matches the Phase 212 IfPhiJoin-minimal fixture shape
        if accumulation_count > 2 {
            return false;
        }

        true
    }
}

/// Phase 219: Classify update kind from RHS expression structure
///
/// Returns UpdateKind based on RHS pattern and self-reference to the assigned carrier.
fn classify_update_kind_from_rhs(var_name: &str, rhs: &crate::ast::ASTNode) -> UpdateKind {
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue};

    match rhs {
        // x = x + 1 → CounterLike
        // x = x + n → AccumulationLike (where n is not 1)
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } => {
            if matches!(operator, BinaryOperator::Add) {
                let is_self_reference =
                    matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == var_name);
                if !is_self_reference {
                    return UpdateKind::Other;
                }

                // Check right operand
                if let ASTNode::Literal { value, .. } = right.as_ref() {
                    if let LiteralValue::Integer(n) = value {
                        if *n == 1 {
                            return UpdateKind::CounterLike; // x = x + 1
                        } else {
                            return UpdateKind::AccumulationLike; // x = x + n
                        }
                    }
                } else {
                    // x = x + expr (variable accumulation)
                    return UpdateKind::AccumulationLike;
                }
            }
            UpdateKind::Other
        }
        _ => UpdateKind::Other,
    }
}

/// Phase 219: Analyze loop updates from loop body AST (assignment-based)
///
/// # New Design (Phase 219)
///
/// - Takes loop body AST as input (not just carrier names)
/// - Only analyzes variables that are ASSIGNED in loop body
/// - Uses RHS structure analysis (NOT name heuristics)
///
/// # Arguments
///
/// * `carrier_names` - Candidate carrier variable names from scope
/// * `loop_body` - Loop body AST for assignment detection
///
/// # Returns
///
/// LoopUpdateSummary with only actually-assigned carriers
/// Phase 219: Extract assignment RHS candidates for a given variable
///
/// Returns every current-loop RHS expression assigning to `var_name`.
fn collect_assignment_rhses<'a>(
    var_name: &str,
    loop_body: &'a [crate::ast::ASTNode],
) -> Vec<&'a crate::ast::ASTNode> {
    use crate::ast::ASTNode;

    fn visit_node<'a>(var_name: &str, node: &'a ASTNode, rhses: &mut Vec<&'a ASTNode>) {
        match node {
            ASTNode::Assignment { target, value, .. } => {
                if let ASTNode::Variable { name, .. } = target.as_ref() {
                    if name == var_name {
                        rhses.push(value.as_ref());
                        return;
                    }
                }
                // Recurse into value for nested assignment expressions that are
                // not already the carrier assignment itself.
                visit_node(var_name, value, rhses);
            }
            ASTNode::If {
                then_body,
                else_body,
                ..
            } => {
                for stmt in then_body {
                    visit_node(var_name, stmt, rhses);
                }
                if let Some(else_stmts) = else_body {
                    for stmt in else_stmts {
                        visit_node(var_name, stmt, rhses);
                    }
                }
            }
            // Nested loops are separate update scopes. Do not use their
            // assignments as proof for the current loop body.
            ASTNode::Loop { .. } => {}
            _ => {}
        }
    }

    let mut rhses = Vec::new();
    for stmt in loop_body {
        visit_node(var_name, stmt, &mut rhses);
    }
    rhses
}

/// Phase 219: Check if variable name looks like loop index
///
/// Simple heuristic: single-letter names (i, j, k, e) or "index"/"idx"
fn is_likely_loop_index(name: &str) -> bool {
    matches!(name, "i" | "j" | "k" | "e" | "idx" | "index" | "pos" | "n")
}

fn disambiguate_update_kind(var_name: &str, kind: UpdateKind) -> UpdateKind {
    match kind {
        UpdateKind::CounterLike if is_likely_loop_index(var_name) => UpdateKind::CounterLike,
        UpdateKind::CounterLike => UpdateKind::AccumulationLike,
        other => other,
    }
}

fn classify_update_kind_from_rhses(
    var_name: &str,
    rhses: &[&crate::ast::ASTNode],
) -> Option<UpdateKind> {
    let mut agreed = None;

    for rhs in rhses {
        let kind = disambiguate_update_kind(var_name, classify_update_kind_from_rhs(var_name, rhs));

        if kind == UpdateKind::Other {
            return Some(UpdateKind::Other);
        }

        match agreed {
            None => agreed = Some(kind),
            Some(previous) if previous == kind => {}
            Some(_) => return Some(UpdateKind::Other),
        }
    }

    agreed
}

pub fn analyze_loop_updates_from_ast(
    carrier_names: &[String],
    loop_body: &[crate::ast::ASTNode],
) -> LoopUpdateSummary {
    // Phase 219-2: Filter carriers to only assigned ones and classify by all
    // current-loop RHS candidates.
    let mut carriers = Vec::new();
    for name in carrier_names {
        let rhses = collect_assignment_rhses(name, loop_body);
        if let Some(kind) = classify_update_kind_from_rhses(name, &rhses) {
            // Phase 219-3: Classify by RHS/self-reference first.
            // Name is only a tie-breaker for the proven `x = x + 1` shape:
            // - likely loop index names (i, j, k) -> CounterLike
            // - other names -> AccumulationLike
            // Multiple RHS candidates must agree after this tie-breaker.

            carriers.push(CarrierUpdateInfo {
                name: name.clone(),
                kind,
                then_expr: None,
                else_expr: None,
            });
        }
    }

    LoopUpdateSummary { carriers }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};

    #[test]
    fn test_update_kind_name() {
        assert_eq!(UpdateKind::CounterLike.name(), "CounterLike");
        assert_eq!(UpdateKind::AccumulationLike.name(), "AccumulationLike");
        assert_eq!(UpdateKind::Other.name(), "Other");
    }

    fn span() -> Span {
        Span::unknown()
    }

    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: span(),
        }
    }

    fn lit_i(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: span(),
        }
    }

    fn add(lhs: ASTNode, rhs: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(lhs),
            right: Box::new(rhs),
            span: span(),
        }
    }

    fn assign(name: &str, value: ASTNode) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(var(name)),
            value: Box::new(value),
            span: span(),
        }
    }

    fn if_with_updates(
        condition: ASTNode,
        then_body: Vec<ASTNode>,
        else_body: Option<Vec<ASTNode>>,
    ) -> ASTNode {
        ASTNode::If {
            condition: Box::new(condition),
            then_body,
            else_body,
            span: span(),
        }
    }

    fn loop_with_body(body: Vec<ASTNode>) -> ASTNode {
        ASTNode::Loop {
            condition: Box::new(var("cond")),
            body,
            span: span(),
        }
    }

    #[test]
    fn test_analyze_single_counter_from_ast() {
        let names = vec!["i".to_string()];
        let loop_body = vec![assign("i", add(var("i"), lit_i(1)))];

        let summary = analyze_loop_updates_from_ast(&names, &loop_body);

        assert!(summary.has_single_counter());
        assert!(!summary.has_accumulation());
        assert_eq!(summary.counter_count(), 1);
        assert_eq!(summary.accumulation_count(), 0);
    }

    #[test]
    fn test_analyze_accumulation_from_ast() {
        let names = vec!["sum".to_string()];
        let loop_body = vec![assign("sum", add(var("sum"), lit_i(1)))];

        let summary = analyze_loop_updates_from_ast(&names, &loop_body);

        assert!(!summary.has_single_counter());
        assert!(summary.has_accumulation());
        assert_eq!(summary.counter_count(), 0);
        assert_eq!(summary.accumulation_count(), 1);
    }

    #[test]
    fn test_analyze_mixed_from_ast() {
        let names = vec!["i".to_string(), "sum".to_string()];
        let loop_body = vec![
            assign("sum", add(var("sum"), var("i"))),
            assign("i", add(var("i"), lit_i(1))),
        ];

        let summary = analyze_loop_updates_from_ast(&names, &loop_body);

        assert!(!summary.has_single_counter());
        assert!(summary.has_accumulation());
        assert_eq!(summary.counter_count(), 1);
        assert_eq!(summary.accumulation_count(), 1);
    }

    #[test]
    fn test_is_if_phi_join_signature_basic_ast() {
        let names = vec!["i".to_string(), "sum".to_string()];
        let loop_body = vec![
            if_with_updates(
                var("cond"),
                vec![assign("sum", add(var("sum"), var("i")))],
                None,
            ),
            assign("i", add(var("i"), lit_i(1))),
        ];

        let summary = analyze_loop_updates_from_ast(&names, &loop_body);

        assert!(summary.is_simple_if_sum_pattern());
        assert_eq!(summary.counter_count(), 1);
        assert_eq!(summary.accumulation_count(), 1);
    }

    #[test]
    fn test_is_if_phi_join_signature_with_count_ast() {
        let names = vec!["i".to_string(), "sum".to_string(), "count".to_string()];
        let loop_body = vec![
            if_with_updates(
                var("cond"),
                vec![
                    assign("sum", add(var("sum"), var("i"))),
                    assign("count", add(var("count"), lit_i(1))),
                ],
                None,
            ),
            assign("i", add(var("i"), lit_i(1))),
        ];

        let summary = analyze_loop_updates_from_ast(&names, &loop_body);

        assert!(summary.is_simple_if_sum_pattern());
        assert_eq!(summary.counter_count(), 1);
        assert_eq!(summary.accumulation_count(), 2);
    }

    #[test]
    fn test_is_if_phi_join_signature_no_accumulator_ast() {
        let names = vec!["i".to_string()];
        let loop_body = vec![assign("i", add(var("i"), lit_i(1)))];

        let summary = analyze_loop_updates_from_ast(&names, &loop_body);

        assert!(!summary.is_simple_if_sum_pattern());
        assert_eq!(summary.counter_count(), 1);
        assert_eq!(summary.accumulation_count(), 0);
    }

    #[test]
    fn test_is_if_phi_join_signature_no_counter_ast() {
        let names = vec!["sum".to_string()];
        let loop_body = vec![assign("sum", add(var("sum"), lit_i(1)))];

        let summary = analyze_loop_updates_from_ast(&names, &loop_body);

        assert!(!summary.is_simple_if_sum_pattern());
        assert_eq!(summary.counter_count(), 0);
        assert_eq!(summary.accumulation_count(), 1);
    }

    #[test]
    fn test_is_if_phi_join_signature_multiple_counters_ast() {
        let names = vec!["i".to_string(), "j".to_string(), "sum".to_string()];
        let loop_body = vec![
            assign("i", add(var("i"), lit_i(1))),
            assign("j", add(var("j"), lit_i(1))),
            assign("sum", add(var("sum"), var("i"))),
        ];

        let summary = analyze_loop_updates_from_ast(&names, &loop_body);

        assert!(!summary.is_simple_if_sum_pattern());
        assert_eq!(summary.counter_count(), 2);
        assert_eq!(summary.accumulation_count(), 1);
    }

    #[test]
    fn loop_update_rhs_first_index_name_requires_self_increment() {
        let names = vec!["i".to_string()];
        let loop_body = vec![assign("i", lit_i(0))];

        let summary = analyze_loop_updates_from_ast(&names, &loop_body);

        assert_eq!(summary.counter_count(), 0);
        assert_eq!(summary.accumulation_count(), 0);
        assert_eq!(summary.carriers[0].kind, UpdateKind::Other);
    }

    #[test]
    fn loop_update_rhs_first_rejects_non_self_reference() {
        let names = vec!["i".to_string()];
        let loop_body = vec![assign("i", add(var("j"), lit_i(1)))];

        let summary = analyze_loop_updates_from_ast(&names, &loop_body);

        assert_eq!(summary.counter_count(), 0);
        assert_eq!(summary.accumulation_count(), 0);
        assert_eq!(summary.carriers[0].kind, UpdateKind::Other);
    }

    #[test]
    fn loop_update_rhs_first_self_plus_one_uses_name_only_as_tiebreaker() {
        let names = vec!["i".to_string(), "sum".to_string()];
        let loop_body = vec![
            assign("i", add(var("i"), lit_i(1))),
            assign("sum", add(var("sum"), lit_i(1))),
        ];

        let summary = analyze_loop_updates_from_ast(&names, &loop_body);

        assert_eq!(summary.counter_count(), 1);
        assert_eq!(summary.accumulation_count(), 1);
    }

    #[test]
    fn loop_update_nested_scope_ignores_nested_loop_assignment() {
        let names = vec!["i".to_string()];
        let loop_body = vec![loop_with_body(vec![assign("i", add(var("i"), lit_i(1)))])];

        let summary = analyze_loop_updates_from_ast(&names, &loop_body);

        assert!(summary.carriers.is_empty());
        assert_eq!(summary.counter_count(), 0);
        assert_eq!(summary.accumulation_count(), 0);
    }

    #[test]
    fn loop_update_nested_scope_keeps_current_if_branch_assignment() {
        let names = vec!["i".to_string()];
        let loop_body = vec![if_with_updates(
            var("cond"),
            vec![assign("i", add(var("i"), lit_i(1)))],
            None,
        )];

        let summary = analyze_loop_updates_from_ast(&names, &loop_body);

        assert_eq!(summary.counter_count(), 1);
        assert_eq!(summary.accumulation_count(), 0);
    }

    #[test]
    fn loop_update_multi_assignment_rejects_conflicting_updates() {
        let names = vec!["i".to_string()];
        let loop_body = vec![assign("i", lit_i(0)), assign("i", add(var("i"), lit_i(1)))];

        let summary = analyze_loop_updates_from_ast(&names, &loop_body);

        assert_eq!(summary.carriers.len(), 1);
        assert_eq!(summary.carriers[0].kind, UpdateKind::Other);
        assert_eq!(summary.counter_count(), 0);
        assert_eq!(summary.accumulation_count(), 0);
    }

    #[test]
    fn loop_update_multi_assignment_rejects_mixed_update_kinds() {
        let names = vec!["i".to_string()];
        let loop_body = vec![
            assign("i", add(var("i"), lit_i(1))),
            assign("i", add(var("i"), lit_i(2))),
        ];

        let summary = analyze_loop_updates_from_ast(&names, &loop_body);

        assert_eq!(summary.carriers.len(), 1);
        assert_eq!(summary.carriers[0].kind, UpdateKind::Other);
        assert_eq!(summary.counter_count(), 0);
        assert_eq!(summary.accumulation_count(), 0);
    }

    #[test]
    fn loop_update_multi_assignment_accepts_agreeing_if_branches() {
        let names = vec!["sum".to_string()];
        let loop_body = vec![if_with_updates(
            var("cond"),
            vec![assign("sum", add(var("sum"), lit_i(1)))],
            Some(vec![assign("sum", add(var("sum"), lit_i(2)))]),
        )];

        let summary = analyze_loop_updates_from_ast(&names, &loop_body);

        assert_eq!(summary.counter_count(), 0);
        assert_eq!(summary.accumulation_count(), 1);
    }
}
