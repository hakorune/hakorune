#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::env;

    use crate::mir::basic_block::{BasicBlockId, EdgeArgs};
    use crate::mir::builder::control_flow::plan::edgecfg_facade::{
        BlockParams, ExitKind, Frag, FragEmitSession,
    };
    use crate::mir::builder::control_flow::plan::features::edgecfg_stubs;
    use crate::mir::function::{FunctionSignature, MirFunction};
    use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
    use crate::mir::types::MirType;
    use crate::mir::{BasicBlock, EffectMask, MirInstruction, ValueId};

    struct DemoIf2 {
        frag: Frag,
        function: MirFunction,
        then_bb: BasicBlockId,
        else_bb: BasicBlockId,
        join_bb: BasicBlockId,
        expr_param: ValueId,
        then_val: ValueId,
        else_val: ValueId,
    }

    fn strict_env_guard() -> impl Drop {
        env::set_var("NYASH_JOINIR_STRICT", "1");
        struct Guard;
        impl Drop for Guard {
            fn drop(&mut self) {
                let _ = env::remove_var("NYASH_JOINIR_STRICT");
            }
        }
        Guard
    }

    fn create_test_function() -> MirFunction {
        let signature = FunctionSignature {
            name: "value_join_demo_if2".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        MirFunction::new(signature, BasicBlockId(0))
    }

    fn build_demo_if2_valuejoin_frag() -> DemoIf2 {
        let mut function = create_test_function();
        let header = BasicBlockId(0);
        let then_bb = BasicBlockId(1);
        let else_bb = BasicBlockId(2);
        let join_bb = BasicBlockId(3);
        function.add_block(BasicBlock::new(then_bb));
        function.add_block(BasicBlock::new(else_bb));
        function.add_block(BasicBlock::new(join_bb));

        let cond = ValueId(10);
        let then_val = ValueId(11);
        let else_val = ValueId(12);
        let expr_param = ValueId(100);

        let branch = edgecfg_stubs::build_loop_cond_branch(header, cond, then_bb, else_bb);

        let wires = vec![
            edgecfg_stubs::build_loop_back_edge_with_args(
                then_bb,
                join_bb,
                EdgeArgs {
                    layout: JumpArgsLayout::ExprResultPlusCarriers,
                    values: vec![then_val],
                },
            ),
            edgecfg_stubs::build_loop_back_edge_with_args(
                else_bb,
                join_bb,
                EdgeArgs {
                    layout: JumpArgsLayout::ExprResultPlusCarriers,
                    values: vec![else_val],
                },
            ),
            edgecfg_stubs::build_edge_stub(
                join_bb,
                ExitKind::Return,
                None,
                EdgeArgs {
                    layout: JumpArgsLayout::CarriersOnly,
                    values: vec![expr_param],
                },
            ),
        ];

        let mut block_params = BTreeMap::new();
        block_params.insert(
            join_bb,
            BlockParams {
                layout: JumpArgsLayout::ExprResultPlusCarriers,
                params: vec![expr_param],
            },
        );

        let frag = Frag {
            entry: header,
            block_params,
            exits: BTreeMap::new(),
            wires,
            branches: vec![branch],
        };

        DemoIf2 {
            frag,
            function,
            then_bb,
            else_bb,
            join_bb,
            expr_param,
            then_val,
            else_val,
        }
    }

    #[test]
    fn demo_if2_valuejoin_emits_phi_and_return() {
        let _guard = strict_env_guard();
        let mut demo = build_demo_if2_valuejoin_frag();

        FragEmitSession::new()
            .emit_and_seal(&mut demo.function, &demo.frag)
            .expect("emit_and_seal should succeed");

        let join_block = demo
            .function
            .get_block(demo.join_bb)
            .expect("join block exists");

        let phi = join_block
            .instructions
            .iter()
            .find(|inst| matches!(inst, MirInstruction::Phi { dst, .. } if *dst == demo.expr_param))
            .expect("phi should be inserted at join");

        match phi {
            MirInstruction::Phi { inputs, .. } => {
                assert_eq!(
                    inputs,
                    &vec![(demo.then_bb, demo.then_val), (demo.else_bb, demo.else_val)]
                );
            }
            _ => unreachable!("phi matcher ensured MirInstruction::Phi"),
        }

        match &join_block.terminator {
            Some(MirInstruction::Return { value }) => {
                assert_eq!(*value, Some(demo.expr_param));
            }
            other => panic!("Expected Return terminator, got {:?}", other),
        }
    }
}
