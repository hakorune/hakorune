use super::*;
use crate::ast::Span;
use crate::mir::join_ir::lowering::loop_body_local_env::LoopBodyLocalEnv;
use crate::mir::join_ir::JoinInst;

#[test]
fn test_resolve_string_length() {
    let method_id = CoreMethodId::iter().find(|m| m.name() == "length");
    assert!(method_id.is_some());
    assert!(method_id.unwrap().allowed_in_condition());
}

#[test]
fn test_lower_string_length_for_condition() {
    let recv_val = ValueId(10);
    let mut value_counter = 100u32;
    let mut alloc_value = || {
        let id = ValueId(value_counter);
        value_counter += 1;
        id
    };
    let mut instructions = Vec::new();
    let env = ConditionEnv::new();

    let result = MethodCallLowerer::lower_for_condition(
        recv_val,
        "length",
        &[],
        &mut alloc_value,
        &env,
        &mut instructions,
    );

    assert!(result.is_ok());
    let result_val = result.unwrap();
    assert_eq!(result_val, ValueId(100));
    assert_eq!(instructions.len(), 1);

    match &instructions[0] {
        JoinInst::Compute(MirLikeInst::BoxCall {
            dst,
            box_name,
            method,
            args,
        }) => {
            assert_eq!(*dst, Some(ValueId(100)));
            assert_eq!(box_name, "StringBox");
            assert_eq!(method, "length");
            assert_eq!(args.len(), 1);
            assert_eq!(args[0], ValueId(10));
        }
        _ => panic!("Expected BoxCall instruction"),
    }
}

#[test]
fn test_not_allowed_in_condition() {
    let recv_val = ValueId(10);
    let mut value_counter = 100u32;
    let mut alloc_value = || {
        let id = ValueId(value_counter);
        value_counter += 1;
        id
    };
    let mut instructions = Vec::new();
    let env = ConditionEnv::new();

    let result = MethodCallLowerer::lower_for_condition(
        recv_val,
        "toUpper",
        &[],
        &mut alloc_value,
        &env,
        &mut instructions,
    );

    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .contains("not allowed in loop condition"));
}

#[test]
fn test_unknown_method() {
    let recv_val = ValueId(10);
    let mut value_counter = 100u32;
    let mut alloc_value = || {
        let id = ValueId(value_counter);
        value_counter += 1;
        id
    };
    let mut instructions = Vec::new();
    let env = ConditionEnv::new();

    let result = MethodCallLowerer::lower_for_condition(
        recv_val,
        "unknownMethod",
        &[],
        &mut alloc_value,
        &env,
        &mut instructions,
    );

    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .contains("not recognized as CoreMethodId"));
}

#[test]
fn test_lower_substring_for_init() {
    let recv_val = ValueId(10);
    let i_val = ValueId(11);
    let j_val = ValueId(12);
    let mut value_counter = 100u32;
    let mut alloc_value = || {
        let id = ValueId(value_counter);
        value_counter += 1;
        id
    };
    let mut instructions = Vec::new();

    let mut env = ConditionEnv::new();
    env.insert("i".to_string(), i_val);
    env.insert("j".to_string(), j_val);

    let arg1_ast = ASTNode::Variable {
        name: "i".to_string(),
        span: crate::ast::Span::unknown(),
    };
    let arg2_ast = ASTNode::Variable {
        name: "j".to_string(),
        span: crate::ast::Span::unknown(),
    };

    let cond_result = MethodCallLowerer::lower_for_condition(
        recv_val,
        "substring",
        &[arg1_ast.clone(), arg2_ast.clone()],
        &mut alloc_value,
        &env,
        &mut instructions,
    );
    assert!(cond_result.is_err());
    assert!(cond_result
        .unwrap_err()
        .contains("not allowed in loop condition"));

    let body_local_env = LoopBodyLocalEnv::new();
    instructions.clear();
    let init_result = MethodCallLowerer::lower_for_init(
        recv_val,
        "substring",
        &[arg1_ast, arg2_ast],
        &mut alloc_value,
        &env,
        &body_local_env,
        &mut instructions,
    );
    assert!(init_result.is_ok());
    assert_eq!(instructions.len(), 1);
}

#[test]
fn test_phase224c_arity_mismatch() {
    let recv_val = ValueId(10);
    let mut value_counter = 100u32;
    let mut alloc_value = || {
        let id = ValueId(value_counter);
        value_counter += 1;
        id
    };
    let mut instructions = Vec::new();
    let env = ConditionEnv::new();

    let dummy_arg = ASTNode::Literal {
        value: crate::ast::LiteralValue::Integer(1),
        span: crate::ast::Span::unknown(),
    };

    let result = MethodCallLowerer::lower_for_condition(
        recv_val,
        "length",
        &[dummy_arg],
        &mut alloc_value,
        &env,
        &mut instructions,
    );

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Arity mismatch"));
}

