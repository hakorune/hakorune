use crate::ast::ASTNode;
use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1,
    VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::resolved_semantics::{
    ShadowResolveErrorV0, SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1,
};
use crate::parser::NyashParser;

use super::*;

const SOURCE: &str = r#"
static box Helpers {
  run(x) { return x }
}

static box Caller {
  direct(x) { return Helpers.run(x) }
  parameter(Helpers, x) { return Helpers.run(x) }
  branch(x) {
    if x == 0 {
      local Helpers = x
      local observed = Helpers.run(x)
    }
    return Helpers.run(x)
  }
  invalid(x) {
    local bad = missing
    return Helpers.run(x)
  }
  current(x) { return me.run(x) }
  nested(x) { return Outer.consume(Helpers.run(x)) }
  looped(x) {
    loop(x > 0) {
      local observed = Helpers.run(x)
      return observed
    }
    return x
  }
}
"#;

fn parse(source: &str) -> ASTNode {
    NyashParser::parse_from_string(source).expect("qualified receiver fixture must parse")
}

fn catalog(root: &ASTNode) -> VerifiedSameModuleCallableDeclarationCatalogV1 {
    VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(root)
        .expect("declaration catalog must seal")
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
        .unwrap_or_else(|| panic!("missing Caller.{method}/{arity}"))
        .key()
        .clone()
}

fn site(segments: Vec<SourcePathSegmentV1>) -> SourceExprSiteV1 {
    SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(segments))
}

fn return_call_site() -> SourceExprSiteV1 {
    site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Value,
    ])
}

fn verify_call<'catalog>(
    declarations: &'catalog VerifiedSameModuleCallableDeclarationCatalogV1,
    caller: &CanonicalSameModuleCallableKeyV1,
    call_site: SourceExprSiteV1,
) -> VerifiedSourceMethodCallSiteV1<'catalog> {
    VerifiedSourceMethodCallSiteV1::verify(declarations, caller, call_site)
        .expect("source MethodCall site must verify")
}

#[test]
fn classifies_parameter_bound_and_direct_owner_proven_unbound() {
    let root = parse(SOURCE);
    let declarations = catalog(&root);
    let direct_key = key(&declarations, "direct", 1);
    let parameter_key = key(&declarations, "parameter", 2);
    let direct = verify_call(&declarations, &direct_key, return_call_site());
    let parameter = verify_call(&declarations, &parameter_key, return_call_site());

    let direct_rows = VerifiedQualifiedReceiverLexicalDispositionsV1::verify(&[&direct]).unwrap();
    let parameter_rows =
        VerifiedQualifiedReceiverLexicalDispositionsV1::verify(&[&parameter]).unwrap();

    assert_eq!(direct_rows.caller(), &direct_key);
    assert_eq!(
        direct_rows.disposition_for(&direct),
        Some(QualifiedReceiverLexicalDispositionV1::ProvenUnbound)
    );
    assert_eq!(
        parameter_rows.disposition_for(&parameter),
        Some(QualifiedReceiverLexicalDispositionV1::Bound)
    );
}

#[test]
fn preserves_nested_scope_shadowing_and_post_scope_unbound() {
    let root = parse(SOURCE);
    let declarations = catalog(&root);
    let caller = key(&declarations, "branch", 1);
    let nested = verify_call(
        &declarations,
        &caller,
        site(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::IfThen(1),
            SourcePathSegmentV1::Initializer(0),
        ]),
    );
    let after_scope = verify_call(
        &declarations,
        &caller,
        site(vec![
            SourcePathSegmentV1::Body(1),
            SourcePathSegmentV1::Value,
        ]),
    );

    let rows =
        VerifiedQualifiedReceiverLexicalDispositionsV1::verify(&[&nested, &after_scope]).unwrap();
    assert_eq!(
        rows.disposition_for(&nested),
        Some(QualifiedReceiverLexicalDispositionV1::Bound)
    );
    assert_eq!(
        rows.disposition_for(&after_scope),
        Some(QualifiedReceiverLexicalDispositionV1::ProvenUnbound)
    );
    assert_eq!(rows.rows().count(), 2);
}

#[test]
fn ordinary_unresolved_variable_outside_the_request_still_rejects() {
    let root = parse(SOURCE);
    let declarations = catalog(&root);
    let caller = key(&declarations, "invalid", 1);
    let call = verify_call(
        &declarations,
        &caller,
        site(vec![
            SourcePathSegmentV1::Body(1),
            SourcePathSegmentV1::Value,
        ]),
    );

    assert!(matches!(
        VerifiedQualifiedReceiverLexicalDispositionsV1::verify(&[&call]),
        Err(QualifiedReceiverLexicalDispositionErrorV1::ShadowTraversal(
            ShadowResolveErrorV0::UnresolvedName { name, site }
        )) if &*name == "missing"
            && site == site_from_segments(&[
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::Initializer(0),
            ])
    ));
}

#[test]
fn rejects_empty_duplicate_mixed_caller_and_mixed_catalog_requests() {
    assert!(matches!(
        VerifiedQualifiedReceiverLexicalDispositionsV1::verify(&[]),
        Err(QualifiedReceiverLexicalDispositionErrorV1::EmptyRequestSet)
    ));

    let root = parse(SOURCE);
    let declarations = catalog(&root);
    let direct_key = key(&declarations, "direct", 1);
    let parameter_key = key(&declarations, "parameter", 2);
    let direct = verify_call(&declarations, &direct_key, return_call_site());
    let parameter = verify_call(&declarations, &parameter_key, return_call_site());
    assert!(matches!(
        VerifiedQualifiedReceiverLexicalDispositionsV1::verify(&[&direct, &direct]),
        Err(QualifiedReceiverLexicalDispositionErrorV1::DuplicateReceiverSite { .. })
    ));
    assert!(matches!(
        VerifiedQualifiedReceiverLexicalDispositionsV1::verify(&[&direct, &parameter]),
        Err(QualifiedReceiverLexicalDispositionErrorV1::MixedCaller { .. })
    ));

    let second_root = parse(SOURCE);
    let second_declarations = catalog(&second_root);
    let second_key = key(&second_declarations, "direct", 1);
    let second = verify_call(&second_declarations, &second_key, return_call_site());
    assert!(matches!(
        VerifiedQualifiedReceiverLexicalDispositionsV1::verify(&[&direct, &second]),
        Err(QualifiedReceiverLexicalDispositionErrorV1::MixedCallerDeclaration { .. })
    ));
}

