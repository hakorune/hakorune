use crate::mir::builder::module_draft_collector::ModuleDraftCollectorV1;
use crate::mir::builder::module_lowering_invocation::ModuleLoweringInvocationV1;
use crate::mir::builder::recursive_child_lowering::{
    drive_legacy_expression_v1, drive_raw_legacy_expression_v1, RawInvocationChildPortV1,
};
use crate::mir::region::function_slot_registry::FunctionSlotRegistry;
use crate::mir::{BindingId, MirBuilder, MirType, ValueId};

use super::*;

fn drive_selected(builder: &mut MirBuilder, node: ASTNode) -> Result<ValueId, String> {
    let mut invocation =
        ModuleLoweringInvocationV1::with_collector(builder, ModuleDraftCollectorV1::default());
    invocation.with_module_port(|builder, module_port| {
        let mut port = RawInvocationChildPortV1::new(module_port);
        drive_legacy_expression_v1(builder, &mut port, node)
    })
}

fn seed_binding(builder: &mut MirBuilder, name: &str, value: ValueId) {
    builder
        .function_state
        .variable_ctx
        .variable_map
        .insert(name.to_owned(), value);
    builder
        .function_state
        .binding_ctx
        .insert(name.to_owned(), BindingId::new(0));
}

fn spanned_instructions(builder: &MirBuilder) -> Vec<(String, Span)> {
    builder
        .function_state
        .current_function
        .as_ref()
        .expect("current function")
        .blocks
        .values()
        .flat_map(|block| block.all_spanned_instructions())
        .map(|instruction| (format!("{:?}", instruction.inst), instruction.span))
        .collect()
}

#[test]
fn selected_print_root_matches_the_raw_legacy_port_exactly() {
    let root = || printed(awaited(checked(vec![integer(14), integer(15)])));
    let mut legacy = MirBuilder::new();
    legacy.enter_function_for_test("print_root_parity/0".to_owned());
    let legacy_value = drive_raw_legacy_expression_v1(&mut legacy, root()).unwrap();

    let mut selected = MirBuilder::new();
    selected.enter_function_for_test("print_root_parity/0".to_owned());
    let selected_value = {
        let mut invocation = ModuleLoweringInvocationV1::with_collector(
            &mut selected,
            ModuleDraftCollectorV1::default(),
        );
        invocation.with_module_port(|builder, module_port| {
            let mut port = RawInvocationChildPortV1::new(module_port);
            drive_legacy_expression_v1(builder, &mut port, root())
        })
    }
    .unwrap();

    assert_eq!(selected_value, legacy_value);
    assert_eq!(
        spanned_instructions(&selected),
        spanned_instructions(&legacy)
    );
}

#[test]
fn selected_nowait_root_matches_raw_legacy_effects_exactly() {
    let root = || nowait("pending", checked(vec![integer(16), integer(17)]));
    let mut legacy = MirBuilder::new();
    legacy.enter_function_for_test("nowait_root_parity/0".to_owned());
    legacy.comp_ctx.current_slot_registry = Some(FunctionSlotRegistry::new());
    let legacy_value = drive_raw_legacy_expression_v1(&mut legacy, root()).unwrap();

    let mut selected = MirBuilder::new();
    selected.enter_function_for_test("nowait_root_parity/0".to_owned());
    selected.comp_ctx.current_slot_registry = Some(FunctionSlotRegistry::new());
    let selected_value = {
        let mut invocation = ModuleLoweringInvocationV1::with_collector(
            &mut selected,
            ModuleDraftCollectorV1::default(),
        );
        invocation.with_module_port(|builder, module_port| {
            let mut port = RawInvocationChildPortV1::new(module_port);
            drive_legacy_expression_v1(builder, &mut port, root())
        })
    }
    .unwrap();

    assert_eq!(selected_value, legacy_value);
    assert_eq!(
        spanned_instructions(&selected),
        spanned_instructions(&legacy)
    );
    let selected_binding = selected
        .function_state
        .variable_ctx
        .variable_map
        .get("pending");
    let legacy_binding = legacy
        .function_state
        .variable_ctx
        .variable_map
        .get("pending");
    assert_eq!(selected_binding, Some(&selected_value));
    assert_eq!(legacy_binding, Some(&legacy_value));
    assert_eq!(
        selected
            .function_state
            .type_ctx
            .value_types
            .get(&selected_value),
        legacy
            .function_state
            .type_ctx
            .value_types
            .get(&legacy_value)
    );
    assert!(matches!(
        selected
            .function_state
            .type_ctx
            .value_types
            .get(&selected_value),
        Some(MirType::Future(inner)) if **inner == MirType::Integer
    ));
    let selected_slot = selected
        .comp_ctx
        .current_slot_registry
        .as_ref()
        .and_then(|registry| registry.get_slot("pending"));
    let legacy_slot = legacy
        .comp_ctx
        .current_slot_registry
        .as_ref()
        .and_then(|registry| registry.get_slot("pending"));
    assert_eq!(selected_slot, legacy_slot);
    assert!(selected_slot.is_some());
}

