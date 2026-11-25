//! Mutation checks at the source-issued comparison boundary.
use super::*;

#[test]
fn borrowed_integer_comparison_rejects_forged_final_mir() {
    use crate::mir::verification::borrowed_integer_comparison::verified_sites;
    use crate::mir::{BasicBlockId, MirInstruction, MirVerifier};

    crate::runtime::ring0::ensure_global_ring0_initialized();
    let text = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/apps/tests/corpus/dynamic-compare/run-integer.hako"
    ));
    let inclusive = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/apps/tests/corpus/dynamic-compare/dynamic-le-i64-integer.hako"
    ));
    let local = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/apps/tests/corpus/compare-normalize-fix/probes/matrix-mixed-lt-local.hako"
    ));
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for (source, mutations) in [
            (
                text,
                &[
                    "missing Fault edge",
                    "different integer",
                    "reversed operands",
                ][..],
            ),
            (inclusive, &["different operator"][..]),
            (local, &["unchecked comparison", "redirected Fault edge"][..]),
        ] {
            for optimize in [false, true] {
                let compiled = MirCompiler::with_options(optimize).compile_normal_with_published(
                    request(source),
                    |view, verification| -> Result<(), String> {
                        require_artifact_verification(verification);
                        let function = &view.module().functions["Probe.run/1"];
                        let sites = verified_sites(function).expect("matched source and comparison");
                        assert_eq!(sites.len(), 1);
                        let ((site_block, site_row), proof) = sites[0];
                        assert_eq!(proof.source.span().line, 4);
                        let normal = match function.blocks[&site_block].terminator.as_ref().unwrap() {
                            MirInstruction::Invoke { normal_landing, .. } => *normal_landing,
                            other => panic!("expected extraction, got {other:?}"),
                        };
                        let input = view.issue_lifecycle_physical_abi_input()?;
                        let function_index = input.program().functions().iter()
                            .position(|row| row.name() == function.signature.name).unwrap();
                        assert_eq!(input.diagnostic_sites().iter().filter(|site|
                            (site.function(), site.block(), site.instruction()) ==
                            (function_index as u32, site_block.0, site_row as u32)
                        ).count(), 1);

                        for &mutation in mutations {
                            let mut forged = function.clone();
                            if mutation == "missing Fault edge" {
                                let site = forged.blocks.get_mut(&site_block).unwrap();
                                let Some(MirInstruction::Invoke { fault_landing, .. }) = &mut site.terminator else {
                                    panic!("checked extraction");
                                };
                                *fault_landing = BasicBlockId(u32::MAX);
                            } else if mutation == "redirected Fault edge" {
                                let site = forged.blocks.get_mut(&site_block).unwrap();
                                let Some(MirInstruction::Invoke { fault_landing, .. }) =
                                    &mut site.terminator
                                else {
                                    panic!("checked extraction");
                                };
                                *fault_landing = normal;
                            } else {
                                if mutation == "unchecked comparison" {
                                    forged.blocks.get_mut(&site_block).unwrap().set_terminator(
                                        MirInstruction::Jump { target: normal, edge_args: None },
                                    );
                                    let landing = forged.blocks.get_mut(&normal).unwrap();
                                    let index = landing.instructions.iter().position(|row| {
                                        matches!(row, MirInstruction::InvokeNormalResult { dst, .. }
                                            if *dst == proof.extracted)
                                    }).expect("checked extraction result");
                                    landing.instructions.remove(index);
                                    landing.instruction_spans.remove(index);
                                }
                                let operand = forged.blocks.get_mut(&normal).unwrap().instructions.iter_mut()
                                    .find(|row| matches!(row, MirInstruction::Compare { dst, .. } if *dst == proof.result))
                                    .expect("ordered compare");
                                let MirInstruction::Compare { op, lhs, rhs, .. } = operand else { unreachable!() };
                                match mutation {
                                    "unchecked comparison" => {
                                        (*lhs, *rhs) = proof.original_operands();
                                    }
                                    "different integer" => *lhs = *rhs,
                                    "reversed operands" => std::mem::swap(lhs, rhs),
                                    "different operator" => {
                                        assert_eq!(*op, crate::mir::CompareOp::Le);
                                        *op = crate::mir::CompareOp::Lt;
                                    }
                                    _ => unreachable!(),
                                }
                            }
                            let errors = MirVerifier::new_strict().verify_function(&forged)
                                .expect_err(mutation);
                            assert!(errors.iter().any(|error| error.to_string()
                                .contains("borrowed-integer/compare-correspondence")),
                                "{mutation}: {errors:?}");
                        }
                        Ok(())
                    },
                );
                compiled.unwrap_or_else(|error| panic!("opt={optimize}: {error}"));
            }
        }
    });
}
