use super::super::MirBuilder;
use super::*;
use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use crate::mir::compiler::normal_source_plan::{
    with_main_thunk_for_test, VerifiedNormalMainThunkResultV1,
};
use crate::mir::{Callee, MirInstruction, MirType};
use std::collections::HashMap;

fn literal(value: LiteralValue) -> ASTNode {
    ASTNode::Literal {
        value,
        span: Span::unknown(),
    }
}

fn return_(value: LiteralValue) -> ASTNode {
    ASTNode::Return {
        value: Some(Box::new(literal(value))),
        span: Span::unknown(),
    }
}

fn main_program(body: Vec<ASTNode>) -> ASTNode {
    let main = ASTNode::FunctionDeclaration {
        name: "main".to_owned(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body,
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    };
    let mut methods = HashMap::new();
    methods.insert("main".to_owned(), main);
    ASTNode::Program {
        statements: vec![ASTNode::BoxDeclaration {
            name: "Main".to_owned(),
            fields: Vec::new(),
            field_decls: Vec::new(),
            public_fields: Vec::new(),
            private_fields: Vec::new(),
            methods,
            constructors: HashMap::new(),
            init_fields: Vec::new(),
            weak_fields: Vec::new(),
            delegates: Vec::new(),
            invariants: Vec::new(),
            transitions: Vec::new(),
            is_interface: false,
            is_sync: false,
            is_record: false,
            type_parameters: Vec::new(),
            extends: Vec::new(),
            implements: Vec::new(),
            is_static: true,
            static_init: None,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    }
}

#[test]
fn transaction_commits_exact_source_main_and_physical_thunk() {
    for (body, result, result_type) in [
        (
            Vec::new(),
            VerifiedNormalMainThunkResultV1::Unit,
            MirType::Void,
        ),
        (
            vec![return_(LiteralValue::Integer(7))],
            VerifiedNormalMainThunkResultV1::Integer,
            MirType::Integer,
        ),
        (
            vec![return_(LiteralValue::Bool(true))],
            VerifiedNormalMainThunkResultV1::Bool,
            MirType::Bool,
        ),
        (
            vec![return_(LiteralValue::Float(1.5))],
            VerifiedNormalMainThunkResultV1::Float,
            MirType::Float,
        ),
    ] {
        with_main_thunk_for_test(main_program(body), |thunk| {
            let batch = NormalCanonicalModuleBatchV1::prepare(thunk).unwrap();
            let mut builder = MirBuilder::new();
            let completed = builder
                .prepare_normal_main_module_transaction(batch)
                .unwrap()
                .commit();
            assert_eq!(completed.result(), result);
            assert_eq!(completed.verification_count(), 2);
            let module = completed.module();
            assert_eq!(module.functions.len(), 2);
            let source = module.get_function("main/0").unwrap();
            let physical = module.get_function("main").unwrap();
            assert_eq!(source.signature.return_type, result_type);
            assert_eq!(physical.signature.return_type, result_type);
            assert_eq!(physical.signature.params.len(), 0);

            let block = physical.entry_block();
            assert_eq!(block.instructions.len(), 1);
            let MirInstruction::Call {
                dst,
                func,
                callee,
                args,
                ..
            } = &block.instructions[0]
            else {
                panic!("physical entry must contain one exact call")
            };
            assert_eq!(func, &crate::mir::ValueId::INVALID);
            assert_eq!(callee, &Some(Callee::Global("main/0".to_owned())));
            assert!(args.is_empty());
            let MirInstruction::Return { value } =
                block.terminator.as_ref().expect("physical return")
            else {
                panic!("physical entry must end in Return")
            };
            assert_eq!(dst, value);
            assert_eq!(
                dst.is_none(),
                result == VerifiedNormalMainThunkResultV1::Unit
            );
        });
    }
}

#[test]
fn same_builder_can_prepare_successive_normal_main_candidates() {
    let mut builder = MirBuilder::new();
    for body in [
        Vec::new(),
        vec![return_(LiteralValue::Integer(1))],
        vec![return_(LiteralValue::Bool(false))],
    ] {
        with_main_thunk_for_test(main_program(body), |thunk| {
            let batch = NormalCanonicalModuleBatchV1::prepare(thunk).unwrap();
            let completed = builder
                .prepare_normal_main_module_transaction(batch)
                .unwrap()
                .commit();
            assert_eq!(completed.module().functions.len(), 2);
        });
    }
}
