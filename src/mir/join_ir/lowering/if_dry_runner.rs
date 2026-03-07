//! Phase 33-10.0: If lowering dry-run スキャナー（箱化版）
//!
//! ## 責務
//! - MIR モジュール内のすべての Branch ブロックをスキャン
//! - try_lower_if_to_joinir() でパターンマッチングを試行（MIR書き換えなし）
//! - パフォーマンス計測と統計情報の収集
//!
//! ## 非責務
//! - MIR の書き換え（Route B実装時に別モジュールで実施）
//! - Loop lowering（別のdispatch経路）

use crate::mir::join_ir::JoinInst;
use crate::mir::{MirFunction, MirInstruction};
use std::collections::BTreeMap;
use std::time::{Duration, Instant};

/// If lowering dry-run スキャナー
pub struct IfLoweringDryRunner {
    debug_level: u8,
}

/// Dry-run スキャン結果の統計情報
#[derive(Debug, Clone)]
pub struct DryRunStats {
    pub total_branches: usize,
    pub lowered_count: usize,
    pub select_count: usize,
    pub ifmerge_count: usize,
    pub scan_duration: Duration,
}

impl IfLoweringDryRunner {
    /// 新しい dry-run スキャナーを作成
    pub fn new(debug_level: u8) -> Self {
        Self { debug_level }
    }

    /// MIR モジュール全体をスキャンして If lowering 成功率を計測
    ///
    /// ## 実装方針（Phase 33-9.2）
    /// - Loop専任関数はスキップ（is_loop_lowered_function()）
    /// - 各 Branch ブロックで try_lower_if_to_joinir() 試行
    /// - パフォーマンス計測（マイクロ秒レベル）
    /// - 統計情報収集（Select/IfMerge分類）
    pub fn scan_module(&self, functions: &BTreeMap<String, MirFunction>) -> DryRunStats {
        let mut total_branches = 0;
        let mut lowered_count = 0;
        let mut select_count = 0;
        let mut ifmerge_count = 0;
        let start_scan = Instant::now();

        for (func_name, func) in functions {
            // Phase 33-9.1: Loop専任関数をスキップ
            if crate::mir::join_ir::lowering::is_loop_lowered_function(func_name) {
                continue;
            }

            // 各Branchブロックに対してtry_lower_if_to_joinir()試行
            for (block_id, block) in &func.blocks {
                if matches!(block.terminator, Some(MirInstruction::Branch { .. })) {
                    total_branches += 1;
                    let start = Instant::now();

                    match crate::mir::join_ir::lowering::try_lower_if_to_joinir(
                        func,
                        *block_id,
                        self.debug_level >= 3,
                        None, // Phase 61-1: Pure If（dry-runは常にPure If）
                    ) {
                        Some(join_inst) => {
                            lowered_count += 1;
                            let elapsed = start.elapsed();

                            let inst_type = match &join_inst {
                                JoinInst::Select { .. } => {
                                    select_count += 1;
                                    "Select"
                                }
                                JoinInst::IfMerge { .. } => {
                                    ifmerge_count += 1;
                                    "IfMerge"
                                }
                                _ => "Other",
                            };

                            if self.debug_level >= 1 {
                                let ring0 = crate::runtime::get_global_ring0();
                                ring0.log.debug(&format!(
                                    "[joinir/if_lowering] ✅ {} block {:?}: {} ({:.2}μs)",
                                    func_name,
                                    block_id,
                                    inst_type,
                                    elapsed.as_micros()
                                ));
                            }
                        }
                        None => {
                            if self.debug_level >= 2 {
                                let ring0 = crate::runtime::get_global_ring0();
                                ring0.log.debug(&format!(
                                    "[joinir/if_lowering] ⏭️ {} block {:?}: shape not matched",
                                    func_name, block_id
                                ));
                            }
                        }
                    }
                }
            }
        }

        let scan_duration = start_scan.elapsed();

        DryRunStats {
            total_branches,
            lowered_count,
            select_count,
            ifmerge_count,
            scan_duration,
        }
    }

    /// 統計情報を標準エラー出力に表示
    pub fn print_stats(&self, stats: &DryRunStats) {
        if self.debug_level >= 1 && stats.total_branches > 0 {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug("[joinir/if_lowering] 📊 Scan complete:");
            ring0
                .log
                .debug(&format!("  Total branches: {}", stats.total_branches));
            ring0.log.debug(&format!(
                "  Lowered: {} ({:.1}%)",
                stats.lowered_count,
                (stats.lowered_count as f64 / stats.total_branches as f64) * 100.0
            ));
            ring0
                .log
                .debug(&format!("  - Select: {}", stats.select_count));
            ring0
                .log
                .debug(&format!("  - IfMerge: {}", stats.ifmerge_count));
            ring0.log.debug(&format!(
                "  Scan time: {:.2}ms",
                stats.scan_duration.as_secs_f64() * 1000.0
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dry_runner_creation() {
        let runner = IfLoweringDryRunner::new(0);
        assert_eq!(runner.debug_level, 0);

        let runner_verbose = IfLoweringDryRunner::new(3);
        assert_eq!(runner_verbose.debug_level, 3);
    }

    #[test]
    fn test_dry_run_stats_default() {
        let stats = DryRunStats {
            total_branches: 0,
            lowered_count: 0,
            select_count: 0,
            ifmerge_count: 0,
            scan_duration: Duration::from_millis(10),
        };

        assert_eq!(stats.total_branches, 0);
        assert_eq!(stats.lowered_count, 0);
    }
}
