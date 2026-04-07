use crate::helpers::*;
use nyash_rust::ast::Span;
use nyash_rust::mir::types::ConstValue;
use nyash_rust::mir::{BasicBlock, BasicBlockId, MirInstruction, ValueId};

pub(super) fn run() {
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
