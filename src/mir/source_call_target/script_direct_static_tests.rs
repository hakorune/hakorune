use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::builder::VerifiedSameModuleCallableDeclarationCatalogV1;
use crate::mir::resolved_semantics::{
    ScriptRootReturnExitAdmissionV1, ScriptRootRuntimeDispositionV1,
    ScriptRootSemanticDispositionV1, ScriptTransferredBoundaryV1, SourcePathSegmentV1,
    SourcePathV1, VerifiedScriptRootDemandEntryV1, VerifiedScriptRootDemandWindowV1,
};
use crate::parser::NyashParser;

use super::{
    ScriptDirectStaticCallTargetErrorV1, VerifiedScriptDirectStaticCallTargetInventoryV1,
    VerifiedStaticImportAliasViewV1,
};

fn return_call(receiver: &str, method: &str, arity: usize) -> ASTNode {
    ASTNode::Return {
        value: Some(Box::new(ASTNode::MethodCall {
            object: Box::new(ASTNode::Variable {
                name: receiver.to_owned(),
                span: Span::unknown(),
            }),
            method: method.to_owned(),
            arguments: (0..arity)
                .map(|_| ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                })
                .collect(),
            span: Span::unknown(),
        })),
        span: Span::unknown(),
    }
}

fn root_with_statements(statements: Vec<ASTNode>) -> ASTNode {
    let mut root = NyashParser::parse_from_string(
        "static box Helpers { run(x) { return x } }",
    )
    .expect("static declaration fixture");
    let ASTNode::Program {
        statements: existing,
        ..
    } = &mut root
    else {
        panic!("parser returned Program");
    };
    existing.extend(statements);
    root
}

fn window_for(root: &ASTNode) -> VerifiedScriptRootDemandWindowV1 {
    let ASTNode::Program { statements, .. } = root else {
        panic!("root must be Program");
    };
    let entries = statements
        .iter()
        .enumerate()
        .map(|(index, _)| {
            let site = SourcePathV1::program_body()
                .child(SourcePathSegmentV1::ProgramBody(index as u32))
                .stmt();
            let semantic = if index == 0 {
                ScriptRootSemanticDispositionV1::Transferred(
                    ScriptTransferredBoundaryV1::ProgramStaticMetadata,
                )
            } else {
                ScriptRootSemanticDispositionV1::Resolved(
                    crate::mir::resolved_semantics::ScriptRootResolvedDemandV1::ReturnExit(
                        ScriptRootReturnExitAdmissionV1::new(),
                    ),
                )
            };
            VerifiedScriptRootDemandEntryV1::new(
                site,
                semantic,
                ScriptRootRuntimeDispositionV1::None,
            )
        })
        .collect();
    VerifiedScriptRootDemandWindowV1::seal(entries, statements.len())
        .expect("complete Script window")
}

fn issue(
    root: &ASTNode,
    aliases: impl IntoIterator<Item = (String, String)>,
) -> Result<VerifiedScriptDirectStaticCallTargetInventoryV1, ScriptDirectStaticCallTargetErrorV1>
{
    let declarations = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(root)
        .expect("declaration catalog");
    let imports = VerifiedStaticImportAliasViewV1::seal(&declarations, aliases)
        .expect("alias view");
    VerifiedScriptDirectStaticCallTargetInventoryV1::issue(
        root,
        &window_for(root),
        &declarations,
        &imports,
    )
}

#[test]
fn exact_program_site_and_imported_target_are_retained() {
    let root = root_with_statements(vec![return_call("Alias", "run", 1)]);
    let inventory = issue(&root, [("Alias".to_owned(), "Helpers".to_owned())]).unwrap();
    assert_eq!(inventory.observed_len(), 1);
    assert_eq!(inventory.target_len(), 1);
    assert_eq!(inventory.noncandidate_len(), 0);
    let site = SourcePathV1::program_body()
        .child(SourcePathSegmentV1::ProgramBody(1))
        .child(SourcePathSegmentV1::Value)
        .expr();
    assert_eq!(inventory.target(&site).unwrap().target().owner(), "Helpers");
    assert_eq!(inventory.site(&site).unwrap().argument_sites().len(), 1);
}

#[test]
fn zero_and_duplicate_sites_are_complete_and_distinct() {
    let empty = root_with_statements(Vec::new());
    let empty_inventory = issue(&empty, []).unwrap();
    assert_eq!(empty_inventory.observed_len(), 0);
    assert_eq!(empty_inventory.target_len(), 0);
    assert_eq!(empty_inventory.noncandidate_len(), 0);

    let two = root_with_statements(vec![
        return_call("Helpers", "run", 1),
        return_call("Helpers", "run", 1),
    ]);
    let two_inventory = issue(&two, []).unwrap();
    assert_eq!(two_inventory.observed_len(), 2);
    assert_eq!(two_inventory.target_len(), 2);
    let first = SourcePathV1::program_body()
        .child(SourcePathSegmentV1::ProgramBody(1))
        .child(SourcePathSegmentV1::Value)
        .expr();
    let second = SourcePathV1::program_body()
        .child(SourcePathSegmentV1::ProgramBody(2))
        .child(SourcePathSegmentV1::Value)
        .expr();
    assert_ne!(first, second);
    assert!(two_inventory.target(&first).is_some());
    assert!(two_inventory.target(&second).is_some());
}

#[test]
fn arity_mismatch_rejects_before_catalog_publication() {
    let root = root_with_statements(vec![return_call("Helpers", "run", 2)]);
    assert!(matches!(
        issue(&root, []),
        Err(ScriptDirectStaticCallTargetErrorV1::TargetOutsideCatalog { arity: 2, .. })
    ));
}

#[test]
fn bound_and_dynamic_receivers_are_explicit_noncandidates() {
    let root = root_with_statements(vec![
        ASTNode::Local {
            variables: vec!["Alias".to_owned()],
            initial_values: vec![Some(Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(0),
                span: Span::unknown(),
            }))],
            declared_type_names: Vec::new(),
            span: Span::unknown(),
        },
        return_call("Alias", "run", 1),
        ASTNode::Return {
            value: Some(Box::new(ASTNode::MethodCall {
                object: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                }),
                method: "run".to_owned(),
                arguments: vec![],
                span: Span::unknown(),
            })),
            span: Span::unknown(),
        },
    ]);
    let inventory = issue(&root, [("Alias".to_owned(), "Helpers".to_owned())]).unwrap();
    assert_eq!(inventory.observed_len(), 2);
    assert_eq!(inventory.target_len(), 0);
    assert_eq!(inventory.noncandidate_len(), 2);
}

#[test]
fn unknown_qualified_target_rejects_before_catalog_publication() {
    let root = root_with_statements(vec![return_call("Missing", "run", 1)]);
    assert!(matches!(
        issue(&root, []),
        Err(ScriptDirectStaticCallTargetErrorV1::TargetOutsideCatalog { .. })
    ));
}

#[test]
fn nested_lambda_is_not_reclassified_as_a_script_target() {
    let root = root_with_statements(vec![ASTNode::Return {
        value: Some(Box::new(ASTNode::Lambda {
            params: Vec::new(),
            body: vec![return_call("Helpers", "run", 1)],
            span: Span::unknown(),
        })),
        span: Span::unknown(),
    }]);
    assert!(matches!(
        issue(&root, []),
        Err(ScriptDirectStaticCallTargetErrorV1::MethodObservation(_))
    ));
}
