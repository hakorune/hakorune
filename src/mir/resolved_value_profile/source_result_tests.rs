use super::source_result::{
    issue_source_result_product_v1, SourceCallDispositionV1, SourceResultClassV1,
    SourceResultProductErrorV1, SourceValueOperationV1, VerifiedSourceResultProductV1,
};
use crate::ast::ASTNode;
use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, NormalCallableSemanticAdmissionV1,
    SameModuleCallableNamespaceV1, VerifiedNormalCallableSemanticSourceV1,
    VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::source_projection::VerifiedSourceProjectionV1;
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::resolved_semantics::{
    CallableFunctionSyntaxViewV1, FunctionSemanticResolverSessionV1,
    ResolveSelectedCallableForestsWithBodyShapesOutcomeV1,
};
use crate::mir::source_call_target::{
    VerifiedSourceCallTargetCatalogV1, VerifiedStaticImportAliasViewV1,
    VerifiedWholeSourceStaticCallTargetInventoryV1,
};
use crate::parser::NyashParser;

fn method_declaration(root: &ASTNode, owner: &str, method: &str) -> ASTNode {
    let ASTNode::Program { statements, .. } = root else {
        panic!("program")
    };
    for statement in statements {
        let ASTNode::BoxDeclaration { name, methods, .. } = statement else {
            continue;
        };
        if name == owner {
            return methods
                .get_declaration(method)
                .expect("method declaration")
                .clone();
        }
    }
    panic!("missing method declaration")
}

fn catalog_key(
    declarations: &VerifiedSameModuleCallableDeclarationCatalogV1,
    owner: &str,
    method: &str,
) -> CanonicalSameModuleCallableKeyV1 {
    let mut found = declarations.declarations().filter(|(key, _)| {
        key.namespace() == SameModuleCallableNamespaceV1::StaticBoxMethod
            && key.owner() == owner
            && key.name() == method
    });
    let Some((key, _)) = found.next() else {
        panic!("catalog key")
    };
    assert!(found.next().is_none(), "ambiguous catalog key");
    key.clone()
}

fn issue(
    source: &str,
    owner: &str,
    method: &str,
) -> Result<VerifiedSourceResultProductV1, SourceResultProductErrorV1> {
    let root = NyashParser::parse_from_string(source).expect("fixture parses");
    let declarations = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root).unwrap();
    let key = catalog_key(&declarations, owner, method);
    let declaration = method_declaration(&root, owner, method);
    let unit =
        VerifiedResolvedSourceUnitV1::resolve_function(declaration).expect("fixture resolves");
    let input = unit.root_function_input().expect("root input");
    let imports = VerifiedStaticImportAliasViewV1::seal(&declarations, []).expect("imports seal");
    let targets = VerifiedWholeSourceStaticCallTargetInventoryV1::verify(&declarations, &imports)
        .expect("route inventory")
        .into_targets();
    issue_source_result_product_v1(&declarations, &key, input, &targets)
}

#[test]
fn source_result_product_preserves_string_conditional_and_null_fact() {
    let product = issue(
        r#"static box Helpers { value() { local sval = "x" return sval != null ? { "a" } : { "b" } } }"#,
        "Helpers",
        "value",
    )
    .unwrap();
    assert_eq!(product.result(), SourceResultClassV1::String);
    assert!(product.operations().iter().any(|row| matches!(
        row,
        SourceValueOperationV1::ConditionalValue {
            class: SourceResultClassV1::String,
            ..
        }
    )));
    assert_eq!(product.bool_facts().len(), 1);
}

#[test]
fn source_result_product_joins_i64_conditional_return() {
    let product = issue(
        r#"static box Helpers { value() { local lhs = 1 local rhs = 2 return (lhs < rhs) ? { 1 } : { 0 } } }"#,
        "Helpers",
        "value",
    )
    .unwrap();
    assert_eq!(product.result(), SourceResultClassV1::I64);
    assert!(product.operations().iter().any(|row| matches!(
        row,
        SourceValueOperationV1::ConditionalValue {
            class: SourceResultClassV1::I64,
            ..
        }
    )));
}

#[test]
fn source_result_product_consumes_initializer_consumer() {
    let product = issue(
        r#"static box Helpers { value() { local flag = true local out = flag ? { "a" } : { "b" } return out } }"#,
        "Helpers",
        "value",
    )
    .unwrap();
    assert_eq!(product.result(), SourceResultClassV1::String);
}

#[test]
fn source_result_product_consumes_rhs_consumer() {
    let product = issue(
        r#"static box Helpers { value() { local flag = true local out = "" + (flag ? { "a" } : { "b" }) return out } }"#,
        "Helpers",
        "value",
    )
    .unwrap();
    assert_eq!(product.result(), SourceResultClassV1::String);
    assert!(product.operations().iter().any(|row| matches!(
        row,
        SourceValueOperationV1::ConditionalValue {
            class: SourceResultClassV1::String,
            ..
        }
    )));
}

