//! Phase 256 P1.5: Minimal Select instruction unit test
//!
//! Verifies that JoinInst::Select remapping and collection work correctly.
//! Focuses on low-level ValueId handling independent of bridge/conversion code.

#[cfg(test)]
mod select_minimal_test {
    use crate::mir::builder::joinir_id_remapper::JoinIrIdRemapper;
    use crate::mir::{MirInstruction, ValueId};

    /// Test: Select instruction ValueId consistency
    ///
    /// Verifies that a Select instruction can be created with all required fields
    /// and that ValueIds are properly stored and retrievable.
    #[test]
    fn test_select_instruction_creation() {
        // Create Select instruction: %4 = select %3 ? %1 : %2
        let select_inst = MirInstruction::Select {
            dst: ValueId::new(4),
            cond: ValueId::new(3),
            then_val: ValueId::new(1),
            else_val: ValueId::new(2),
        };

        // Verify instruction can be matched and ValueIds extracted
        match select_inst {
            MirInstruction::Select {
                dst,
                cond,
                then_val,
                else_val,
            } => {
                assert_eq!(dst.0, 4, "dst ValueId should be 4");
                assert_eq!(cond.0, 3, "cond ValueId should be 3");
                assert_eq!(then_val.0, 1, "then_val ValueId should be 1");
                assert_eq!(else_val.0, 2, "else_val ValueId should be 2");

                eprintln!("[test] ✅ Select instruction created and verified:");
                eprintln!(
                    "[test]   dst: %{}, cond: %{}, then_val: %{}, else_val: %{}",
                    dst.0, cond.0, then_val.0, else_val.0
                );
            }
            _ => panic!("Expected Select instruction"),
        }
    }

    /// Test: Remapper handles Select instruction
    ///
    /// Verifies that JoinIrIdRemapper.remap_instruction() properly remaps
    /// all ValueIds in Select instruction.
    #[test]
    fn test_remapper_handles_select() {
        let mut remapper = JoinIrIdRemapper::new();

        // Setup: Map ValueIds from JoinIR local range (1000+) to host range (<1000)
        remapper.set_value(ValueId::new(1001), ValueId::new(51)); // cond
        remapper.set_value(ValueId::new(1002), ValueId::new(52)); // then_val
        remapper.set_value(ValueId::new(1003), ValueId::new(53)); // else_val
        remapper.set_value(ValueId::new(1004), ValueId::new(54)); // dst

        // Create Select instruction with JoinIR-local ValueIds
        let select_inst = MirInstruction::Select {
            dst: ValueId::new(1004),
            cond: ValueId::new(1001),
            then_val: ValueId::new(1002),
            else_val: ValueId::new(1003),
        };

        // Remap the instruction
        let remapped = remapper.remap_instruction(&select_inst);

        // Verify: all ValueIds are remapped
        match remapped {
            MirInstruction::Select {
                dst,
                cond,
                then_val,
                else_val,
            } => {
                assert_eq!(dst.0, 54, "dst should be remapped to 54");
                assert_eq!(cond.0, 51, "cond should be remapped to 51");
                assert_eq!(then_val.0, 52, "then_val should be remapped to 52");
                assert_eq!(else_val.0, 53, "else_val should be remapped to 53");

                eprintln!("[test] ✅ Remapper correctly remaps Select instruction:");
                eprintln!("[test]   dst: 1004 → {}", dst.0);
                eprintln!("[test]   cond: 1001 → {}", cond.0);
                eprintln!("[test]   then_val: 1002 → {}", then_val.0);
                eprintln!("[test]   else_val: 1003 → {}", else_val.0);
            }
            _ => panic!("Expected Select instruction after remapping"),
        }
    }

    /// Test: ValueId collection includes Select
    ///
    /// Verifies that JoinIrIdRemapper.collect_values_in_instruction()
    /// properly collects all ValueIds from Select instruction.
    #[test]
    fn test_collector_handles_select() {
        let remapper = JoinIrIdRemapper::new();

        // Create Select instruction
        let select_inst = MirInstruction::Select {
            dst: ValueId::new(100),
            cond: ValueId::new(101),
            then_val: ValueId::new(102),
            else_val: ValueId::new(103),
        };

        // Collect ValueIds
        let collected = remapper.collect_values_in_instruction(&select_inst);

        // Verify: all 4 ValueIds are collected
        assert_eq!(
            collected.len(),
            4,
            "Should collect 4 ValueIds, got {}",
            collected.len()
        );

        assert!(
            collected.contains(&ValueId::new(100)),
            "dst should be collected"
        );
        assert!(
            collected.contains(&ValueId::new(101)),
            "cond should be collected"
        );
        assert!(
            collected.contains(&ValueId::new(102)),
            "then_val should be collected"
        );
        assert!(
            collected.contains(&ValueId::new(103)),
            "else_val should be collected"
        );

        eprintln!("[test] ✅ Collector properly collects Select ValueIds:");
        eprintln!("[test]   Collected: {:?}", collected);
    }
}
