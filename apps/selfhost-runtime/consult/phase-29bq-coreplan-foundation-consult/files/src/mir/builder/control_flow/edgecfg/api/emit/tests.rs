    use super::*;
    use crate::mir::basic_block::EdgeArgs;
    use crate::mir::function::{FunctionSignature, MirFunction};
    use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
    use crate::mir::types::MirType;
    use crate::mir::{BasicBlock, EffectMask, ValueId};

    /// テスト用の MirFunction を作成（最小構成）
    fn create_test_function() -> MirFunction {
        let signature = FunctionSignature {
            name: "test_func".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let entry_block = BasicBlockId(0);
        MirFunction::new(signature, entry_block)
    }

    #[test]
    fn test_emit_wires_jump_basic() {
        // Setup: MirFunction with 2 blocks
        let mut function = create_test_function();
        let bb0 = BasicBlockId(0); // entry
        let bb1 = BasicBlockId(1);
        function.add_block(BasicBlock::new(bb1));

        // Setup: wire (bb0 → bb1)
        let stub = EdgeStub::with_target(
            bb0,
            ExitKind::Normal,
            bb1,
            EdgeArgs {
                layout: JumpArgsLayout::CarriersOnly,
                values: vec![ValueId(100)],
            },
        );

        let wires = vec![stub];

        // Execute
        let result = emit_wires(&mut function, &wires);

        // Verify: success
        assert!(result.is_ok(), "emit_wires failed: {:?}", result.err());

        // Verify: bb0 has Jump terminator
        let block0 = function.get_block(bb0).unwrap();
        assert!(
            block0.is_terminated(),
            "bb0 should have a terminator"
        );

        match &block0.terminator {
            Some(MirInstruction::Jump { target, edge_args }) => {
                assert_eq!(*target, bb1, "Jump target should be bb1");
                assert!(edge_args.is_some(), "Jump should have edge_args");
                let args = edge_args.as_ref().unwrap();
                assert_eq!(
                    args.values,
                    vec![ValueId(100)],
                    "Edge args values mismatch"
                );
            }
            other => panic!("Expected Jump terminator, got {:?}", other),
        }

        // Verify: successors updated
        assert!(
            block0.successors.contains(&bb1),
            "bb0 successors should contain bb1"
        );
    }

    #[test]
    fn test_emit_wires_return_basic() {
        // Setup: MirFunction with 1 block
        let mut function = create_test_function();
        let bb0 = BasicBlockId(0); // entry

        // Setup: Return wire（target=None OK、意味を持たない）
        let stub = EdgeStub {
            from: bb0,
            kind: ExitKind::Return,
            target: None, // Return は target 不要（emit_wires で無視される）
            args: EdgeArgs {
                layout: JumpArgsLayout::CarriersOnly,
                values: vec![ValueId(200)],
            },
        };

        let wires = vec![stub];

        // Execute
        let result = emit_wires(&mut function, &wires);

        // Verify: success
        assert!(result.is_ok(), "emit_wires failed: {:?}", result.err());

        // Verify: bb0 has Return terminator
        let block0 = function.get_block(bb0).unwrap();
        match &block0.terminator {
            Some(MirInstruction::Return { value }) => {
                assert_eq!(
                    *value,
                    Some(ValueId(200)),
                    "Return value mismatch"
                );
            }
            other => panic!("Expected Return terminator, got {:?}", other),
        }

        // Verify: return_env set
        let return_env = block0.return_env().expect("return_env should be set");
        assert_eq!(
            return_env.values,
            vec![ValueId(200)],
            "return_env values mismatch"
        );
    }

    #[test]
    fn test_emit_wires_unwired_stub_fails() {
        // Setup: EdgeStub with target=None（Normal は target 必須）
        let mut function = create_test_function();
        let bb0 = BasicBlockId(0);

        let stub = EdgeStub::without_args(bb0, ExitKind::Normal);
        // stub.target = None（未配線）

        let wires = vec![stub];

        // Execute
        let result = emit_wires(&mut function, &wires);

        // Verify: failure
        assert!(result.is_err(), "Expected error for unwired Normal stub");
        let err_msg = result.unwrap_err();
        assert!(
            err_msg.contains("Unwired EdgeStub"),
            "Error message should mention 'Unwired EdgeStub', got: {}",
            err_msg
        );
    }

    #[test]
    fn test_emit_wires_multiple_from_same_block_fails() {
        // Setup: 同じ from に2本の wire（1 block = 1 terminator 違反）
        let mut function = create_test_function();
        let bb0 = BasicBlockId(0);
        let bb1 = BasicBlockId(1);
        let bb2 = BasicBlockId(2);
        function.add_block(BasicBlock::new(bb1));
        function.add_block(BasicBlock::new(bb2));

        let stub1 = EdgeStub {
            from: bb0,
            kind: ExitKind::Normal,
            target: Some(bb1),
            args: EdgeArgs {
                layout: JumpArgsLayout::CarriersOnly,
                values: vec![],
            },
        };

        let stub2 = EdgeStub {
            from: bb0, // 同じ from
            kind: ExitKind::Normal,
            target: Some(bb2),
            args: EdgeArgs {
                layout: JumpArgsLayout::CarriersOnly,
                values: vec![],
            },
        };

        let wires = vec![stub1, stub2];

        // Execute
        let result = emit_wires(&mut function, &wires);

        // Verify: failure
        assert!(
            result.is_err(),
            "Expected error for multiple wires from same block"
        );
        let err_msg = result.unwrap_err();
        assert!(
            err_msg.contains("Multiple wires from same block"),
            "Error message should mention 'Multiple wires from same block', got: {}",
            err_msg
        );
    }

    // ========================================================================
    // Phase 267 P0: emit_frag() テスト（3個）
    // ========================================================================

    #[test]
    fn test_emit_frag_branch_basic() {
        use super::super::branch_stub::BranchStub;
        use super::super::frag::Frag;
        use std::collections::BTreeMap;

        // Setup: MirFunction with 3 blocks (header, then, else)
        let mut function = create_test_function();
        let header = BasicBlockId(0);
        let then_bb = BasicBlockId(1);
        let else_bb = BasicBlockId(2);
        function.add_block(BasicBlock::new(then_bb));
        function.add_block(BasicBlock::new(else_bb));

        // Setup: BranchStub (header → then/else)
        let branch = BranchStub {
            from: header,
            cond: ValueId(100),
            then_target: then_bb,
            then_args: EdgeArgs {
                layout: JumpArgsLayout::CarriersOnly,
                values: vec![ValueId(101)],
            },
            else_target: else_bb,
            else_args: EdgeArgs {
                layout: JumpArgsLayout::CarriersOnly,
                values: vec![ValueId(102)],
            },
        };

        let frag = Frag {
            entry: header,
            block_params: BTreeMap::new(),
            exits: BTreeMap::new(),
            wires: vec![],
            branches: vec![branch],
        };

        // Execute
        let result = emit_frag(&mut function, &frag);

        // Verify: success
        assert!(result.is_ok(), "emit_frag failed: {:?}", result.err());

        // Verify: header has Branch terminator
        let block = function.get_block(header).unwrap();
        match &block.terminator {
            Some(MirInstruction::Branch {
                condition,
                then_bb: t,
                else_bb: e,
                then_edge_args,
                else_edge_args,
            }) => {
                assert_eq!(*condition, ValueId(100));
                assert_eq!(*t, then_bb);
                assert_eq!(*e, else_bb);
                assert!(then_edge_args.is_some());
                assert!(else_edge_args.is_some());
                assert_eq!(
                    then_edge_args.as_ref().unwrap().values,
                    vec![ValueId(101)]
                );
                assert_eq!(
                    else_edge_args.as_ref().unwrap().values,
                    vec![ValueId(102)]
                );
            }
            other => panic!("Expected Branch, got {:?}", other),
        }

        // Verify: successors updated
        assert!(block.successors.contains(&then_bb));
        assert!(block.successors.contains(&else_bb));
    }

    #[test]
    fn test_emit_frag_block_params_inserts_phi() {
        use super::super::block_params::BlockParams;
        use super::super::frag::Frag;
        use std::collections::BTreeMap;

        let mut function = create_test_function();
        let pred1 = BasicBlockId(1);
        let pred2 = BasicBlockId(2);
        let target = BasicBlockId(3);
        function.add_block(BasicBlock::new(pred1));
        function.add_block(BasicBlock::new(pred2));
        function.add_block(BasicBlock::new(target));

        let args1 = EdgeArgs {
            layout: JumpArgsLayout::ExprResultPlusCarriers,
            values: vec![ValueId(10), ValueId(11)],
        };
        let args2 = EdgeArgs {
            layout: JumpArgsLayout::ExprResultPlusCarriers,
            values: vec![ValueId(12), ValueId(13)],
        };

        let wires = vec![
            EdgeStub {
                from: pred1,
                kind: ExitKind::Normal,
                target: Some(target),
                args: args1,
            },
            EdgeStub {
                from: pred2,
                kind: ExitKind::Normal,
                target: Some(target),
                args: args2,
            },
        ];

        let mut block_params = BTreeMap::new();
        block_params.insert(
            target,
            BlockParams {
                layout: JumpArgsLayout::ExprResultPlusCarriers,
                params: vec![ValueId(100), ValueId(101)],
            },
        );

        let frag = Frag {
            entry: pred1,
            block_params,
            exits: BTreeMap::new(),
            wires,
            branches: vec![],
        };

        emit_frag(&mut function, &frag).expect("emit_frag should succeed");

        let block = function.get_block(target).unwrap();
        match &block.instructions[0] {
            MirInstruction::Phi { dst, inputs, .. } => {
                assert_eq!(*dst, ValueId(100));
                assert_eq!(
                    inputs,
                    &vec![(pred1, ValueId(10)), (pred2, ValueId(12))]
                );
            }
            other => panic!("Expected Phi at head, got {:?}", other),
        }
        match &block.instructions[1] {
            MirInstruction::Phi { dst, inputs, .. } => {
                assert_eq!(*dst, ValueId(101));
                assert_eq!(
                    inputs,
                    &vec![(pred1, ValueId(11)), (pred2, ValueId(13))]
                );
            }
            other => panic!("Expected second Phi, got {:?}", other),
        }
    }

    #[test]
    fn test_emit_frag_branch_wire_conflict_fails() {
        use super::super::branch_stub::BranchStub;
        use super::super::frag::Frag;
        use std::collections::BTreeMap;

        // Setup: 同じ block に branch と wire（1 block = 1 terminator 違反）
        let mut function = create_test_function();
        let bb0 = BasicBlockId(0);
        let bb1 = BasicBlockId(1);
        let bb2 = BasicBlockId(2);
        function.add_block(BasicBlock::new(bb1));
        function.add_block(BasicBlock::new(bb2));

        let branch = BranchStub {
            from: bb0,
            cond: ValueId(100),
            then_target: bb1,
            then_args: EdgeArgs {
                layout: JumpArgsLayout::CarriersOnly,
                values: vec![],
            },
            else_target: bb2,
            else_args: EdgeArgs {
                layout: JumpArgsLayout::CarriersOnly,
                values: vec![],
            },
        };

        let wire = EdgeStub::with_target(
            bb0, // 同じ from
            ExitKind::Normal,
            bb1,
            EdgeArgs {
                layout: JumpArgsLayout::CarriersOnly,
                values: vec![],
            },
        );

        let frag = Frag {
            entry: bb0,
            block_params: BTreeMap::new(),
            exits: BTreeMap::new(),
            wires: vec![wire],
            branches: vec![branch],
        };

        // Execute
        let result = emit_frag(&mut function, &frag);

        // Verify: failure
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("both wire and branch"));
    }

    #[test]
    fn test_compose_if_creates_branch() {
        use super::super::compose::if_;
        use super::super::frag::Frag;
        use std::collections::BTreeMap;

        // Setup: header, then, else, join blocks
        let header = BasicBlockId(0);
        let then_entry = BasicBlockId(1);
        let else_entry = BasicBlockId(2);
        let join_entry = BasicBlockId(3);

        let then_frag = Frag {
            entry: then_entry,
            block_params: BTreeMap::new(),
            exits: {
                let mut exits = BTreeMap::new();
                exits.insert(
                    ExitKind::Normal,
                    vec![EdgeStub::without_args(then_entry, ExitKind::Normal)],
                );
                exits
            },
            wires: vec![],
            branches: vec![],
        };

        let else_frag = Frag {
            entry: else_entry,
            block_params: BTreeMap::new(),
            exits: {
                let mut exits = BTreeMap::new();
                exits.insert(
                    ExitKind::Normal,
                    vec![EdgeStub::without_args(else_entry, ExitKind::Normal)],
                );
                exits
            },
            wires: vec![],
            branches: vec![],
        };

        let join_frag = Frag {
            entry: join_entry,
            block_params: BTreeMap::new(),
            exits: BTreeMap::new(),
            wires: vec![],
            branches: vec![],
        };

        let cond = ValueId(100);

        // Execute
        let result = if_(
            header,
            cond,
            then_frag,
            EdgeArgs {
                layout: JumpArgsLayout::CarriersOnly,
                values: vec![],
            },
            else_frag,
            EdgeArgs {
                layout: JumpArgsLayout::CarriersOnly,
                values: vec![],
            },
            join_frag,
        );

        // Verify: 1本の BranchStub が生成された
        assert_eq!(result.branches.len(), 1);

        let branch = &result.branches[0];
        assert_eq!(branch.from, header);
        assert_eq!(branch.cond, cond);
        assert_eq!(branch.then_target, then_entry);
        assert_eq!(branch.else_target, else_entry);
    }
