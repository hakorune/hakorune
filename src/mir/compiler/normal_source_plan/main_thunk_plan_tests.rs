use super::*;
use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use crate::mir::builder::CanonicalNormalMainEntryTargetV1;
use crate::mir::resolved_control_flow::{FunctionUnitOriginV1, SealedFunctionExitDispositionV1};
use crate::mir::resolved_value_profile::product::{
    TrivialRepresentationV1, TrivialTerminalProfileV1,
};
use std::collections::HashMap;

use super::super::{
    NormalMainFunctionPreflightV1, NormalSourcePlanClassifierV1, PreparedNormalSourcePlanInputV1,
    SealedNormalScalarRootV1, SealedNormalSourcePlanV1,
};

fn literal(value: LiteralValue) -> ASTNode {
    ASTNode::Literal {
        value,
        span: Span::unknown(),
    }
}

fn return_(value: Option<LiteralValue>) -> ASTNode {
    ASTNode::Return {
        value: value.map(|value| Box::new(literal(value))),
        span: Span::unknown(),
    }
}

fn function(result: Option<&str>, body: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "main".to_owned(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: result.map(str::to_owned),
        body,
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn main_program(result: Option<&str>, body: Vec<ASTNode>) -> ASTNode {
    let mut methods = HashMap::new();
    methods.insert("main".to_owned(), function(result, body));
    ASTNode::Program {
        statements: vec![ASTNode::BoxDeclaration {
            name: "Main".to_owned(),
            fields: Vec::new(),
            field_decls: Vec::new(),
            public_fields: Vec::new(),
            private_fields: Vec::new(),
            methods: crate::ast::BoxMethodInventoryV1::from_legacy_ast_map(methods),
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

fn with_plan<R>(
    result: Option<&str>,
    body: Vec<ASTNode>,
    inspect: impl FnOnce(VerifiedNormalMainFunctionPlanV1<'_>) -> R,
) -> R {
    let input =
        PreparedNormalSourcePlanInputV1::new(main_program(result, body), "main-thunk-plan-test");
    let plan = NormalSourcePlanClassifierV1::seal(input).expect("valid Main0");
    let SealedNormalSourcePlanV1::ScalarRoot(SealedNormalScalarRootV1::Main0(main)) = plan else {
        panic!("expected Main0");
    };
    let source = main.prepare_function_source().expect("exact Main source");
    let resolved = source
        .prepare_embedded_resolved_main()
        .expect("embedded Main resolution");
    let plan = NormalMainFunctionPreflightV1::seal(&resolved).expect("Main F1 plan");
    inspect(plan)
}

#[test]
fn thunk_seals_unit_origins_and_scalar_results() {
    for (result, body, expected) in [
        (
            None,
            Vec::new(),
            VerifiedNormalMainThunkResultV1::Unit {
                origin: FunctionUnitOriginV1::EmptyBody,
            },
        ),
        (
            None,
            vec![return_(None)],
            VerifiedNormalMainThunkResultV1::Unit {
                origin: FunctionUnitOriginV1::BareReturn,
            },
        ),
        (
            None,
            vec![return_(Some(LiteralValue::Void))],
            VerifiedNormalMainThunkResultV1::Unit {
                origin: FunctionUnitOriginV1::ExplicitVoid,
            },
        ),
        (
            None,
            vec![return_(Some(LiteralValue::Null))],
            VerifiedNormalMainThunkResultV1::Unit {
                origin: FunctionUnitOriginV1::ExplicitNull,
            },
        ),
        (
            None,
            vec![return_(Some(LiteralValue::Integer(7)))],
            VerifiedNormalMainThunkResultV1::Integer,
        ),
        (
            None,
            vec![return_(Some(LiteralValue::Bool(true)))],
            VerifiedNormalMainThunkResultV1::Bool,
        ),
        (
            None,
            vec![return_(Some(LiteralValue::Float(1.5)))],
            VerifiedNormalMainThunkResultV1::Float,
        ),
        (
            Some("void"),
            vec![return_(Some(LiteralValue::Void))],
            VerifiedNormalMainThunkResultV1::Unit {
                origin: FunctionUnitOriginV1::ExplicitVoid,
            },
        ),
        (
            Some("i64"),
            vec![return_(Some(LiteralValue::Integer(42)))],
            VerifiedNormalMainThunkResultV1::Integer,
        ),
    ] {
        with_plan(result, body, |source| {
            let thunk = VerifiedNormalMainThunkPlanV1::seal(source).expect("exact thunk plan");
            assert_eq!(thunk.source_result(), expected);
            assert_eq!(thunk.source_header().symbol().as_mir_name(), "main/0");
            assert_eq!(thunk.source_header().arity(), 0);
            assert_eq!(thunk.entry().source_owner(), thunk.source_header().owner());
            assert_eq!(thunk.entry().physical_symbol(), "main");
            assert_eq!(thunk.entry().physical_arity(), 0);
            let owner = thunk.source_header().owner();
            let source = thunk.into_source();
            assert_eq!(source.completion().function_exit_contract().owner(), owner);
        });
    }
}

#[test]
fn thunk_rejects_physical_entry_relation_drift_and_retains_source() {
    with_plan(None, Vec::new(), |source| {
        let rejected = prepare_with_physical_for_test(
            source,
            CanonicalNormalMainEntryTargetV1::from_unchecked_parts_for_test("main", 1),
        )
        .expect_err("physical arity drift must reject");
        assert_eq!(
            rejected.error(),
            &NormalMainThunkPlanErrorV1::EntryRelationMismatch
        );
        assert!(!rejected.owner_for_test().completion().returns_value());
        rejected.discard();
    });
}

#[test]
fn thunk_rejects_completion_profile_disagreement() {
    with_plan(None, Vec::new(), |unit| {
        with_plan(
            None,
            vec![return_(Some(LiteralValue::Integer(1)))],
            |value| {
                let error = seal_result(
                    unit.completion().function_exit_contract().disposition(),
                    value.terminal_profile(),
                )
                .expect_err("F1 completion and terminal profile must agree");
                assert_eq!(
                    error,
                    NormalMainThunkPlanErrorV1::CompletionRepresentationMismatch
                );
            },
        );
    });
}

#[test]
fn thunk_rejects_no_value_representation_on_explicit_value_route() {
    with_plan(
        None,
        vec![return_(Some(LiteralValue::Integer(1)))],
        |value| {
            let SealedFunctionExitDispositionV1::ExplicitValue { .. } =
                value.completion().function_exit_contract().disposition()
            else {
                panic!("expected explicit value disposition");
            };
            let TrivialTerminalProfileV1::ExplicitValue {
                statement,
                value: site,
                ..
            } = value.terminal_profile()
            else {
                panic!("expected explicit value profile");
            };
            let unsupported = TrivialTerminalProfileV1::ExplicitValue {
                statement: statement.clone(),
                value: site.clone(),
                representation: TrivialRepresentationV1::ExplicitVoidValue,
            };
            let error = seal_result(
                value.completion().function_exit_contract().disposition(),
                &unsupported,
            )
            .expect_err("no-value representation is not a value carrier");
            assert_eq!(
                error,
                NormalMainThunkPlanErrorV1::UnsupportedResultCarrier {
                    representation: TrivialRepresentationV1::ExplicitVoidValue,
                }
            );
        },
    );
}
