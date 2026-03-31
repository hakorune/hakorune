use crate::helpers::*;
use nyash_rust::ast::Span;
use nyash_rust::mir::types::ConstValue;
use nyash_rust::mir::{BasicBlock, BasicBlockId, Callee, EffectMask, MirInstruction, ValueId};

pub(super) fn run() {
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
}
