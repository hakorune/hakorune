use super::*;
use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::control_tree::step_tree::StepTreeBuilderBox;

#[test]
fn test_loop_true_break_once_passes_updated_env_to_k_exit() {
    let span = Span::unknown();
    let ast = ASTNode::Program {
        statements: vec![
            ASTNode::Assignment {
                target: Box::new(ASTNode::Variable {
                    name: "x".to_string(),
                    span: span.clone(),
                }),
                value: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(0),
                    span: span.clone(),
                }),
                span: span.clone(),
            },
            ASTNode::Loop {
                condition: Box::new(ASTNode::Literal {
                    value: LiteralValue::Bool(true),
                    span: span.clone(),
                }),
                body: vec![
                    ASTNode::Assignment {
                        target: Box::new(ASTNode::Variable {
                            name: "x".to_string(),
                            span: span.clone(),
                        }),
                        value: Box::new(ASTNode::Literal {
                            value: LiteralValue::Integer(1),
                            span: span.clone(),
                        }),
                        span: span.clone(),
                    },
                    ASTNode::Break { span: span.clone() },
                ],
                span: span.clone(),
            },
            ASTNode::Return {
                value: Some(Box::new(ASTNode::Variable {
                    name: "x".to_string(),
                    span: span.clone(),
                })),
                span: span.clone(),
            },
        ],
        span,
    };

    let step_tree = StepTreeBuilderBox::build_from_ast(&ast);
    let env_layout = EnvLayout::from_contract(&step_tree.contract, &BTreeMap::new());

    let Some((module, _meta)) =
        LoopTrueBreakOnceBuilderBox::lower(&step_tree, &env_layout).expect("lower failed")
    else {
        panic!("expected loop_true_break_once pattern to be in-scope");
    };

    let loop_body_id = JoinFuncId::new(3);
    let k_exit_id = JoinFuncId::new(2);
    let loop_body = module
        .functions
        .get(&loop_body_id)
        .expect("phase131 test: missing loop_body function");
    let k_exit = module
        .functions
        .get(&k_exit_id)
        .expect("phase131 test: missing k_exit function");

    let const_one_dst = loop_body
        .body
        .iter()
        .find_map(|inst| match inst {
            JoinInst::Compute(MirLikeInst::Const {
                dst,
                value: ConstValue::Integer(1),
            }) => Some(*dst),
            _ => None,
        })
        .expect("missing const 1 in loop_body");

    let call_args = loop_body
        .body
        .iter()
        .find_map(|inst| match inst {
            JoinInst::Call {
                func, args, k_next, ..
            } if *func == k_exit.id && k_next.is_none() => Some(args.clone()),
            _ => None,
        })
        .expect("missing tail call to k_exit from loop_body");

    assert!(!call_args.is_empty(), "k_exit args must include env fields");
    assert_eq!(
        call_args[0], const_one_dst,
        "k_exit must receive updated x value"
    );
    assert_ne!(
        call_args[0], loop_body.params[0],
        "k_exit must not receive the pre-update x param"
    );

    assert!(
        k_exit
            .body
            .iter()
            .any(|inst| matches!(inst, JoinInst::Ret { value: Some(_) })),
        "k_exit must return Some(value)"
    );

    let mir_module = crate::mir::join_ir_vm_bridge::bridge_joinir_to_mir(&module)
        .expect("bridge_joinir_to_mir failed");
    let mir_loop_body_name = join_func_name(loop_body_id);
    let mir_loop_body = mir_module
        .functions
        .values()
        .find(|f| f.signature.name == mir_loop_body_name)
        .expect("missing loop_body in bridged MirModule");
    let entry = mir_loop_body.entry_block;
    let entry_block = mir_loop_body
        .blocks
        .get(&entry)
        .expect("missing loop_body entry block");
    assert!(
        entry_block.return_env().is_some(),
        "loop_body entry block must have return_env metadata in bridged MIR"
    );

    let loop_only_ast = ASTNode::Loop {
        condition: Box::new(ASTNode::Literal {
            value: LiteralValue::Bool(true),
            span: Span::unknown(),
        }),
        body: vec![
            ASTNode::Assignment {
                target: Box::new(ASTNode::Variable {
                    name: "x".to_string(),
                    span: Span::unknown(),
                }),
                value: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            },
            ASTNode::Break {
                span: Span::unknown(),
            },
        ],
        span: Span::unknown(),
    };
    let loop_only_tree = StepTreeBuilderBox::build_from_ast(&loop_only_ast);
    let loop_only_layout = EnvLayout::from_contract(&loop_only_tree.contract, &BTreeMap::new());
    let Some((loop_only_module, _)) =
        LoopTrueBreakOnceBuilderBox::lower(&loop_only_tree, &loop_only_layout)
            .expect("lower failed")
    else {
        panic!("expected loop-only pattern to be in-scope");
    };
    let loop_step_id = JoinFuncId::new(1);
    let loop_step = loop_only_module
        .functions
        .get(&loop_step_id)
        .expect("phase131 test (loop-only): missing loop_step function");
    assert!(
        matches!(loop_step.body.first(), Some(JoinInst::Call { .. })),
        "loop_step must be a tail-call to loop_body (not Jump/Ret)"
    );

    let mut k_exit_call_count = 0usize;
    for f in loop_only_module.functions.values() {
        for inst in &f.body {
            if matches!(inst, JoinInst::Call { func, .. } if *func == k_exit_id) {
                k_exit_call_count += 1;
            }
        }
    }
    assert_eq!(
        k_exit_call_count, 1,
        "loop_only module must have exactly 1 tail-call to k_exit"
    );

    let loop_only_mir = crate::mir::join_ir_vm_bridge::bridge_joinir_to_mir(&loop_only_module)
        .expect("bridge_joinir_to_mir failed (loop-only)");
    const FUNC_NAME_ID_BASE: u32 = 90000;
    let k_exit_func_id = crate::mir::ValueId(FUNC_NAME_ID_BASE + k_exit_id.0);
    let mut args0 = std::collections::BTreeSet::new();
    for f in loop_only_mir.functions.values() {
        for bb in f.blocks.values() {
            for inst in &bb.instructions {
                if let crate::mir::MirInstruction::Call { func, args, .. } = inst {
                    if *func == k_exit_func_id {
                        if let Some(a0) = args.first().copied() {
                            args0.insert(a0);
                        }
                    }
                }
            }
        }
    }
    assert_eq!(
        args0.len(),
        1,
        "loop-only bridged MIR must have a single consistent k_exit arg[0]"
    );
}
