use crate::ast::ASTNode;
use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1,
    VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::policies::source_method_reserved_route::{
    SourceMethodReservedRouteDispositionV1, SourceMethodReservedRouteFailureV1,
};
use crate::mir::resolved_semantics::{SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1};
use crate::parser::NyashParser;

use super::*;

const SOURCE: &str = r#"
static box Helpers { run(x) { return x } }
static box Caller {
  direct(x) { return Helpers.run(x) }
  imported(Alias, x) { return Alias.run(x) }
  reserved(x) { return __repl.other(x) }
  two(Helpers, x) {
    local first = Helpers.run(x)
    return Helpers.run(first)
  }
  fastmem_call(x) {
    fastmem PageMapV0 {
      mem.unknown(x)
    }
    return x
  }
  outside_fastmem(x) { return mem.unknown(x) }
}
"#;

fn parse(source: &str) -> ASTNode {
    NyashParser::parse_from_string(source).expect("route fact fixture must parse")
}

fn catalog(root: &ASTNode) -> VerifiedSameModuleCallableDeclarationCatalogV1 {
    VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(root).unwrap()
}

fn key(
    declarations: &VerifiedSameModuleCallableDeclarationCatalogV1,
    method: &str,
    arity: usize,
) -> CanonicalSameModuleCallableKeyV1 {
    declarations
        .declaration_for(
            SameModuleCallableNamespaceV1::StaticBoxMethod,
            "Caller",
            method,
            arity,
        )
        .unwrap()
        .key()
        .clone()
}

fn site(segments: Vec<SourcePathSegmentV1>) -> SourceExprSiteV1 {
    SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(segments))
}

fn return_site() -> SourceExprSiteV1 {
    site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Value,
    ])
}

fn call<'catalog>(
    declarations: &'catalog VerifiedSameModuleCallableDeclarationCatalogV1,
    caller: &CanonicalSameModuleCallableKeyV1,
    call_site: SourceExprSiteV1,
) -> VerifiedSourceMethodCallSiteV1<'catalog> {
    VerifiedSourceMethodCallSiteV1::verify(declarations, caller, call_site).unwrap()
}

#[test]
fn direct_unbound_and_bound_import_alias_follow_exact_precedence() {
    let root = parse(SOURCE);
    let declarations = catalog(&root);

    let direct_key = key(&declarations, "direct", 1);
    let direct = call(&declarations, &direct_key, return_site());
    let direct_lexical =
        VerifiedQualifiedReceiverLexicalDispositionsV1::verify(&[&direct]).unwrap();
    let no_imports = VerifiedStaticImportAliasViewV1::seal(&declarations, []).unwrap();
    let direct_facts =
        VerifiedQualifiedCallRouteFactsV1::verify(&direct, &direct_lexical, &no_imports).unwrap();
    assert_eq!(
        direct_facts.admission(),
        QualifiedReceiverAdmissionV1::DirectCanonicalOwner
    );
    assert_eq!(direct_facts.canonical_owner(), "Helpers");
    assert_eq!(
        direct_facts.lexical_disposition(),
        QualifiedReceiverLexicalDispositionV1::ProvenUnbound
    );

    let imported_key = key(&declarations, "imported", 2);
    let imported = call(&declarations, &imported_key, return_site());
    let imported_lexical =
        VerifiedQualifiedReceiverLexicalDispositionsV1::verify(&[&imported]).unwrap();
    let imports = VerifiedStaticImportAliasViewV1::seal(
        &declarations,
        [("Alias".to_string(), "Helpers".to_string())],
    )
    .unwrap();
    let imported_facts =
        VerifiedQualifiedCallRouteFactsV1::verify(&imported, &imported_lexical, &imports).unwrap();
    assert_eq!(
        imported_facts.admission(),
        QualifiedReceiverAdmissionV1::ImportedAlias
    );
    assert_eq!(imported_facts.canonical_owner(), "Helpers");
    assert_eq!(
        imported_facts.lexical_disposition(),
        QualifiedReceiverLexicalDispositionV1::Bound
    );
}

#[test]
fn bound_direct_receiver_without_alias_rejects() {
    let root = parse(SOURCE);
    let declarations = catalog(&root);
    let caller = key(&declarations, "imported", 2);
    let call = call(&declarations, &caller, return_site());
    let lexical = VerifiedQualifiedReceiverLexicalDispositionsV1::verify(&[&call]).unwrap();
    let imports = VerifiedStaticImportAliasViewV1::seal(&declarations, []).unwrap();

    assert!(matches!(
        VerifiedQualifiedCallRouteFactsV1::verify(&call, &lexical, &imports),
        Err(QualifiedCallRouteFactsErrorV1::DirectReceiverLexicallyBound {
            receiver,
            ..
        }) if &*receiver == "Alias"
    ));
}

