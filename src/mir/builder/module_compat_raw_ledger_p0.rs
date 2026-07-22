//! CUT0-S0-COMPAT0-P0: disconnected callable-Main receipt bridge.

use std::panic::{catch_unwind, AssertUnwindSafe};

use super::calls::CanonicalFunctionSessionErrorV1;
use super::decls::CallableMainCompatibilityLoweringErrorV1;
use super::module_compat_policy::CallableMainCompatibilityPolicyV1;
use super::module_lowering_invocation::{
    LegacyChildDraftAdmissionV1, ModuleLoweringInvocationV1, ModuleLoweringPortChildErrorV1,
};
use super::raw_expansion_receipt_ledger::{
    AbortedRawExpansionReceiptLedgerV1, RawCallableMainCompatibilityDispositionV1,
    RawExpansionAbortReasonV1, RawExpansionReceiptLedgerErrorV1, RawExpansionReceiptLedgerV1,
};
use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirBuilder, MirFunction, MirType};

#[allow(dead_code)]
#[derive(Debug)]
enum CallableMainReceiptAttemptErrorV1 {
    Ledger(RawExpansionReceiptLedgerErrorV1),
    Child {
        source: CallableMainCompatibilityLoweringErrorV1,
        aborted: AbortedRawExpansionReceiptLedgerV1,
    },
    Panic {
        aborted: AbortedRawExpansionReceiptLedgerV1,
    },
}

fn draft(symbol: &str, arity: usize) -> MirFunction {
    MirFunction::new(
        FunctionSignature {
            name: symbol.to_owned(),
            params: vec![MirType::Integer; arity],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    )
}

fn run_optional<'builder>(
    policy: CallableMainCompatibilityPolicyV1,
    invocation: &mut ModuleLoweringInvocationV1<'builder>,
    ledger: RawExpansionReceiptLedgerV1,
    symbol: &str,
    body: Vec<crate::ast::ASTNode>,
    lower: impl FnOnce(&mut MirBuilder) -> Result<MirFunction, String>,
) -> Result<RawExpansionReceiptLedgerV1, CallableMainReceiptAttemptErrorV1> {
    if !policy.is_required() {
        return Ok(ledger);
    }

    let mut ledger = ledger;
    let reservation = ledger
        .reserve(
            super::raw_expansion_receipt_ledger::RawExpansionDraftRequestV1::
                callable_main_compatibility(symbol.to_owned(), 0)
                .map_err(CallableMainReceiptAttemptErrorV1::Ledger)?,
        )
        .map_err(CallableMainReceiptAttemptErrorV1::Ledger)?;
    let admission = LegacyChildDraftAdmissionV1::legacy_symbol(symbol.to_owned(), 0);
    let outcome = catch_unwind(AssertUnwindSafe(|| {
        invocation.with_module_port(|builder, port| {
            port.complete_legacy_child(builder, body, admission, lower)
        })
    }));
    match outcome {
        Ok(Ok(receipt)) => {
            ledger
                .complete(reservation, receipt)
                .map_err(CallableMainReceiptAttemptErrorV1::Ledger)?;
            Ok(ledger)
        }
        Ok(Err(source)) => {
            let aborted = ledger
                .abort(reservation, abort_reason(&source))
                .map_err(CallableMainReceiptAttemptErrorV1::Ledger)?;
            Err(CallableMainReceiptAttemptErrorV1::Child {
                source: CallableMainCompatibilityLoweringErrorV1::from(source),
                aborted,
            })
        }
        Err(_) => {
            let aborted = ledger
                .abort(reservation, RawExpansionAbortReasonV1::Panic)
                .map_err(CallableMainReceiptAttemptErrorV1::Ledger)?;
            Err(CallableMainReceiptAttemptErrorV1::Panic { aborted })
        }
    }
}

fn abort_reason(error: &ModuleLoweringPortChildErrorV1) -> RawExpansionAbortReasonV1 {
    match error {
        ModuleLoweringPortChildErrorV1::Session(CanonicalFunctionSessionErrorV1::Primary(_)) => {
            RawExpansionAbortReasonV1::Primary
        }
        ModuleLoweringPortChildErrorV1::Session(CanonicalFunctionSessionErrorV1::Cleanup(_)) => {
            RawExpansionAbortReasonV1::Cleanup
        }
        ModuleLoweringPortChildErrorV1::Session(
            CanonicalFunctionSessionErrorV1::DuringCleanup { .. },
        ) => RawExpansionAbortReasonV1::Cleanup,
        ModuleLoweringPortChildErrorV1::Session(CanonicalFunctionSessionErrorV1::Publication(
            _,
        ))
        | ModuleLoweringPortChildErrorV1::Admission(_) => RawExpansionAbortReasonV1::Admission,
    }
}

fn selected_ledger() -> RawExpansionReceiptLedgerV1 {
    RawExpansionReceiptLedgerV1::new(RawCallableMainCompatibilityDispositionV1::Selected)
}

