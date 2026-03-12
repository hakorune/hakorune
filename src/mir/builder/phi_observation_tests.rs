#[cfg(debug_assertions)]
use crate::mir::builder::MirBuilder;
#[cfg(debug_assertions)]
use crate::mir::join_ir::verify_phi_reserved::{
    analyze_distribution, disable_observation, enable_observation, get_observations,
};
#[cfg(debug_assertions)]
use crate::mir::types::ConstValue;
#[cfg(debug_assertions)]
use crate::mir::MirInstruction;

/// Phase 72-1: Observe PHI dst distribution from MirBuilder usage
///
/// This test manually creates MIR scenarios to observe PHI dst allocation.
#[cfg(debug_assertions)]
#[test]
fn test_phase72_observe_phi_dst_via_builder() {
    enable_observation();

    // Create multiple builders to simulate different compilation contexts
    for scenario in 0..10 {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test(format!("test_func_{}", scenario));

        // Allocate some values (simulating loop setup)
        let _entry_block = builder.current_block_for_test().expect("entry block");

        // Simulate loop header block
        builder.push_block_for_test().expect("push block");
        let _header_block = builder.current_block_for_test().expect("header block");

        // Allocate initial values
        let v1 = builder.alloc_value_for_test();
        let v2 = builder.alloc_value_for_test();
        let v3 = builder.alloc_value_for_test();

        // Emit some instructions
        builder
            .emit_for_test(MirInstruction::Const {
                dst: v1,
                value: ConstValue::Integer(0),
            })
            .expect("emit const v1");
        builder
            .emit_for_test(MirInstruction::Const {
                dst: v2,
                value: ConstValue::Integer(100),
            })
            .expect("emit const v2");
        builder
            .emit_for_test(MirInstruction::Const {
                dst: v3,
                value: ConstValue::Integer(1),
            })
            .expect("emit const v3");

        // Now allocate PHI dst candidates (what would be PHI dsts)
        // These come from builder.next_value_id() in loop_header_phi_builder
        let phi_dst_1 = builder.alloc_value_for_test();
        let phi_dst_2 = builder.alloc_value_for_test();
        let phi_dst_3 = builder.alloc_value_for_test();

        // Manually observe these as if they were PHI dsts
        #[cfg(debug_assertions)]
        {
            crate::mir::join_ir::verify_phi_reserved::observe_phi_dst(phi_dst_1);
            crate::mir::join_ir::verify_phi_reserved::observe_phi_dst(phi_dst_2);
            crate::mir::join_ir::verify_phi_reserved::observe_phi_dst(phi_dst_3);
        }

        builder.exit_function_for_test();
    }

    // Collect observations
    let observations = get_observations();
    let report = analyze_distribution(&observations);

    if crate::config::env::joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .debug("\n========== Phase 72: PHI dst Distribution Analysis ==========");
        ring0.log.debug(&report.summary());
        ring0.log.debug("");
        ring0.log.debug("Detailed breakdown:");
        ring0.log.debug(&format!(
            "  - Reserved region (0-99):   {} PHI dsts",
            report.in_reserved
        ));
        ring0.log.debug(&format!(
            "  - Param region (100-999):   {} PHI dsts",
            report.in_param
        ));
        ring0.log.debug(&format!(
            "  - Local region (1000+):     {} PHI dsts",
            report.in_local
        ));
        ring0.log.debug("");

        if report.is_all_reserved() {
            ring0
                .log
                .debug("OK: CONCLUSION: All PHI dsts are in reserved region (0-99)");
            ring0
                .log
                .debug("   -> Safe to strengthen verifier with reserved region check");
        } else {
            ring0
                .log
                .debug("WARN: CONCLUSION: Some PHI dsts are OUTSIDE reserved region");
            ring0
                .log
                .debug("   -> PHI dst allocation does NOT respect reserved boundary");
            ring0
                .log
                .debug("   -> Document this finding and skip verifier strengthening");

            if let (Some(min), Some(max)) = (report.min_val, report.max_val) {
                ring0.log.debug("");
                ring0
                    .log
                    .debug(&format!("Observed range: [{}, {}]", min, max));
                ring0.log.debug("Expected range: [0, 99]");
            }
        }
        ring0
            .log
            .debug("==============================================================\n");
    }

    disable_observation();

    // This test always passes - it's for observation and decision-making
    assert!(true, "Observation complete - see output above for analysis");
}

#[cfg(not(debug_assertions))]
#[test]
fn test_phase72_observe_phi_dst_via_builder() {
    assert!(true, "Observation test is debug-only");
}
