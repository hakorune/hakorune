use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::join_ir::lowering::condition_env::ConditionEnv;
use crate::mir::join_ir::lowering::join_value_space::JoinValueSpace;
use crate::mir::join_ir::lowering::loop_scope_shape::LoopScopeShape;
use crate::mir::join_ir::lowering::loop_update_analyzer::UpdateExpr;
use crate::mir::join_ir::lowering::loop_with_break_minimal::lower_loop_with_break_minimal;
use crate::mir::join_ir::lowering::loop_with_if_phi_if_sum::lower_if_sum_pattern;
use crate::mir::join_ir::{
    CompareOp, ConstValue, JoinFuncId, JoinFunction, JoinInst, JoinModule, MirLikeInst,
};
use crate::mir::{BasicBlockId, ValueId};
use crate::{config::env::joinir_dev_enabled, config::env::joinir_test_debug_enabled};
use crate::runtime::get_global_ring0;
use std::collections::{BTreeMap, BTreeSet};

use super::utils::{bin_add, compare, const_i64, select, unary_not};

/// Structured Pattern2 (joinir_min_loop 相当) をテスト用に生成するヘルパー。
pub fn build_pattern2_minimal_structured() -> JoinModule {
    let loop_cond = ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(ASTNode::Variable {
            name: "i".to_string(),
            span: Span::unknown(),
        }),
        right: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(3),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };

    let break_cond = ASTNode::BinaryOp {
        operator: BinaryOperator::GreaterEqual,
        left: Box::new(ASTNode::Variable {
            name: "i".to_string(),
            span: Span::unknown(),
        }),
        right: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(2),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };

    let mut scope = LoopScopeShape {
        header: BasicBlockId(0),
        body: BasicBlockId(1),
        latch: BasicBlockId(2),
        exit: BasicBlockId(3),
        pinned: BTreeSet::new(),
        carriers: BTreeSet::new(),
        body_locals: BTreeSet::new(),
        exit_live: BTreeSet::new(),
        progress_carrier: None,
        variable_definitions: BTreeMap::new(),
    };
    scope.pinned.insert("i".to_string());

    let mut condition_env = ConditionEnv::new();
    condition_env.insert("i".to_string(), ValueId(100));

    let carrier_info = crate::mir::join_ir::lowering::carrier_info::CarrierInfo {
        loop_var_name: "i".to_string(),
        loop_var_id: ValueId(1),
        carriers: vec![],
        trim_helper: None,
        promoted_loopbodylocals: vec![],
        #[cfg(feature = "normalized_dev")]
        promoted_bindings: std::collections::BTreeMap::new(),
    };

    let carrier_updates: BTreeMap<String, UpdateExpr> = BTreeMap::new();
    let mut join_value_space = JoinValueSpace::new();

    let (module, _) = lower_loop_with_break_minimal(
        crate::mir::join_ir::lowering::loop_with_break_minimal::LoopWithBreakLoweringInputs {
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
            body_local_derived_recipe: None, // Phase 94: None for fixture
            body_local_derived_slot_recipe: None, // Phase 29ab P4: None for fixture
            balanced_depth_scan_recipe: None, // Phase 107: None for fixture
        },
    )
    .expect("pattern2 minimal lowering should succeed");

    module
}

