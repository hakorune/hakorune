use super::*;

#[test]
fn normal_package_routes_top_level_generic_g0_through_existing_terminal() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let source = "static function generic_g0(i: i64, j: i64): i64 { loop(i < 3) { loop(j < 3) { j = j + 1 } i = i + 1 } return j } static box Main { main() { return 0 } }";
        let mut compiler = MirCompiler::with_options(false);
        let result = compiler
            .compile_normal(published_request(source))
            .expect("normal package Generic G0 compile");
        let function = result
            .module
            .functions
            .get("generic_g0/2")
            .expect("top-level Generic G0 definition");
        assert_eq!(function.signature.params.len(), 2);
        assert!(function
            .blocks
            .values()
            .flat_map(|block| block.all_instructions())
            .any(|instruction| matches!(instruction, crate::mir::MirInstruction::Phi { .. })));
    });
}

#[test]
fn normal_ingress_routes_app_main_static_loop_child_through_callable_consumer() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for bound in [0, 1, 3] {
            let source = format!("function to_i64(value: i64): i64 {{ return value }} static box Main {{ main() {{ return 0 }} int_to_str(n: i64): i64 {{ local value = to_i64(n) local i = 0 loop(i < {bound}) {{ i = i + 1 }} return value }} }}");
            let mut compiler = MirCompiler::with_options(false);
            let result = compiler
                .compile_normal(published_request(&source))
                .expect("App Main static loop child compile");
            let helper = result
                .module
                .functions
                .get("Main.int_to_str/1")
                .expect("Main int_to_str definition");
            let phi_count = helper
                .blocks
                .values()
                .flat_map(|block| block.all_instructions())
                .filter(|instruction| matches!(instruction, crate::mir::MirInstruction::Phi { .. }))
                .count();
            assert!(phi_count > 0, "selected loop consumer must publish a PHI");
            let bound_constants = helper
                .blocks
                .values()
                .flat_map(|block| block.all_instructions())
                .filter_map(|instruction| match instruction {
                    crate::mir::MirInstruction::Const {
                        value: crate::mir::ConstValue::Integer(value),
                        ..
                    } => Some(*value),
                    _ => None,
                })
                .collect::<Vec<_>>();
            assert!(
                bound_constants.contains(&bound),
                "selected consumer must retain condition bound {bound}: {bound_constants:?}"
            );
            let (header_id, condition_lhs, body_id, exit_id) = helper
                .blocks
                .iter()
                .find_map(|(block_id, block)| {
                    let condition = block.all_instructions().find_map(|instruction| {
                        let crate::mir::MirInstruction::Compare { lhs, rhs, op, .. } = instruction
                        else {
                            return None;
                        };
                        if *op != crate::mir::CompareOp::Lt
                            || !block.all_instructions().any(|candidate| match candidate {
                                crate::mir::MirInstruction::Const {
                                    dst,
                                    value: crate::mir::ConstValue::Integer(value),
                                } => *dst == *rhs && *value == bound,
                                _ => false,
                            })
                        {
                            return None;
                        }
                        Some(*lhs)
                    })?;
                    let (then_bb, else_bb) = block.all_instructions().find_map(|instruction| {
                        let crate::mir::MirInstruction::Branch {
                            then_bb, else_bb, ..
                        } = instruction
                        else {
                            return None;
                        };
                        Some((*then_bb, *else_bb))
                    })?;
                    Some((*block_id, condition, then_bb, else_bb))
                })
                .expect("selected loop header with bound branch");
            let header = helper.blocks.get(&header_id).expect("loop header");
            let preheader_id = *header
                .predecessors
                .iter()
                .find(|candidate| **candidate != body_id)
                .expect("loop preheader");
            let condition_phi = header
                .all_instructions()
                .find_map(|instruction| match instruction {
                    crate::mir::MirInstruction::Phi { dst, inputs, .. }
                        if *dst == condition_lhs =>
                    {
                        Some(inputs)
                    }
                    _ => None,
                })
                .expect("header PHI for condition");
            assert!(condition_phi.iter().any(|(pred, _)| *pred == preheader_id));
            assert!(condition_phi.iter().any(|(pred, _)| *pred == body_id));
            let body = helper.blocks.get(&body_id).expect("loop body");
            let body_phi = body
                .all_instructions()
                .find_map(|instruction| match instruction {
                    crate::mir::MirInstruction::Phi { dst, inputs, .. }
                        if inputs
                            .iter()
                            .any(|(pred, value)| *pred == header_id && *value == condition_lhs) =>
                    {
                        Some(*dst)
                    }
                    _ => None,
                })
                .expect("body PHI consumes header generation");
            assert!(body.all_instructions().any(|instruction| {
                match instruction {
                    crate::mir::MirInstruction::BinOp {
                        op: crate::mir::BinaryOp::Add,
                        lhs,
                        ..
                    } => *lhs == body_phi,
                    _ => false,
                }
            }));
            let exit = helper.blocks.get(&exit_id).expect("loop exit");
            let returned = exit
                .all_instructions()
                .find_map(|instruction| match instruction {
                    crate::mir::MirInstruction::Return { value } => *value,
                    _ => None,
                })
                .expect("loop exit return");
            assert!(header.all_instructions().any(|instruction| {
                matches!(instruction, crate::mir::MirInstruction::Phi { dst, inputs, .. }
                    if *dst == returned
                        && inputs.iter().any(|(pred, _)| *pred == preheader_id)
                        && inputs.iter().any(|(pred, _)| *pred == body_id))
            }));
        }
    });
}

#[test]
fn optimized_pair_root_with_callable_loop_child_reaches_physical_abi() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let source = "function to_i64(value: i64): i64 { return value } box Pair { left: i64 right: i64 birth(left, right) { me.left = left me.right = right } } static box Main { main() { local pair = new Pair(10, 20) return pair.left + pair.right } int_to_str(n: i64): i64 { local value = to_i64(n) local i = 0 loop(i < 3) { i = i + 1 } return value } }";
        let mut compiler = MirCompiler::new();
        compiler
            .compile_normal_with_published(published_request(source), |view, verification| {
                assert!(verification.is_ok(), "{verification:?}");
                let helper = view
                    .module()
                    .functions
                    .get("Main.int_to_str/1")
                    .expect("selected loop child");
                assert!(helper
                    .blocks
                    .values()
                    .flat_map(|block| block.all_instructions())
                    .any(|instruction| matches!(
                        instruction,
                        crate::mir::MirInstruction::Phi { .. }
                    )));
                let input = view.issue_lifecycle_physical_abi_input()?;
                assert_eq!(
                    input.entry().root_result(),
                    published_backend_view::CompiledEntryRootResultV1::I64
                );
                Ok::<(), String>(())
            })
            .expect("optimized Pair-plus-Loop module must reach physical ABI");
    });
}
