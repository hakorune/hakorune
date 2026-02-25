//! Phase 61-2: If-in-loop JoinIR dry-run + PHI生成 A/B比較テスト
//!
//! 目的: JoinIR経路でPHI仕様を計算し、PhiBuilderBox経路との一致を検証
//!
//! 注意: Phase 61-2はdry-run検証のみ。実行結果は変わらない。
//!       JoinIRパターンマッチは Phase 33の厳格な条件に依存する。

#[cfg(test)]
mod tests {
    #[test]
    fn phase61_2_dry_run_flag_available() {
        // Phase 61-2: dry-runフラグが正しく読み取れることを確認
        std::env::set_var("HAKO_JOINIR_IF_IN_LOOP_DRYRUN", "1");
        assert_eq!(crate::config::env::joinir_if_in_loop_dryrun_enabled(), true);

        std::env::remove_var("HAKO_JOINIR_IF_IN_LOOP_DRYRUN");
        assert_eq!(
            crate::config::env::joinir_if_in_loop_dryrun_enabled(),
            false
        );

        eprintln!("[Test] phase61_2_dry_run_flag_available passed");
    }

    #[test]
    fn phase61_2_phi_spec_creation() {
        use crate::mir::join_ir::lowering::if_phi_spec::PhiSpec;

        let mut spec1 = PhiSpec::new();
        assert_eq!(spec1.header_count(), 0);
        assert_eq!(spec1.exit_count(), 0);

        spec1.header_phis.insert("x".to_string());
        spec1.header_phis.insert("y".to_string());
        assert_eq!(spec1.header_count(), 2);

        let mut spec2 = PhiSpec::new();
        spec2.header_phis.insert("x".to_string());
        spec2.header_phis.insert("y".to_string());

        assert!(spec1.matches(&spec2));

        eprintln!("[Test] phase61_2_phi_spec_creation passed");
    }
}

// Note: E2E tests for actual if-in-loop JoinIR lowering will be added in Phase 61-3
// when the production switch is made. Phase 61-2 focuses on dry-run infrastructure.
