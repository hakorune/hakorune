use super::MirBuilder;
use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
use crate::mir::{Callee, EffectMask, MirInstruction};

fn builder_with_entry(name: &str) -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test(name.to_owned());
    builder
}

#[test]
fn receiverless_method_rejects_before_builder_publication() {
    let mut builder = builder_with_entry("method_none_reject/0");
    let before = builder.current_function_instructions().len();
    let instruction = MirInstruction::call(
        None,
        Callee::Method {
            box_name: "MapBox".to_owned(),
            method: "get".to_owned(),
            receiver: None,
            certainty: TypeCertainty::Known,
            box_kind: CalleeBoxKind::RuntimeData,
        },
        vec![],
        EffectMask::PURE,
    );

    let error = builder
        .emit_instruction(instruction)
        .expect_err("receiverless Method must be retired at Builder publication");

    assert_eq!(
        error,
        "[mir/call/method-none-retired] receiverless Method is not a Builder publication shape"
    );
    assert_eq!(builder.current_function_instructions().len(), before);
}

#[test]
fn typed_global_still_publishes_after_method_none_retirement() {
    let mut builder = builder_with_entry("method_none_global_positive/0");
    let instruction = MirInstruction::call(
        None,
        Callee::Global(crate::mir::test_global_target("terminal_positive/0")),
        vec![],
        EffectMask::PURE,
    );

    builder
        .emit_instruction(instruction)
        .expect("typed Global remains a canonical Builder publication shape");

    assert!(matches!(
        builder.current_function_instructions().as_slice(),
        [MirInstruction::LegacyCallV0 {
            callee: Some(Callee::Global(_)),
            ..
        }]
    ));
}
