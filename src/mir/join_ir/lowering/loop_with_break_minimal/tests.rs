use super::*;
use crate::ast::Span;
use crate::mir::join_ir::lowering::loop_scope_shape::LoopScopeShape;
use crate::mir::loop_route_detection::loop_condition_scope::CondVarScope;
use std::collections::{BTreeMap, BTreeSet};

fn var_node(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: Span::unknown(),
    }
}

fn int_literal_node(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: crate::ast::LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn binop_node(left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator: crate::ast::BinaryOperator::Less,
        left: Box::new(left),
        right: Box::new(right),
        span: Span::unknown(),
    }
}

fn minimal_scope() -> LoopScopeShape {
    LoopScopeShape {
        header: crate::mir::BasicBlockId(0),
        body: crate::mir::BasicBlockId(1),
        latch: crate::mir::BasicBlockId(2),
        exit: crate::mir::BasicBlockId(3),
        pinned: BTreeSet::new(),
        carriers: BTreeSet::new(),
        body_locals: BTreeSet::new(),
        exit_live: BTreeSet::new(),
        progress_carrier: None,
        variable_definitions: BTreeMap::new(),
    }
}

fn scope_with_outer_var(var_name: &str) -> LoopScopeShape {
    let mut scope = minimal_scope();
    let mut pinned = BTreeSet::new();
    pinned.insert(var_name.to_string());
    scope.pinned = pinned;
    scope
}

fn scope_with_body_local_var(var_name: &str) -> LoopScopeShape {
    let mut scope = minimal_scope();
    let mut body_locals = BTreeSet::new();
    body_locals.insert(var_name.to_string());
    scope.body_locals = body_locals;
    scope
}

#[test]
fn test_loop_break_accepts_loop_param_only() {
    let loop_cond = binop_node(var_node("i"), int_literal_node(10));
    let break_cond = binop_node(var_node("i"), int_literal_node(5));

    let scope = scope_with_outer_var("i");
    let cond_scope = LoopConditionScopeBox::analyze("i", &[&loop_cond, &break_cond], Some(&scope));

    assert!(!cond_scope.has_loop_body_local());
    assert_eq!(cond_scope.var_names().len(), 1);
    assert!(cond_scope.var_names().contains("i"));
}

#[test]
fn test_loop_break_accepts_outer_scope_variables() {
    let loop_cond = binop_node(var_node("i"), var_node("end"));
    let break_cond = binop_node(var_node("i"), var_node("threshold"));

    let mut scope = minimal_scope();
    let mut pinned = BTreeSet::new();
    pinned.insert("i".to_string());
    pinned.insert("end".to_string());
    pinned.insert("threshold".to_string());
    scope.pinned = pinned;

    let cond_scope = LoopConditionScopeBox::analyze("i", &[&loop_cond, &break_cond], Some(&scope));

    assert!(!cond_scope.has_loop_body_local());
    assert_eq!(cond_scope.var_names().len(), 3);
}

#[test]
fn test_loop_break_rejects_loop_body_local_variables() {
    let loop_cond = binop_node(var_node("i"), var_node("10"));
    let break_cond = binop_node(var_node("ch"), var_node("' '"));

    let scope = scope_with_body_local_var("ch");
    let cond_scope = LoopConditionScopeBox::analyze("i", &[&loop_cond, &break_cond], Some(&scope));

    assert!(cond_scope.has_loop_body_local());
    let body_local_vars: Vec<&String> = cond_scope
        .vars
        .iter()
        .filter(|v| v.scope == CondVarScope::LoopBodyLocal)
        .map(|v| &v.name)
        .collect();
    assert_eq!(body_local_vars.len(), 1);
    assert_eq!(*body_local_vars[0], "ch");
}

#[test]
fn test_loop_break_header_condition_via_exprlowerer() {
    use crate::mir::join_ir::lowering::condition_env::ConditionEnv;
    use crate::mir::join_ir::lowering::join_value_space::JoinValueSpace;

    let loop_cond = binop_node(var_node("i"), int_literal_node(10));
    let break_cond = binop_node(var_node("i"), int_literal_node(5));

    let scope = minimal_scope();

    let mut condition_env = ConditionEnv::new();
    condition_env.insert("i".to_string(), ValueId(100));

    let carrier_info = CarrierInfo {
        loop_var_name: "i".to_string(),
        loop_var_id: ValueId(1),
        carriers: vec![],
        trim_helper: None,
        promoted_body_locals: vec![],
    };

    let carrier_updates = BTreeMap::new();
    let mut join_value_space = JoinValueSpace::new();

    let result = lower_loop_with_break_minimal(super::LoopWithBreakLoweringInputs {
        scope,
        condition: &loop_cond,
        break_condition: &break_cond,
        env: &condition_env,
        carrier_info: &carrier_info,
        carrier_updates: &carrier_updates,
        body_ast: &[],
        body_local_env: None,
        allowed_body_locals_for_conditions: None,
        join_value_space: &mut join_value_space,
        skeleton: None, // Phase 92 P0-3: skeleton=None for backward compatibility
        condition_only_recipe: None, // Phase 93 P0: None for normal loops
        body_local_derived_recipe: None, // Phase 94: None for normal loops
        body_local_derived_slot_recipe: None, // Phase 29ab P4: None for normal loops
        balanced_depth_scan_recipe: None, // Phase 107: None for normal loops
        current_static_box_name: None, // Phase 252: No static box context in test
    });

    assert!(result.is_ok(), "ExprLowerer header path should succeed");

    let (join_module, _fragment_meta) = result.unwrap();
    assert_eq!(join_module.functions.len(), 3);

    let loop_step_func = join_module
        .functions
        .values()
        .find(|f| f.name == "loop_step")
        .expect("loop_step should exist");

    let compare_count = loop_step_func
        .body
        .iter()
        .filter(|inst| matches!(inst, JoinInst::Compute(MirLikeInst::Compare { .. })))
        .count();

    assert!(
        compare_count >= 2,
        "header + break should emit at least two Compare instructions"
    );
}