#[test]
fn source_result_product_publishes_branded_static_call_disposition() {
    let product = issue(
        r#"static box Helpers { int_to_str(v) : String { return "x" } } static box Api { normalize(flag) { return flag ? { "?" } : Helpers.int_to_str(1) } }"#,
        "Api",
        "normalize",
    )
    .unwrap();
    assert_eq!(product.result(), SourceResultClassV1::String);
    let target = CanonicalSameModuleCallableKeyV1::static_box_method("Helpers", "int_to_str", 1);
    assert!(product.call_dispositions().iter().any(|row| matches!(
        row,
        SourceCallDispositionV1::Static { target: key, .. } if key == &target
    )));
}

#[test]
fn source_result_product_publishes_absent_disposition_for_unproven_call() {
    let product = issue(
        r#"static box Api { probe(frac2) { local ok = (frac2.length() == 0) ? { 1 } : { 0 } return ok } }"#,
        "Api",
        "probe",
    )
    .unwrap();
    assert_eq!(product.result(), SourceResultClassV1::I64);
    assert_eq!(product.call_dispositions().len(), 1);
    assert!(matches!(
        product.call_dispositions()[0],
        SourceCallDispositionV1::Absent { .. }
    ));
}

#[test]
fn source_result_product_rejects_unproven_call_tail() {
    let result = issue(
        r#"static box Helpers { int_to_str(v) { return "x" } } static box Api { normalize(flag) { return flag ? { "?" } : Helpers.int_to_str(1) } }"#,
        "Api",
        "normalize",
    );
    // `int_to_str` has no declared return class: the branded Static route
    // exists but the branch tail class is unproven.
    assert!(matches!(
        result,
        Err(SourceResultProductErrorV1::MixedReturnClasses)
            | Err(SourceResultProductErrorV1::UnprovenResultClass(_))
    ));
}

#[test]
fn source_result_product_rejects_mixed_conditional_classes() {
    let result = issue(
        r#"static box Helpers { value() { return 1 != 0 ? { 1 } : { "b" } } }"#,
        "Helpers",
        "value",
    );
    assert!(matches!(
        result,
        Err(SourceResultProductErrorV1::MixedReturnClasses)
    ));
}

#[test]
fn source_result_product_rejects_nullable_guard_without_string_binding() {
    let result = issue(
        r#"static box Helpers { value() { return 1 != null ? { 1 } : { 0 } } }"#,
        "Helpers",
        "value",
    );
    assert!(matches!(
        result,
        Err(SourceResultProductErrorV1::NullableOperandNotString(_))
    ));
}

#[test]
fn source_result_product_rejects_statement_if_scaffold() {
    let result = issue(
        r#"static box Helpers { value() { local flag = true if flag { return 1 } return 0 } }"#,
        "Helpers",
        "value",
    );
    assert!(matches!(
        result,
        Err(SourceResultProductErrorV1::UnsupportedStatement)
    ));
}

#[test]
fn source_result_product_rejects_foreign_owner_before_ledger_consumption() {
    let root =
        NyashParser::parse_from_string("static box Helpers { value() { return 1 } }").unwrap();
    let declarations = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root).unwrap();
    let declaration = method_declaration(&root, "Helpers", "value");
    let unit =
        VerifiedResolvedSourceUnitV1::resolve_function(declaration).expect("fixture resolves");
    let input = unit.root_function_input().expect("root input");
    let imports = VerifiedStaticImportAliasViewV1::seal(&declarations, []).expect("imports seal");
    let targets = VerifiedWholeSourceStaticCallTargetInventoryV1::verify(&declarations, &imports)
        .expect("route inventory")
        .into_targets();
    let foreign = CanonicalSameModuleCallableKeyV1::static_box_method("Other", "value", 0);
    assert!(matches!(
        issue_source_result_product_v1(&declarations, &foreign, input, &targets),
        Err(SourceResultProductErrorV1::ForeignCallable)
    ));
}