#[test]
fn selected_grouped_assignment_matches_raw_legacy_effects_exactly() {
    let root = || {
        grouped_assignment(
            "x",
            ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(integer(21)),
                right: Box::new(integer(22)),
                span: Span::unknown(),
            },
        )
    };
    let mut legacy = MirBuilder::new();
    legacy.enter_function_for_test("grouped_assignment_root_parity/0".to_owned());
    let legacy_old = crate::mir::builder::emission::constant::emit_integer(&mut legacy, 7).unwrap();
    seed_binding(&mut legacy, "x", legacy_old);
    let legacy_value = drive_raw_legacy_expression_v1(&mut legacy, root()).unwrap();

    let mut selected = MirBuilder::new();
    selected.enter_function_for_test("grouped_assignment_root_parity/0".to_owned());
    let selected_old =
        crate::mir::builder::emission::constant::emit_integer(&mut selected, 7).unwrap();
    seed_binding(&mut selected, "x", selected_old);
    let selected_value = drive_selected(&mut selected, root()).unwrap();

    assert_eq!(selected_value, legacy_value);
    assert_eq!(
        spanned_instructions(&selected),
        spanned_instructions(&legacy)
    );
    assert_eq!(
        selected.function_state.variable_ctx.variable_map.get("x"),
        Some(&selected_value)
    );
    assert_eq!(
        legacy.function_state.variable_ctx.variable_map.get("x"),
        Some(&legacy_value)
    );
}

#[test]
fn selected_grouped_assignment_preflights_and_reuses_without_retry() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("grouped_assignment_root_failure/0".to_owned());
    let before = spanned_instructions(&builder);
    let error =
        drive_selected(&mut builder, grouped_assignment("missing", integer(99))).unwrap_err();
    assert!(error.contains("Undefined variable: missing"));
    assert_eq!(spanned_instructions(&builder), before);

    let old = crate::mir::builder::emission::constant::emit_integer(&mut builder, 5).unwrap();
    seed_binding(&mut builder, "x", old);
    let rhs_error = drive_selected(
        &mut builder,
        grouped_assignment("x", variable("missing_rhs")),
    )
    .unwrap_err();
    assert!(rhs_error.contains("Undefined variable: missing_rhs"));
    assert_eq!(
        builder.function_state.variable_ctx.variable_map.get("x"),
        Some(&old)
    );

    let value = drive_selected(&mut builder, grouped_assignment("x", integer(100))).unwrap();
    assert_eq!(
        builder.function_state.variable_ctx.variable_map.get("x"),
        Some(&value)
    );
}