#[test]
fn selected_callable_main_consumes_one_exact_receipt() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("legacy_parent/0".to_owned());
    let mut invocation = ModuleLoweringInvocationV1::open(&mut builder);
    let ledger = run_optional(
        CallableMainCompatibilityPolicyV1::Required,
        &mut invocation,
        selected_ledger(),
        "Main.main/0",
        Vec::new(),
        |_| Ok(draft("Main.main/0", 0)),
    )
    .unwrap();
    assert_eq!(ledger.completed_event_count(), 1);
    let event = ledger.last_completed_event().unwrap();
    assert_eq!(
        event.role(),
        super::raw_expansion_receipt_ledger::RawExpansionDraftRoleV1::CallableMainCompatibility
    );
    assert_eq!(event.symbol(), "Main.main/0");
    assert_eq!(event.arity(), 0);
    assert_eq!(
        event.key(),
        &super::module_draft_collector::FunctionDraftKeyV1::LegacySymbol("Main.main/0".to_owned())
    );
    assert_eq!(
        ledger.seal().unwrap_err(),
        RawExpansionReceiptLedgerErrorV1::MissingRootMain
    );
    invocation.with_header_port(|builder, headers| {
        assert_eq!(headers.symbol_count(), 1);
        assert_eq!(headers.signature("Main.main/0").unwrap().params.len(), 0);
        assert_eq!(
            builder
                .function_state
                .current_function
                .as_ref()
                .unwrap()
                .signature
                .name,
            "legacy_parent/0"
        );
    });
}

#[test]
fn omitted_callable_main_never_reserves_or_lowers() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("legacy_parent/0".to_owned());
    let mut invocation = ModuleLoweringInvocationV1::open(&mut builder);
    let ledger = run_optional(
        CallableMainCompatibilityPolicyV1::Omitted,
        &mut invocation,
        RawExpansionReceiptLedgerV1::new(RawCallableMainCompatibilityDispositionV1::NotSelected),
        "Main.main/0",
        Vec::new(),
        |_| panic!("NotSelected must not lower"),
    )
    .unwrap();
    assert_eq!(ledger.completed_event_count(), 0);
    invocation.with_header_port(|_builder, headers| assert_eq!(headers.symbol_count(), 0));
}

#[test]
fn selected_primary_failure_aborts_without_touching_prefix() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("legacy_parent/0".to_owned());
    let mut invocation = ModuleLoweringInvocationV1::open(&mut builder);
    invocation
        .with_module_port(|builder, port| {
            port.complete_legacy_child(
                builder,
                Vec::new(),
                LegacyChildDraftAdmissionV1::legacy_symbol("prefix/0".to_owned(), 0),
                |_| Ok(draft("prefix/0", 0)),
            )
        })
        .unwrap();
    let error = run_optional(
        CallableMainCompatibilityPolicyV1::Required,
        &mut invocation,
        selected_ledger(),
        "Main.main/0",
        Vec::new(),
        |_| Err("selected primary".to_owned()),
    )
    .unwrap_err();
    let CallableMainReceiptAttemptErrorV1::Child { source, aborted } = error else {
        panic!("expected typed child failure");
    };
    assert!(matches!(
        source,
        CallableMainCompatibilityLoweringErrorV1::Child(ModuleLoweringPortChildErrorV1::Session(
            CanonicalFunctionSessionErrorV1::Primary(_)
        ))
    ));
    assert_eq!(
        aborted.failed_role(),
        super::raw_expansion_receipt_ledger::RawExpansionDraftRoleV1::CallableMainCompatibility
    );
    assert_eq!(aborted.final_count(), 0);
    invocation.with_header_port(|builder, headers| {
        assert_eq!(headers.symbol_count(), 1);
        assert!(headers.contains_symbol("prefix/0"));
        assert_eq!(
            builder
                .function_state
                .current_function
                .as_ref()
                .unwrap()
                .signature
                .name,
            "legacy_parent/0"
        );
    });
}

#[test]
fn selected_cleanup_admission_and_panic_failures_abort_without_receipts() {
    let cases: [(
        &str,
        Box<dyn FnOnce(&mut MirBuilder) -> Result<MirFunction, String>>,
    ); 3] = [
        (
            "cleanup",
            Box::new(|builder| {
                builder.recursion_depth = 1;
                Ok(draft("Main.main/0", 0))
            }),
        ),
        ("admission", Box::new(|_| Ok(draft("wrong/0", 0)))),
        ("panic", Box::new(|_| panic!("selected panic"))),
    ];
    for (label, lower) in cases {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("legacy_parent/0".to_owned());
        let mut invocation = ModuleLoweringInvocationV1::open(&mut builder);
        let error = run_optional(
            CallableMainCompatibilityPolicyV1::Required,
            &mut invocation,
            selected_ledger(),
            "Main.main/0",
            Vec::new(),
            lower,
        )
        .unwrap_err();
        match (label, error) {
            ("panic", CallableMainReceiptAttemptErrorV1::Panic { aborted }) => {
                assert_eq!(aborted.reason(), RawExpansionAbortReasonV1::Panic);
            }
            ("cleanup", CallableMainReceiptAttemptErrorV1::Child { source, aborted }) => {
                assert!(matches!(
                    source,
                    CallableMainCompatibilityLoweringErrorV1::Child(
                        ModuleLoweringPortChildErrorV1::Session(
                            CanonicalFunctionSessionErrorV1::Cleanup(_)
                        )
                    )
                ));
                assert_eq!(aborted.reason(), RawExpansionAbortReasonV1::Cleanup);
            }
            ("admission", CallableMainReceiptAttemptErrorV1::Child { source, aborted }) => {
                assert!(matches!(
                    source,
                    CallableMainCompatibilityLoweringErrorV1::Child(
                        ModuleLoweringPortChildErrorV1::Admission(_)
                    )
                ));
                assert_eq!(aborted.reason(), RawExpansionAbortReasonV1::Admission);
            }
            _ => panic!("unexpected selected failure envelope"),
        }
        invocation.with_header_port(|builder, headers| {
            assert_eq!(headers.symbol_count(), 0);
            assert_eq!(
                builder
                    .function_state
                    .current_function
                    .as_ref()
                    .unwrap()
                    .signature
                    .name,
                "legacy_parent/0"
            );
        });
    }
}
