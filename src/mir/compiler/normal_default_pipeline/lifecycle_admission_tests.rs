//! Mutation witnesses for the finalization-owned physical admission boundary.
//! The handoff is issued from normal source; mutated modules never reach an artifact.

use super::super::tests::published_request;
use super::*;
use crate::mir::compiler::MirCompiler;
use crate::mir::{BasicBlock, BasicBlockId, EffectMask};
use hakorune_mir_defs::CanonicalSameModuleCallableKeyV1;

#[test]
fn lifecycle_admission_preserves_foreign_return_only_and_birth_call_rejections() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        MirCompiler::with_options(false)
            .compile_normal_with_published(
                published_request(include_str!(
                    "../../../../apps/typed-object-birth-min/main.hako"
                )),
                |view, _| {
                    let root = view.retained_root().unwrap().signature.name.as_str();
                    let births = view.retained_birth_abi().unwrap();
                    let key = births[0].target();
                    let symbol = view
                        .module()
                        .canonical_callable_definition_symbol(key)
                        .unwrap();
                    assert_eq!(validate_functions(view.module(), root, births), Ok(()));

                    let mut foreign = view.module().clone();
                    foreign.canonical_callable_definitions.remove(key);
                    assert_eq!(
                        validate_functions(&foreign, root, births),
                        Err(fault("function-not-cataloged"))
                    );

                    let mut non_birth = view.module().clone();
                    non_birth.canonical_callable_definitions.remove(key);
                    non_birth.canonical_callable_definitions.insert(
                        CanonicalSameModuleCallableKeyV1::free_function("foreign", 0),
                        symbol.to_owned(),
                    );
                    assert_eq!(
                        validate_functions(&non_birth, root, births),
                        Err(fault("function-not-birth"))
                    );

                    let mut return_only = view.module().clone();
                    let function = return_only.functions.get_mut(symbol).unwrap();
                    function.blocks.clear();
                    let mut block = BasicBlock::new(BasicBlockId::new(0));
                    block.terminator = Some(MirInstruction::Return { value: None });
                    function.blocks.insert(block.id, block);
                    assert_eq!(validate_functions(&return_only, root, births), Ok(()));
                    return_only
                        .functions
                        .get_mut(symbol)
                        .unwrap()
                        .signature
                        .name
                        .push_str("-drift");
                    assert_eq!(
                        validate_functions(&return_only, root, births),
                        Err(fault("function-not-birth"))
                    );

                    for caller in [root, symbol] {
                        let mut bad_call = view.module().clone();
                        bad_call
                            .functions
                            .get_mut(caller)
                            .unwrap()
                            .blocks
                            .values_mut()
                            .next()
                            .unwrap()
                            .add_instruction(MirInstruction::call(
                                None,
                                Callee::BirthConstructor {
                                    key: key.clone(),
                                    receiver: ValueId::INVALID,
                                },
                                vec![],
                                EffectMask::PURE,
                            ));
                        assert_eq!(
                            validate_functions(&bad_call, root, births),
                            Err(fault("birth-call-drift"))
                        );
                    }
                    Ok(())
                },
            )
            .expect("normal source reaches the admission observer once");
    });
}
