//! Finalized canonical packet loans retain original actual and producer identity.
use super::*;
use crate::mir::compiler::{MirCompiler, NormalCompileRequestV1};
use crate::mir::instruction::InvokeOperation;

pub(super) fn selected_request() -> NormalCompileRequestV1 {
    let text = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("lang/src/hako_alloc/memory/size_class_box.hako"),
    )
    .unwrap();
    let text =
        format!("{text}\nstatic box Main {{ main() {{ return SizeClassBox.size_to_bin(9) }} }}");
    let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
        &text,
        crate::parser::ParserBuildConfig::default(),
    )
    .unwrap();
    let transformed = crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap();
    let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed else {
        panic!("original source identity must survive");
    };
    NormalCompileRequestV1::for_mir_mode_callable_source(source, None, Default::default())
        .for_closed_app_executable()
}

#[test]
fn finalized_selected_loop_actual_loan_preserves_source_and_finished_producer() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let mut compiler = MirCompiler::with_options(false);
        compiler
            .compile_normal_with_published(
                selected_request(),
                |view, verification| -> Result<(), String> {
                    assert!(verification.is_ok(), "final module verification");
                    let source = view
                        .retained_root_source()
                        .expect("retained original source");
                    let mut visited = 0;
                    source.visit_finalized_lexical_call_nodes_v1(
                        view.module(),
                        |owner, _, emitted, _, caller, coordinate, _| {
                            let CallPacketSourceLoanV1::SelectedStaticLoop {
                                packet,
                                observation,
                            } = emitted.original_source()
                            else {
                                return Ok(());
                            };
                            assert!(packet.corroborates_local_call(observation));
                            let (original_caller, site) = packet.publication_source();
                            let expected =
                                packet.selected_forwarded_actual_for_call(original_caller, site)?;
                            let actuals = source
                                .borrowed_packet_actuals_v1(emitted)?
                                .expect("finalized selected actual loan");
                            assert_eq!(actuals.len(), 1);
                            assert!(
                                std::ptr::eq(&actuals[0], expected),
                                "lend the same checked witness"
                            );
                            assert_eq!(expected.ordinal, 0);
                            assert_eq!(&expected.site, packet.argument_site());
                            assert_eq!(emitted.call_site().owner(), owner);
                            let mut independent = MirCompiler::with_options(false);
                            independent.compile_normal_with_published(
                                selected_request(),
                                |foreign_view, foreign_verification| -> Result<(), String> {
                                    assert!(foreign_verification.is_ok());
                                    let foreign = foreign_view.retained_root_source()
                                        .expect("independent retained original source");
                                    let error = foreign.borrowed_packet_actuals_v1(emitted)
                                        .unwrap_err();
                                    assert!(error.contains("selected-static-packet/canonical-source-drift"),
                                        "a genuine packet from another invocation must be rejected: {error}");
                                    Ok(())
                                },
                            )?;
                            let error = emitted
                                .original_source()
                                .borrowed_actuals(&source.ledger)
                                .unwrap_err();
                            assert!(
                                error.contains("selected-static-packet/canonical-entry-required"),
                                "generic admission must remain closed: {error}"
                            );
                            let instruction = caller.blocks[&coordinate.0]
                                .all_instructions()
                                .nth(coordinate.1)
                                .unwrap();
                            assert!(matches!(
                                instruction,
                                MirInstruction::Invoke {
                                    operation: InvokeOperation::Call { .. },
                                    ..
                                }
                            ));
                            visited += 1;
                            Ok(())
                        },
                    )?;
                    assert_eq!(
                        visited, 1,
                        "original selected entry call, not name-selected mirrors"
                    );
                    let mut bodies = 0;
                    source.visit_finalized_borrowed_call_nodes_v1(
                        view.module(),
                        |node, _, caller, coordinate, copies| {
                            let FinalizedBorrowedCallPacketV1::LoopBody(packet) = node else {
                                return Ok(());
                            };
                            bodies += 1;
                            assert!(packet.corroborates_retained_scalar());
                            assert!(copies.is_empty(), "canonical read is not a synthetic Copy");
                            let actuals = source.borrowed_finalized_call_actuals_v1(node)?
                                .expect("retained scalar actual");
                            assert_eq!(actuals.len(), 1);
                            assert!(std::ptr::eq(&actuals[0], packet.scalar_actual()));
                            assert_eq!(actuals[0].formal.owner(), node.callee_owner());
                            assert_eq!(node.call_site(), packet.original_source().call_site());
                            let mut independent = MirCompiler::with_options(false);
                            independent.compile_normal_with_published(
                                selected_request(),
                                |foreign, verification| -> Result<(), String> {
                                    assert!(verification.is_ok());
                                    assert!(foreign.retained_root_source().unwrap()
                                        .borrowed_finalized_call_actuals_v1(node)
                                        .unwrap_err().contains("retained-source-missing"));
                                    Ok(())
                                },
                            )?;
                            for mutation in 0..3 {
                                let mut changed = view.module().clone();
                                let block = changed.functions.get_mut(&caller.signature.name)
                                    .unwrap().blocks.get_mut(&coordinate.0).unwrap();
                                if mutation == 0 {
                                    block.terminator = None;
                                } else if mutation == 1 {
                                    block.instructions.push(block.terminator.as_ref().unwrap().clone());
                                } else {
                                    let MirInstruction::Invoke {
                                        operation: InvokeOperation::Call { call, .. }, ..
                                    } = block.terminator.as_mut().unwrap() else {
                                        panic!("original body Invoke");
                                    };
                                    call.args[0] = ValueId(999999);
                                }
                                assert!(source.visit_finalized_borrowed_call_nodes_v1(
                                    &changed, |_, _, _, _, _| Ok(())
                                ).is_err(), "missing/duplicate/changed original body producer");
                            }
                            Ok(())
                        },
                    )?;
                    assert_eq!(bodies, 1, "retain and visit the original selected body once");
                    for mutation in 0..2 {
                        let mut changed = view.module().clone();
                        source.visit_finalized_lexical_call_nodes_v1(
                            view.module(),
                            |_, _, emitted, _, caller, coordinate, _| {
                                if !matches!(
                                    emitted.original_source(),
                                    CallPacketSourceLoanV1::SelectedStaticLoop { .. }
                                ) {
                                    return Ok(());
                                }
                                let instruction = changed
                                    .functions
                                    .get_mut(&caller.signature.name)
                                    .unwrap()
                                    .blocks
                                    .get_mut(&coordinate.0)
                                    .unwrap()
                                    .terminator
                                    .as_mut()
                                    .unwrap();
                                let MirInstruction::Invoke {
                                    operation: InvokeOperation::Call { call, .. },
                                    ..
                                } = instruction
                                else {
                                    panic!("original Invoke terminator");
                                };
                                if mutation == 0 {
                                    call.args[0] = ValueId(999999);
                                } else {
                                    call.callee =
                                        crate::mir::Callee::Global(
                                            crate::mir::builder::CanonicalSameModuleCallableKeyV1::static_box_method("Foreign", "other", 1)
                                                .canonical_global_target_v1().unwrap()
                                        );
                                }
                                Ok(())
                            },
                        )?;
                        assert!(
                            source
                                .visit_finalized_lexical_call_nodes_v1(
                                    &changed,
                                    |_, _, _, _, _, _, _| Ok(())
                                )
                                .is_err(),
                            "foreign producer/source target cannot acquire finalized loans"
                        );
                    }
                    view.issue_lifecycle_compiled_entry_contract()?;
                    Ok(())
                },
            )
            .unwrap();
    });
}
