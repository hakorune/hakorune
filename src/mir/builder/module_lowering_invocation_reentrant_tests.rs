//! HEADERPORT0-REENTRANT-TERM0-P0: capture/commit lifetime proofs.
//!
//! These fixtures exercise the disconnected capture-only and commit-only
//! seams.  They do not activate raw production lowering.

use crate::mir::builder::calls::CanonicalFunctionSessionErrorV1;
use crate::mir::builder::module_lowering_invocation::{
    LegacyChildDraftAdmissionV1, ModuleLoweringInvocationV1, ModuleLoweringPortChildErrorV1,
};
use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirBuilder, MirFunction, MirType};

fn draft(symbol: &str) -> MirFunction {
    MirFunction::new(
        FunctionSignature {
            name: symbol.to_owned(),
            params: Vec::new(),
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    )
}

fn seeded<'builder>(builder: &'builder mut MirBuilder) -> ModuleLoweringInvocationV1<'builder> {
    builder.enter_function_for_test("reentrant_parent/0".to_owned());
    ModuleLoweringInvocationV1::open(builder)
}

#[test]
fn pending_capture_ends_before_header_loan_and_commit() {
    let mut builder = MirBuilder::new();
    let mut invocation = seeded(&mut builder);

    invocation
        .with_module_port(|builder, port| {
            let pending = port
                .capture_legacy_pending(builder, "inner/0", Vec::new(), |_| Ok(draft("inner/0")))
                .unwrap();

            port.with_headers(|headers| {
                assert!(!headers.contains_symbol("inner/0"));
                assert_eq!(headers.symbol_count(), 0);
            });

            port.commit_legacy_pending(
                pending,
                LegacyChildDraftAdmissionV1::legacy_symbol("inner/0".into(), 0),
            )
        })
        .unwrap();

    invocation.with_header_port(|builder, headers| {
        assert!(headers.contains_symbol("inner/0"));
        assert_eq!(headers.symbol_count(), 1);
        assert_eq!(
            builder
                .function_state
                .current_function
                .as_ref()
                .unwrap()
                .signature
                .name,
            "reentrant_parent/0"
        );
    });
}

#[test]
fn rejected_commit_restores_parent_without_collector_delta() {
    let mut builder = MirBuilder::new();
    let mut invocation = seeded(&mut builder);

    let result = invocation.with_module_port(|builder, port| {
        let pending = port
            .capture_legacy_pending(builder, "inner/0", Vec::new(), |_| Ok(draft("inner/0")))
            .unwrap();
        port.commit_legacy_pending(
            pending,
            LegacyChildDraftAdmissionV1::legacy_symbol("wrong/0".into(), 0),
        )
    });

    assert!(matches!(
        result,
        Err(ModuleLoweringPortChildErrorV1::Admission(_))
    ));
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
            "reentrant_parent/0"
        );
    });
}

#[test]
fn capture_failure_never_reaches_commit_terminal() {
    let mut builder = MirBuilder::new();
    let mut invocation = seeded(&mut builder);

    let result = invocation.with_module_port(|builder, port| {
        let captured = port.capture_legacy_pending(
            builder,
            "failed/0",
            Vec::new(),
            move |_| -> Result<MirFunction, String> { Err("nested body failure".to_owned()) },
        );
        captured.map(|pending| {
            drop(pending);
        })
    });

    assert!(matches!(
        result,
        Err(ModuleLoweringPortChildErrorV1::Session(
            CanonicalFunctionSessionErrorV1::Primary(_)
        ))
    ));
    invocation.with_header_port(|_builder, headers| assert_eq!(headers.symbol_count(), 0));
}
