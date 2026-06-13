use super::*;
use crate::mir::join_ir::lowering::carrier_info::CarrierRole;

#[test]
fn test_builder_basic() {
    let boundary = JoinInlineBoundaryBuilder::new()
        .with_inputs(vec![ValueId(0)], vec![ValueId(4)])
        .build();

    assert_eq!(boundary.join_inputs, vec![ValueId(0)]);
    assert_eq!(boundary.host_inputs, vec![ValueId(4)]);
    assert_eq!(boundary.exit_bindings.len(), 0);
    assert_eq!(boundary.condition_bindings.len(), 0);
    assert_eq!(boundary.expr_result, None);
    assert_eq!(boundary.loop_var_name, None);
}

#[test]
fn test_builder_full() {
    let condition_binding = ConditionBinding {
        name: "start".to_string(),
        host_value: ValueId(33),
        join_value: ValueId(1),
    };

    let exit_binding = LoopExitBinding {
        carrier_name: "sum".to_string(),
        join_exit_value: ValueId(18),
        host_slot: ValueId(5),
        role: CarrierRole::LoopState,
    };

    let boundary = JoinInlineBoundaryBuilder::new()
        .with_inputs(vec![ValueId(0)], vec![ValueId(4)])
        .with_loop_var_name(Some("i".to_string()))
        .with_condition_bindings(vec![condition_binding])
        .with_exit_bindings(vec![exit_binding])
        .with_expr_result(Some(ValueId(20)))
        .build();

    assert_eq!(boundary.join_inputs, vec![ValueId(0)]);
    assert_eq!(boundary.host_inputs, vec![ValueId(4)]);
    assert_eq!(boundary.loop_var_name, Some("i".to_string()));
    assert_eq!(boundary.condition_bindings.len(), 1);
    assert_eq!(boundary.exit_bindings.len(), 1);
    assert_eq!(boundary.expr_result, Some(ValueId(20)));
}

#[test]
#[should_panic(expected = "join_inputs and host_inputs must have same length")]
fn test_builder_mismatched_inputs() {
    JoinInlineBoundaryBuilder::new()
        .with_inputs(vec![ValueId(0), ValueId(1)], vec![ValueId(4)])
        .build();
}

#[test]
fn test_builder_default() {
    let builder = JoinInlineBoundaryBuilder::default();
    let boundary = builder.build();

    assert_eq!(boundary.join_inputs.len(), 0);
    assert_eq!(boundary.host_inputs.len(), 0);
}

#[test]
fn test_builder_if_phi_join_style() {
    // IfPhiJoin style: Two carriers (i + sum), exit_bindings, loop_var_name

    let boundary = JoinInlineBoundaryBuilder::new()
        .with_inputs(
            vec![ValueId(0), ValueId(1)],
            vec![ValueId(100), ValueId(101)],
        )
        .with_exit_bindings(vec![LoopExitBinding {
            carrier_name: "sum".to_string(),
            join_exit_value: ValueId(18),
            host_slot: ValueId(101),
            role: CarrierRole::LoopState,
        }])
        .with_loop_var_name(Some("i".to_string()))
        .build();

    assert_eq!(boundary.join_inputs.len(), 2);
    assert_eq!(boundary.host_inputs.len(), 2);
    assert_eq!(boundary.exit_bindings.len(), 1);
    assert_eq!(boundary.exit_bindings[0].carrier_name, "sum");
    assert_eq!(boundary.loop_var_name, Some("i".to_string()));
    assert_eq!(boundary.expr_result, None);
}

