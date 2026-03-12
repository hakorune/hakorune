use super::{run_joinir_function, JoinValue};
use crate::backend::mir_interpreter::MirInterpreter;
use crate::mir::join_ir::{
    ConstValue, JoinFuncId, JoinFunction, JoinInst, JoinModule, MirLikeInst,
};
use crate::mir::ValueId;

#[test]
fn test_select_true() {
    // let result = if true { 1 } else { 2 }
    // expected: result == 1
    let mut module = JoinModule::new();
    let mut func = JoinFunction::new(JoinFuncId::new(0), "test_func".to_string(), vec![]);

    let v_cond = ValueId(1);
    let v_then = ValueId(2);
    let v_else = ValueId(3);
    let v_result = ValueId(4);

    // const v1 = true
    func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: v_cond,
        value: ConstValue::Bool(true),
    }));

    // const v2 = 1
    func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: v_then,
        value: ConstValue::Integer(1),
    }));

    // const v3 = 2
    func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: v_else,
        value: ConstValue::Integer(2),
    }));

    // select v4 = v1 ? v2 : v3
    func.body.push(JoinInst::Select {
        dst: v_result,
        cond: v_cond,
        then_val: v_then,
        else_val: v_else,
        type_hint: None, // Phase 63-3
    });

    // return v4
    func.body.push(JoinInst::Ret {
        value: Some(v_result),
    });

    module.add_function(func);

    let mut vm = MirInterpreter::new();
    let result = run_joinir_function(&mut vm, &module, JoinFuncId::new(0), &[]).unwrap();

    assert_eq!(result, JoinValue::Int(1));
}

#[test]
fn test_select_false() {
    // let result = if false { 1 } else { 2 }
    // expected: result == 2
    let mut module = JoinModule::new();
    let mut func = JoinFunction::new(JoinFuncId::new(0), "test_func".to_string(), vec![]);

    let v_cond = ValueId(1);
    let v_then = ValueId(2);
    let v_else = ValueId(3);
    let v_result = ValueId(4);

    // const v1 = false
    func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: v_cond,
        value: ConstValue::Bool(false),
    }));

    // const v2 = 1
    func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: v_then,
        value: ConstValue::Integer(1),
    }));

    // const v3 = 2
    func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: v_else,
        value: ConstValue::Integer(2),
    }));

    // select v4 = v1 ? v2 : v3
    func.body.push(JoinInst::Select {
        dst: v_result,
        cond: v_cond,
        then_val: v_then,
        else_val: v_else,
        type_hint: None, // Phase 63-3
    });

    // return v4
    func.body.push(JoinInst::Ret {
        value: Some(v_result),
    });

    module.add_function(func);

    let mut vm = MirInterpreter::new();
    let result = run_joinir_function(&mut vm, &module, JoinFuncId::new(0), &[]).unwrap();

    assert_eq!(result, JoinValue::Int(2));
}

#[test]
fn test_select_int_cond() {
    // cond=Int(0) → false、Int(1) → true
    let mut module = JoinModule::new();
    let mut func = JoinFunction::new(JoinFuncId::new(0), "test_func".to_string(), vec![]);

    let v_cond = ValueId(1);
    let v_then = ValueId(2);
    let v_else = ValueId(3);
    let v_result = ValueId(4);

    // const v1 = 0 (treated as false)
    func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: v_cond,
        value: ConstValue::Integer(0),
    }));

    // const v2 = 100
    func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: v_then,
        value: ConstValue::Integer(100),
    }));

    // const v3 = 200
    func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: v_else,
        value: ConstValue::Integer(200),
    }));

    // select v4 = v1 ? v2 : v3
    func.body.push(JoinInst::Select {
        dst: v_result,
        cond: v_cond,
        then_val: v_then,
        else_val: v_else,
        type_hint: None, // Phase 63-3
    });

    // return v4
    func.body.push(JoinInst::Ret {
        value: Some(v_result),
    });

    module.add_function(func);

    let mut vm = MirInterpreter::new();
    let result = run_joinir_function(&mut vm, &module, JoinFuncId::new(0), &[]).unwrap();

    assert_eq!(result, JoinValue::Int(200)); // 0 is false, so should select else
}

