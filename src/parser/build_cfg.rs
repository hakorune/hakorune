//! Parser support for AST-level build conditionals.
//!
//! `gate` is intentionally parser-contextual instead of a tokenizer keyword so
//! existing source may keep ordinary identifiers named `gate`.

mod predicate;
mod prune;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildGateExplainReport {
    pub output_contract: &'static str,
    pub conditional_group_count: usize,
    pub active_branch_count: usize,
    pub inactive_branch_count: usize,
    pub inactive_branch_mir_count: usize,
}

impl BuildGateExplainReport {
    pub const OUTPUT_CONTRACT: &'static str = "hakorune-build-cfg-explain-v0";

    pub fn new() -> Self {
        Self {
            output_contract: Self::OUTPUT_CONTRACT,
            conditional_group_count: 0,
            active_branch_count: 0,
            inactive_branch_count: 0,
            inactive_branch_mir_count: 0,
        }
    }

    pub fn to_kv_lines(&self) -> Vec<String> {
        vec![
            format!("output_contract={}", self.output_contract),
            format!("conditional_group_count={}", self.conditional_group_count),
            format!("active_branch_count={}", self.active_branch_count),
            format!("inactive_branch_count={}", self.inactive_branch_count),
            format!(
                "inactive_branch_mir_count={}",
                self.inactive_branch_mir_count
            ),
            "summary=ok".to_string(),
        ]
    }
}
