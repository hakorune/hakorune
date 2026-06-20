use super::*;
use crate::mir::ValueId;

fn trim_ready_carrier_info() -> crate::mir::join_ir::lowering::carrier_info::CarrierInfo {
    crate::mir::join_ir::lowering::carrier_info::CarrierInfo {
        loop_var_name: "start".to_string(),
        loop_var_id: ValueId(1),
        carriers: vec![],
        trim_helper: Some(
            crate::mir::loop_route_detection::support::trim::TrimLoopHelper {
                original_var: "ch".to_string(),
                carrier_name: "is_ch_match".to_string(),
                whitespace_chars: vec![" ".to_string()],
            },
        ),
        promoted_body_locals: vec!["ch".to_string()],
    }
}

#[test]
fn test_boundary_inputs_only() {
    let boundary = JoinInlineBoundary::new_inputs_only(
        vec![ValueId(0)], // JoinIR uses ValueId(0) for loop var
        vec![ValueId(4)], // Host has loop var at ValueId(4)
    );

    assert_eq!(boundary.join_inputs.len(), 1);
    assert_eq!(boundary.host_inputs.len(), 1);
    assert_eq!(boundary.loop_invariants.len(), 0);
    assert_eq!(boundary.exit_bindings.len(), 0);
    assert_eq!(boundary.condition_bindings.len(), 0);
}

#[test]
#[should_panic(expected = "join_inputs and host_inputs must have same length")]
fn test_boundary_mismatched_inputs() {
    JoinInlineBoundary::new_inputs_only(vec![ValueId(0), ValueId(1)], vec![ValueId(4)]);
}

#[test]
fn test_jump_args_layout_rejects_expr_result_carrier_mismatch() {
    let boundary = JoinInlineBoundary {
        join_inputs: vec![],
        host_inputs: vec![],
        loop_invariants: vec![],
        exit_bindings: vec![LoopExitBinding {
            carrier_name: "result".to_string(),
            join_exit_value: ValueId(10),
            host_slot: ValueId(1),
            role: crate::mir::join_ir::lowering::carrier_info::CarrierRole::LoopState,
        }],
        condition_bindings: vec![],
        expr_result: Some(ValueId(10)),
        jump_args_layout: JumpArgsLayout::ExprResultPlusCarriers,
        loop_var_name: None,
        loop_header_func_name: None,
        carrier_info: None,
        continuation_func_ids: JoinInlineBoundary::default_continuations(),
        exit_reconnect_mode:
            crate::mir::join_ir::lowering::carrier_info::ExitReconnectMode::default(),
    };

    let err = boundary
        .validate_jump_args_layout()
        .expect_err("layout mismatch must fail-fast");
    assert!(err.contains("jump_args_layout_mismatch"));
}

#[test]
fn route_boundary_trim_readiness_allows_ready_inputs() {
    let boundary = JoinInlineBoundary {
        join_inputs: vec![],
        host_inputs: vec![],
        loop_invariants: vec![],
        exit_bindings: vec![],
        condition_bindings: vec![
            crate::mir::join_ir::lowering::condition_env::ConditionBinding::new(
                "is_ch_match".to_string(),
                ValueId(20),
                ValueId(200),
            ),
        ],
        expr_result: None,
        jump_args_layout: JumpArgsLayout::CarriersOnly,
        loop_var_name: None,
        loop_header_func_name: None,
        carrier_info: Some(trim_ready_carrier_info()),
        continuation_func_ids: JoinInlineBoundary::default_continuations(),
        exit_reconnect_mode:
            crate::mir::join_ir::lowering::carrier_info::ExitReconnectMode::default(),
    };

    assert_eq!(
        boundary.trim_route_lowering_readiness(),
        crate::mir::loop_route_detection::support::trim::TrimRouteLoweringReadinessDecision::Ready
    );
}

#[test]
fn route_boundary_trim_readiness_denies_missing_identity() {
    let boundary = JoinInlineBoundary {
        join_inputs: vec![],
        host_inputs: vec![],
        loop_invariants: vec![],
        exit_bindings: vec![],
        condition_bindings: vec![],
        expr_result: None,
        jump_args_layout: JumpArgsLayout::CarriersOnly,
        loop_var_name: None,
        loop_header_func_name: None,
        carrier_info: Some(trim_ready_carrier_info()),
        continuation_func_ids: JoinInlineBoundary::default_continuations(),
        exit_reconnect_mode:
            crate::mir::join_ir::lowering::carrier_info::ExitReconnectMode::default(),
    };

    assert_eq!(
        boundary.trim_route_lowering_readiness(),
        crate::mir::loop_route_detection::support::trim::TrimRouteLoweringReadinessDecision::Deny(
            crate::mir::loop_route_detection::support::trim::TrimRouteLoweringReadinessDeny::MissingConditionBindingIdentity
        )
    );
}
