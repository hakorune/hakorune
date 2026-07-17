use crate::ast::ASTNode;
use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1,
    VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::resolved_semantics::{SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1};
use crate::parser::NyashParser;

use super::*;

const QUALIFIED_SOURCE: &str = r#"
static box Helpers {
  run(x) { return x }
  zero() { return 0 }
}

static box Caller {
  invoke(x) { return Helpers.run(x) }
  second() { return Helpers.zero() }
}
"#;

fn parse(source: &str) -> ASTNode {
    NyashParser::parse_from_string(source).expect("qualified target fixture must parse")
}

fn catalog(source: &str) -> VerifiedSameModuleCallableDeclarationCatalogV1 {
    VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&parse(source))
        .expect("declaration catalog must seal")
}

fn key(
    declarations: &VerifiedSameModuleCallableDeclarationCatalogV1,
    owner: &str,
    method: &str,
    arity: usize,
) -> CanonicalSameModuleCallableKeyV1 {
    declarations
        .declaration_for(
            SameModuleCallableNamespaceV1::StaticBoxMethod,
            owner,
            method,
            arity,
        )
        .unwrap_or_else(|| panic!("missing declaration {owner}.{method}/{arity}"))
        .key()
        .clone()
}

fn site(index: u32) -> SourceExprSiteV1 {
    SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
        SourcePathSegmentV1::Body(index),
        SourcePathSegmentV1::Value,
    ]))
}

fn candidate(
    caller: CanonicalSameModuleCallableKeyV1,
    site: SourceExprSiteV1,
    receiver: &str,
    method: &str,
    arity: usize,
) -> QualifiedStaticCallCandidateV1 {
    QualifiedStaticCallCandidateV1::new(
        caller,
        site,
        receiver,
        method,
        arity,
        QualifiedReceiverLexicalFactV1::Unbound,
        ReservedQualifiedReceiverRouteV1::Ordinary,
    )
    .unwrap()
}

fn empty_imports(
    declarations: &VerifiedSameModuleCallableDeclarationCatalogV1,
) -> VerifiedStaticImportAliasViewV1<'_> {
    VerifiedStaticImportAliasViewV1::seal(declarations, []).unwrap()
}

fn qualified<'a>(
    targets: &'a VerifiedSourceStaticCallTargetCatalogV1,
    caller: &CanonicalSameModuleCallableKeyV1,
    site: &SourceExprSiteV1,
) -> &'a VerifiedQualifiedStaticCallTargetV1 {
    match targets.target(caller, site).expect("target row") {
        VerifiedSourceStaticCallTargetV1::QualifiedStatic(row) => row,
        VerifiedSourceStaticCallTargetV1::CurrentOwnerStatic(_) => {
            panic!("expected qualified target row")
        }
    }
}

fn method_call_shape(root: &ASTNode, owner: &str, method: &str) -> (String, String, usize) {
    let ASTNode::Program { statements, .. } = root else {
        panic!("program")
    };
    let function = statements
        .iter()
        .find_map(|statement| match statement {
            ASTNode::BoxDeclaration { name, methods, .. } if name == owner => methods.get(method),
            _ => None,
        })
        .expect("caller method");
    let ASTNode::FunctionDeclaration { body, .. } = function else {
        panic!("function")
    };
    let ASTNode::Return {
        value: Some(value), ..
    } = &body[0]
    else {
        panic!("return value")
    };
    let ASTNode::MethodCall {
        object,
        method,
        arguments,
        ..
    } = value.as_ref()
    else {
        panic!("method call")
    };
    let ASTNode::Variable { name: receiver, .. } = object.as_ref() else {
        panic!("qualified variable receiver")
    };
    (receiver.clone(), method.clone(), arguments.len())
}

#[test]
fn seals_direct_canonical_qualified_targets_by_exact_site() {
    let declarations = catalog(QUALIFIED_SOURCE);
    let imports = empty_imports(&declarations);
    let caller = key(&declarations, "Caller", "invoke", 1);
    let call_site = site(0);
    let targets = VerifiedSourceStaticCallTargetCatalogV1::seal_qualified(
        &declarations,
        &imports,
        [candidate(
            caller.clone(),
            call_site.clone(),
            "Helpers",
            "run",
            1,
        )],
    )
    .unwrap();

    assert_eq!(targets.len(), 1);
    let row = qualified(&targets, &caller, &call_site);
    assert_eq!(row.target().owner(), "Helpers");
    assert_eq!(row.target().name(), "run");
    assert_eq!(row.target().arity(), 1);
    assert_eq!(
        row.receiver(),
        &QualifiedStaticReceiverV1::UnshadowedCanonicalOwner {
            canonical_owner: "Helpers".into(),
        }
    );
}

