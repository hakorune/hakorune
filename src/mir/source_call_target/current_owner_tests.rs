use crate::ast::ASTNode;
use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1,
    VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::resolved_semantics::{SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1};
use crate::parser::NyashParser;

use super::current_owner::checked_explicit_arity;
use super::*;

const CURRENT_OWNER_SOURCE: &str = r#"
static box Helpers {
  call(x) { return me.target(x) }
  target(x) { return x }
  zero() { return me.zero_target() }
  zero_target() { return 0 }
}

static box Foreign {
  target(x) { return x }
}
"#;

fn parse(source: &str) -> ASTNode {
    NyashParser::parse_from_string(source).expect("current-owner fixture must parse")
}

fn catalog(root: &ASTNode) -> VerifiedSameModuleCallableDeclarationCatalogV1 {
    VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(root)
        .expect("declaration catalog must seal")
}

fn key(
    declarations: &VerifiedSameModuleCallableDeclarationCatalogV1,
    namespace: SameModuleCallableNamespaceV1,
    owner: &str,
    method: &str,
    arity: usize,
) -> CanonicalSameModuleCallableKeyV1 {
    declarations
        .declaration_for(namespace, owner, method, arity)
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

fn expression(
    declarations: &VerifiedSameModuleCallableDeclarationCatalogV1,
    owner: &str,
    method: &str,
    arity: usize,
) -> ASTNode {
    let declaration = declarations
        .declaration_for(
            SameModuleCallableNamespaceV1::StaticBoxMethod,
            owner,
            method,
            arity,
        )
        .expect("caller declaration");
    let [ASTNode::Return {
        value: Some(value), ..
    }] = declaration.body()
    else {
        panic!("caller must be one value return")
    };
    value.as_ref().clone()
}

fn empty_targets(
    declarations: &VerifiedSameModuleCallableDeclarationCatalogV1,
) -> VerifiedSourceStaticCallTargetCatalogV1 {
    let imports = VerifiedStaticImportAliasViewV1::seal(declarations, []).unwrap();
    VerifiedSourceStaticCallTargetCatalogV1::seal_qualified(declarations, &imports, []).unwrap()
}

fn candidate(
    caller: CanonicalSameModuleCallableKeyV1,
    call_site: SourceExprSiteV1,
    expression: &ASTNode,
) -> CurrentOwnerStaticCallCandidateV1 {
    CurrentOwnerStaticCallCandidateV1::from_method_call(caller, call_site, expression).unwrap()
}

fn current_owner<'a>(
    targets: &'a VerifiedSourceStaticCallTargetCatalogV1,
    caller: &CanonicalSameModuleCallableKeyV1,
    call_site: &SourceExprSiteV1,
) -> &'a VerifiedCurrentOwnerStaticCallTargetV1 {
    match targets.target(caller, call_site).expect("target row") {
        VerifiedSourceStaticCallTargetV1::CurrentOwnerStatic(row) => row,
        VerifiedSourceStaticCallTargetV1::QualifiedStatic(_) => {
            panic!("expected current-owner target row")
        }
    }
}

#[test]
fn seals_current_owner_static_targets_with_exact_arity() {
    let root = parse(CURRENT_OWNER_SOURCE);
    let declarations = catalog(&root);
    let caller = key(
        &declarations,
        SameModuleCallableNamespaceV1::StaticBoxMethod,
        "Helpers",
        "call",
        1,
    );
    let call_site = site(0);
    let targets = empty_targets(&declarations)
        .extend_current_owner(
            &declarations,
            [candidate(
                caller.clone(),
                call_site.clone(),
                &expression(&declarations, "Helpers", "call", 1),
            )],
        )
        .unwrap();

    let row = current_owner(&targets, &caller, &call_site);
    assert_eq!(row.receiver(), CurrentOwnerStaticReceiverV1::CanonicalMe);
    assert_eq!(row.target().owner(), "Helpers");
    assert_eq!(row.target().name(), "target");
    assert_eq!(row.target().arity(), 1);
}

