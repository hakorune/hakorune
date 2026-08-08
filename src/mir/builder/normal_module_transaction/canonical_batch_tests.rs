use super::*;
use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use crate::mir::builder::module_draft_collector::FunctionDraftKeyV1;
use crate::mir::compiler::normal_source_plan::{
    with_main_thunk_for_test, VerifiedNormalMainThunkResultV1,
};
use crate::mir::resolved_control_flow::FunctionUnitOriginV1;
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

#[test]
fn main_only_batch_seals_unit_and_scalar_manifests() {
    for (body, expected) in [
        (
            Vec::new(),
            VerifiedNormalMainThunkResultV1::Unit {
                origin: FunctionUnitOriginV1::EmptyBody,
            },
        ),
        (
            vec![return_(LiteralValue::Integer(7))],
            VerifiedNormalMainThunkResultV1::Integer,
        ),
        (
            vec![return_(LiteralValue::Bool(true))],
            VerifiedNormalMainThunkResultV1::Bool,
        ),
        (
            vec![return_(LiteralValue::Float(1.5))],
            VerifiedNormalMainThunkResultV1::Float,
        ),
    ] {
        with_main_thunk_for_test(main_program(body), |thunk| {
            let prepared =
                NormalCanonicalModuleBatchV1::prepare(thunk).expect("canonical Main batch");
            assert_eq!(prepared.thunk().source_result(), expected);
            let rows = prepared.schema().rows();
            assert_eq!(rows.len(), 2);
            assert!(matches!(
                rows[0].role(),
                NormalModuleDraftRoleV1::SourceMain { .. }
            ));
            assert_eq!(rows[0].symbol(), "main/0");
            assert_eq!(rows[0].arity(), 0);
            assert!(matches!(
                rows[1].role(),
                NormalModuleDraftRoleV1::PhysicalEntry
            ));
            assert_eq!(rows[1].symbol(), "main");
            assert_eq!(rows[1].arity(), 0);
        });
    }
}

#[test]
fn batch_projection_uses_thunk_identity_without_raw_policy() {
    with_main_thunk_for_test(main_program(Vec::new()), |thunk| {
        let source_owner = thunk.source_header().owner();
        let prepared = NormalCanonicalModuleBatchV1::prepare(thunk).expect("exact batch");
        assert_eq!(
            prepared.schema().rows()[0].key(),
            &FunctionDraftKeyV1::CanonicalResolvedOwner(source_owner)
        );
        assert_eq!(prepared.schema().rows()[1].key(), &FunctionDraftKeyV1::Main);
    });
}

#[test]
fn malformed_batch_retains_complete_thunk_owner() {
    with_main_thunk_for_test(main_program(Vec::new()), |thunk| {
        let header = thunk.source_header();
        let entry = thunk.entry();
        let source_result = thunk.source_result();
        let malformed = NormalModuleTransactionDraftV1::new(
            vec![NormalModuleDraftExpectationV1::source_main(
                header.owner(),
                header.symbol().as_mir_name(),
                header.arity(),
            )],
            NormalModuleEntryRelationV1::new(
                header.owner(),
                header.symbol().as_mir_name(),
                header.arity(),
                entry.physical_symbol(),
                entry.physical_arity(),
            ),
        );
        let rejected = super::canonical_batch::prepare_draft_for_test(thunk, malformed)
            .expect_err("missing physical entry");
        assert_eq!(
            rejected.error(),
            &NormalCanonicalModuleBatchErrorV1::Schema(
                NormalModuleTransactionSchemaErrorV1::MissingPhysicalEntry
            )
        );
        assert_eq!(rejected.owner().source_result(), source_result);
        rejected.discard();
    });
}
