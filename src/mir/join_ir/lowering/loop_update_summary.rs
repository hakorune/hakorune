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

mod assignment_scan;
mod rhs_classification;

#[cfg(test)]
mod tests;

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
pub fn analyze_loop_updates_from_ast(
    carrier_names: &[String],
    loop_body: &[crate::ast::ASTNode],
) -> LoopUpdateSummary {
    // Phase 219-2: Filter carriers to only assigned ones and classify by all
    // current-loop RHS candidates.
    let mut carriers = Vec::new();
    for name in carrier_names {
        let rhses = assignment_scan::collect_assignment_rhses(name, loop_body);
        if let Some(kind) = rhs_classification::classify_update_kind_from_rhses(name, &rhses) {
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
