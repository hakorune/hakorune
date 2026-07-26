use std::collections::HashMap;

use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, ParamDecl, Span};
use crate::mir::builder::resolved_lowering::NormalFunctionDraftLoweringStageV1;
use crate::mir::compiler::normal_source_plan::{
    CompletedNormalMainHelperResolutionV1, NormalMainDirectCallPreflightV1,
    NormalSourcePlanClassifierV1, PreparedNormalSourcePlanInputV1, SealedNormalSourcePlanV1,
};
use crate::mir::MirBuilder;

use super::*;

fn literal(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn helper(name: &str) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.to_owned(),
        params: vec!["n".to_owned()],
        param_decls: vec![ParamDecl {
            name: "n".to_owned(),
            declared_type_name: Some("i64".to_owned()),
        }],
        return_type_name: Some("i64".to_owned()),
        body: vec![ASTNode::Return {
            value: Some(Box::new(ASTNode::Variable {
                name: "n".to_owned(),
                span: Span::unknown(),
            })),
            span: Span::unknown(),
        }],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn main_box() -> ASTNode {
    let mut methods = HashMap::new();
    methods.insert(
        "main".to_owned(),
        ASTNode::FunctionDeclaration {
            name: "main".to_owned(),
            params: Vec::new(),
            param_decls: Vec::new(),
            return_type_name: None,
            body: Vec::new(),
            uses: Vec::new(),
            contracts: Vec::new(),
            is_static: true,
            is_override: false,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        },
    );
    ASTNode::BoxDeclaration {
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
    }
}

pub(crate) fn completed_for_main_physical(names: &[&str]) -> CompletedNormalMainHelperResolutionV1 {
    let mut statements = vec![main_box()];
    statements.extend(names.iter().map(|name| helper(name)));
    let plan = NormalSourcePlanClassifierV1::seal(PreparedNormalSourcePlanInputV1::new(
        ASTNode::Program {
            statements,
            span: Span::unknown(),
        },
        "normal-helper-prefix-test",
    ))
    .unwrap();
    let SealedNormalSourcePlanV1::CallableModule(source) = plan else {
        panic!("fixture must classify as a callable module")
    };
    let source = source
        .prepare_callable_source()
        .unwrap()
        .prepare_helper_catalog(13)
        .unwrap()
        .prepare_main_with_helper_catalog()
        .unwrap();
    NormalMainDirectCallPreflightV1::seal(source)
        .unwrap()
        .prepare_helper_resolution()
        .resolve()
        .unwrap()
}

#[test]
fn prefix_lowers_helpers_once_in_canonical_key_order() {
    let mut builder = MirBuilder::new();
    let prepared = builder
        .prepare_normal_helper_draft_prefix_v1(
            completed_for_main_physical(&["beta", "alpha"]).into_tx0_handoff(),
        )
        .unwrap();
    let prefix = prepared.prefix();
    let keys: Vec<_> = prefix
        .drafts()
        .iter()
        .map(|draft| draft.key().name())
        .collect();
    assert_eq!(keys, ["alpha", "beta"]);
    for draft in prefix.drafts() {
        assert_eq!(
            draft.draft().signature.name,
            format!("{}/{}", draft.key().name(), draft.key().arity())
        );
        assert_eq!(
            draft.draft().signature.params.len(),
            draft.key().arity() as usize
        );
    }
    let (transaction, _) = prepared.into_parts();
    assert!(transaction.has_main_lowering_proof());
    assert!(builder.function_state.current_function.is_none());
    assert!(builder.function_state.current_block.is_none());
}

#[test]
fn injected_middle_failure_retains_exact_prefix_and_allows_later_success() {
    let mut builder = MirBuilder::new();
    let transaction = completed_for_main_physical(&["beta", "alpha", "gamma"]).into_tx0_handoff();
    let (transaction, outcome) = transaction
        .with_helper_plans(|source, schedule| {
            let mut ordinal = 0;
            lower_helper_schedule_with_v1(source, schedule, |plan| {
                let current = ordinal;
                ordinal += 1;
                if current == 1 {
                    drop(plan);
                    return Err(NormalFunctionDraftLoweringStageV1::BodyLowering);
                }
                builder
                    .lower_resolved_trivial_function_draft_retaining_failure_v1(plan)
                    .map_err(discard_rejected_lowering)
            })
        })
        .unwrap();
    let (prefix, failure) = outcome.unwrap_err();
    assert_eq!(
        prefix
            .drafts()
            .iter()
            .map(|draft| draft.key().name())
            .collect::<Vec<_>>(),
        ["alpha"]
    );
    assert!(matches!(
        failure,
        NormalHelperDraftPrefixFailureV1::Lowering(receipt)
            if receipt.key().name() == "beta"
                && receipt.ordinal() == 1
                && receipt.stage() == NormalFunctionDraftLoweringStageV1::BodyLowering
    ));
    assert!(transaction.has_main_lowering_proof());
    assert!(builder.function_state.current_function.is_none());
    assert!(builder.function_state.current_block.is_none());

    let later = builder
        .prepare_normal_helper_draft_prefix_v1(
            completed_for_main_physical(&["later"]).into_tx0_handoff(),
        )
        .unwrap();
    assert_eq!(later.prefix().drafts()[0].key().name(), "later");
}

#[test]
fn catalog_correspondence_rejects_before_a_draft_enters_the_prefix() {
    let mut builder = MirBuilder::new();
    let transaction = completed_for_main_physical(&["alpha"]).into_tx0_handoff();
    let (_transaction, outcome) = transaction
        .with_helper_plans(|source, schedule| {
            lower_helper_schedule_with_v1(source, schedule, |plan| {
                let mut draft = builder
                    .lower_resolved_trivial_function_draft_retaining_failure_v1(plan)
                    .map_err(discard_rejected_lowering)?;
                draft.signature.name = "wrong/1".to_owned();
                Ok(draft)
            })
        })
        .unwrap();
    let (prefix, failure) = outcome.unwrap_err();
    assert!(prefix.drafts().is_empty());
    assert!(matches!(
        failure,
        NormalHelperDraftPrefixFailureV1::Correspondence(
            NormalHelperDraftCorrespondenceErrorV1::SymbolMismatch { .. }
        )
    ));
}
