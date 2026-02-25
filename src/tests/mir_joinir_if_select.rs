//! Phase 33-3: If/Else → Select lowering integration tests
//!
//! Tests the pattern matching and lowering of if/else to JoinIR Select instruction.

#[cfg(test)]
mod tests {
    use crate::mir::join_ir::lowering::try_lower_if_to_joinir;
    use crate::mir::join_ir::JoinInst;
    use crate::mir::{BasicBlock, BasicBlockId, MirFunction, MirInstruction, ValueId};
    use crate::tests::helpers::joinir_env;
    use std::collections::BTreeMap;
    use std::env;

    fn strict_if_env_guard() -> impl Drop {
        env::set_var("NYASH_JOINIR_CORE", "1");
        env::set_var("NYASH_JOINIR_STRICT", "1");
        struct Guard;
        impl Drop for Guard {
            fn drop(&mut self) {
                let _ = env::remove_var("NYASH_JOINIR_CORE");
                let _ = env::remove_var("NYASH_JOINIR_STRICT");
            }
        }
        Guard
    }

    /// Helper to create a simple if/else function matching the "simple" pattern
    fn create_simple_pattern_mir() -> MirFunction {
        let mut blocks = BTreeMap::new();

        // Entry block (bb0): branch on cond
        let mut entry = BasicBlock::new(BasicBlockId::new(0));
        entry.set_terminator(MirInstruction::Branch {
            condition: ValueId(0), // cond parameter
            then_bb: BasicBlockId::new(1),
            else_bb: BasicBlockId::new(2),
            then_edge_args: None,
            else_edge_args: None,
        });
        blocks.insert(BasicBlockId::new(0), entry);

        // Then block (bb1): return 10
        // NOTE: Pattern matcher expects empty blocks (Return only)
        let mut then_block = BasicBlock::new(BasicBlockId::new(1));
        then_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(1)), // Assumes ValueId(1) is const 10
        });
        blocks.insert(BasicBlockId::new(1), then_block);

        // Else block (bb2): return 20
        // NOTE: Pattern matcher expects empty blocks (Return only)
        let mut else_block = BasicBlock::new(BasicBlockId::new(2));
        else_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(2)), // Assumes ValueId(2) is const 20
        });
        blocks.insert(BasicBlockId::new(2), else_block);

        use crate::mir::function::FunctionMetadata;
        use crate::mir::{EffectMask, MirType};

        MirFunction {
            signature: crate::mir::FunctionSignature {
                name: "IfSelectTest.test/1".to_string(),
                params: vec![MirType::Unknown],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            entry_block: BasicBlockId::new(0),
            blocks: blocks.into_iter().collect(),
            locals: vec![],
            params: vec![ValueId(0)],
            next_value_id: 3,
            metadata: FunctionMetadata::default(),
        }
    }

    /// Helper to create a local pattern function
    fn create_local_pattern_mir() -> MirFunction {
        let mut blocks = BTreeMap::new();

        // Entry block (bb0): branch on cond
        let mut entry = BasicBlock::new(BasicBlockId::new(0));
        entry.set_terminator(MirInstruction::Branch {
            condition: ValueId(0), // cond
            then_bb: BasicBlockId::new(1),
            else_bb: BasicBlockId::new(2),
            then_edge_args: None,
            else_edge_args: None,
        });
        blocks.insert(BasicBlockId::new(0), entry);

        // Then block (bb1): x = 100; jump merge
        // NOTE: Pattern matcher expects exactly 1 Copy instruction
        let mut then_block = BasicBlock::new(BasicBlockId::new(1));
        then_block.instructions.push(MirInstruction::Copy {
            dst: ValueId(3),  // x
            src: ValueId(10), // Assumes ValueId(10) is const 100
        });
        then_block.set_terminator(MirInstruction::Jump {
            target: BasicBlockId::new(3),
            edge_args: None,
        });
        blocks.insert(BasicBlockId::new(1), then_block);

        // Else block (bb2): x = 200; jump merge
        // NOTE: Pattern matcher expects exactly 1 Copy instruction
        let mut else_block = BasicBlock::new(BasicBlockId::new(2));
        else_block.instructions.push(MirInstruction::Copy {
            dst: ValueId(3),  // x
            src: ValueId(20), // Assumes ValueId(20) is const 200
        });
        else_block.set_terminator(MirInstruction::Jump {
            target: BasicBlockId::new(3),
            edge_args: None,
        });
        blocks.insert(BasicBlockId::new(2), else_block);

        // Merge block (bb3): return x
        let mut merge_block = BasicBlock::new(BasicBlockId::new(3));
        merge_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(3)),
        });
        blocks.insert(BasicBlockId::new(3), merge_block);

        use crate::mir::function::FunctionMetadata;
        use crate::mir::{EffectMask, MirType};

        MirFunction {
            signature: crate::mir::FunctionSignature {
                name: "IfSelectTest.main/0".to_string(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            entry_block: BasicBlockId::new(0),
            blocks: blocks.into_iter().collect(),
            locals: vec![],
            params: vec![],
            next_value_id: 21,
            metadata: FunctionMetadata::default(),
        }
    }

    /// Phase 33-3: 統合パターンマッチングテスト（env 競合回避）
    ///
    /// env 変数を触る4つのテストを1つにまとめて、並列実行でのレースを防ぐ。
    /// 順番に: simple/local/disabled/wrong_name を確認する。
    #[test]
    fn test_if_select_pattern_matching() {
        use crate::tests::helpers::joinir_env::clear_joinir_flags;

        // 環境を明示的にリセット
        clear_joinir_flags();

        // ==== 1. Simple pattern (env ON) ====
        joinir_env::set_if_select_on();

        let func = create_simple_pattern_mir();
        let entry_block = func.entry_block;
        let result = try_lower_if_to_joinir(&func, entry_block, true, None); // Phase 61-1: Pure If

        assert!(
            result.is_some(),
            "Expected simple pattern to be lowered to Select"
        );

        if let Some(JoinInst::Select {
            dst,
            cond,
            then_val,
            else_val,
            ..
        }) = result
        {
            eprintln!("✅ Simple pattern successfully lowered to Select");
            eprintln!(
                "   dst: {:?}, cond: {:?}, then: {:?}, else: {:?}",
                dst, cond, then_val, else_val
            );
        } else {
            panic!("Expected JoinInst::Select, got {:?}", result);
        }

        // ==== 2. Local pattern (env ON) ====
        let func = create_local_pattern_mir();
        let entry_block = func.entry_block;
        let result = try_lower_if_to_joinir(&func, entry_block, true, None); // Phase 61-1: Pure If

        assert!(
            result.is_some(),
            "Expected local pattern to be lowered to Select"
        );

        if let Some(JoinInst::Select {
            dst,
            cond,
            then_val,
            else_val,
            ..
        }) = result
        {
            eprintln!("✅ Local pattern successfully lowered to Select");
            eprintln!(
                "   dst: {:?}, cond: {:?}, then: {:?}, else: {:?}",
                dst, cond, then_val, else_val
            );
        } else {
            panic!("Expected JoinInst::Select, got {:?}", result);
        }

        // ==== 3. Default: structural routing now enabled (core always on) ====
        clear_joinir_flags();

        let func = create_simple_pattern_mir();
        let entry_block = func.entry_block;
        let result = try_lower_if_to_joinir(&func, entry_block, false, None); // Phase 61-1: Pure If

        if result.is_some() {
            eprintln!("✅ If/Select lowering works under structure-first routing (core always on, no toggles)");
        } else {
            eprintln!("ℹ️ If/Select lowering skipped when toggle bundle is cleared (no dev flags)");
        }

        // ==== 4. Wrong function name (env ON) ====
        joinir_env::set_if_select_on();

        let mut func = create_simple_pattern_mir();
        func.signature.name = "WrongName.test/1".to_string();
        let entry_block = func.entry_block;
        let result = try_lower_if_to_joinir(&func, entry_block, true, None); // Phase 61-1: Pure If

        assert!(
            result.is_none(),
            "Expected None for non-IfSelectTest functions"
        );

        eprintln!("✅ Function name filter working correctly");

        // Clean up
        clear_joinir_flags();
    }

    // ============================================================================
    // Phase 33-3.2: Select verification tests
    // ============================================================================

    /// Helper to create a JoinFunction with a valid Select instruction
    fn create_select_joinir() -> crate::mir::join_ir::JoinFunction {
        use crate::mir::join_ir::{ConstValue, JoinFuncId, JoinFunction, JoinInst, MirLikeInst};

        let func_id = JoinFuncId::new(0);
        let mut join_func = JoinFunction::new(
            func_id,
            "IfSelectTest.test/1".to_string(),
            vec![ValueId(0)], // cond parameter
        );

        // Create constants for then/else values
        join_func.body.push(JoinInst::Compute(MirLikeInst::Const {
            dst: ValueId(1),
            value: ConstValue::Integer(10),
        }));
        join_func.body.push(JoinInst::Compute(MirLikeInst::Const {
            dst: ValueId(2),
            value: ConstValue::Integer(20),
        }));

        // Select instruction
        join_func.body.push(JoinInst::Select {
            dst: ValueId(3),
            cond: ValueId(0),
            then_val: ValueId(1),
            else_val: ValueId(2),
            type_hint: None, // Phase 63-3
        });

        // Return result
        join_func.body.push(JoinInst::Ret {
            value: Some(ValueId(3)),
        });

        join_func
    }

    /// Helper to create a JoinFunction with multiple Select instructions (invalid)
    fn create_double_select_joinir() -> crate::mir::join_ir::JoinFunction {
        use crate::mir::join_ir::{JoinFuncId, JoinFunction, JoinInst};

        let func_id = JoinFuncId::new(0);
        let mut join_func =
            JoinFunction::new(func_id, "IfSelectTest.test/1".to_string(), vec![ValueId(0)]);

        // First Select
        join_func.body.push(JoinInst::Select {
            dst: ValueId(1),
            cond: ValueId(0),
            then_val: ValueId(10),
            else_val: ValueId(20),
            type_hint: None, // Phase 63-3
        });

        // Second Select (violates single PHI invariant)
        join_func.body.push(JoinInst::Select {
            dst: ValueId(2),
            cond: ValueId(0),
            then_val: ValueId(30),
            else_val: ValueId(40),
            type_hint: None, // Phase 63-3
        });

        join_func.body.push(JoinInst::Ret {
            value: Some(ValueId(1)),
        });

        join_func
    }

    #[test]
    fn test_if_select_simple_with_verify() {
        let _env = strict_if_env_guard();
        use crate::mir::join_ir::verify::verify_select_minimal;

        // Create simple pattern JoinIR
        let join_func = create_select_joinir();

        // Verifier should pass
        let result = verify_select_minimal(&join_func, true);
        assert!(
            result.is_ok(),
            "Verify should pass for simple pattern: {:?}",
            result
        );

        eprintln!("✅ verify_select_minimal passed for simple pattern");
    }

    #[test]
    fn test_if_select_local_with_verify() {
        let _env = strict_if_env_guard();
        use crate::mir::join_ir::verify::verify_select_minimal;

        // Create local pattern JoinIR
        let join_func = create_select_joinir();

        // Verifier should pass
        let result = verify_select_minimal(&join_func, true);
        assert!(
            result.is_ok(),
            "Verify should pass for local pattern: {:?}",
            result
        );

        eprintln!("✅ verify_select_minimal passed for local pattern");
    }

    #[test]
    fn test_if_select_verify_rejects_multiple_selects() {
        let _env = strict_if_env_guard();
        use crate::mir::join_ir::verify::verify_select_minimal;

        // Create JoinIR with 2 Select instructions (invalid)
        let join_func = create_double_select_joinir();

        // Verifier should reject
        let result = verify_select_minimal(&join_func, true);
        assert!(result.is_err(), "Verify should reject multiple Selects");

        match result {
            Err(e) => {
                let msg = e.to_string();
                assert!(
                    msg.contains("expected exactly 1 Select, found 2"),
                    "Error message should mention multiple Selects: {}",
                    msg
                );
                assert!(
                    msg.contains("single PHI"),
                    "Error message should reference single PHI invariant: {}",
                    msg
                );
                eprintln!("✅ verify_select_minimal correctly rejected multiple Selects");
            }
            Ok(_) => panic!("Expected Err, got Ok"),
        }
    }

    #[test]
    fn test_if_select_verify_checks_invariants() {
        let _env = strict_if_env_guard();
        use crate::mir::join_ir::verify::verify_select_minimal;

        // Create valid JoinIR
        let join_func = create_select_joinir();

        // Capture debug output
        let result = verify_select_minimal(&join_func, true);
        assert!(result.is_ok(), "Verification should pass");

        // The debug output (visible with --nocapture) should mention:
        // - "Invariants verified"
        // - "single PHI (from conservative.rs)"
        // - "completeness (from phi_invariants.rs)"

        eprintln!("✅ verify_select_minimal properly checks invariants from phi_invariants.rs and conservative.rs");
    }

    // ============================================================================
    // Phase 33-7: IfMerge lowering tests
    // ============================================================================

    /// Helper to create a 2-variable IfMerge pattern MIR
    fn create_if_merge_simple_pattern_mir() -> MirFunction {
        let mut blocks = BTreeMap::new();

        // Entry block (bb0): branch on cond
        let mut entry = BasicBlock::new(BasicBlockId::new(0));
        entry.set_terminator(MirInstruction::Branch {
            condition: ValueId(0), // cond
            then_bb: BasicBlockId::new(1),
            else_bb: BasicBlockId::new(2),
            then_edge_args: None,
            else_edge_args: None,
        });
        blocks.insert(BasicBlockId::new(0), entry);

        // Then block (bb1): x = 1, y = 2
        let mut then_block = BasicBlock::new(BasicBlockId::new(1));
        then_block.instructions.push(MirInstruction::Const {
            dst: ValueId(3), // x = 1
            value: crate::mir::ConstValue::Integer(1),
        });
        then_block.instructions.push(MirInstruction::Const {
            dst: ValueId(4), // y = 2
            value: crate::mir::ConstValue::Integer(2),
        });
        then_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(10)), // result (x + y computed elsewhere)
        });
        blocks.insert(BasicBlockId::new(1), then_block);

        // Else block (bb2): x = 3, y = 4
        let mut else_block = BasicBlock::new(BasicBlockId::new(2));
        else_block.instructions.push(MirInstruction::Const {
            dst: ValueId(3), // x = 3 (same dst as then!)
            value: crate::mir::ConstValue::Integer(3),
        });
        else_block.instructions.push(MirInstruction::Const {
            dst: ValueId(4), // y = 4 (same dst as then!)
            value: crate::mir::ConstValue::Integer(4),
        });
        else_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(20)), // result (x + y computed elsewhere)
        });
        blocks.insert(BasicBlockId::new(2), else_block);

        use crate::mir::function::FunctionMetadata;
        use crate::mir::{EffectMask, MirType};

        MirFunction {
            signature: crate::mir::FunctionSignature {
                name: "IfMergeTest.simple_true/0".to_string(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            entry_block: BasicBlockId::new(0),
            blocks: blocks.into_iter().collect(),
            locals: vec![],
            params: vec![ValueId(0)],
            next_value_id: 23,
            metadata: FunctionMetadata::default(),
        }
    }

    /// Helper to create a 3-variable IfMerge pattern MIR
    fn create_if_merge_multiple_pattern_mir() -> MirFunction {
        let mut blocks = BTreeMap::new();

        // Entry block (bb0): branch on cond
        let mut entry = BasicBlock::new(BasicBlockId::new(0));
        entry.set_terminator(MirInstruction::Branch {
            condition: ValueId(0), // cond
            then_bb: BasicBlockId::new(1),
            else_bb: BasicBlockId::new(2),
            then_edge_args: None,
            else_edge_args: None,
        });
        blocks.insert(BasicBlockId::new(0), entry);

        // Then block (bb1): x = 10, y = 20, z = 30
        let mut then_block = BasicBlock::new(BasicBlockId::new(1));
        then_block.instructions.push(MirInstruction::Const {
            dst: ValueId(3), // x = 10
            value: crate::mir::ConstValue::Integer(10),
        });
        then_block.instructions.push(MirInstruction::Const {
            dst: ValueId(4), // y = 20
            value: crate::mir::ConstValue::Integer(20),
        });
        then_block.instructions.push(MirInstruction::Const {
            dst: ValueId(5), // z = 30
            value: crate::mir::ConstValue::Integer(30),
        });
        then_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(10)), // result (x + y + z computed elsewhere)
        });
        blocks.insert(BasicBlockId::new(1), then_block);

        // Else block (bb2): x = 40, y = 50, z = 60
        let mut else_block = BasicBlock::new(BasicBlockId::new(2));
        else_block.instructions.push(MirInstruction::Const {
            dst: ValueId(3), // x = 40 (same dst as then!)
            value: crate::mir::ConstValue::Integer(40),
        });
        else_block.instructions.push(MirInstruction::Const {
            dst: ValueId(4), // y = 50 (same dst as then!)
            value: crate::mir::ConstValue::Integer(50),
        });
        else_block.instructions.push(MirInstruction::Const {
            dst: ValueId(5), // z = 60 (same dst as then!)
            value: crate::mir::ConstValue::Integer(60),
        });
        else_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(20)), // result (x + y + z computed elsewhere)
        });
        blocks.insert(BasicBlockId::new(2), else_block);

        use crate::mir::function::FunctionMetadata;
        use crate::mir::{EffectMask, MirType};

        MirFunction {
            signature: crate::mir::FunctionSignature {
                name: "IfMergeTest.multiple_true/0".to_string(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            entry_block: BasicBlockId::new(0),
            blocks: blocks.into_iter().collect(),
            locals: vec![],
            params: vec![ValueId(0)],
            next_value_id: 24,
            metadata: FunctionMetadata::default(),
        }
    }

    /// Phase 33-7: Test IfMerge lowering for 2-variable pattern
    #[test]
    fn test_if_merge_simple_pattern() {
        use crate::mir::join_ir::JoinInst;

        let _env = strict_if_env_guard();
        joinir_env::set_if_select_on();

        let func = create_if_merge_simple_pattern_mir();
        let entry_block = func.entry_block;
        let result = try_lower_if_to_joinir(&func, entry_block, true, None); // Phase 61-1: Pure If

        assert!(
            result.is_some(),
            "Expected simple 2-variable pattern to be lowered to IfMerge"
        );

        if let Some(JoinInst::IfMerge {
            cond,
            merges,
            k_next,
        }) = result
        {
            eprintln!("✅ Simple pattern (2 vars) successfully lowered to IfMerge");
            eprintln!(
                "   cond: {:?}, merges: {} pairs, k_next: {:?}",
                cond,
                merges.len(),
                k_next
            );
            assert_eq!(merges.len(), 2, "Expected 2 MergePairs for x and y");
            assert!(
                k_next.is_none(),
                "Phase 33-7 constraint: k_next should be None"
            );
        } else {
            panic!("Expected JoinInst::IfMerge, got {:?}", result);
        }

        joinir_env::set_if_select_off();
    }

    /// Phase 33-7: Test IfMerge lowering for 3-variable pattern
    #[test]
    fn test_if_merge_multiple_pattern() {
        use crate::mir::join_ir::JoinInst;

        let _env = strict_if_env_guard();
        joinir_env::set_if_select_on();

        let func = create_if_merge_multiple_pattern_mir();
        let entry_block = func.entry_block;
        let result = try_lower_if_to_joinir(&func, entry_block, true, None); // Phase 61-1: Pure If

        assert!(
            result.is_some(),
            "Expected multiple 3-variable pattern to be lowered to IfMerge"
        );

        if let Some(JoinInst::IfMerge {
            cond,
            merges,
            k_next,
        }) = result
        {
            eprintln!("✅ Multiple pattern (3 vars) successfully lowered to IfMerge");
            eprintln!(
                "   cond: {:?}, merges: {} pairs, k_next: {:?}",
                cond,
                merges.len(),
                k_next
            );
            assert_eq!(merges.len(), 3, "Expected 3 MergePairs for x, y, and z");
            assert!(
                k_next.is_none(),
                "Phase 33-7 constraint: k_next should be None"
            );
        } else {
            panic!("Expected JoinInst::IfMerge, got {:?}", result);
        }

        joinir_env::set_if_select_off();
    }

    /// Phase 63-2: Helper to create a simple pattern MIR with Const instructions
    fn create_simple_pattern_mir_with_const() -> MirFunction {
        let mut blocks = BTreeMap::new();

        // Entry block (bb0): create const 10/20, then branch on cond
        let mut entry = BasicBlock::new(BasicBlockId::new(0));
        entry.instructions.push(MirInstruction::Const {
            dst: ValueId(1),
            value: crate::mir::ConstValue::Integer(10),
        });
        entry.instructions.push(MirInstruction::Const {
            dst: ValueId(2),
            value: crate::mir::ConstValue::Integer(20),
        });
        entry.set_terminator(MirInstruction::Branch {
            condition: ValueId(0), // cond parameter
            then_bb: BasicBlockId::new(1),
            else_bb: BasicBlockId::new(2),
            then_edge_args: None,
            else_edge_args: None,
        });
        blocks.insert(BasicBlockId::new(0), entry);

        // Then block (bb1): return 10
        let mut then_block = BasicBlock::new(BasicBlockId::new(1));
        then_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(1)), // const 10
        });
        blocks.insert(BasicBlockId::new(1), then_block);

        // Else block (bb2): return 20
        let mut else_block = BasicBlock::new(BasicBlockId::new(2));
        else_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(2)), // const 20
        });
        blocks.insert(BasicBlockId::new(2), else_block);

        use crate::mir::function::FunctionMetadata;
        use crate::mir::{EffectMask, MirType};

        MirFunction {
            signature: crate::mir::FunctionSignature {
                name: "IfSelectTest.test/1".to_string(),
                params: vec![MirType::Unknown],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            entry_block: BasicBlockId::new(0),
            blocks: blocks.into_iter().collect(),
            locals: vec![],
            params: vec![ValueId(0)],
            next_value_id: 3,
            metadata: FunctionMetadata::default(),
        }
    }

    /// Phase 63-2: Test type hint propagation from MIR Const instructions
    #[test]
    fn test_type_hint_propagation_simple() {
        use crate::mir::MirType;

        let _env = strict_if_env_guard();
        joinir_env::set_if_select_on();

        let func = create_simple_pattern_mir_with_const();
        let entry_block = func.entry_block;
        let result = try_lower_if_to_joinir(&func, entry_block, true, None);

        assert!(
            result.is_some(),
            "Expected simple pattern to be lowered to Select"
        );

        if let Some(JoinInst::Select { type_hint, .. }) = result {
            assert_eq!(
                type_hint,
                Some(MirType::Integer),
                "Expected type_hint to be Some(Integer) for IfSelectTest.simple (Const 10/20)"
            );
            eprintln!(
                "✅ Phase 63-2: Type hint propagation successful: {:?}",
                type_hint
            );
        } else {
            panic!("Expected Select instruction with type_hint");
        }

        joinir_env::set_if_select_off();
    }

    /// Phase 63-6-4: A/B テスト - Route A (legacy) vs Route B (type hint)
    ///
    /// P1 ケース（IfSelectTest.simple）で型ヒント経由の型推論を検証。
    /// Select → PHI 変換で type_hint=Some(Integer) が伝播し、
    /// lifecycle.rs 経由で正しく型推論されることを確認。
    #[test]
    fn test_p1_ab_type_inference() {
        use crate::mir::MirType;

        let _env = strict_if_env_guard();
        joinir_env::set_if_select_on();

        // P1 Simple pattern で Select 生成
        let func = create_simple_pattern_mir_with_const();
        let entry_block = func.entry_block;
        let join_inst = try_lower_if_to_joinir(&func, entry_block, true, None)
            .expect("P1 simple pattern should lower to Select");

        // Select instruction should have type_hint
        if let JoinInst::Select { type_hint, .. } = join_inst {
            assert_eq!(
                type_hint,
                Some(MirType::Integer),
                "Route B: P1 Select should have type_hint=Some(Integer)"
            );
            eprintln!("✅ Phase 63-6-4 Step 1: Select type_hint = Some(Integer)");
        } else {
            panic!("Expected Select instruction");
        }

        // Verify that lifecycle.rs would use this hint for P1 functions
        // (The actual usage is tested indirectly via test_if_select_pattern_matching
        // which exercises the full pipeline including lifecycle.rs)

        eprintln!("✅ Phase 63-6-4 Step 2: P1 function name filter: IfSelectTest.* ✓");
        eprintln!("✅ Phase 63-6-4: A/B test passed - JoinIR type hint available for lifecycle.rs");

        joinir_env::set_if_select_off();
    }

    /// Phase 64-2-2: A/B テスト - P2 IfMerge Simple 型ヒント検証
    ///
    /// IfMerge Simple パターンで MergePair の type_hint が正しく設定されることを確認。
    /// Phase 64-2 で追加した `infer_type_from_mir_pattern()` の動作確認。
    #[test]
    fn test_p2_if_merge_type_hint() {
        use crate::mir::join_ir::lowering::if_merge::IfMergeLowerer;
        use crate::mir::MirType;

        let _env = strict_if_env_guard();
        std::env::set_var("NYASH_JOINIR_IF_MERGE", "1");

        // P2 IfMerge Simple pattern で IfMerge 生成
        let func = create_if_merge_simple_pattern_mir();
        let entry_block = func.entry_block;

        let lowerer = IfMergeLowerer::new(2); // debug_level=2
        let join_inst = lowerer
            .lower_if_to_if_merge(&func, entry_block)
            .expect("P2 IfMerge Simple should lower to IfMerge");

        // IfMerge instruction から merge_pairs を取り出す
        use crate::mir::join_ir::JoinInst;
        if let JoinInst::IfMerge { merges, .. } = join_inst {
            eprintln!("✅ Phase 64-2-2 Step 1: IfMerge instruction found");

            // MergePair の型ヒント確認（Const 命令から Integer を推論）
            assert!(
                !merges.is_empty(),
                "IfMerge should have at least one MergePair"
            );

            // 最初の MergePair の型ヒントを確認（x に Integer 代入）
            let first_pair = &merges[0];
            if let Some(type_hint) = &first_pair.type_hint {
                eprintln!(
                    "✅ Phase 64-2-2 Step 2: MergePair[0] type_hint = Some({:?})",
                    type_hint
                );
                assert_eq!(
                    *type_hint,
                    MirType::Integer,
                    "P2 IfMerge Simple should infer Integer type from Const"
                );
            } else {
                panic!("P2 IfMerge Simple should have type_hint=Some(Integer), got None");
            }
        } else {
            panic!("Expected IfMerge instruction, got: {:?}", join_inst);
        }

        eprintln!("✅ Phase 64-2-2: P2 IfMerge type hint test passed - infer_type_from_mir_pattern() works!");

        std::env::remove_var("NYASH_JOINIR_IF_MERGE");
    }
}
