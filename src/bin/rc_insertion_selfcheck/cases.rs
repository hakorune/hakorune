use super::helpers::*;
use nyash_rust::ast::Span;
use nyash_rust::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
use nyash_rust::mir::types::ConstValue;
use nyash_rust::mir::{BasicBlock, BasicBlockId, Callee, EdgeArgs, EffectMask, MirInstruction, ValueId};

pub fn main() {
    // Case 1: Overwrite release (Store -> Store)
    let ptr = ValueId::new(100);
    let v1 = ValueId::new(1);
    let v2 = ValueId::new(2);

    let mut block = BasicBlock::new(BasicBlockId::new(0));
    block.instructions = vec![
        MirInstruction::Store { value: v1, ptr },
        MirInstruction::Store { value: v2, ptr },
    ];
    block.instruction_spans = vec![Span::unknown()];
    let entry = block.id;
    let module = build_module_with_block(block, "selfcheck_overwrite", "selfcheck_mod1");
    assert_release_inserted(module, "selfcheck_overwrite", entry, 1, "overwrite");

    // Case 2: Explicit drop (Store -> Const null -> Store null)
    let ptr = ValueId::new(200);
    let v1 = ValueId::new(10);
    let null_v = ValueId::new(11);

    let mut block = BasicBlock::new(BasicBlockId::new(0));
    block.instructions = vec![
        MirInstruction::Store { value: v1, ptr },
        MirInstruction::Const {
            dst: null_v,
            value: ConstValue::Null,
        },
        MirInstruction::Store { value: null_v, ptr },
    ];
    block.instruction_spans = vec![Span::unknown()];
    let entry = block.id;
    let module = build_module_with_block(block, "selfcheck_drop", "selfcheck_mod2");
    assert_release_inserted(module, "selfcheck_drop", entry, 1, "explicit_drop");

    // Case 3: Block-end cleanup on Return (remaining tracked values)
    let ptr = ValueId::new(300);
    let v1 = ValueId::new(20);

    let mut block = BasicBlock::new(BasicBlockId::new(0));
    block.instructions = vec![MirInstruction::Store { value: v1, ptr }];
    block.instruction_spans = vec![Span::unknown()];
    block.terminator = Some(MirInstruction::Return { value: None });
    block.terminator_span = Some(Span::unknown());
    let entry = block.id;
    let module = build_module_with_block(block, "selfcheck_return_cleanup", "selfcheck_mod3");
    assert_release_inserted(
        module,
        "selfcheck_return_cleanup",
        entry,
        1,
        "return_cleanup",
    );
    println!("[rc_phase2_queue/case] early_exit=ok");
    println!("[rc_three_rules] overwrite=ok explicit_drop=ok scope_end=ok");

    // Case 3.1: phase2 call queue ordering
    // Queue contract:
    // - call itself stays in-place.
    // - overwrite release is inserted before Store(new) (instruction queue).
    // - return cleanup release is inserted before Return (early-exit queue).
    let ptr = ValueId::new(340);
    let v_old = ValueId::new(44);
    let v_new = ValueId::new(45);
    let call_func = ValueId::new(9045);

    let mut block = BasicBlock::new(BasicBlockId::new(0));
    block.instructions = vec![
        MirInstruction::Store { value: v_old, ptr },
        MirInstruction::Call {
            dst: Some(v_new),
            func: call_func,
            callee: Some(Callee::Global("selfcheck.phase2.queue_call".to_string())),
            args: vec![],
            effects: EffectMask::PURE,
        },
        MirInstruction::Store { value: v_new, ptr },
    ];
    block.instruction_spans = vec![Span::unknown(), Span::unknown(), Span::unknown()];
    block.terminator = Some(MirInstruction::Return { value: None });
    block.terminator_span = Some(Span::unknown());
    let entry = block.id;
    let module = build_module_with_block(block, "selfcheck_call_phase2_queue", "selfcheck_mod3p1");
    assert_call_overwrite_and_return_queue_order(
        module,
        "selfcheck_call_phase2_queue",
        entry,
        v_old,
        v_new,
        "call_phase2_queue",
    );
    println!("[rc_phase2_queue/case] call=ok");

    // Case 3.5: Jump -> Return single-predecessor propagation
    let ptr = ValueId::new(350);
    let v1 = ValueId::new(21);

    let mut block_a = BasicBlock::new(BasicBlockId::new(0));
    block_a.instructions = vec![MirInstruction::Store { value: v1, ptr }];
    block_a.instruction_spans = vec![Span::unknown()];
    block_a.terminator = Some(MirInstruction::Jump {
        target: BasicBlockId::new(1),
        edge_args: None,
    });
    block_a.terminator_span = Some(Span::unknown());

    let mut block_b = BasicBlock::new(BasicBlockId::new(1));
    block_b.instructions = vec![];
    block_b.instruction_spans = vec![];
    block_b.terminator = Some(MirInstruction::Return { value: None });
    block_b.terminator_span = Some(Span::unknown());

    let module = build_module_with_blocks(
        vec![block_a, block_b],
        BasicBlockId::new(0),
        "selfcheck_jump_return_single_pred",
        "selfcheck_mod3p5",
    );
    assert_release_counts_in_blocks(
        module,
        "selfcheck_jump_return_single_pred",
        1,
        &[(BasicBlockId::new(0), 0), (BasicBlockId::new(1), 1)],
        "jump_return_single_pred",
    );

    // Case 3.6: Jump chain -> Return (single-predecessor only)
    let ptr = ValueId::new(360);
    let v1 = ValueId::new(22);

    let mut block_a = BasicBlock::new(BasicBlockId::new(0));
    block_a.instructions = vec![MirInstruction::Store { value: v1, ptr }];
    block_a.instruction_spans = vec![Span::unknown()];
    block_a.terminator = Some(MirInstruction::Jump {
        target: BasicBlockId::new(1),
        edge_args: None,
    });
    block_a.terminator_span = Some(Span::unknown());

    let mut block_b = BasicBlock::new(BasicBlockId::new(1));
    block_b.instructions = vec![];
    block_b.instruction_spans = vec![];
    block_b.terminator = Some(MirInstruction::Jump {
        target: BasicBlockId::new(2),
        edge_args: None,
    });
    block_b.terminator_span = Some(Span::unknown());

    let mut block_c = BasicBlock::new(BasicBlockId::new(2));
    block_c.instructions = vec![];
    block_c.instruction_spans = vec![];
    block_c.terminator = Some(MirInstruction::Return { value: None });
    block_c.terminator_span = Some(Span::unknown());

    let module = build_module_with_blocks(
        vec![block_a, block_b, block_c],
        BasicBlockId::new(0),
        "selfcheck_jump_chain_single_pred",
        "selfcheck_mod3p6",
    );
    assert_release_counts_in_blocks(
        module,
        "selfcheck_jump_chain_single_pred",
        1,
        &[
            (BasicBlockId::new(0), 0),
            (BasicBlockId::new(1), 0),
            (BasicBlockId::new(2), 1),
        ],
        "jump_chain_single_pred",
    );

    // Case 3.7: Multi-predecessor Return with MATCHING states
    // P5: すべての incoming end_state が完全一致する場合のみ ReturnCleanup を成立させる
    let ptr = ValueId::new(370);
    let v_shared = ValueId::new(23); // 同じValueIdを両predecessorで使用

    let mut block_a = BasicBlock::new(BasicBlockId::new(0));
    block_a.instructions = vec![MirInstruction::Store {
        value: v_shared,
        ptr,
    }];
    block_a.instruction_spans = vec![Span::unknown()];
    block_a.terminator = Some(MirInstruction::Jump {
        target: BasicBlockId::new(2),
        edge_args: None,
    });
    block_a.terminator_span = Some(Span::unknown());

    let mut block_b = BasicBlock::new(BasicBlockId::new(1));
    block_b.instructions = vec![MirInstruction::Store {
        value: v_shared,
        ptr,
    }];
    block_b.instruction_spans = vec![Span::unknown()];
    block_b.terminator = Some(MirInstruction::Jump {
        target: BasicBlockId::new(2),
        edge_args: None,
    });
    block_b.terminator_span = Some(Span::unknown());

    let mut block_ret = BasicBlock::new(BasicBlockId::new(2));
    block_ret.instructions = vec![];
    block_ret.instruction_spans = vec![];
    block_ret.terminator = Some(MirInstruction::Return { value: None });
    block_ret.terminator_span = Some(Span::unknown());

    let module = build_module_with_blocks(
        vec![block_a, block_b, block_ret],
        BasicBlockId::new(0),
        "selfcheck_multi_pred_return_match",
        "selfcheck_mod3p7",
    );
    assert_release_counts_in_blocks(
        module,
        "selfcheck_multi_pred_return_match",
        1, // 全体で1（Return blockのみ）
        &[
            (BasicBlockId::new(0), 0), // Jump block: cleanup禁止
            (BasicBlockId::new(1), 0), // Jump block: cleanup禁止
            (BasicBlockId::new(2), 1), // Return block: multi-pred join成功
        ],
        "multi_pred_return_match",
    );

    // Case 3.8: Multi-predecessor Return with MISMATCHING states (negative test)
    // P5: state不一致 → join state を作らない → ReturnCleanup なし
    let ptr = ValueId::new(380);
    let v1 = ValueId::new(24);
    let v2 = ValueId::new(25); // 異なるValueId → state不一致

    let mut block_a = BasicBlock::new(BasicBlockId::new(0));
    block_a.instructions = vec![MirInstruction::Store { value: v1, ptr }];
    block_a.instruction_spans = vec![Span::unknown()];
    block_a.terminator = Some(MirInstruction::Jump {
        target: BasicBlockId::new(2),
        edge_args: None,
    });
    block_a.terminator_span = Some(Span::unknown());

    let mut block_b = BasicBlock::new(BasicBlockId::new(1));
    block_b.instructions = vec![MirInstruction::Store { value: v2, ptr }]; // v2 != v1 → 不一致
    block_b.instruction_spans = vec![Span::unknown()];
    block_b.terminator = Some(MirInstruction::Jump {
        target: BasicBlockId::new(2),
        edge_args: None,
    });
    block_b.terminator_span = Some(Span::unknown());

    let mut block_ret = BasicBlock::new(BasicBlockId::new(2));
    block_ret.instructions = vec![];
    block_ret.instruction_spans = vec![];
    block_ret.terminator = Some(MirInstruction::Return { value: None });
    block_ret.terminator_span = Some(Span::unknown());

    let module = build_module_with_blocks(
        vec![block_a, block_b, block_ret],
        BasicBlockId::new(0),
        "selfcheck_multi_pred_return_mismatch",
        "selfcheck_mod3p8",
    );
    assert_release_counts_in_blocks(
        module,
        "selfcheck_multi_pred_return_mismatch",
        0, // 全体で0（join stateを作らないのでcleanupなし）
        &[
            (BasicBlockId::new(0), 0), // Jump block: cleanup禁止
            (BasicBlockId::new(1), 0), // Jump block: cleanup禁止
            (BasicBlockId::new(2), 0), // Return block: state不一致 → join失敗
        ],
        "multi_pred_return_mismatch",
    );

    // Case 3.9: Multi-predecessor Return with PARTIAL MATCH (intersection)
    // P6: ptr1=v1 は一致、ptr2 は値が違う → intersection = {ptr1=v1} のみ cleanup
    let ptr1 = ValueId::new(391);
    let ptr2 = ValueId::new(392);
    let v1 = ValueId::new(26);
    let v2 = ValueId::new(27);
    let v3 = ValueId::new(28);

    let mut block_a = BasicBlock::new(BasicBlockId::new(0));
    block_a.instructions = vec![
        MirInstruction::Store {
            value: v1,
            ptr: ptr1,
        },
        MirInstruction::Store {
            value: v2,
            ptr: ptr2,
        },
    ];
    block_a.instruction_spans = vec![Span::unknown(), Span::unknown()];
    block_a.terminator = Some(MirInstruction::Jump {
        target: BasicBlockId::new(2),
        edge_args: None,
    });
    block_a.terminator_span = Some(Span::unknown());

    let mut block_b = BasicBlock::new(BasicBlockId::new(1));
    block_b.instructions = vec![
        MirInstruction::Store {
            value: v1,
            ptr: ptr1,
        }, // ptr1=v1 (一致)
        MirInstruction::Store {
            value: v3,
            ptr: ptr2,
        }, // ptr2=v3 (不一致)
    ];
    block_b.instruction_spans = vec![Span::unknown(), Span::unknown()];
    block_b.terminator = Some(MirInstruction::Jump {
        target: BasicBlockId::new(2),
        edge_args: None,
    });
    block_b.terminator_span = Some(Span::unknown());

    let mut block_ret = BasicBlock::new(BasicBlockId::new(2));
    block_ret.instructions = vec![];
    block_ret.instruction_spans = vec![];
    block_ret.terminator = Some(MirInstruction::Return { value: None });
    block_ret.terminator_span = Some(Span::unknown());

    let module = build_module_with_blocks(
        vec![block_a, block_b, block_ret],
        BasicBlockId::new(0),
        "selfcheck_intersection_partial",
        "selfcheck_mod3p9",
    );
    assert_release_counts_in_blocks(
        module,
        "selfcheck_intersection_partial",
        1, // 全体で1（intersection = v1 のみ）
        &[
            (BasicBlockId::new(0), 0), // Jump block: cleanup禁止
            (BasicBlockId::new(1), 0), // Jump block: cleanup禁止
            (BasicBlockId::new(2), 1), // Return block: intersection で cleanup
        ],
        "intersection_partial",
    );

    // Case 3.10: Multi-predecessor Return with EMPTY intersection
    // P6: 同じ ptr だが value が違う → intersection が空 → cleanup なし
    let ptr = ValueId::new(3100);
    let v1 = ValueId::new(29);
    let v2 = ValueId::new(30);

    let mut block_a = BasicBlock::new(BasicBlockId::new(0));
    block_a.instructions = vec![MirInstruction::Store { value: v1, ptr }];
    block_a.instruction_spans = vec![Span::unknown()];
    block_a.terminator = Some(MirInstruction::Jump {
        target: BasicBlockId::new(2),
        edge_args: None,
    });
    block_a.terminator_span = Some(Span::unknown());

    let mut block_b = BasicBlock::new(BasicBlockId::new(1));
    block_b.instructions = vec![MirInstruction::Store { value: v2, ptr }]; // 同じ ptr、違う value
    block_b.instruction_spans = vec![Span::unknown()];
    block_b.terminator = Some(MirInstruction::Jump {
        target: BasicBlockId::new(2),
        edge_args: None,
    });
    block_b.terminator_span = Some(Span::unknown());

    let mut block_ret = BasicBlock::new(BasicBlockId::new(2));
    block_ret.instructions = vec![];
    block_ret.instruction_spans = vec![];
    block_ret.terminator = Some(MirInstruction::Return { value: None });
    block_ret.terminator_span = Some(Span::unknown());

    let module = build_module_with_blocks(
        vec![block_a, block_b, block_ret],
        BasicBlockId::new(0),
        "selfcheck_intersection_empty",
        "selfcheck_mod3p10",
    );
    assert_release_counts_in_blocks(
        module,
        "selfcheck_intersection_empty",
        0, // 全体で0（intersection が空）
        &[
            (BasicBlockId::new(0), 0), // Jump block: cleanup禁止
            (BasicBlockId::new(1), 0), // Jump block: cleanup禁止
            (BasicBlockId::new(2), 0), // Return block: intersection 空 → cleanup なし
        ],
        "intersection_empty",
    );

    // Case 3.10b: Break-edge cleanup on empty Jump -> multi-pred Return
    // X9-min: break dispatcher block（empty Jump）にのみ pred-local release を挿入し、
    // Return block は intersection 分のみ ReturnCleanup を維持する。
    //
    // Shape:
    //   bb0(entry):       store outer -> jump bb1
    //   bb1(loop header): branch(cond, bb2 body, bb4 return)
    //   bb2(loop body):   store tmp   -> jump bb3
    //   bb3(break jump):  jump bb4    (empty jump block)
    //   bb4(return):      return
    //
    // Expected:
    //   - bb3: ReleaseStrong 1 (tmp: break-edge cleanup)
    //   - bb4: ReleaseStrong 0 (this shape intentionally tracks only break-edge delta)
    //   - total: 1
    let ptr_outer = ValueId::new(3200);
    let ptr_tmp = ValueId::new(3201);
    let v_outer = ValueId::new(31);
    let v_tmp = ValueId::new(32);
    let cond_true = ValueId::new(33);

    let mut block0 = BasicBlock::new(BasicBlockId::new(0));
    block0.instructions = vec![MirInstruction::Store {
        value: v_outer,
        ptr: ptr_outer,
    }];
    block0.instruction_spans = vec![Span::unknown()];
    block0.terminator = Some(MirInstruction::Jump {
        target: BasicBlockId::new(1),
        edge_args: None,
    });
    block0.terminator_span = Some(Span::unknown());

    let mut block1 = BasicBlock::new(BasicBlockId::new(1));
    block1.instructions = vec![MirInstruction::Const {
        dst: cond_true,
        value: ConstValue::Bool(true),
    }];
    block1.instruction_spans = vec![Span::unknown()];
    block1.terminator = Some(MirInstruction::Branch {
        condition: cond_true,
        then_bb: BasicBlockId::new(2),
        else_bb: BasicBlockId::new(4),
        then_edge_args: None,
        else_edge_args: None,
    });
    block1.terminator_span = Some(Span::unknown());

    let mut block2 = BasicBlock::new(BasicBlockId::new(2));
    block2.instructions = vec![MirInstruction::Store {
        value: v_tmp,
        ptr: ptr_tmp,
    }];
    block2.instruction_spans = vec![Span::unknown()];
    block2.terminator = Some(MirInstruction::Jump {
        target: BasicBlockId::new(3),
        edge_args: None,
    });
    block2.terminator_span = Some(Span::unknown());

    let mut block3 = BasicBlock::new(BasicBlockId::new(3));
    block3.instructions = vec![];
    block3.instruction_spans = vec![];
    block3.terminator = Some(MirInstruction::Jump {
        target: BasicBlockId::new(4),
        edge_args: None,
    });
    block3.terminator_span = Some(Span::unknown());

    let mut block4 = BasicBlock::new(BasicBlockId::new(4));
    block4.instructions = vec![];
    block4.instruction_spans = vec![];
    block4.terminator = Some(MirInstruction::Return { value: None });
    block4.terminator_span = Some(Span::unknown());

    let module = build_module_with_blocks(
        vec![block0, block1, block2, block3, block4],
        BasicBlockId::new(0),
        "selfcheck_break_cleanup_min",
        "selfcheck_mod3p10b",
    );
    assert_release_counts_in_blocks(
        module,
        "selfcheck_break_cleanup_min",
        1,
        &[
            (BasicBlockId::new(0), 0),
            (BasicBlockId::new(1), 0),
            (BasicBlockId::new(2), 0),
            (BasicBlockId::new(3), 1),
            (BasicBlockId::new(4), 0),
        ],
        "break_cleanup_min",
    );

    // Case 3.10c: Continue-edge cleanup on empty Jump -> multi-pred Branch header
    // X10-min: continue dispatcher block（empty Jump）にのみ pred-local release を挿入する。
    //
    // Shape:
    //   bb0(entry):          store outer -> jump bb1
    //   bb1(loop header):    branch(cond, bb2 body, bb4 return)
    //   bb2(loop body):      store tmp   -> jump bb3
    //   bb3(continue jump):  jump bb1    (empty jump block)
    //   bb4(return):         return
    //
    // Expected:
    //   - bb3: ReleaseStrong 1 (tmp: continue-edge cleanup)
    //   - total: 1
    let ptr_outer = ValueId::new(3300);
    let ptr_tmp = ValueId::new(3301);
    let v_outer = ValueId::new(34);
    let v_tmp = ValueId::new(35);
    let cond_true = ValueId::new(36);

    let mut block0 = BasicBlock::new(BasicBlockId::new(0));
    block0.instructions = vec![MirInstruction::Store {
        value: v_outer,
        ptr: ptr_outer,
    }];
    block0.instruction_spans = vec![Span::unknown()];
    block0.terminator = Some(MirInstruction::Jump {
        target: BasicBlockId::new(1),
        edge_args: None,
    });
    block0.terminator_span = Some(Span::unknown());

    let mut block1 = BasicBlock::new(BasicBlockId::new(1));
    block1.instructions = vec![MirInstruction::Const {
        dst: cond_true,
        value: ConstValue::Bool(true),
    }];
    block1.instruction_spans = vec![Span::unknown()];
    block1.terminator = Some(MirInstruction::Branch {
        condition: cond_true,
        then_bb: BasicBlockId::new(2),
        else_bb: BasicBlockId::new(4),
        then_edge_args: None,
        else_edge_args: None,
    });
    block1.terminator_span = Some(Span::unknown());

    let mut block2 = BasicBlock::new(BasicBlockId::new(2));
    block2.instructions = vec![MirInstruction::Store {
        value: v_tmp,
        ptr: ptr_tmp,
    }];
    block2.instruction_spans = vec![Span::unknown()];
    block2.terminator = Some(MirInstruction::Jump {
        target: BasicBlockId::new(3),
        edge_args: None,
    });
    block2.terminator_span = Some(Span::unknown());

    let mut block3 = BasicBlock::new(BasicBlockId::new(3));
    block3.instructions = vec![];
    block3.instruction_spans = vec![];
    block3.terminator = Some(MirInstruction::Jump {
        target: BasicBlockId::new(1),
        edge_args: None,
    });
    block3.terminator_span = Some(Span::unknown());

    let mut block4 = BasicBlock::new(BasicBlockId::new(4));
    block4.instructions = vec![];
    block4.instruction_spans = vec![];
    block4.terminator = Some(MirInstruction::Return { value: None });
    block4.terminator_span = Some(Span::unknown());

    let module = build_module_with_blocks(
        vec![block0, block1, block2, block3, block4],
        BasicBlockId::new(0),
        "selfcheck_continue_cleanup_min",
        "selfcheck_mod3p10c",
    );
    assert_release_counts_in_blocks(
        module,
        "selfcheck_continue_cleanup_min",
        1,
        &[
            (BasicBlockId::new(0), 0),
            (BasicBlockId::new(1), 0),
            (BasicBlockId::new(2), 0),
            (BasicBlockId::new(3), 1),
            (BasicBlockId::new(4), 0),
        ],
        "continue_cleanup_min",
    );
    println!("[rc_phase2_queue/case] loop=ok");

    // Case 3.10d: X11 PHI/edge verifier (edge_args contract)
    // Contradiction shape:
    // - break cleanup would release v_tmp on bb3
    // - but bb3->bb4 edge forwards v_tmp via edge_args
    // -> must fail-fast with rc_insertion/phi_edge_mismatch tag.
    let ptr_outer = ValueId::new(3400);
    let ptr_tmp = ValueId::new(3401);
    let v_outer = ValueId::new(37);
    let v_tmp = ValueId::new(38);
    let cond_true = ValueId::new(39);

    let mut block0 = BasicBlock::new(BasicBlockId::new(0));
    block0.instructions = vec![MirInstruction::Store {
        value: v_outer,
        ptr: ptr_outer,
    }];
    block0.instruction_spans = vec![Span::unknown()];
    block0.terminator = Some(MirInstruction::Jump {
        target: BasicBlockId::new(1),
        edge_args: None,
    });
    block0.terminator_span = Some(Span::unknown());

    let mut block1 = BasicBlock::new(BasicBlockId::new(1));
    block1.instructions = vec![MirInstruction::Const {
        dst: cond_true,
        value: ConstValue::Bool(true),
    }];
    block1.instruction_spans = vec![Span::unknown()];
    block1.terminator = Some(MirInstruction::Branch {
        condition: cond_true,
        then_bb: BasicBlockId::new(2),
        else_bb: BasicBlockId::new(4),
        then_edge_args: None,
        else_edge_args: None,
    });
    block1.terminator_span = Some(Span::unknown());

    let mut block2 = BasicBlock::new(BasicBlockId::new(2));
    block2.instructions = vec![MirInstruction::Store {
        value: v_tmp,
        ptr: ptr_tmp,
    }];
    block2.instruction_spans = vec![Span::unknown()];
    block2.terminator = Some(MirInstruction::Jump {
        target: BasicBlockId::new(3),
        edge_args: None,
    });
    block2.terminator_span = Some(Span::unknown());

    let mut block3 = BasicBlock::new(BasicBlockId::new(3));
    block3.instructions = vec![];
    block3.instruction_spans = vec![];
    block3.terminator = Some(MirInstruction::Jump {
        target: BasicBlockId::new(4),
        edge_args: Some(EdgeArgs {
            layout: JumpArgsLayout::CarriersOnly,
            values: vec![v_tmp],
        }),
    });
    block3.terminator_span = Some(Span::unknown());

    let mut block4 = BasicBlock::new(BasicBlockId::new(4));
    block4.instructions = vec![];
    block4.instruction_spans = vec![];
    block4.terminator = Some(MirInstruction::Return { value: None });
    block4.terminator_span = Some(Span::unknown());

    let module = build_module_with_blocks(
        vec![block0, block1, block2, block3, block4],
        BasicBlockId::new(0),
        "selfcheck_phi_edge_edge_args_contract",
        "selfcheck_mod3p10d",
    );
    assert_fail_fast_tag_from_insert(
        module,
        RC_PHI_EDGE_MISMATCH_TAG,
        "phi_edge_edge_args_contract",
    );

    // Case 3.10e: X11 PHI/edge verifier (phi-input contract)
    // Contradiction shape:
    // - continue cleanup would release v_tmp on bb3
    // - header bb1 has Phi input (bb3 -> v_tmp)
    // -> must fail-fast with rc_insertion/phi_edge_mismatch tag.
    let ptr_outer = ValueId::new(3500);
    let ptr_tmp = ValueId::new(3501);
    let v_outer = ValueId::new(40);
    let v_tmp = ValueId::new(41);
    let cond_true = ValueId::new(42);
    let phi_dst = ValueId::new(43);

    let mut block0 = BasicBlock::new(BasicBlockId::new(0));
    block0.instructions = vec![MirInstruction::Store {
        value: v_outer,
        ptr: ptr_outer,
    }];
    block0.instruction_spans = vec![Span::unknown()];
    block0.terminator = Some(MirInstruction::Jump {
        target: BasicBlockId::new(1),
        edge_args: None,
    });
    block0.terminator_span = Some(Span::unknown());

    let mut block1 = BasicBlock::new(BasicBlockId::new(1));
    block1.instructions = vec![
        MirInstruction::Phi {
            dst: phi_dst,
            inputs: vec![
                (BasicBlockId::new(0), v_outer),
                (BasicBlockId::new(3), v_tmp),
            ],
            type_hint: None,
        },
        MirInstruction::Const {
            dst: cond_true,
            value: ConstValue::Bool(true),
        },
    ];
    block1.instruction_spans = vec![Span::unknown(), Span::unknown()];
    block1.terminator = Some(MirInstruction::Branch {
        condition: cond_true,
        then_bb: BasicBlockId::new(2),
        else_bb: BasicBlockId::new(4),
        then_edge_args: None,
        else_edge_args: None,
    });
    block1.terminator_span = Some(Span::unknown());

    let mut block2 = BasicBlock::new(BasicBlockId::new(2));
    block2.instructions = vec![MirInstruction::Store {
        value: v_tmp,
        ptr: ptr_tmp,
    }];
    block2.instruction_spans = vec![Span::unknown()];
    block2.terminator = Some(MirInstruction::Jump {
        target: BasicBlockId::new(3),
        edge_args: None,
    });
    block2.terminator_span = Some(Span::unknown());

    let mut block3 = BasicBlock::new(BasicBlockId::new(3));
    block3.instructions = vec![];
    block3.instruction_spans = vec![];
    block3.terminator = Some(MirInstruction::Jump {
        target: BasicBlockId::new(1),
        edge_args: None,
    });
    block3.terminator_span = Some(Span::unknown());

    let mut block4 = BasicBlock::new(BasicBlockId::new(4));
    block4.instructions = vec![];
    block4.instruction_spans = vec![];
    block4.terminator = Some(MirInstruction::Return { value: None });
    block4.terminator_span = Some(Span::unknown());

    let module = build_module_with_blocks(
        vec![block0, block1, block2, block3, block4],
        BasicBlockId::new(0),
        "selfcheck_phi_edge_phi_input_contract",
        "selfcheck_mod3p10e",
    );
    assert_fail_fast_tag_from_insert(
        module,
        RC_PHI_EDGE_MISMATCH_TAG,
        "phi_edge_phi_input_contract",
    );

    // Case 4: Jump terminator should NOT inject block-end cleanup (unsafe cross-block)
    let ptr = ValueId::new(400);
    let v1 = ValueId::new(30);

    let mut block = BasicBlock::new(BasicBlockId::new(0));
    block.instructions = vec![MirInstruction::Store { value: v1, ptr }];
    block.instruction_spans = vec![Span::unknown()];
    block.terminator = Some(MirInstruction::Jump {
        target: BasicBlockId::new(1),
        edge_args: None,
    });
    block.terminator_span = Some(Span::unknown());
    let entry = block.id;
    let module = build_module_with_block(block, "selfcheck_jump_skip", "selfcheck_mod4");
    assert_release_inserted(module, "selfcheck_jump_skip", entry, 0, "jump_no_cleanup");

    // Case 5: Branch terminator should NOT inject block-end cleanup (unsafe cross-block)
    let ptr = ValueId::new(500);
    let v1 = ValueId::new(40);
    let cond = ValueId::new(41);

    let mut block = BasicBlock::new(BasicBlockId::new(0));
    block.instructions = vec![MirInstruction::Store { value: v1, ptr }];
    block.instruction_spans = vec![Span::unknown()];
    block.terminator = Some(MirInstruction::Branch {
        condition: cond,
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    block.terminator_span = Some(Span::unknown());
    let entry = block.id;
    let module = build_module_with_block(block, "selfcheck_branch_skip", "selfcheck_mod5");
    assert_release_inserted(
        module,
        "selfcheck_branch_skip",
        entry,
        0,
        "branch_no_cleanup",
    );

    // Case 3.11: ReleaseStrong values are sorted (deterministic ordering)
    // P7: 複数 ptr に異なる順序で store し、ReturnCleanup の values が昇順になることを検証
    let ptr1 = ValueId::new(3111);
    let ptr2 = ValueId::new(3112);
    let v_high = ValueId::new(100); // 大きい index
    let v_low = ValueId::new(50); // 小さい index

    let mut block = BasicBlock::new(BasicBlockId::new(0));
    block.instructions = vec![
        MirInstruction::Store {
            value: v_high,
            ptr: ptr1,
        }, // 先に v100 を store
        MirInstruction::Store {
            value: v_low,
            ptr: ptr2,
        }, // 後に v50 を store
    ];
    block.instruction_spans = vec![Span::unknown(), Span::unknown()];
    block.terminator = Some(MirInstruction::Return { value: None });
    block.terminator_span = Some(Span::unknown());
    let entry = block.id;
    let module = build_module_with_block(block, "selfcheck_sorted_values", "selfcheck_mod3p11");

    // 関数全体の ReleaseStrong が昇順であることを検証
    assert_release_inserted(
        module.clone(),
        "selfcheck_sorted_values",
        entry,
        1,
        "sorted_values_count",
    );
    assert_all_release_values_sorted(module, "selfcheck_sorted_values", "sorted_values_order");

    // Case 3.12: Null propagation across Jump-chain
    // P8: null_val が Block A から Block B に伝播され、Block B で Store null_val が explicit drop として認識される
    // Block A: Store real_val → ptr1, Const null_val = Null, Store null_val → ptr1 (explicit drop)
    // Block B (Jump from A): Store real_val → ptr2, Store null_val → ptr2
    // → null_val は A から B に伝播
    // → Block B で ptr2 に null_val を Store すると explicit drop 扱い
    //
    // 期待:
    //   - Block A: ReleaseStrong 1 個（Store null_val → ptr1 の explicit drop で old(real_val) を release）
    //   - Block B: ReleaseStrong 1 個（Store null_val → ptr2 の explicit drop で old(real_val) を release）
    //   - ReturnCleanup: 0 個（ptr2 は null store で消えている）
    //   - 合計: 2 個
    //   - ❌ 失敗時: B=2（ReturnCleanup が残る → null 伝播が効いていない）
    let ptr1 = ValueId::new(3120);
    let ptr2 = ValueId::new(3121);
    let null_val = ValueId::new(3122);
    let real_val = ValueId::new(3123);

    let block_a_id = BasicBlockId::new(0);
    let block_b_id = BasicBlockId::new(1);

    // Block A
    let mut block_a = BasicBlock::new(block_a_id);
    block_a.instructions = vec![
        MirInstruction::Store {
            value: real_val,
            ptr: ptr1,
        },
        MirInstruction::Const {
            dst: null_val,
            value: ConstValue::Null,
        },
        MirInstruction::Store {
            value: null_val,
            ptr: ptr1,
        }, // explicit drop → ReleaseStrong 1
    ];
    block_a.instruction_spans = vec![Span::unknown(); 3];
    block_a.terminator = Some(MirInstruction::Jump {
        target: block_b_id,
        edge_args: None,
    });
    block_a.terminator_span = Some(Span::unknown());

    // Block B
    let mut block_b = BasicBlock::new(block_b_id);
    block_b.instructions = vec![
        MirInstruction::Store {
            value: real_val,
            ptr: ptr2,
        },
        MirInstruction::Store {
            value: null_val,
            ptr: ptr2,
        }, // null_val from A (propagated) → ReleaseStrong 1
    ];
    block_b.instruction_spans = vec![Span::unknown(); 2];
    block_b.terminator = Some(MirInstruction::Return { value: None });
    block_b.terminator_span = Some(Span::unknown());

    let module = build_module_with_blocks(
        vec![block_a, block_b],
        block_a_id,
        "selfcheck_null_propagation",
        "selfcheck_mod3p12",
    );

    // 検証: Block A = 1, Block B = 1（ReturnCleanup なし）
    // 失敗時は Block B = 2（ReturnCleanup が残る = null 伝播が効いていない）
    assert_release_counts_in_blocks(
        module,
        "selfcheck_null_propagation",
        2, // 合計 2（Block A=1 + Block B=1）
        &[
            (block_a_id, 1), // explicit drop
            (block_b_id, 1), // explicit drop (null propagated)
        ],
        "null_propagation",
    );

    println!("[rc_phase2_queue] loop=ok call=ok early_exit=ok");
    println!("[PASS] rc_insertion_selfcheck");
}