#[test]
fn test_lower_index_of_with_arg() {
    let recv_val = ValueId(10);
    let ch_val = ValueId(11);
    let mut value_counter = 100u32;
    let mut alloc_value = || {
        let id = ValueId(value_counter);
        value_counter += 1;
        id
    };
    let mut instructions = Vec::new();

    let body_local_env = LoopBodyLocalEnv::new();

    let mut env = ConditionEnv::new();
    env.insert("ch".to_string(), ch_val);

    let arg_ast = ASTNode::Variable {
        name: "ch".to_string(),
        span: crate::ast::Span::unknown(),
    };

    let result = MethodCallLowerer::lower_for_init(
        recv_val,
        "indexOf",
        &[arg_ast],
        &mut alloc_value,
        &env,
        &body_local_env,
        &mut instructions,
    );

    assert!(result.is_ok());
    let result_val = result.unwrap();
    assert_eq!(result_val, ValueId(100));
    assert_eq!(instructions.len(), 1);

    match &instructions[0] {
        JoinInst::Compute(MirLikeInst::BoxCall {
            dst,
            box_name,
            method,
            args,
        }) => {
            assert_eq!(*dst, Some(ValueId(100)));
            assert_eq!(box_name, "StringBox");
            assert_eq!(method, "indexOf");
            assert_eq!(args.len(), 2);
            assert_eq!(args[0], ValueId(10));
            assert_eq!(args[1], ValueId(11));
        }
        _ => panic!("Expected BoxCall instruction"),
    }
}

#[test]
fn test_cascading_resolves_body_local_first() {
    let mut body_env = LoopBodyLocalEnv::new();
    body_env.insert("ch".to_string(), ValueId(2));

    let mut cond_env = ConditionEnv::new();
    cond_env.insert("s".to_string(), ValueId(1));

    let recv_val = ValueId(1);
    let mut next = 100u32;
    let mut alloc_value = || {
        let id = ValueId(next);
        next += 1;
        id
    };
    let mut instructions = Vec::new();

    let result = MethodCallLowerer::lower_for_init(
        recv_val,
        "indexOf",
        &[ASTNode::Variable {
            name: "ch".to_string(),
            span: Span::unknown(),
        }],
        &mut alloc_value,
        &cond_env,
        &body_env,
        &mut instructions,
    )
    .expect("lower_for_init should succeed");

    let boxcall = instructions
        .iter()
        .find_map(|inst| match inst {
            JoinInst::Compute(MirLikeInst::BoxCall { args, .. }) => Some(args.clone()),
            _ => None,
        })
        .expect("BoxCall not emitted");
    assert_eq!(boxcall, vec![ValueId(1), ValueId(2)]);
    assert!(result.0 >= 100);
}

#[test]
fn test_lower_substring_with_args() {
    let recv_val = ValueId(10);
    let i_val = ValueId(11);
    let j_val = ValueId(12);
    let mut value_counter = 100u32;
    let mut alloc_value = || {
        let id = ValueId(value_counter);
        value_counter += 1;
        id
    };
    let mut instructions = Vec::new();

    let body_local_env = LoopBodyLocalEnv::new();

    let mut env = ConditionEnv::new();
    env.insert("i".to_string(), i_val);
    env.insert("j".to_string(), j_val);

    let arg1_ast = ASTNode::Variable {
        name: "i".to_string(),
        span: crate::ast::Span::unknown(),
    };
    let arg2_ast = ASTNode::Variable {
        name: "j".to_string(),
        span: crate::ast::Span::unknown(),
    };

    let result = MethodCallLowerer::lower_for_init(
        recv_val,
        "substring",
        &[arg1_ast, arg2_ast],
        &mut alloc_value,
        &env,
        &body_local_env,
        &mut instructions,
    );

    assert!(result.is_ok());
    let result_val = result.unwrap();
    assert_eq!(result_val, ValueId(100));
    assert_eq!(instructions.len(), 1);

    match &instructions[0] {
        JoinInst::Compute(MirLikeInst::BoxCall {
            dst,
            box_name,
            method,
            args,
        }) => {
            assert_eq!(*dst, Some(ValueId(100)));
            assert_eq!(box_name, "StringBox");
            assert_eq!(method, "substring");
            assert_eq!(args.len(), 3);
            assert_eq!(args[0], ValueId(10));
            assert_eq!(args[1], ValueId(11));
            assert_eq!(args[2], ValueId(12));
        }
        _ => panic!("Expected BoxCall instruction"),
    }
}