// Phase 33-6: IfMerge instruction tests
#[test]
fn test_if_merge_true() {
    // if true { x=1; y=2 } else { x=3; y=4 }
    // expected: x=1, y=2
    let mut module = JoinModule::new();
    let mut func = JoinFunction::new(JoinFuncId::new(0), "test_func".to_string(), vec![]);

    let v_cond = ValueId(1);
    let v_then_x = ValueId(2);
    let v_then_y = ValueId(3);
    let v_else_x = ValueId(4);
    let v_else_y = ValueId(5);
    let v_result_x = ValueId(6);
    let v_result_y = ValueId(7);

    // const v1 = true
    func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: v_cond,
        value: ConstValue::Bool(true),
    }));

    // const v2 = 1 (then x)
    func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: v_then_x,
        value: ConstValue::Integer(1),
    }));

    // const v3 = 2 (then y)
    func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: v_then_y,
        value: ConstValue::Integer(2),
    }));

    // const v4 = 3 (else x)
    func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: v_else_x,
        value: ConstValue::Integer(3),
    }));

    // const v5 = 4 (else y)
    func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: v_else_y,
        value: ConstValue::Integer(4),
    }));

    // if_merge v1 { v6=v2; v7=v3 } else { v6=v4; v7=v5 }
    func.body.push(JoinInst::IfMerge {
        cond: v_cond,
        merges: vec![
            crate::mir::join_ir::MergePair {
                dst: v_result_x,
                then_val: v_then_x,
                else_val: v_else_x,
                type_hint: None, // Phase 63-3
            },
            crate::mir::join_ir::MergePair {
                dst: v_result_y,
                then_val: v_then_y,
                else_val: v_else_y,
                type_hint: None, // Phase 63-3
            },
        ],
        k_next: None,
    });

    // return v6 + v7
    let v_sum = ValueId(8);
    func.body.push(JoinInst::Compute(MirLikeInst::BinOp {
        dst: v_sum,
        op: crate::mir::join_ir::BinOpKind::Add,
        lhs: v_result_x,
        rhs: v_result_y,
    }));

    func.body.push(JoinInst::Ret { value: Some(v_sum) });

    module.add_function(func);

    let mut vm = MirInterpreter::new();
    let result = run_joinir_function(&mut vm, &module, JoinFuncId::new(0), &[]).unwrap();

    assert_eq!(result, JoinValue::Int(3)); // 1 + 2 = 3
}

#[test]
fn test_if_merge_false() {
    // if false { x=1; y=2 } else { x=3; y=4 }
    // expected: x=3, y=4
    let mut module = JoinModule::new();
    let mut func = JoinFunction::new(JoinFuncId::new(0), "test_func".to_string(), vec![]);

    let v_cond = ValueId(1);
    let v_then_x = ValueId(2);
    let v_then_y = ValueId(3);
    let v_else_x = ValueId(4);
    let v_else_y = ValueId(5);
    let v_result_x = ValueId(6);
    let v_result_y = ValueId(7);

    // const v1 = false
    func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: v_cond,
        value: ConstValue::Bool(false),
    }));

    // const v2 = 1 (then x)
    func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: v_then_x,
        value: ConstValue::Integer(1),
    }));

    // const v3 = 2 (then y)
    func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: v_then_y,
        value: ConstValue::Integer(2),
    }));

    // const v4 = 3 (else x)
    func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: v_else_x,
        value: ConstValue::Integer(3),
    }));

    // const v5 = 4 (else y)
    func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: v_else_y,
        value: ConstValue::Integer(4),
    }));

    // if_merge v1 { v6=v2; v7=v3 } else { v6=v4; v7=v5 }
    func.body.push(JoinInst::IfMerge {
        cond: v_cond,
        merges: vec![
            crate::mir::join_ir::MergePair {
                dst: v_result_x,
                then_val: v_then_x,
                else_val: v_else_x,
                type_hint: None, // Phase 63-3
            },
            crate::mir::join_ir::MergePair {
                dst: v_result_y,
                then_val: v_then_y,
                else_val: v_else_y,
                type_hint: None, // Phase 63-3
            },
        ],
        k_next: None,
    });

    // return v6 + v7
    let v_sum = ValueId(8);
    func.body.push(JoinInst::Compute(MirLikeInst::BinOp {
        dst: v_sum,
        op: crate::mir::join_ir::BinOpKind::Add,
        lhs: v_result_x,
        rhs: v_result_y,
    }));

    func.body.push(JoinInst::Ret { value: Some(v_sum) });

    module.add_function(func);

    let mut vm = MirInterpreter::new();
    let result = run_joinir_function(&mut vm, &module, JoinFuncId::new(0), &[]).unwrap();

    assert_eq!(result, JoinValue::Int(7)); // 3 + 4 = 7
}

