use crate::helpers::*;
use nyash_rust::ast::Span;
use nyash_rust::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
use nyash_rust::mir::types::ConstValue;
use nyash_rust::mir::{BasicBlock, BasicBlockId, EdgeArgs, MirInstruction, ValueId};

pub(super) fn run() {
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
}
