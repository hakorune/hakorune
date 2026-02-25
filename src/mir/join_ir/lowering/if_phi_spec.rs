//! Phase 61-2: JoinIR経路でのPHI仕様計算
//!
//! JoinInst（Select/IfMerge）から、どの変数がPHIを必要とするかを計算する。

use crate::mir::join_ir::lowering::if_phi_context::IfPhiContext;
use crate::mir::join_ir::JoinInst;
use crate::runtime::get_global_ring0;
// Phase 61-6.2: ValueId, BTreeMap 削除（A/B観察関数削除で不要に）
use std::collections::BTreeSet;

/// PHI仕様（どの変数がPHIを持つべきか）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhiSpec {
    /// Header PHI候補（ループキャリア変数）
    pub header_phis: BTreeSet<String>,

    /// Exit PHI候補（ループ脱出時の値）
    pub exit_phis: BTreeSet<String>,
}

impl PhiSpec {
    pub fn new() -> Self {
        Self {
            header_phis: BTreeSet::new(),
            exit_phis: BTreeSet::new(),
        }
    }

    /// Header PHI数を取得
    pub fn header_count(&self) -> usize {
        self.header_phis.len()
    }

    /// Exit PHI数を取得
    pub fn exit_count(&self) -> usize {
        self.exit_phis.len()
    }

    /// 2つのPhiSpecが一致するか検証
    pub fn matches(&self, other: &PhiSpec) -> bool {
        self.header_phis == other.header_phis && self.exit_phis == other.exit_phis
    }
}

/// JoinInstからPHI仕様を計算
///
/// # Arguments
///
/// * `ctx` - If-in-loopコンテキスト（carrier_names情報）
/// * `join_inst` - JoinIR命令（Select/IfMerge）
///
/// # Returns
///
/// PHI仕様（header/exit PHI候補）
pub fn compute_phi_spec_from_joinir(ctx: &IfPhiContext, join_inst: &JoinInst) -> PhiSpec {
    let mut spec = PhiSpec::new();

    match join_inst {
        JoinInst::Select { .. } => {
            // Select命令: 単一変数のPHI
            // carrier_namesに含まれる変数をheader PHIとして扱う
            // TODO Phase 61-3: dstからvariable_nameを逆引き（MIR Builderのvariable_map参照）
            spec.header_phis = ctx.carrier_names.clone();
        }
        JoinInst::IfMerge { merges, .. } => {
            // IfMerge命令: 複数変数のPHI
            // TODO Phase 61-3: merge_pair.dstからvariable_nameを逆引き
            // 暫定: carrier_namesに含まれる変数をheader PHIとして扱う
            spec.header_phis = ctx.carrier_names.clone();

            if ctx.in_loop_body {
                if crate::config::env::joinir_dev::debug_enabled() {
                    get_global_ring0().log.debug(&format!(
                        "[Phase 61-2] IfMerge with {} merge pairs in loop body",
                        merges.len()
                    ));
                }
            }
        }
        _ => {
            if crate::config::env::joinir_dev::debug_enabled() {
                get_global_ring0()
                    .log
                    .debug("[Phase 61-2] ⚠️ Unexpected JoinInst variant for PHI spec");
            }
        }
    }

    spec
}

// Phase 61-6.2: A/B観察関数削除（JoinIR経路完全動作確認済み）
//
// 削除された関数:
// - extract_phi_spec_from_builder(): PhiBuilderBox経路の観察用
// - compare_and_log_phi_specs(): A/B比較ログ出力
//
// SSOT確立: compute_phi_spec_from_joinir() のみが PHI 仕様の計算ロジック

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phi_spec_creation() {
        let spec = PhiSpec::new();
        assert_eq!(spec.header_count(), 0);
        assert_eq!(spec.exit_count(), 0);
    }

    #[test]
    fn test_phi_spec_matches() {
        let mut spec1 = PhiSpec::new();
        spec1.header_phis.insert("x".to_string());
        spec1.header_phis.insert("y".to_string());

        let mut spec2 = PhiSpec::new();
        spec2.header_phis.insert("x".to_string());
        spec2.header_phis.insert("y".to_string());

        assert!(spec1.matches(&spec2));
    }

    #[test]
    fn test_phi_spec_mismatch() {
        let mut spec1 = PhiSpec::new();
        spec1.header_phis.insert("x".to_string());

        let mut spec2 = PhiSpec::new();
        spec2.header_phis.insert("y".to_string());

        assert!(!spec1.matches(&spec2));
    }
}
