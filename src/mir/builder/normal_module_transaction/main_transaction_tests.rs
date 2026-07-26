use super::super::MirBuilder;
use super::*;
use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use crate::mir::compiler::normal_source_plan::{
    with_main_thunk_for_test, VerifiedNormalMainThunkResultV1,
};
use crate::mir::resolved_control_flow::FunctionUnitOriginV1;
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
    main_program_with_result(None, body)
}

fn main_program_with_result(result: Option<&str>, body: Vec<ASTNode>) -> ASTNode {
    let main = ASTNode::FunctionDeclaration {
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
            VerifiedNormalMainThunkResultV1::Unit {
                origin: FunctionUnitOriginV1::EmptyBody,
            },
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
            let source_owner = thunk.source_header().owner();
            let batch = NormalCanonicalModuleBatchV1::prepare(thunk).unwrap();
            let mut builder = MirBuilder::new();
            let completed = builder
                .prepare_normal_main_module_transaction(batch)
                .unwrap()
                .commit();
            assert_eq!(completed.result(), result);
            assert_eq!(completed.source_owner(), source_owner);
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
                matches!(result, VerifiedNormalMainThunkResultV1::Unit { .. })
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

#[test]
fn transaction_admits_exact_unit_spelling_and_annotation_matrix() {
    let cases = [
        main_program(vec![ASTNode::Return {
            value: None,
            span: Span::unknown(),
        }]),
        main_program(vec![return_(LiteralValue::Void)]),
        main_program(vec![return_(LiteralValue::Null)]),
        main_program_with_result(Some("void"), Vec::new()),
        main_program_with_result(Some("i64"), vec![return_(LiteralValue::Integer(41))]),
    ];
    for program in cases {
        with_main_thunk_for_test(program, |thunk| {
            let result = thunk.source_result();
            let batch = NormalCanonicalModuleBatchV1::prepare(thunk).unwrap();
            let completed = MirBuilder::new()
                .prepare_normal_main_module_transaction(batch)
                .unwrap()
                .commit();
            assert_eq!(completed.result(), result);
            assert_eq!(completed.module().functions.len(), 2);
        });
    }
}

#[test]
fn every_rejection_stage_retains_exact_progress_and_builder_reuse() {
    use super::main_transaction::{
        reject_normal_main_batch_at_stage_for_test, RetainedNormalMainPreparedDraftKindV1,
    };

    let matrix = [
        (
            NormalMainModuleTransactionStageV1::SourceDraft,
            RetainedNormalMainPreparedDraftKindV1::None,
        ),
        (
            NormalMainModuleTransactionStageV1::PhysicalThunk,
            RetainedNormalMainPreparedDraftKindV1::Source,
        ),
        (
            NormalMainModuleTransactionStageV1::BatchCorrespondence,
            RetainedNormalMainPreparedDraftKindV1::SourceAndPhysical,
        ),
        (
            NormalMainModuleTransactionStageV1::CandidateVerification,
            RetainedNormalMainPreparedDraftKindV1::SourceAndPhysical,
        ),
    ];
    let mut builder = MirBuilder::new();
    for (stage, expected_progress) in matrix {
        with_main_thunk_for_test(
            main_program(vec![return_(LiteralValue::Integer(9))]),
            |thunk| {
                let source_owner = thunk.source_header().owner();
                let batch = NormalCanonicalModuleBatchV1::prepare(thunk).unwrap();
                let rejected =
                    reject_normal_main_batch_at_stage_for_test(&mut builder, batch, stage);
                assert_eq!(rejected.stage(), stage);
                assert_eq!(rejected.retained_source_owner(), source_owner);
                assert_eq!(rejected.prepared_kind(), expected_progress);
                assert!(rejected.has_restoration_receipt());
                match (stage, rejected.error()) {
                    (
                        NormalMainModuleTransactionStageV1::SourceDraft,
                        NormalMainModuleTransactionErrorV1::SourceLowering(_),
                    )
                    | (
                        NormalMainModuleTransactionStageV1::PhysicalThunk,
                        NormalMainModuleTransactionErrorV1::PhysicalThunk(_),
                    )
                    | (
                        NormalMainModuleTransactionStageV1::BatchCorrespondence,
                        NormalMainModuleTransactionErrorV1::BatchCorrespondence(_),
                    )
                    | (
                        NormalMainModuleTransactionStageV1::CandidateVerification,
                        NormalMainModuleTransactionErrorV1::CandidateVerification(_),
                    ) => {}
                    relation => panic!("stage/error drift: {relation:?}"),
                }
                rejected.discard();
            },
        );
        with_main_thunk_for_test(main_program(Vec::new()), |thunk| {
            let batch = NormalCanonicalModuleBatchV1::prepare(thunk).unwrap();
            let completed = builder
                .prepare_normal_main_module_transaction(batch)
                .unwrap()
                .commit();
            assert_eq!(completed.module().functions.len(), 2);
        });
    }
}