#[test]
fn selected_variable_assignment_composes_without_retry() {
    let roots = [
        assignment(variable("x"), integer(101)),
        block_expr(vec![assignment(variable("x"), integer(102))], variable("x")),
        task_scope("co", vec![assignment(variable("x"), awaited(integer(103)))]),
    ];
    for root in roots {
        let mut legacy = MirBuilder::new();
        legacy.enter_function_for_test("variable_assignment_parity/0".to_owned());
        let old = crate::mir::builder::emission::constant::emit_integer(&mut legacy, 7).unwrap();
        seed_binding(&mut legacy, "x", old);
        let legacy_value = drive_raw_legacy_expression_v1(&mut legacy, root.clone()).unwrap();

        let mut selected = MirBuilder::new();
        selected.enter_function_for_test("variable_assignment_parity/0".to_owned());
        let old = crate::mir::builder::emission::constant::emit_integer(&mut selected, 7).unwrap();
        seed_binding(&mut selected, "x", old);
        let selected_value = drive_selected(&mut selected, root).unwrap();

        assert_eq!(selected_value, legacy_value);
        assert_eq!(
            spanned_instructions(&selected),
            spanned_instructions(&legacy)
        );
        assert_eq!(
            selected.function_state.variable_ctx.variable_map,
            legacy.function_state.variable_ctx.variable_map
        );
    }

    let mut selected = MirBuilder::new();
    selected.enter_function_for_test("variable_assignment_failure/0".to_owned());
    let error = drive_selected(
        &mut selected,
        assignment(variable("missing"), variable("rhs")),
    )
    .unwrap_err();
    assert!(error.contains("Undefined variable: missing"));
    assert!(spanned_instructions(&selected).is_empty());

    let old = crate::mir::builder::emission::constant::emit_integer(&mut selected, 9).unwrap();
    seed_binding(&mut selected, "x", old);
    let error =
        drive_selected(&mut selected, assignment(variable("x"), variable("rhs"))).unwrap_err();
    assert!(error.contains("Undefined variable: rhs"));
    assert_eq!(
        selected.function_state.variable_ctx.variable_map.get("x"),
        Some(&old)
    );
    drive_selected(&mut selected, assignment(variable("x"), integer(104))).unwrap();
}

#[test]
fn selected_index_matches_raw_legacy_effects_exactly() {
    let roots = [
        indexed(array(vec![integer(26), integer(27)]), integer(1)),
        indexed(map(vec![("key", integer(28))]), string("key")),
    ];

    for root in roots {
        let mut legacy = MirBuilder::new();
        legacy.enter_function_for_test("index_root_parity/0".to_owned());
        let legacy_value = drive_raw_legacy_expression_v1(&mut legacy, root.clone()).unwrap();

        let mut selected = MirBuilder::new();
        selected.enter_function_for_test("index_root_parity/0".to_owned());
        let selected_value = drive_selected(&mut selected, root).unwrap();

        assert_eq!(selected_value, legacy_value);
        assert_eq!(
            spanned_instructions(&selected),
            spanned_instructions(&legacy)
        );
        assert_eq!(
            selected.function_state.type_ctx.value_types,
            legacy.function_state.type_ctx.value_types
        );
        assert_eq!(
            selected.function_state.type_ctx.value_origin_newbox,
            legacy.function_state.type_ctx.value_origin_newbox
        );
        assert_eq!(
            format!(
                "{:?}",
                selected
                    .function_state
                    .current_function
                    .as_ref()
                    .expect("selected function")
                    .metadata
                    .fastmem_index_access_sites
            ),
            format!(
                "{:?}",
                legacy
                    .function_state
                    .current_function
                    .as_ref()
                    .expect("legacy function")
                    .metadata
                    .fastmem_index_access_sites
            )
        );
    }
}

#[test]
fn selected_safe_block_prelude_matches_raw_legacy_effects_exactly() {
    let root = || {
        block_expr(
            vec![printed(integer(31)), nowait("pending", integer(32))],
            variable("pending"),
        )
    };
    let mut legacy = MirBuilder::new();
    legacy.enter_function_for_test("safe_block_prelude_root_parity/0".to_owned());
    legacy.comp_ctx.current_slot_registry = Some(FunctionSlotRegistry::new());
    let legacy_value = drive_raw_legacy_expression_v1(&mut legacy, root()).unwrap();

    let mut selected = MirBuilder::new();
    selected.enter_function_for_test("safe_block_prelude_root_parity/0".to_owned());
    selected.comp_ctx.current_slot_registry = Some(FunctionSlotRegistry::new());
    let selected_value = drive_selected(&mut selected, root()).unwrap();

    assert_eq!(selected_value, legacy_value);
    assert_eq!(
        spanned_instructions(&selected),
        spanned_instructions(&legacy)
    );
    assert_eq!(
        selected.function_state.variable_ctx.variable_map,
        legacy.function_state.variable_ctx.variable_map
    );
    assert_eq!(
        selected.function_state.type_ctx.value_types,
        legacy.function_state.type_ctx.value_types
    );
    assert_eq!(
        selected
            .comp_ctx
            .current_slot_registry
            .as_ref()
            .and_then(|registry| registry.get_slot("pending")),
        legacy
            .comp_ctx
            .current_slot_registry
            .as_ref()
            .and_then(|registry| registry.get_slot("pending"))
    );
}

