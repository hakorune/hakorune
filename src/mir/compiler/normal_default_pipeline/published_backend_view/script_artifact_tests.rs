//! Real published compilation retains Script Array products in the one borrowed slot.
use super::super::super::tests::published_request;
use super::*;
use crate::mir::compiler::MirCompiler;

#[test]
fn script_array_reaches_published_callback_and_rejects_callable_admission() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for optimize in [false, true] {
            for terminal in ["return 30", "return"] {
                let source = format!("local first: Array<i64> = [10, 20]\nlocal alias = first\nlocal second: Array<u8> = []\n{terminal}");
                let mut calls = 0;
                MirCompiler::with_options(optimize)
                    .compile_normal_with_published(
                        published_request(&source),
                        |view, verification| {
                            calls += 1;
                            assert!(verification.is_ok());
                            let array =
                                view.retained_script_array().expect("retained Script Array");
                            assert_eq!(array.acquisition_count(), 2);
                            let handoff = view.retained_handoff.expect("one borrowed handoff");
                            assert!(std::ptr::eq(array, handoff.script_array().unwrap()));
                                assert!(view.retained_root_source().is_none());
                            assert!(view.retained_root_result().is_none());
                            assert!(view.retained_birth_abi().is_none());
                            assert!(view.lifecycle_storage_profile().is_none());
                            let generic = PublishedMirBackendView::try_new(view.module()).unwrap();
                            assert!(generic.retained_script_array().is_none());
                            let rebound =
                                generic.bind_finalized_root_handoff(Some(handoff)).unwrap();
                            let error = super::super::super::lifecycle_admission::admit_lifecycle(
                                rebound,
                                &PublishedObjectStorageProfileV1::SafeMutex,
                            )
                            .err()
                            .expect("Script is not a callable lifecycle root");
                            assert!(error.contains("script-root-not-callable"), "{error}");
                            Ok(())
                        },
                    )
                    .unwrap();
                assert_eq!(calls, 1);
            }
        }
    });
}

#[test]
fn script_artifact_view_rejects_missing_key_and_entry_drift() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        MirCompiler::with_options(false)
            .compile_normal_with_published(
                published_request("local a: Array<i64> = [10, 20]\nreturn 30"),
                |view, _| {
                    let handoff = view.retained_handoff.unwrap();
                    for mutation in 0..3 {
                        let mut module = view.module().clone();
                        match mutation {
                            0 => {
                                module.functions.remove(handoff.root_key());
                            }
                            1 => {
                                module
                                    .functions
                                    .get_mut(handoff.root_key())
                                    .unwrap()
                                    .signature
                                    .name = "foreign".into();
                            }
                            2 => {
                                module
                                    .functions
                                    .get_mut(handoff.root_key())
                                    .unwrap()
                                    .entry_block = crate::mir::BasicBlockId::new(u32::MAX);
                            }
                            _ => unreachable!(),
                        }
                        let generic = PublishedMirBackendView::try_new(&module).unwrap();
                        let error = generic
                            .bind_finalized_root_handoff(Some(handoff))
                            .err()
                            .expect("root drift rejects");
                        assert!(
                            error.contains(match mutation {
                                0 => "RetainedRootMissing",
                                1 => "retained-root-key-drift",
                                2 => "artifact-entry-drift",
                                _ => unreachable!(),
                            }),
                            "{mutation}: {error}"
                        );
                    }
                    Ok(())
                },
            )
            .unwrap();
    });
}