#[test]
fn test_builder_loop_continue_only_style() {
    // LoopContinueOnly style: Dynamic carrier count, continue support
    let boundary = JoinInlineBoundaryBuilder::new()
        .with_inputs(
            vec![ValueId(0), ValueId(1), ValueId(2)], // i + 2 carriers
            vec![ValueId(100), ValueId(101), ValueId(102)],
        )
        .with_exit_bindings(vec![
            LoopExitBinding {
                carrier_name: "i".to_string(),
                join_exit_value: ValueId(11),
                host_slot: ValueId(100),
                role: CarrierRole::LoopState,
            },
            LoopExitBinding {
                carrier_name: "sum".to_string(),
                join_exit_value: ValueId(20),
                host_slot: ValueId(101),
                role: CarrierRole::LoopState,
            },
        ])
        .with_loop_var_name(Some("i".to_string()))
        .build();

    assert_eq!(boundary.exit_bindings.len(), 2);
    assert!(boundary.loop_var_name.is_some());
    assert_eq!(boundary.join_inputs.len(), 3);
    assert_eq!(boundary.host_inputs.len(), 3);
}

// Phase 200-A: ParamRole tests
#[test]
fn test_param_role_loop_param() {
    let mut builder = JoinInlineBoundaryBuilder::new();
    builder.add_param_with_role("i", ValueId(100), ParamRole::LoopParam);

    let boundary = builder.build();
    assert_eq!(boundary.join_inputs.len(), 1);
    assert_eq!(boundary.host_inputs.len(), 1);
    assert_eq!(boundary.host_inputs[0], ValueId(100));
}

#[test]
fn test_param_role_condition() {
    let mut builder = JoinInlineBoundaryBuilder::new();
    // Phase 200-B: Condition role is added to condition_bindings
    builder.add_param_with_role("digits", ValueId(42), ParamRole::Condition);

    let boundary = builder.build();
    // Phase 200-B: Condition params go to condition_bindings, not join_inputs
    assert_eq!(boundary.join_inputs.len(), 0);
    assert_eq!(boundary.condition_bindings.len(), 1);
    assert_eq!(boundary.condition_bindings[0].name, "digits");
    assert_eq!(boundary.condition_bindings[0].host_value, ValueId(42));
}

#[test]
fn test_param_role_carrier() {
    let mut builder = JoinInlineBoundaryBuilder::new();
    builder.add_param_with_role("sum", ValueId(101), ParamRole::Carrier);

    let boundary = builder.build();
    assert_eq!(boundary.join_inputs.len(), 1);
    assert_eq!(boundary.host_inputs.len(), 1);
    assert_eq!(boundary.host_inputs[0], ValueId(101));
}

#[test]
fn test_with_k_exit_continuation() {
    // Phase 256 P1.7: Test convenience method for k_exit registration
    let boundary = JoinInlineBoundaryBuilder::new()
        .with_inputs(vec![ValueId(0)], vec![ValueId(100)])
        .with_k_exit_continuation()
        .build();

    assert_eq!(boundary.continuation_func_ids.len(), 1);
    assert!(boundary.continuation_func_ids.contains("k_exit"));
}

#[test]
fn test_with_continuation_funcs_manual() {
    // Phase 256 P1.7: Test manual continuation registration.
    use std::collections::BTreeSet;
    let boundary = JoinInlineBoundaryBuilder::new()
        .with_inputs(vec![ValueId(0)], vec![ValueId(100)])
        .with_continuation_funcs(BTreeSet::from(["k_exit".to_string()]))
        .build();

    assert_eq!(boundary.continuation_func_ids.len(), 1);
    assert!(boundary.continuation_func_ids.contains("k_exit"));
}

#[test]
fn test_with_k_exit_and_additional_continuation() {
    // Phase 256 P1.7: Test combining convenience method with additional continuations
    use std::collections::BTreeSet;
    let mut continuations = BTreeSet::new();
    continuations.insert("post_k".to_string());

    let boundary = JoinInlineBoundaryBuilder::new()
        .with_inputs(vec![ValueId(0)], vec![ValueId(100)])
        .with_k_exit_continuation()
        .with_continuation_funcs(continuations)
        .build();

    // with_continuation_funcs replaces the set, so only post_k should be present
    assert_eq!(boundary.continuation_func_ids.len(), 1);
    assert!(boundary.continuation_func_ids.contains("post_k"));
    assert!(!boundary.continuation_func_ids.contains("k_exit"));
}
