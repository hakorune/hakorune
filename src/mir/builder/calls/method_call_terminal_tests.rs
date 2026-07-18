use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::builder::callable_declaration_catalog::VerifiedSameModuleCallableDeclarationCatalogV1;
use crate::mir::builder::recursive_child_lowering::RawLegacyChildLoweringPortV1;
use crate::mir::{Callee, ConstValue, EffectMask, MirBuilder, MirInstruction, MirType};
use crate::parser::NyashParser;

use super::extern_calls::EnvMethodSpec;
use super::method_call_descent::RawLegacyMethodCallInputV1;
use super::method_call_terminal::MethodCallValueTerminalPortV1;

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn input() -> RawLegacyMethodCallInputV1 {
    RawLegacyMethodCallInputV1::new(integer(0), "terminal".to_string(), Vec::new())
}

fn builder(name: &str) -> MirBuilder {
    let mut builder = MirBuilder::new();
    let root = NyashParser::parse_from_string(
        "static box TerminalCatalogOwner { call(left, right) { return left + right } }",
    )
    .unwrap();
    let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root).unwrap();
    builder
        .comp_ctx
        .install_callable_declaration_catalog(catalog)
        .unwrap();
    builder.enter_function_for_test(name.to_string());
    builder
}

fn instructions(builder: &MirBuilder) -> Vec<MirInstruction> {
    builder
        .scope_ctx
        .current_function
        .as_ref()
        .unwrap()
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter().cloned())
        .collect()
}

fn ordinary_copy_root(
    instructions: &[MirInstruction],
    mut value: crate::mir::ValueId,
) -> crate::mir::ValueId {
    let mut remaining = instructions.len();
    while remaining > 0 {
        let Some(src) = instructions
            .iter()
            .find_map(|instruction| match instruction {
                MirInstruction::Copy { dst, src } if *dst == value => Some(*src),
                _ => None,
            })
        else {
            break;
        };
        value = src;
        remaining -= 1;
    }
    value
}

fn normalized_const_value(
    instructions: &[MirInstruction],
    value: crate::mir::ValueId,
) -> Option<ConstValue> {
    let root = ordinary_copy_root(instructions, value);
    instructions
        .iter()
        .find_map(|instruction| match instruction {
            MirInstruction::Const { dst, value } if *dst == root => Some(value.clone()),
            _ => None,
        })
}

#[test]
fn disconnected_typeop_terminals_preserve_check_cast_value_type_and_destination() {
    let mut builder = builder("terminal_typeop/0");
    let value = builder.build_expression(integer(7)).unwrap();
    let mut port = RawLegacyChildLoweringPortV1;

    let check = port
        .emit_typeop_value_terminal(
            &mut builder,
            &input(),
            value,
            crate::mir::TypeOpKind::Check,
            MirType::Integer,
        )
        .unwrap();
    let cast = port
        .emit_typeop_value_terminal(
            &mut builder,
            &input(),
            value,
            crate::mir::TypeOpKind::Cast,
            MirType::Integer,
        )
        .unwrap();

    assert!(instructions(&builder).iter().any(|instruction| matches!(
        instruction,
        MirInstruction::TypeOp {
            dst,
            op: crate::mir::TypeOpKind::Check,
            value: actual,
            ty: MirType::Integer,
        } if *dst == check && *actual == value
    )));
    assert!(instructions(&builder).iter().any(|instruction| matches!(
        instruction,
        MirInstruction::TypeOp {
            dst,
            op: crate::mir::TypeOpKind::Cast,
            value: actual,
            ty: MirType::Integer,
        } if *dst == cast && *actual == value
    )));
}

