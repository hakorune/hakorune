#[cfg(test)]
mod tests {

    use crate::mir::{MirCompiler, MirInstruction, MirType, ValueId};
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