#[test]
fn selected_block_prelude_local_keeps_existing_scope_failure() {
    let root = || {
        block_expr(
            vec![local(&["x"], vec![Some(integer(33))], vec![None])],
            variable("x"),
        )
    };
    let mut legacy = MirBuilder::new();
    legacy.enter_function_for_test("block_prelude_local_failure/0".to_owned());
    let legacy_error = drive_raw_legacy_expression_v1(&mut legacy, root()).unwrap_err();

    let mut selected = MirBuilder::new();
    selected.enter_function_for_test("block_prelude_local_failure/0".to_owned());
    let selected_error = drive_selected(&mut selected, root()).unwrap_err();

    assert_eq!(selected_error, legacy_error);
    assert!(selected_error.contains("local declaration outside lexical scope"));
    assert_eq!(
        spanned_instructions(&selected),
        spanned_instructions(&legacy)
    );
}

#[test]
fn selected_task_scope_matches_raw_legacy_effects_exactly() {
    let root = || {
        task_scope(
            "co",
            vec![
                printed(integer(34)),
                task_scope("task_scope", vec![nowait("pending", integer(35))]),
                printed(variable("pending")),
            ],
        )
    };
    let mut legacy = MirBuilder::new();
    legacy.enter_function_for_test("task_scope_root_parity/0".to_owned());
    legacy.comp_ctx.current_slot_registry = Some(FunctionSlotRegistry::new());
    let legacy_value = drive_raw_legacy_expression_v1(&mut legacy, root()).unwrap();

    let mut selected = MirBuilder::new();
    selected.enter_function_for_test("task_scope_root_parity/0".to_owned());
    selected.comp_ctx.current_slot_registry = Some(FunctionSlotRegistry::new());
    let selected_value = drive_selected(&mut selected, root()).unwrap();

    assert_eq!(selected_value, legacy_value);
    assert_eq!(
        spanned_instructions(&selected),
        spanned_instructions(&legacy)
    );
    assert_eq!(
        selected.function_state.variable_ctx.variable_map,
        legacy.function_state.variable_ctx.variable_map
    );
    assert_eq!(
        selected.function_state.type_ctx.value_types,
        legacy.function_state.type_ctx.value_types
    );
}

#[test]
fn selected_task_scope_child_failure_keeps_pop_order_without_retry() {
    let root = || {
        task_scope(
            "co",
            vec![printed(variable("missing")), printed(integer(36))],
        )
    };
    let mut legacy = MirBuilder::new();
    legacy.enter_function_for_test("task_scope_failure_parity/0".to_owned());
    let legacy_error = drive_raw_legacy_expression_v1(&mut legacy, root()).unwrap_err();

    let mut selected = MirBuilder::new();
    selected.enter_function_for_test("task_scope_failure_parity/0".to_owned());
    let selected_error = drive_selected(&mut selected, root()).unwrap_err();

    assert_eq!(selected_error, legacy_error);
    assert!(selected_error.contains("Undefined variable: missing"));
    assert_eq!(
        spanned_instructions(&selected),
        spanned_instructions(&legacy)
    );
    let task_scope_calls = spanned_instructions(&selected)
        .into_iter()
        .filter(|(instruction, _)| instruction.contains("env.task_scope"))
        .count();
    assert_eq!(task_scope_calls, 2, "push and pop must both remain emitted");
}

#[test]
fn selected_empty_block_expr_matches_raw_legacy_effects_exactly() {
    let root = || {
        block_expr(
            Vec::new(),
            block_expr(
                Vec::new(),
                ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: Box::new(integer(31)),
                    right: Box::new(integer(32)),
                    span: Span::unknown(),
                },
            ),
        )
    };
    let mut legacy = MirBuilder::new();
    legacy.enter_function_for_test("empty_block_expr_root_parity/0".to_owned());
    let legacy_value = drive_raw_legacy_expression_v1(&mut legacy, root()).unwrap();

    let mut selected = MirBuilder::new();
    selected.enter_function_for_test("empty_block_expr_root_parity/0".to_owned());
    let selected_value = drive_selected(&mut selected, root()).unwrap();

    assert_eq!(selected_value, legacy_value);
    assert_eq!(
        spanned_instructions(&selected),
        spanned_instructions(&legacy)
    );
    assert_eq!(
        selected.function_state.type_ctx.value_types,
        legacy.function_state.type_ctx.value_types
    );
}