/// Phase 47-B: Pattern3 if-sum (multi carrier) を Structured で組み立てるヘルパー。
///
/// Fixture: docs/private/roadmap2/phases/normalized_dev/fixtures/pattern3_if_sum_multi_min.program.json
pub fn build_pattern3_if_sum_multi_min_structured_for_normalized_dev() -> JoinModule {
    // 手書き JoinIR（i/sum/count の 3 キャリア）
    let mut module = JoinModule::new();
    let mut next_id = 0u32;
    let mut alloc = || {
        let id = ValueId(next_id);
        next_id += 1;
        id
    };

    let main_id = JoinFuncId::new(0);
    let loop_id = JoinFuncId::new(1);
    let exit_id = JoinFuncId::new(2);

    // main: init i/sum/count = 0 → tail call loop_step
    let i0 = alloc();
    let sum0 = alloc();
    let count0 = alloc();
    let mut main = JoinFunction::new(main_id, "main".to_string(), vec![]);
    const_i64(&mut main, i0, 0);
    const_i64(&mut main, sum0, 0);
    const_i64(&mut main, count0, 0);
    main.body.push(JoinInst::Call {
        func: loop_id,
        args: vec![i0, sum0, count0],
        k_next: None,
        dst: None,
    });
    module.add_function(main);

    // loop_step params: i, sum, count
    let i_param = alloc();
    let sum_param = alloc();
    let count_param = alloc();
    let mut loop_step = JoinFunction::new(
        loop_id,
        "loop_step".to_string(),
        vec![i_param, sum_param, count_param],
    );

    // loop condition: i < 3
    let limit_const = alloc();
    let cmp_loop = alloc();
    let exit_cond = alloc();
    const_i64(&mut loop_step, limit_const, 3);
    compare(
        &mut loop_step,
        cmp_loop,
        CompareOp::Lt,
        i_param,
        limit_const,
    );
    unary_not(&mut loop_step, exit_cond, cmp_loop);
    loop_step.body.push(JoinInst::Jump {
        cont: exit_id.as_cont(),
        args: vec![sum_param, count_param],
        cond: Some(exit_cond),
    });

    // if condition: i > 0
    let cond_cmp = alloc();
    let zero_const = alloc();
    const_i64(&mut loop_step, zero_const, 0);
    compare(&mut loop_step, cond_cmp, CompareOp::Gt, i_param, zero_const);

    // then: sum = sum + 1, count = count + 1
    let one_const = alloc();
    let sum_then = alloc();
    let count_then = alloc();
    const_i64(&mut loop_step, one_const, 1);
    bin_add(&mut loop_step, sum_then, sum_param, one_const);
    bin_add(&mut loop_step, count_then, count_param, one_const);

    // else: identity
    let sum_else = alloc();
    let count_else = alloc();
    bin_add(&mut loop_step, sum_else, sum_param, zero_const);
    bin_add(&mut loop_step, count_else, count_param, zero_const);

    // select
    let sum_new = alloc();
    let count_new = alloc();
    select(&mut loop_step, sum_new, cond_cmp, sum_then, sum_else);
    select(&mut loop_step, count_new, cond_cmp, count_then, count_else);

    // counter update: i = i + 1
    let one_const2 = alloc();
    let i_next = alloc();
    const_i64(&mut loop_step, one_const2, 1);
    bin_add(&mut loop_step, i_next, i_param, one_const2);

    loop_step.body.push(JoinInst::Call {
        func: loop_id,
        args: vec![i_next, sum_new, count_new],
        k_next: None,
        dst: None,
    });
    module.add_function(loop_step);

    // k_exit(sum, count)
    let sum_final = alloc();
    let count_final = alloc();
    let mut k_exit = JoinFunction::new(exit_id, "k_exit".to_string(), vec![sum_final, count_final]);
    k_exit.body.push(JoinInst::Ret {
        value: Some(sum_final),
    });
    module.add_function(k_exit);

    module.entry = Some(main_id);
    module.phase = crate::mir::join_ir::JoinIrPhase::Structured;
    module
}

