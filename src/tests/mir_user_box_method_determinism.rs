#[cfg(test)]
mod tests {
    use crate::mir::definitions::call_unified::TypeCertainty;
    use crate::mir::{Callee, MirCompiler, MirInstruction, MirType, ValueId};
    use crate::parser::NyashParser;
    use std::sync::Once;

    fn ensure_ring0_initialized() {
        use crate::runtime::ring0::{default_ring0, init_global_ring0};
        static INIT: Once = Once::new();
        INIT.call_once(|| {
            init_global_ring0(default_ring0());
        });
    }

    fn compile_counter_step_chain() -> crate::mir::MirCompileResult {
        ensure_ring0_initialized();
        std::env::set_var("NYASH_FEATURES", "stage3");
        let src = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/benchmarks/bench_kilo_micro_userbox_counter_step_chain.hako"
        ));
        let ast = NyashParser::parse_from_string(src).expect("parse counter_step_chain benchmark");
        let mut compiler = MirCompiler::new();
        compiler
            .compile(ast)
            .expect("compile counter_step_chain benchmark")
    }

    #[test]
    fn counter_step_chain_uses_known_receiver_method_shape() {
        for _ in 0..8 {
            let compile_result = compile_counter_step_chain();
            let func = compile_result
                .module
                .functions
                .get("Counter.step_chain/0")
                .expect("Counter.step_chain/0 must exist");

            let has_known_receiver_call = func.blocks.values().any(|block| {
                block.all_spanned_instructions().any(|sp| {
                    matches!(
                        &sp.inst,
                        MirInstruction::Call {
                            callee: Some(Callee::Method {
                                box_name,
                                method,
                                receiver: Some(_),
                                certainty: TypeCertainty::Known,
                                ..
                            }),
                            args,
                            ..
                        } if box_name == "Counter" && method == "step" && args.is_empty()
                    )
                })
            });
            assert!(
                has_known_receiver_call,
                "Counter.step_chain/0 must lower through a known-receiver method call"
            );

            let has_global_fallback = func.blocks.values().any(|block| {
                block.all_spanned_instructions().any(|sp| {
                    matches!(
                        &sp.inst,
                        MirInstruction::Call {
                            callee: Some(Callee::Global(name)),
                            ..
                        } if name == "Counter.step/0"
                    )
                })
            });
            assert!(
                !has_global_fallback,
                "Counter.step_chain/0 must not fall back to Global(\"Counter.step/0\")"
            );
        }
    }

    #[test]
    fn counter_step_chain_persists_receiver_box_type_metadata() {
        let compile_result = compile_counter_step_chain();
        let func = compile_result
            .module
            .functions
            .get("Counter.step_chain/0")
            .expect("Counter.step_chain/0 must exist");

        assert_eq!(
            func.metadata.value_types.get(&ValueId::new(0)),
            Some(&MirType::Box("Counter".to_string())),
            "instance method receiver %0 must keep Box(Counter) metadata"
        );

        let has_typed_receiver_copy = func.blocks.values().any(|block| {
            block.instructions.iter().any(|inst| {
                matches!(
                    inst,
                    MirInstruction::Copy { dst, src: ValueId(0) }
                        if func.metadata.value_types.get(dst)
                            == Some(&MirType::Box("Counter".to_string()))
                )
            })
        });
        assert!(
            has_typed_receiver_copy,
            "receiver copies must preserve Box(Counter) metadata for callsite canonicalization"
        );
    }
}