#[test]
fn zero_arity_and_qualified_rows_share_one_catalog() {
    let root = parse(CURRENT_OWNER_SOURCE);
    let declarations = catalog(&root);
    let imports = VerifiedStaticImportAliasViewV1::seal(&declarations, []).unwrap();
    let qualified_caller = key(
        &declarations,
        SameModuleCallableNamespaceV1::StaticBoxMethod,
        "Helpers",
        "call",
        1,
    );
    let qualified = QualifiedStaticCallCandidateV1::new(
        qualified_caller.clone(),
        site(7),
        "Foreign",
        "target",
        1,
        QualifiedReceiverLexicalFactV1::Unbound,
        ReservedQualifiedReceiverRouteV1::Ordinary,
    )
    .unwrap();
    let caller = key(
        &declarations,
        SameModuleCallableNamespaceV1::StaticBoxMethod,
        "Helpers",
        "zero",
        0,
    );
    let call_site = site(1);
    let targets = VerifiedSourceStaticCallTargetCatalogV1::seal_qualified(
        &declarations,
        &imports,
        [qualified],
    )
    .unwrap()
    .extend_current_owner(
        &declarations,
        [candidate(
            caller.clone(),
            call_site.clone(),
            &expression(&declarations, "Helpers", "zero", 0),
        )],
    )
    .unwrap();

    assert_eq!(targets.len(), 2);
    assert_eq!(
        current_owner(&targets, &caller, &call_site).target().name(),
        "zero_target"
    );
}

#[test]
fn actual_string_helpers_projects_digit_value_to_caller_owner() {
    let root = parse(include_str!(concat!(
        "../../../lang/src/shared/common/",
        "string_helpers.hako"
    )));
    let declarations = catalog(&root);
    let caller = key(
        &declarations,
        SameModuleCallableNamespaceV1::StaticBoxMethod,
        "StringHelpers",
        "to_i64",
        1,
    );
    let declaration = declarations.declaration(&caller).unwrap();
    let call = declaration
        .body()
        .iter()
        .find_map(|statement| match statement {
            ASTNode::Loop { body, .. } => body.iter().find_map(|nested| match nested {
                ASTNode::Local {
                    variables,
                    initial_values,
                    ..
                } if variables == &["dpos"] => initial_values.first()?.as_deref(),
                _ => None,
            }),
            _ => None,
        })
        .expect("to_i64 must retain dpos current-owner call");
    let call_site = site(12);
    let targets = empty_targets(&declarations)
        .extend_current_owner(
            &declarations,
            [candidate(caller.clone(), call_site.clone(), call)],
        )
        .unwrap();

    let row = current_owner(&targets, &caller, &call_site);
    assert_eq!(row.target().owner(), "StringHelpers");
    assert_eq!(row.target().name(), "_digit_value");
    assert_eq!(row.target().arity(), 1);
}

#[test]
fn rejects_non_me_source_shapes() {
    let root = parse(CURRENT_OWNER_SOURCE);
    let declarations = catalog(&root);
    let caller = key(
        &declarations,
        SameModuleCallableNamespaceV1::StaticBoxMethod,
        "Helpers",
        "call",
        1,
    );
    let not_call = parse("static box X { f() { return 0 } }");
    assert_eq!(
        CurrentOwnerStaticCallCandidateV1::from_method_call(caller.clone(), site(0), &not_call)
            .unwrap_err(),
        CurrentOwnerStaticCallTargetErrorV1::SourceMethodCallRequired
    );
    let qualified = parse("static box X { f(x) { return Other.target(x) } }");
    let qualified_catalog = catalog(&qualified);
    let qualified_expr = expression(&qualified_catalog, "X", "f", 1);
    assert_eq!(
        CurrentOwnerStaticCallCandidateV1::from_method_call(caller, site(0), &qualified_expr)
            .unwrap_err(),
        CurrentOwnerStaticCallTargetErrorV1::CanonicalMeReceiverRequired
    );
}