#[test]
fn imported_alias_precedes_same_spelled_lexical_binding() {
    let declarations = catalog(QUALIFIED_SOURCE);
    let imports = VerifiedStaticImportAliasViewV1::seal(
        &declarations,
        [("Alias".to_string(), "Helpers".to_string())],
    )
    .unwrap();
    let caller = key(&declarations, "Caller", "invoke", 1);
    let call_site = site(0);
    let candidate = QualifiedStaticCallCandidateV1::new(
        caller.clone(),
        call_site.clone(),
        "Alias",
        "run",
        1,
        QualifiedReceiverLexicalFactV1::Bound,
        ReservedQualifiedReceiverRouteV1::Ordinary,
    )
    .unwrap();
    let targets = VerifiedSourceStaticCallTargetCatalogV1::seal_qualified(
        &declarations,
        &imports,
        [candidate],
    )
    .unwrap();

    assert_eq!(imports.len(), 1);
    assert_eq!(
        qualified(&targets, &caller, &call_site).receiver(),
        &QualifiedStaticReceiverV1::ImportedAlias {
            source_alias: "Alias".into(),
            canonical_owner: "Helpers".into(),
        }
    );
}

#[test]
fn rejects_shadowed_direct_receiver_without_import_binding() {
    let declarations = catalog(QUALIFIED_SOURCE);
    let imports = empty_imports(&declarations);
    let caller = key(&declarations, "Caller", "invoke", 1);
    let candidate = QualifiedStaticCallCandidateV1::new(
        caller,
        site(0),
        "Helpers",
        "run",
        1,
        QualifiedReceiverLexicalFactV1::Bound,
        ReservedQualifiedReceiverRouteV1::Ordinary,
    )
    .unwrap();

    assert_eq!(
        VerifiedSourceStaticCallTargetCatalogV1::seal_qualified(
            &declarations,
            &imports,
            [candidate]
        )
        .unwrap_err(),
        QualifiedStaticCallTargetErrorV1::DirectReceiverLexicallyShadowed {
            receiver: "Helpers".into(),
        }
    );
}

#[test]
fn reserved_receiver_routes_fail_before_catalog_lookup() {
    let declarations = catalog(QUALIFIED_SOURCE);
    let imports = empty_imports(&declarations);
    let caller = key(&declarations, "Caller", "invoke", 1);
    for (index, route) in [
        ReservedQualifiedReceiverRouteV1::FastMem,
        ReservedQualifiedReceiverRouteV1::MirIntrinsic,
        ReservedQualifiedReceiverRouteV1::ReplIntrinsic,
    ]
    .into_iter()
    .enumerate()
    {
        let candidate = QualifiedStaticCallCandidateV1::new(
            caller.clone(),
            site(index as u32),
            "Helpers",
            "run",
            1,
            QualifiedReceiverLexicalFactV1::Unbound,
            route,
        )
        .unwrap();
        assert_eq!(
            VerifiedSourceStaticCallTargetCatalogV1::seal_qualified(
                &declarations,
                &imports,
                [candidate]
            )
            .unwrap_err(),
            QualifiedStaticCallTargetErrorV1::ReservedReceiverRoute {
                receiver: "Helpers".into(),
                route,
            }
        );
    }
}

#[test]
fn rejects_wrong_arity_missing_target_and_duplicate_site() {
    let declarations = catalog(QUALIFIED_SOURCE);
    let imports = empty_imports(&declarations);
    let caller = key(&declarations, "Caller", "invoke", 1);
    let wrong = candidate(caller.clone(), site(0), "Helpers", "run", 2);
    assert_eq!(
        VerifiedSourceStaticCallTargetCatalogV1::seal_qualified(&declarations, &imports, [wrong])
            .unwrap_err(),
        QualifiedStaticCallTargetErrorV1::TargetOutsideCatalog {
            receiver: "Helpers".into(),
            canonical_owner: "Helpers".into(),
            method: "run".into(),
            arity: 2,
        }
    );

    let first = candidate(caller.clone(), site(0), "Helpers", "run", 1);
    let duplicate = candidate(caller.clone(), site(0), "Helpers", "zero", 0);
    assert_eq!(
        VerifiedSourceStaticCallTargetCatalogV1::seal_qualified(
            &declarations,
            &imports,
            [first, duplicate]
        )
        .unwrap_err(),
        QualifiedStaticCallTargetErrorV1::DuplicateCallSite {
            caller,
            site: site(0),
        }
    );
}

#[test]
fn rejects_caller_key_from_a_foreign_catalog() {
    let declarations = catalog(QUALIFIED_SOURCE);
    let imports = empty_imports(&declarations);
    let foreign = catalog("static box Foreign { invoke(x) { return x } }");
    let foreign_caller = key(&foreign, "Foreign", "invoke", 1);
    let call = candidate(foreign_caller.clone(), site(0), "Helpers", "run", 1);

    assert_eq!(
        VerifiedSourceStaticCallTargetCatalogV1::seal_qualified(&declarations, &imports, [call])
            .unwrap_err(),
        QualifiedStaticCallTargetErrorV1::CallerOutsideCatalog {
            caller: foreign_caller,
        }
    );
}