#[test]
fn source_result_product_rejects_same_shape_resolved_input_from_foreign_declaration() {
    // Two declarations with the same parameter shape in different boxes:
    // the input resolved from Alpha.value must not seal under Beta.value's
    // key. (Body-identical foreign declarations remain indistinguishable —
    // parser nodes carry Span::unknown() — but identical content produces
    // identical product rows, so only the owner label could differ.)
    let root = NyashParser::parse_from_string(
        "static box Alpha { value(x) { return \"v\" } }\n\
         static box Beta { value(x) { return 0 } }",
    )
    .unwrap();
    let declarations = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root).unwrap();
    let beta_key = catalog_key(&declarations, "Beta", "value");
    let alpha_declaration = method_declaration(&root, "Alpha", "value");
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(alpha_declaration)
        .expect("fixture resolves");
    let input = unit.root_function_input().expect("root input");
    let imports = VerifiedStaticImportAliasViewV1::seal(&declarations, []).expect("imports seal");
    let targets = VerifiedWholeSourceStaticCallTargetInventoryV1::verify(&declarations, &imports)
        .expect("route inventory")
        .into_targets();
    assert!(matches!(
        issue_source_result_product_v1(&declarations, &beta_key, input, &targets),
        Err(SourceResultProductErrorV1::ForeignResolvedInput)
    ));
    let alpha_key = catalog_key(&declarations, "Alpha", "value");
    let alpha_declaration = method_declaration(&root, "Alpha", "value");
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(alpha_declaration)
        .expect("fixture resolves");
    let input = unit.root_function_input().expect("root input");
    assert!(
        issue_source_result_product_v1(&declarations, &alpha_key, input, &targets).is_ok(),
        "the exact declaration's own input must still seal"
    );
}

#[test]
fn source_result_product_rejects_foreign_call_target_catalog() {
    let source = r#"static box Helpers { value() { return 1 } }"#;
    let root = NyashParser::parse_from_string(source).unwrap();
    let declarations = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root).unwrap();
    let key = catalog_key(&declarations, "Helpers", "value");
    let declaration = method_declaration(&root, "Helpers", "value");
    let unit =
        VerifiedResolvedSourceUnitV1::resolve_function(declaration).expect("fixture resolves");
    let input = unit.root_function_input().expect("root input");
    // A route catalog branded by a different declaration catalog must not
    // compose with this issuer.
    let foreign_root =
        NyashParser::parse_from_string("static box Helpers { value() { return 1 } }").unwrap();
    let foreign_declarations =
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&foreign_root).unwrap();
    let foreign_imports =
        VerifiedStaticImportAliasViewV1::seal(&foreign_declarations, []).expect("imports");
    let foreign_targets = VerifiedWholeSourceStaticCallTargetInventoryV1::verify(
        &foreign_declarations,
        &foreign_imports,
    )
    .expect("route inventory")
    .into_targets();
    assert!(matches!(
        issue_source_result_product_v1(&declarations, &key, input, &foreign_targets),
        Err(SourceResultProductErrorV1::ForeignCallTargetCatalog)
    ));
}

#[test]
fn source_result_product_rejects_missing_conditional_row() {
    // The input was resolved from a different body than the catalog row:
    // the declaration/input identity boundary rejects it before the ledger
    // is consumed.
    let declared_root = NyashParser::parse_from_string(
        r#"static box Api { broken(flag) { return flag ? { 1 } : { 0 } } }"#,
    )
    .unwrap();
    let declarations =
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&declared_root).unwrap();
    let key = catalog_key(&declarations, "Api", "broken");
    let other_root =
        NyashParser::parse_from_string(r#"static box Api { broken(flag) { return 0 } }"#).unwrap();
    let declaration = method_declaration(&other_root, "Api", "broken");
    let unit =
        VerifiedResolvedSourceUnitV1::resolve_function(declaration).expect("fixture resolves");
    let input = unit.root_function_input().expect("root input");
    let imports = VerifiedStaticImportAliasViewV1::seal(&declarations, []).expect("imports seal");
    let targets = VerifiedWholeSourceStaticCallTargetInventoryV1::verify(&declarations, &imports)
        .expect("route inventory")
        .into_targets();
    assert!(matches!(
        issue_source_result_product_v1(&declarations, &key, input, &targets),
        Err(SourceResultProductErrorV1::ForeignResolvedInput)
    ));
}

/// Seals the whole-program normal callable source batch so Dynamic member
/// routes can extend the branded catalog for one exact caller.
fn seal_source<'source>(
    program: &'source ASTNode,
    catalog: &VerifiedSameModuleCallableDeclarationCatalogV1,
) -> VerifiedNormalCallableSemanticSourceV1<'source> {
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let NormalCallableSemanticAdmissionV1::Complete(source) =
        VerifiedNormalCallableSemanticSourceV1::seal(
            program,
            catalog.selected_source_inventory(),
            false,
            &mut resolver,
        )
        .unwrap()
    else {
        panic!("fixture must admit a complete source batch")
    };
    source
}