#[test]
fn rejects_foreign_instance_duplicate_and_missing_targets() {
    let root = parse(CURRENT_OWNER_SOURCE);
    let declarations = catalog(&root);
    let caller = key(
        &declarations,
        SameModuleCallableNamespaceV1::StaticBoxMethod,
        "Helpers",
        "call",
        1,
    );
    let call_expr = expression(&declarations, "Helpers", "call", 1);
    let foreign_root =
        parse("static box Missing { call(x) { return me.target(x) } target(x) { return x } }");
    let foreign_declarations = catalog(&foreign_root);
    let foreign = key(
        &foreign_declarations,
        SameModuleCallableNamespaceV1::StaticBoxMethod,
        "Missing",
        "call",
        1,
    );
    assert_eq!(
        empty_targets(&declarations)
            .extend_current_owner(
                &declarations,
                [candidate(foreign.clone(), site(0), &call_expr)]
            )
            .unwrap_err(),
        CurrentOwnerStaticCallTargetErrorV1::CallerOutsideCatalog { caller: foreign }
    );

    let instance_root =
        parse("box Ordinary { call(x) { return me.target(x) } target(x) { return x } }");
    let instance_declarations = catalog(&instance_root);
    let instance_caller = key(
        &instance_declarations,
        SameModuleCallableNamespaceV1::InstanceBoxMethod,
        "Ordinary",
        "call",
        1,
    );
    let ASTNode::Program { statements, .. } = &instance_root else {
        unreachable!()
    };
    let ASTNode::BoxDeclaration { methods, .. } = &statements[0] else {
        unreachable!()
    };
    let ASTNode::FunctionDeclaration { body, .. } = methods.get("call").unwrap() else {
        unreachable!()
    };
    let ASTNode::Return {
        value: Some(instance_expr),
        ..
    } = &body[0]
    else {
        unreachable!()
    };
    assert_eq!(
        empty_targets(&instance_declarations)
            .extend_current_owner(
                &instance_declarations,
                [candidate(instance_caller.clone(), site(0), instance_expr)]
            )
            .unwrap_err(),
        CurrentOwnerStaticCallTargetErrorV1::CallerNotStaticBoxMethod {
            caller: instance_caller
        }
    );

    let duplicate_site = site(0);
    let duplicate = candidate(caller.clone(), duplicate_site.clone(), &call_expr);
    let qualified_imports = VerifiedStaticImportAliasViewV1::seal(&declarations, []).unwrap();
    let qualified = QualifiedStaticCallCandidateV1::new(
        caller.clone(),
        duplicate_site.clone(),
        "Foreign",
        "target",
        1,
        QualifiedReceiverLexicalFactV1::Unbound,
        ReservedQualifiedReceiverRouteV1::Ordinary,
    )
    .unwrap();
    assert_eq!(
        VerifiedSourceStaticCallTargetCatalogV1::seal_qualified(
            &declarations,
            &qualified_imports,
            [qualified]
        )
        .unwrap()
        .extend_current_owner(&declarations, [duplicate])
        .unwrap_err(),
        CurrentOwnerStaticCallTargetErrorV1::DuplicateCallSite {
            caller: caller.clone(),
            site: duplicate_site,
        }
    );

    let missing = parse("static box Helpers { call(x) { return me.absent(x) } }");
    let missing_declarations = catalog(&missing);
    let missing_caller = key(
        &missing_declarations,
        SameModuleCallableNamespaceV1::StaticBoxMethod,
        "Helpers",
        "call",
        1,
    );
    let missing_expr = expression(&missing_declarations, "Helpers", "call", 1);
    assert_eq!(
        empty_targets(&missing_declarations)
            .extend_current_owner(
                &missing_declarations,
                [candidate(missing_caller, site(0), &missing_expr)]
            )
            .unwrap_err(),
        CurrentOwnerStaticCallTargetErrorV1::TargetOutsideCatalog {
            owner: "Helpers".into(),
            method: "absent".into(),
            arity: 1,
        }
    );
}

#[test]
fn checked_arity_rejects_overflow_when_platform_can_represent_it() {
    if let Ok(overflow) = usize::try_from(u64::from(u32::MAX) + 1) {
        assert_eq!(
            checked_explicit_arity("target", overflow).unwrap_err(),
            CurrentOwnerStaticCallTargetErrorV1::ArityOverflow {
                method: "target".into()
            }
        );
    }
}