#[test]
fn import_view_rejects_duplicate_and_foreign_owners() {
    let declarations = catalog(QUALIFIED_SOURCE);
    assert_eq!(
        VerifiedStaticImportAliasViewV1::seal(
            &declarations,
            [
                ("Alias".to_string(), "Helpers".to_string()),
                ("Alias".to_string(), "Helpers".to_string()),
            ]
        )
        .unwrap_err(),
        StaticImportAliasViewErrorV1::DuplicateAlias {
            alias: "Alias".into(),
        }
    );
    assert_eq!(
        VerifiedStaticImportAliasViewV1::seal(
            &declarations,
            [("Alias".to_string(), "Foreign".to_string())]
        )
        .unwrap_err(),
        StaticImportAliasViewErrorV1::TargetOwnerOutsideCatalog {
            alias: "Alias".into(),
            canonical_owner: "Foreign".into(),
        }
    );
}

#[test]
fn actual_parser_wrapper_projects_import_alias_to_string_helpers() {
    let source = format!(
        "{}\n{}",
        include_str!("../../../lang/src/shared/common/string_helpers.hako"),
        include_str!("../../../lang/src/compiler/parser/scan/parser_string_utils_box.hako"),
    );
    let root = parse(&source);
    let declarations = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root).unwrap();
    let imports = VerifiedStaticImportAliasViewV1::seal(
        &declarations,
        [("StringHelpers".to_string(), "StringHelpers".to_string())],
    )
    .unwrap();
    let caller = key(&declarations, "ParserStringUtilsBox", "skip_ws", 2);
    let (receiver, method, arity) = method_call_shape(&root, "ParserStringUtilsBox", "skip_ws");
    let call_site = site(0);
    let targets = VerifiedSourceStaticCallTargetCatalogV1::seal_qualified(
        &declarations,
        &imports,
        [candidate(
            caller.clone(),
            call_site.clone(),
            &receiver,
            &method,
            arity,
        )],
    )
    .unwrap();

    let row = qualified(&targets, &caller, &call_site);
    assert_eq!(row.target().owner(), "StringHelpers");
    assert_eq!(row.target().name(), "skip_ws");
    assert_eq!(row.target().arity(), 2);
}

#[test]
fn declaration_reorder_preserves_normalized_target_rows() {
    let left = QUALIFIED_SOURCE;
    let right = r#"
static box Caller {
  invoke(x) { return Helpers.run(x) }
  second() { return Helpers.zero() }
}
static box Helpers {
  zero() { return 0 }
  run(x) { return x }
}
"#;

    fn normalized(source: &str) -> Vec<(String, String, u32)> {
        let declarations = catalog(source);
        let imports = empty_imports(&declarations);
        let caller = key(&declarations, "Caller", "invoke", 1);
        VerifiedSourceStaticCallTargetCatalogV1::seal_qualified(
            &declarations,
            &imports,
            [candidate(caller, site(0), "Helpers", "run", 1)],
        )
        .unwrap()
        .rows()
        .map(|(_, row)| match row {
            VerifiedSourceStaticCallTargetV1::QualifiedStatic(row) => (
                row.target().owner().to_string(),
                row.target().name().to_string(),
                row.target().arity(),
            ),
            VerifiedSourceStaticCallTargetV1::CurrentOwnerStatic(_) => {
                panic!("qualified-only fixture gained a current-owner row")
            }
        })
        .collect()
    }

    assert_eq!(normalized(left), normalized(right));
}

#[test]
fn candidate_constructor_rejects_empty_names_and_arity_overflow() {
    let declarations = catalog(QUALIFIED_SOURCE);
    let caller = key(&declarations, "Caller", "invoke", 1);
    assert_eq!(
        QualifiedStaticCallCandidateV1::new(
            caller.clone(),
            site(0),
            "",
            "run",
            1,
            QualifiedReceiverLexicalFactV1::Unbound,
            ReservedQualifiedReceiverRouteV1::Ordinary,
        )
        .unwrap_err(),
        QualifiedStaticCallTargetErrorV1::EmptyReceiver
    );
    assert_eq!(
        QualifiedStaticCallCandidateV1::new(
            caller.clone(),
            site(0),
            "Helpers",
            "",
            1,
            QualifiedReceiverLexicalFactV1::Unbound,
            ReservedQualifiedReceiverRouteV1::Ordinary,
        )
        .unwrap_err(),
        QualifiedStaticCallTargetErrorV1::EmptyMethod {
            receiver: "Helpers".into(),
        }
    );
    if let Ok(overflow_arity) = usize::try_from(u64::from(u32::MAX) + 1) {
        assert_eq!(
            QualifiedStaticCallCandidateV1::new(
                caller,
                site(0),
                "Helpers",
                "run",
                overflow_arity,
                QualifiedReceiverLexicalFactV1::Unbound,
                ReservedQualifiedReceiverRouteV1::Ordinary,
            )
            .unwrap_err(),
            QualifiedStaticCallTargetErrorV1::ArityOverflow {
                receiver: "Helpers".into(),
                method: "run".into(),
            }
        );
    }
}