#[test]
fn reserved_route_rejects_before_matching_import_alias() {
    let root = parse(SOURCE);
    let declarations = catalog(&root);
    let caller = key(&declarations, "reserved", 1);
    let call = call(&declarations, &caller, return_site());
    let lexical = VerifiedQualifiedReceiverLexicalDispositionsV1::verify(&[&call]).unwrap();
    let imports = VerifiedStaticImportAliasViewV1::seal(
        &declarations,
        [("__repl".to_string(), "Helpers".to_string())],
    )
    .unwrap();

    assert_eq!(
        VerifiedQualifiedCallRouteFactsV1::verify(&call, &lexical, &imports).unwrap_err(),
        QualifiedCallRouteFactsErrorV1::ReservedRouteRejected {
            caller,
            site: return_site(),
            reason: SourceMethodReservedRouteFailureV1::UnsupportedReplMethod,
        }
    );
}

#[test]
fn source_site_alone_derives_fastmem_context() {
    let root = parse(SOURCE);
    let declarations = catalog(&root);
    let imports = VerifiedStaticImportAliasViewV1::seal(&declarations, []).unwrap();

    let inside_key = key(&declarations, "fastmem_call", 1);
    let inside_site = site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::FastMemBody(0),
    ]);
    let inside = call(&declarations, &inside_key, inside_site.clone());
    let inside_lexical =
        VerifiedQualifiedReceiverLexicalDispositionsV1::verify(&[&inside]).unwrap();
    assert_eq!(
        VerifiedQualifiedCallRouteFactsV1::verify(&inside, &inside_lexical, &imports).unwrap_err(),
        QualifiedCallRouteFactsErrorV1::ReservedRouteSelected {
            caller: inside_key,
            site: inside_site,
            disposition: SourceMethodReservedRouteDispositionV1::FastMem,
        }
    );

    let outside_key = key(&declarations, "outside_fastmem", 1);
    let outside = call(&declarations, &outside_key, return_site());
    let outside_lexical =
        VerifiedQualifiedReceiverLexicalDispositionsV1::verify(&[&outside]).unwrap();
    let facts =
        VerifiedQualifiedCallRouteFactsV1::verify(&outside, &outside_lexical, &imports).unwrap();
    assert_eq!(facts.canonical_owner(), "mem");
}

#[test]
fn rejects_missing_lexical_row_and_foreign_catalog_alias_view() {
    let root = parse(SOURCE);
    let declarations = catalog(&root);
    let caller = key(&declarations, "two", 2);
    let first = call(
        &declarations,
        &caller,
        site(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::Initializer(0),
        ]),
    );
    let second = call(
        &declarations,
        &caller,
        site(vec![
            SourcePathSegmentV1::Body(1),
            SourcePathSegmentV1::Value,
        ]),
    );
    let first_only = VerifiedQualifiedReceiverLexicalDispositionsV1::verify(&[&first]).unwrap();
    let imports = VerifiedStaticImportAliasViewV1::seal(&declarations, []).unwrap();
    assert!(matches!(
        VerifiedQualifiedCallRouteFactsV1::verify(&second, &first_only, &imports),
        Err(QualifiedCallRouteFactsErrorV1::LexicalDispositionUnavailable { .. })
    ));

    let foreign_root = parse(SOURCE);
    let foreign_declarations = catalog(&foreign_root);
    let foreign_imports = VerifiedStaticImportAliasViewV1::seal(&foreign_declarations, []).unwrap();
    let lexical = VerifiedQualifiedReceiverLexicalDispositionsV1::verify(&[&second]).unwrap();
    assert!(matches!(
        VerifiedQualifiedCallRouteFactsV1::verify(&second, &lexical, &foreign_imports),
        Err(QualifiedCallRouteFactsErrorV1::ImportCatalogMismatch { .. })
    ));
}

#[test]
fn declaration_reorder_preserves_normalized_route_facts() {
    fn normalized(source: &str) -> (QualifiedReceiverAdmissionV1, String) {
        let root = parse(source);
        let declarations = catalog(&root);
        let caller = key(&declarations, "direct", 1);
        let call = call(&declarations, &caller, return_site());
        let lexical = VerifiedQualifiedReceiverLexicalDispositionsV1::verify(&[&call]).unwrap();
        let imports = VerifiedStaticImportAliasViewV1::seal(&declarations, []).unwrap();
        let facts = VerifiedQualifiedCallRouteFactsV1::verify(&call, &lexical, &imports).unwrap();
        (facts.admission(), facts.canonical_owner().into())
    }

    let reordered = SOURCE.replacen("static box Helpers { run(x) { return x } }\n", "", 1)
        + "\nstatic box Helpers { run(x) { return x } }\n";
    assert_eq!(normalized(SOURCE), normalized(&reordered));
}
