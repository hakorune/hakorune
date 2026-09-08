//! Whole-batch placement rejection before local registration.
use super::*;
use crate::ast::{LiteralValue, Span};
use crate::mir::builder::recursive_child_lowering::RawLegacyChildLoweringPortV1;
use crate::mir::builder::stmts::{drive_local_statement_with_placement_v1, RawLegacyLocalInputV1};

#[test]
fn placement_callback_rejection_registers_no_earlier_local() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("placement_batch".into());
    let input = RawLegacyLocalInputV1::new(ASTNode::Local {
        variables: vec!["first".into(), "second".into()],
        initial_values: vec![
            Some(Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            }));
            2
        ],
        declared_type_names: vec![None, None],
        span: Span::unknown(),
    });
    let mut visited = Vec::new();
    let error = drive_local_statement_with_placement_v1(
        &mut builder,
        &mut RawLegacyChildLoweringPortV1,
        input,
        |ordinal, _| {
            visited.push(ordinal);
            if ordinal == 1 {
                Err("placement-test-refusal".into())
            } else {
                Ok(LocalValuePlacement::ReuseInitializer)
            }
        },
    )
    .err()
    .expect("second placement rejects");
    assert_eq!(error, "placement-test-refusal");
    assert_eq!(visited, vec![0, 1]);
    assert!(!builder
        .function_state
        .variable_ctx
        .variable_map
        .contains_key("first"));
    assert!(!builder
        .function_state
        .variable_ctx
        .variable_map
        .contains_key("second"));
    assert!(!builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .blocks
        .values()
        .flat_map(|b| b.all_instructions())
        .any(|i| matches!(i, crate::mir::MirInstruction::Copy { .. })));
}

#[test]
fn opaque_contract_preflight_rejects_before_any_local_registration() {
    for annotation in ["i64", "Array<i64>"] {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("opaque_contract_batch".into());
        let error = build_local_statement_from_values_with_types_and_preclaims_with_receipt(
            &mut builder,
            vec!["first".into(), "second".into()],
            vec![ValueId(1), ValueId(2)],
            vec![None, Some(annotation.into())],
            vec![],
            None,
            &[
                LocalValuePlacement::ReuseInitializer,
                LocalValuePlacement::ReuseInitializer,
            ],
        )
        .unwrap_err();
        assert!(
            error.contains("local-placement/opaque-contract"),
            "{annotation}: {error}"
        );
        assert!(!builder
            .function_state
            .variable_ctx
            .variable_map
            .contains_key("first"));
        assert!(!builder
            .function_state
            .variable_ctx
            .variable_map
            .contains_key("second"));
        assert!(builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .blocks
            .values()
            .all(|b| b.all_instructions().next().is_none()));
    }
}