/// Phase 47-B: JsonParser if-sum mini を Structured で組み立てるヘルパー。
///
/// Fixture: docs/private/roadmap2/phases/normalized_dev/fixtures/jsonparser_if_sum_min.program.json
pub fn build_pattern3_json_if_sum_min_structured_for_normalized_dev() -> JoinModule {
    // 手書き JoinIR（i/sum の 2 キャリア、JsonParser 由来の簡約 if-sum）
    let mut module = JoinModule::new();
    let mut next_id = 0u32;
    let mut alloc = || {
        let id = ValueId(next_id);
        next_id += 1;
        id
    };

    let main_id = JoinFuncId::new(0);
    let loop_id = JoinFuncId::new(1);
    let exit_id = JoinFuncId::new(2);

    // main: init i/sum = 0 → tail call loop_step
    let i0 = alloc();
    let sum0 = alloc();
    let mut main = JoinFunction::new(main_id, "main".to_string(), vec![]);
    main.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: i0,
        value: ConstValue::Integer(0),
    }));
    main.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: sum0,
        value: ConstValue::Integer(0),
    }));
    main.body.push(JoinInst::Call {
        func: loop_id,
        args: vec![i0, sum0],
        k_next: None,
        dst: None,
    });
    module.add_function(main);

    // loop_step params: i, sum
    let i_param = alloc();
    let sum_param = alloc();
    let mut loop_step =
        JoinFunction::new(loop_id, "loop_step".to_string(), vec![i_param, sum_param]);

    // loop condition: i < 5
    let limit_const = alloc();
    let cmp_loop = alloc();
    let exit_cond = alloc();
    loop_step.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: limit_const,
        value: ConstValue::Integer(5),
    }));
    compare(
        &mut loop_step,
        cmp_loop,
        CompareOp::Lt,
        i_param,
        limit_const,
    );
    unary_not(&mut loop_step, exit_cond, cmp_loop);
    loop_step.body.push(JoinInst::Jump {
        cont: exit_id.as_cont(),
        args: vec![sum_param],
        cond: Some(exit_cond),
    });

    // if condition: i > 0
    let cond_cmp = alloc();
    let zero_const = alloc();
    const_i64(&mut loop_step, zero_const, 0);
    compare(&mut loop_step, cond_cmp, CompareOp::Gt, i_param, zero_const);

    // then: sum = sum + i
    let sum_then = alloc();
    bin_add(&mut loop_step, sum_then, sum_param, i_param);

    // else: identity sum
    let sum_else = alloc();
    bin_add(&mut loop_step, sum_else, sum_param, zero_const);

    // select
    let sum_new = alloc();
    select(&mut loop_step, sum_new, cond_cmp, sum_then, sum_else);

    // counter update: i = i + 1
    let one_const = alloc();
    let i_next = alloc();
    const_i64(&mut loop_step, one_const, 1);
    bin_add(&mut loop_step, i_next, i_param, one_const);

    loop_step.body.push(JoinInst::Call {
        func: loop_id,
        args: vec![i_next, sum_new],
        k_next: None,
        dst: None,
    });
    module.add_function(loop_step);

    // k_exit(sum)
    let sum_final = alloc();
    let mut k_exit = JoinFunction::new(exit_id, "k_exit".to_string(), vec![sum_final]);
    k_exit.body.push(JoinInst::Ret {
        value: Some(sum_final),
    });
    module.add_function(k_exit);

    module.entry = Some(main_id);
    module.phase = crate::mir::join_ir::JoinIrPhase::Structured;
    module
}

/// Pattern3 if-sum minimal ループ（phase212_if_sum_min.hako 相当）を Structured で組み立てるヘルパー。
///
/// Phase 47-A: P3 Normalized の最小ケース検証用（dev-only）。
pub fn build_pattern3_if_sum_min_structured_for_normalized_dev() -> JoinModule {
    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn int_lit(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn bin(op: BinaryOperator, left: ASTNode, right: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator: op,
            left: Box::new(left),
            right: Box::new(right),
            span: Span::unknown(),
        }
    }

    fn assignment(target: ASTNode, value: ASTNode) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(target),
            value: Box::new(value),
            span: Span::unknown(),
        }
    }

    // Minimal if-sum pattern:
    // loop(i < 3) {
    //   if (i > 0) { sum = sum + 1 } else { sum = sum + 0 }
    //   i = i + 1
    // }
    let loop_condition = bin(BinaryOperator::Less, var("i"), int_lit(3));
    let if_condition = bin(BinaryOperator::Greater, var("i"), int_lit(0));

    let then_update = assignment(var("sum"), bin(BinaryOperator::Add, var("sum"), int_lit(1)));
    let else_update = assignment(var("sum"), bin(BinaryOperator::Add, var("sum"), int_lit(0)));
    let counter_update = assignment(var("i"), bin(BinaryOperator::Add, var("i"), int_lit(1)));

    let if_stmt = ASTNode::If {
        condition: Box::new(if_condition),
        then_body: vec![then_update],
        else_body: Some(vec![else_update]),
        span: Span::unknown(),
    };
    let body = vec![if_stmt.clone(), counter_update];

    let mut join_value_space = JoinValueSpace::new();
    let mut cond_env = ConditionEnv::new();

    // Phase 220-D: ConditionEnv には少なくともループ変数 i を登録しておく。
    let i_id = join_value_space.alloc_param();
    cond_env.insert("i".to_string(), i_id);

    let (module, _meta) = lower_if_sum_pattern(
        &loop_condition,
        &if_stmt,
        &body,
        &cond_env,
        &mut join_value_space,
    )
    .expect("if-sum lowering should succeed for minimal pattern3");

    if joinir_dev_enabled() && joinir_test_debug_enabled() {
        get_global_ring0().log.debug(&format!(
            "[joinir/normalized-dev] pattern3_if_sum_min structured module: {:#?}",
            module
        ));
    }

    module
}
