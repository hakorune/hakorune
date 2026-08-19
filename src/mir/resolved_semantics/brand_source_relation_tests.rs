use crate::analysis::brand_program_declaration_catalog::{
    BrandDeclarationSourceIdV1, BrandProgramDeclarationCatalogDraftV1,
};
use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};

use super::shadow::{
    resolve_owner_shadow_view_with_profile_and_brand_catalog_v1, ShadowResolveErrorV0,
    ShadowTraversalProfileV1,
};
use super::{
    BrandCallSourceRelationKindV1, FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1,
    ResolveScriptForestOutcomeV1, ScriptRootResolvedDemandV1, ScriptRootRuntimeDispositionV1,
    ScriptRootSemanticDispositionV1, ScriptSyntaxViewV1, SourcePathSegmentV1, SourcePathV1,
    VerifiedScriptRootDemandEntryV1, VerifiedScriptRootDemandWindowV1,
};

fn int(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn var(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn call(name: &str, arguments: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionCall {
        name: name.into(),
        arguments,
        span: Span::unknown(),
    }
}

fn method(object: ASTNode, selector: &str, arguments: Vec<ASTNode>) -> ASTNode {
    ASTNode::MethodCall {
        object: Box::new(object),
        method: selector.into(),
        arguments,
        span: Span::unknown(),
    }
}

fn function(body: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "fixture".into(),
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
    }
}

fn catalog(
) -> crate::analysis::brand_program_declaration_catalog::VerifiedBrandProgramDeclarationCatalogV1 {
    let mut draft = BrandProgramDeclarationCatalogDraftV1::default();
    draft
        .record_effective_declaration(
            BrandDeclarationSourceIdV1::from_program_item_ordinal(4).unwrap(),
            "PageId",
            "i64",
        )
        .unwrap();
    draft.seal()
}

#[test]
fn callable_shadow_excludes_constructor_from_direct_calls_and_seals_unwrap() {
    let catalog = catalog();
    let tree = function(vec![
        call("PageId", vec![int(7)]),
        method(var("PageId"), "unwrap", vec![int(7)]),
    ]);
    let view = FunctionSyntaxViewV1::from_ast(&tree).unwrap();
    let shadow = resolve_owner_shadow_view_with_profile_and_brand_catalog_v1(
        view,
        Default::default(),
        ShadowTraversalProfileV1::SelectedCallableV1,
        &catalog,
    )
    .unwrap()
    .function;

    assert_eq!(shadow.brand_calls.len(), 2);
    assert!(shadow.direct_calls.is_empty());

    let mut resolver = FunctionSemanticResolverSessionV1::new(811).unwrap();
    let outcome = resolver
        .resolve_selected_callable_forests_with_body_shapes_and_brand_catalog(
            &[view],
            Some(&catalog),
        )
        .unwrap();
    let super::ResolveSelectedCallableForestsWithBodyShapesOutcomeV1::Complete { forests, .. } =
        outcome
    else {
        panic!("catalog-owned Brand calls must not defer")
    };
    let [forest] = forests.as_ref() else {
        panic!("one forest")
    };
    let [owner] = forest.roots() else {
        panic!("one owner")
    };
    let product = forest.owner(*owner).unwrap();
    let relations = product
        .brand_call_relations()
        .map(|(_, row)| row)
        .collect::<Vec<_>>();
    assert_eq!(relations.len(), 2);
    assert_eq!(
        relations[0].kind(),
        BrandCallSourceRelationKindV1::Constructor
    );
    assert_eq!(relations[1].kind(), BrandCallSourceRelationKindV1::Unwrap);
    assert_eq!(relations[0].declaration().program_item_ordinal(), 4);
    assert_eq!(relations[0].underlying_type(), "i64");
    assert!(relations[0].receiver_site().is_none());
    assert!(relations[1].receiver_site().is_some());
}

#[test]
fn script_owner_uses_the_same_catalog_relation() {
    let catalog = catalog();
    let program = ASTNode::Program {
        statements: vec![call("PageId", vec![int(9)])],
        span: Span::unknown(),
    };
    let entry = VerifiedScriptRootDemandEntryV1::new(
        SourcePathV1::program_body()
            .child(SourcePathSegmentV1::ProgramBody(0))
            .stmt(),
        ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::LexicalCore),
        ScriptRootRuntimeDispositionV1::RetainedExistingTerminal,
    );
    let window = VerifiedScriptRootDemandWindowV1::seal(vec![entry], 1).unwrap();
    let mut resolver = FunctionSemanticResolverSessionV1::new(812).unwrap();
    let outcome = resolver
        .resolve_script_forest_with_declaration_views(
            ScriptSyntaxViewV1::from_program(&program).unwrap(),
            &window,
            &(),
            &(),
            &(),
            &catalog,
        )
        .unwrap();
    let ResolveScriptForestOutcomeV1::Complete(forest) = outcome else {
        panic!("Brand Script relation must complete")
    };
    let [owner] = forest.roots() else {
        panic!("one Script owner")
    };
    let product = forest
        .semantic_owner(*owner)
        .and_then(|product| product.as_script())
        .unwrap();
    let relations = product.brand_call_relations().collect::<Vec<_>>();
    let [(site, relation)] = relations.as_slice() else {
        panic!("one Brand relation")
    };
    assert_eq!(relation.owner(), *owner);
    assert_eq!(relation.call_site(), *site);
    assert_eq!(relation.name(), "PageId");
}

#[test]
fn invalid_brand_forms_reject_before_argument_traversal() {
    let catalog = catalog();
    let invalid_constructor = function(vec![call("PageId", vec![var("missing"), int(1)])]);
    let error = resolve_owner_shadow_view_with_profile_and_brand_catalog_v1(
        FunctionSyntaxViewV1::from_ast(&invalid_constructor).unwrap(),
        Default::default(),
        ShadowTraversalProfileV1::SelectedCallableV1,
        &catalog,
    )
    .unwrap_err();
    assert!(matches!(
        error,
        ShadowResolveErrorV0::BrandConstructorArity { actual: 2, .. }
    ));

    let unsupported = function(vec![method(var("PageId"), "other", vec![var("missing")])]);
    let error = resolve_owner_shadow_view_with_profile_and_brand_catalog_v1(
        FunctionSyntaxViewV1::from_ast(&unsupported).unwrap(),
        Default::default(),
        ShadowTraversalProfileV1::SelectedCallableV1,
        &catalog,
    )
    .unwrap_err();
    assert!(matches!(
        error,
        ShadowResolveErrorV0::UnsupportedBrandStaticMethod { .. }
    ));
}