#[test]
fn rejects_current_owner_me_from_the_qualified_receiver_product() {
    let root = parse(SOURCE);
    let declarations = catalog(&root);
    let caller = key(&declarations, "current", 1);
    let call = verify_call(&declarations, &caller, return_call_site());
    assert!(matches!(
        VerifiedQualifiedReceiverLexicalDispositionsV1::verify(&[&call]),
        Err(QualifiedReceiverLexicalDispositionErrorV1::QualifiedReceiverVariableRequired { .. })
    ));
}

#[test]
fn actual_parser_string_utils_receiver_is_positive_proven_unbound() {
    let source = format!(
        "{}\n{}",
        include_str!(concat!(
            "../../../lang/src/shared/common/",
            "string_helpers.hako"
        )),
        include_str!(concat!(
            "../../../lang/src/compiler/parser/scan/",
            "parser_string_utils_box.hako"
        )),
    );
    let root = parse(&source);
    let declarations = catalog(&root);
    let caller = declarations
        .declaration_for(
            SameModuleCallableNamespaceV1::StaticBoxMethod,
            "ParserStringUtilsBox",
            "skip_ws",
            2,
        )
        .unwrap()
        .key()
        .clone();
    let call = verify_call(&declarations, &caller, return_call_site());
    let rows = VerifiedQualifiedReceiverLexicalDispositionsV1::verify(&[&call]).unwrap();
    assert_eq!(
        rows.disposition_for(&call),
        Some(QualifiedReceiverLexicalDispositionV1::ProvenUnbound)
    );
}

#[test]
fn observes_nested_argument_and_loop_initializer_receiver_sites() {
    let root = parse(SOURCE);
    let declarations = catalog(&root);

    let nested_caller = key(&declarations, "nested", 1);
    let outer = verify_call(&declarations, &nested_caller, return_call_site());
    let inner = verify_call(
        &declarations,
        &nested_caller,
        site(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::Value,
            SourcePathSegmentV1::Argument(0),
        ]),
    );
    let nested_rows =
        VerifiedQualifiedReceiverLexicalDispositionsV1::verify(&[&outer, &inner]).unwrap();
    assert_eq!(
        nested_rows.disposition_for(&outer),
        Some(QualifiedReceiverLexicalDispositionV1::ProvenUnbound)
    );
    assert_eq!(
        nested_rows.disposition_for(&inner),
        Some(QualifiedReceiverLexicalDispositionV1::ProvenUnbound)
    );

    let loop_caller = key(&declarations, "looped", 1);
    let loop_call = verify_call(
        &declarations,
        &loop_caller,
        site(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::LoopBody(0),
            SourcePathSegmentV1::Initializer(0),
        ]),
    );
    let loop_rows = VerifiedQualifiedReceiverLexicalDispositionsV1::verify(&[&loop_call]).unwrap();
    assert_eq!(
        loop_rows.disposition_for(&loop_call),
        Some(QualifiedReceiverLexicalDispositionV1::ProvenUnbound)
    );
}

#[test]
fn declaration_reorder_preserves_normalized_lexical_dispositions() {
    let reordered = SOURCE.replacen("static box Helpers {\n  run(x) { return x }\n}\n\n", "", 1)
        + "\nstatic box Helpers { run(x) { return x } }\n";

    fn normalized(source: &str) -> Vec<(SourceExprSiteV1, QualifiedReceiverLexicalDispositionV1)> {
        let root = parse(source);
        let declarations = catalog(&root);
        let caller = key(&declarations, "direct", 1);
        let call = verify_call(&declarations, &caller, return_call_site());
        VerifiedQualifiedReceiverLexicalDispositionsV1::verify(&[&call])
            .unwrap()
            .rows()
            .map(|(site, disposition)| (site.clone(), disposition))
            .collect()
    }

    assert_eq!(normalized(SOURCE), normalized(&reordered));
}

#[test]
fn instance_catalog_namespace_supplies_the_existing_receiver_policy() {
    let root = parse(
        r#"
        static box Helpers { run(x) { return x } }
        box InstanceCaller {
          invoke(x) {
            local self_ref = me
            return Helpers.run(x)
          }
        }
        "#,
    );
    let declarations = catalog(&root);
    let caller = declarations
        .declaration_for(
            SameModuleCallableNamespaceV1::InstanceBoxMethod,
            "InstanceCaller",
            "invoke",
            1,
        )
        .unwrap()
        .key()
        .clone();
    let call = verify_call(
        &declarations,
        &caller,
        site(vec![
            SourcePathSegmentV1::Body(1),
            SourcePathSegmentV1::Value,
        ]),
    );
    let rows = VerifiedQualifiedReceiverLexicalDispositionsV1::verify(&[&call]).unwrap();
    assert_eq!(
        rows.disposition_for(&call),
        Some(QualifiedReceiverLexicalDispositionV1::ProvenUnbound)
    );
}

fn site_from_segments(segments: &[SourcePathSegmentV1]) -> SourceExprSiteV1 {
    site(segments.to_vec())
}