#[test]
fn source_result_product_consumes_nested_direct_call_observation() {
    // `missing(1)` is an ObserveOnly direct call: the resolver records the
    // observation with no target row, so the issuer publishes Absent and the
    // call result stays unproven without blocking the i64 join.  Direct-call
    // observations require the ObserveOnly resolver batch, which also emits
    // the co-sealed body-shape inventory the issuer consumes.
    let source = r#"static box Api { probe(flag) { return missing(1) != 0 ? { 1 } : { 0 } } }"#;
    let root = NyashParser::parse_from_string(source).expect("fixture parses");
    let declarations = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root).unwrap();
    let key = catalog_key(&declarations, "Api", "probe");
    let declaration = method_declaration(&root, "Api", "probe");
    let syntax = CallableFunctionSyntaxViewV1::from_function_ast(&declaration)
        .expect("function syntax view");
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).expect("resolver session");
    let ResolveSelectedCallableForestsWithBodyShapesOutcomeV1::Complete {
        forests,
        mut body_shapes,
    } = resolver
        .resolve_selected_callable_forests_with_body_shapes(&[syntax.function()])
        .expect("resolved forest")
    else {
        panic!("fixture unexpectedly deferred")
    };
    let mut forests = forests.into_vec();
    assert_eq!(forests.len(), 1);
    let forest = forests.pop().expect("one forest");
    let projection = VerifiedSourceProjectionV1::seal_with_root_profile(
        &declaration,
        &forest,
        syntax.function().root_profile(),
    )
    .expect("source projection");
    let input = ResolvedFunctionLoweringInputV1::from_exact_parts_without_callable(
        &declaration,
        &forest,
        &projection,
    )
    .expect("lowering input");
    let body_shape = body_shapes
        .remove(&input.owner())
        .expect("body shape for owner");
    let input = input.with_body_shape(&body_shape);
    let imports = VerifiedStaticImportAliasViewV1::seal(&declarations, []).expect("imports seal");
    let targets = VerifiedWholeSourceStaticCallTargetInventoryV1::verify(&declarations, &imports)
        .expect("route inventory")
        .into_targets();
    let product = issue_source_result_product_v1(&declarations, &key, input, &targets).unwrap();
    assert_eq!(product.result(), SourceResultClassV1::I64);
    assert_eq!(product.call_dispositions().len(), 1);
    assert!(matches!(
        product.call_dispositions()[0],
        SourceCallDispositionV1::Absent { .. }
    ));
}

#[test]
fn source_result_product_publishes_dynamic_member_disposition() {
    // `frac2` is an untyped parameter, so `frac2.length()` is a source-bound
    // dynamic member route: the branded catalog proves Dynamic, the call
    // result stays unproven, and the surrounding i64 join still issues.
    let program = NyashParser::parse_from_string(
        r#"static box Api { probe(frac2) { local ok = (frac2.length() == 0) ? { 1 } : { 0 } return ok } }"#,
    )
    .unwrap();
    let declarations =
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&program).unwrap();
    let key = catalog_key(&declarations, "Api", "probe");
    let source = seal_source(&program, &declarations);
    let imports = VerifiedStaticImportAliasViewV1::seal(&declarations, []).expect("imports seal");
    let targets = VerifiedSourceCallTargetCatalogV1::seal_qualified(&imports, [])
        .expect("empty qualified catalog")
        .extend_complete_dynamic_sources(&source)
        .expect("dynamic extension");
    let declaration = method_declaration(&program, "Api", "probe");
    let unit =
        VerifiedResolvedSourceUnitV1::resolve_function(declaration).expect("fixture resolves");
    let input = unit.root_function_input().expect("root input");
    let product = issue_source_result_product_v1(&declarations, &key, input, &targets).unwrap();
    assert_eq!(product.result(), SourceResultClassV1::I64);
    assert_eq!(product.call_dispositions().len(), 1);
    assert!(matches!(
        product.call_dispositions()[0],
        SourceCallDispositionV1::Dynamic { .. }
    ));
}

#[test]
fn source_result_product_keeps_catalog_identity_across_catalog_move() {
    let root =
        NyashParser::parse_from_string("static box Helpers { value() { return 1 } }").unwrap();
    let declarations = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root).unwrap();
    let identity = declarations.brand().identity();
    let key = catalog_key(&declarations, "Helpers", "value");
    let declaration = method_declaration(&root, "Helpers", "value");
    let unit =
        VerifiedResolvedSourceUnitV1::resolve_function(declaration).expect("fixture resolves");
    let input = unit.root_function_input().expect("root input");
    let imports = VerifiedStaticImportAliasViewV1::seal(&declarations, []).expect("imports seal");
    let targets = VerifiedWholeSourceStaticCallTargetInventoryV1::verify(&declarations, &imports)
        .expect("route inventory")
        .into_targets();
    let product = issue_source_result_product_v1(&declarations, &key, input, &targets).unwrap();
    assert_eq!(product.catalog_identity(), identity);
}