#[test]
fn test_if_merge_multiple() {
    // if true { x=10; y=20; z=30 } else { x=1; y=2; z=3 }
    // expected: x=10, y=20, z=30 → sum=60
    let mut module = JoinModule::new();
    let mut func = JoinFunction::new(JoinFuncId::new(0), "test_func".to_string(), vec![]);

    let v_cond = ValueId(1);
    let v_then_x = ValueId(2);
    let v_then_y = ValueId(3);
    let v_then_z = ValueId(4);
    let v_else_x = ValueId(5);
    let v_else_y = ValueId(6);
    let v_else_z = ValueId(7);
    let v_result_x = ValueId(8);
    let v_result_y = ValueId(9);
    let v_result_z = ValueId(10);

    // const v1 = true
    func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: v_cond,
        value: ConstValue::Bool(true),
    }));

    // then values
    func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: v_then_x,
        value: ConstValue::Integer(10),
    }));
    func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: v_then_y,
        value: ConstValue::Integer(20),
    }));
    func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: v_then_z,
        value: ConstValue::Integer(30),
    }));

    // else values
    func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: v_else_x,
        value: ConstValue::Integer(1),
    }));
    func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: v_else_y,
        value: ConstValue::Integer(2),
    }));
    func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: v_else_z,
        value: ConstValue::Integer(3),
    }));

    // if_merge with 3 variables
    func.body.push(JoinInst::IfMerge {
        cond: v_cond,
        merges: vec![
            crate::mir::join_ir::MergePair {
                dst: v_result_x,
                then_val: v_then_x,
                else_val: v_else_x,
                type_hint: None, // Phase 63-3
            },
            crate::mir::join_ir::MergePair {
                dst: v_result_y,
                then_val: v_then_y,
                else_val: v_else_y,
                type_hint: None, // Phase 63-3
            },
            crate::mir::join_ir::MergePair {
                dst: v_result_z,
                then_val: v_then_z,
                else_val: v_else_z,
                type_hint: None, // Phase 63-3
            },
        ],
        k_next: None,
    });

    // return x + y + z
    let v_sum_xy = ValueId(11);
    func.body.push(JoinInst::Compute(MirLikeInst::BinOp {
        dst: v_sum_xy,
        op: crate::mir::join_ir::BinOpKind::Add,
        lhs: v_result_x,
        rhs: v_result_y,
    }));

    let v_sum_xyz = ValueId(12);
    func.body.push(JoinInst::Compute(MirLikeInst::BinOp {
        dst: v_sum_xyz,
        op: crate::mir::join_ir::BinOpKind::Add,
        lhs: v_sum_xy,
        rhs: v_result_z,
    }));

    func.body.push(JoinInst::Ret {
        value: Some(v_sum_xyz),
    });

    module.add_function(func);

    let mut vm = MirInterpreter::new();
    let result = run_joinir_function(&mut vm, &module, JoinFuncId::new(0), &[]).unwrap();

    assert_eq!(result, JoinValue::Int(60)); // 10 + 20 + 30 = 60
}