#[test]
fn disconnected_static_and_me_global_terminals_preserve_semantic_target_and_arguments() {
    let mut builder = builder("terminal_global/0");
    let left = builder.build_expression(integer(3)).unwrap();
    let right = builder.build_expression(integer(4)).unwrap();
    let mut port = RawLegacyChildLoweringPortV1;

    let static_result = port
        .emit_static_global_value_terminal(
            &mut builder,
            &input(),
            "TerminalCatalogOwner",
            "call",
            2,
            vec![left, right],
        )
        .unwrap();
    let me_result = port
        .emit_me_lowered_global_value_terminal(
            &mut builder,
            &input(),
            "TerminalCatalogOwner",
            "call",
            2,
            vec![left, right],
        )
        .unwrap();

    let emitted = instructions(&builder);
    let calls = emitted
        .iter()
        .filter_map(|instruction| match instruction {
            MirInstruction::Call {
                dst,
                callee: Some(Callee::Global(name)),
                args,
                ..
            } if name == "TerminalCatalogOwner.call/2" => Some((*dst, args.clone())),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(calls.len(), 2);
    assert_eq!(calls[0].0, Some(static_result));
    assert_eq!(calls[1].0, Some(me_result));
    assert_eq!(calls[0].1.len(), 2);
    assert_eq!(calls[1].1.len(), 2);
    assert_eq!(
        normalized_const_value(&emitted, calls[0].1[0]),
        normalized_const_value(&emitted, left),
    );
    assert_eq!(
        normalized_const_value(&emitted, calls[0].1[1]),
        normalized_const_value(&emitted, right),
    );
    assert_eq!(
        normalized_const_value(&emitted, calls[1].1[0]),
        normalized_const_value(&emitted, left),
    );
    assert_eq!(
        normalized_const_value(&emitted, calls[1].1[1]),
        normalized_const_value(&emitted, right),
    );
}

#[test]
fn disconnected_env_terminals_preserve_returning_and_no_result_laws() {
    let mut builder = builder("terminal_env/0");
    let argument = builder.build_expression(integer(5)).unwrap();
    let mut port = RawLegacyChildLoweringPortV1;
    let returning = EnvMethodSpec {
        iface_name: "env.fs".to_string(),
        method_name: "exists".to_string(),
        effects: EffectMask::READ,
        returns: true,
    };
    let no_result = EnvMethodSpec {
        iface_name: "env.console".to_string(),
        method_name: "log".to_string(),
        effects: EffectMask::IO,
        returns: false,
    };

    let returning_value = port
        .emit_env_value_terminal(&mut builder, &input(), &returning, vec![argument])
        .unwrap();
    let void_value = port
        .emit_env_value_terminal(&mut builder, &input(), &no_result, vec![argument])
        .unwrap();

    let emitted = instructions(&builder);
    assert!(emitted.iter().any(|instruction| matches!(
        instruction,
        MirInstruction::Call {
            dst: Some(dst),
            callee: Some(Callee::Extern(name)),
            effects: EffectMask::READ,
            ..
        } if *dst == returning_value && name == "env.fs.exists"
    )));
    assert!(emitted.iter().any(|instruction| matches!(
        instruction,
        MirInstruction::Call {
            dst: None,
            callee: Some(Callee::Extern(name)),
            effects,
            ..
        } if name == "env.console.log" && *effects == no_result.effects
    )));
    assert!(emitted.iter().any(|instruction| matches!(
        instruction,
        MirInstruction::Const {
            dst,
            value: ConstValue::Void,
        } if *dst == void_value
    )));
    let env_calls = emitted
        .iter()
        .filter_map(|instruction| match instruction {
            MirInstruction::Call {
                callee: Some(Callee::Extern(_)),
                args,
                ..
            } => Some(args),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(env_calls.len(), 2);
    for args in env_calls {
        assert_eq!(args.len(), 1);
        assert_eq!(
            normalized_const_value(&emitted, args[0]),
            normalized_const_value(&emitted, argument),
        );
    }
}

#[test]
fn disconnected_standard_terminal_preserves_method_identity_and_completed_destination() {
    let mut builder = builder("terminal_standard/0");
    let receiver = builder.build_expression(integer(8)).unwrap();
    let argument = builder.build_expression(integer(9)).unwrap();
    let mut port = RawLegacyChildLoweringPortV1;

    let result = port
        .emit_standard_value_terminal(
            &mut builder,
            &input(),
            receiver,
            "terminalMethod".to_string(),
            vec![argument],
        )
        .unwrap();

    let emitted = instructions(&builder);
    let (actual_receiver, actual_arguments) = emitted
        .iter()
        .find_map(|instruction| match instruction {
            MirInstruction::Call {
                dst: Some(dst),
                callee:
                    Some(Callee::Method {
                        method,
                        receiver: Some(actual_receiver),
                        ..
                    }),
                args,
                ..
            } if *dst == result && method == "terminalMethod" => {
                Some((*actual_receiver, args.as_slice()))
            }
            _ => None,
        })
        .expect("standard terminal must emit one method call");
    assert_eq!(
        normalized_const_value(&emitted, actual_receiver),
        normalized_const_value(&emitted, receiver),
    );
    assert_eq!(actual_arguments.len(), 1);
    assert_eq!(
        normalized_const_value(&emitted, actual_arguments[0]),
        normalized_const_value(&emitted, argument),
    );
}
