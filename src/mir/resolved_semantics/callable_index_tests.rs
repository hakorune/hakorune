use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl, Span};
use std::collections::BTreeMap;

use super::*;
use crate::mir::resolved_semantics::FunctionOwnerIssuerV1;

fn owner() -> FunctionOwnerIdV1 {
    let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
    issuer.issue().unwrap()
}

fn function(name: &str, params: &[(&str, Option<&str>)], result: Option<&str>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.to_string(),
        params: params.iter().map(|(name, _)| (*name).to_string()).collect(),
        param_decls: params
            .iter()
            .map(|(name, ty)| ParamDecl {
                name: (*name).to_string(),
                declared_type_name: ty.map(str::to_string),
            })
            .collect(),
        return_type_name: result.map(str::to_string),
        body: Vec::new(),
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn seal(tree: &ASTNode) -> Result<VerifiedCallableIndexV1, CallableIndexSealErrorV1> {
    VerifiedCallableIndexV1::seal_one(
        owner(),
        CallableHeaderSyntaxViewV1::from_function_ast(tree).unwrap(),
    )
}

#[test]
fn seals_one_exact_static_i64_header_without_new_identity() {
    let tree = function("countdown", &[("n", Some("i64"))], Some("i64"));
    let owner_id = owner();
    let index = VerifiedCallableIndexV1::seal_one(
        owner_id,
        CallableHeaderSyntaxViewV1::from_function_ast(&tree).unwrap(),
    )
    .unwrap();

    assert_eq!(index.len(), 1);
    let header = index.sole_header().unwrap();
    assert_eq!(header.callable().owner(), owner_id);
    assert_eq!(
        header.source_key().namespace(),
        CallableNamespaceV1::FreeStatic
    );
    assert_eq!(header.source_key().name(), "countdown");
    assert_eq!(header.source_key().arity(), 1);
    assert_eq!(header.symbol().as_mir_name(), "countdown/1");
    assert_eq!(header.signature().arity(), 1);
    assert_eq!(header.signature().params(), &[ExactTrivialScalarAbiV1::I64]);
    assert_eq!(header.signature().result(), ExactTrivialScalarAbiV1::I64);
    assert_eq!(index.lookup(header.source_key()), Some(header));
    assert_eq!(index.header_for_callable(header.callable()), Ok(header));
    assert_eq!(index.header_for_symbol(header.symbol()), Ok(header));

    let wrong_name = CanonicalCallableKeyV1::free_static("countup", 1);
    let wrong_arity = CanonicalCallableKeyV1::free_static("countdown", 2);
    assert_eq!(index.lookup(&wrong_name), None);
    assert_eq!(index.lookup(&wrong_arity), None);
    let missing_callable = ResolvedCallableRefV1::new(owner());
    assert_eq!(
        index.header_for_callable(missing_callable),
        Err(CallableLookupErrorV1::MissingCallableIdentity)
    );
    let missing_symbol = CanonicalCallableSymbolV1::from_key(&wrong_name);
    assert_eq!(
        index.header_for_symbol(&missing_symbol),
        Err(CallableLookupErrorV1::MissingPhysicalSymbol)
    );
}

#[test]
fn sole_header_reports_cardinality_without_panicking() {
    let empty = VerifiedCallableIndexV1 {
        headers_by_key: BTreeMap::new(),
        key_by_callable: BTreeMap::new(),
        key_by_symbol: BTreeMap::new(),
    };
    assert_eq!(
        empty.sole_header(),
        Err(CallableCatalogCardinalityErrorV1 { actual: 0 })
    );

    let mut headers_by_key = BTreeMap::new();
    let mut key_by_callable = BTreeMap::new();
    let mut key_by_symbol = BTreeMap::new();
    for name in ["f", "g"] {
        let tree = function(name, &[("x", Some("i64"))], Some("i64"));
        let header = seal_exact_i64_header(
            owner(),
            CallableHeaderSyntaxViewV1::from_function_ast(&tree).unwrap(),
        )
        .unwrap();
        let key = header.source_key().clone();
        key_by_callable.insert(header.callable(), key.clone());
        key_by_symbol.insert(header.symbol().clone(), key.clone());
        headers_by_key.insert(key, header);
    }
    let two = VerifiedCallableIndexV1 {
        headers_by_key,
        key_by_callable,
        key_by_symbol,
    };
    let error = two.sole_header().unwrap_err();
    assert_eq!(error.actual(), 2);
}

#[test]
fn rejects_non_exact_parameter_and_return_spellings() {
    for spelling in ["int", "Integer", "I64", " i64", "i64 "] {
        let bad_param = function("f", &[("x", Some(spelling))], Some("i64"));
        assert_eq!(
            seal(&bad_param),
            Err(CallableIndexSealErrorV1::ParameterTypeOutsideProfile { index: 0 })
        );

        let bad_return = function("f", &[("x", Some("i64"))], Some(spelling));
        assert_eq!(
            seal(&bad_return),
            Err(CallableIndexSealErrorV1::ReturnTypeOutsideProfile)
        );
    }
}

#[test]
fn rejects_missing_or_mismatched_parameter_declarations() {
    let zero = function("f", &[], Some("i64"));
    assert_eq!(seal(&zero), Err(CallableIndexSealErrorV1::ZeroParameters));

    let mut missing = function("f", &[("x", Some("i64"))], Some("i64"));
    let ASTNode::FunctionDeclaration { param_decls, .. } = &mut missing else {
        unreachable!()
    };
    param_decls.clear();
    assert_eq!(
        seal(&missing),
        Err(CallableIndexSealErrorV1::ParameterDeclarationCardinality)
    );

    let mut mismatch = function("f", &[("x", Some("i64"))], Some("i64"));
    let ASTNode::FunctionDeclaration { param_decls, .. } = &mut mismatch else {
        unreachable!()
    };
    param_decls[0].name = "y".to_string();
    assert_eq!(
        seal(&mismatch),
        Err(CallableIndexSealErrorV1::ParameterNameMismatch { index: 0 })
    );
}

#[test]
fn rejects_owner_families_and_metadata_outside_first_profile() {
    let mut instance = function("f", &[("x", Some("i64"))], Some("i64"));
    let ASTNode::FunctionDeclaration { is_static, .. } = &mut instance else {
        unreachable!()
    };
    *is_static = false;
    assert_eq!(
        seal(&instance),
        Err(CallableIndexSealErrorV1::StaticRequired)
    );

    let main = function("main", &[("x", Some("i64"))], Some("i64"));
    assert_eq!(seal(&main), Err(CallableIndexSealErrorV1::MainUnsupported));

    let mut overridden = function("f", &[("x", Some("i64"))], Some("i64"));
    let ASTNode::FunctionDeclaration { is_override, .. } = &mut overridden else {
        unreachable!()
    };
    *is_override = true;
    assert_eq!(
        seal(&overridden),
        Err(CallableIndexSealErrorV1::OverrideUnsupported)
    );

    let mut attributed = function("f", &[("x", Some("i64"))], Some("i64"));
    let ASTNode::FunctionDeclaration { uses, .. } = &mut attributed else {
        unreachable!()
    };
    uses.push("io".to_string());
    assert_eq!(
        seal(&attributed),
        Err(CallableIndexSealErrorV1::MetadataOutsideProfile)
    );

    let physical = function("f/1", &[("x", Some("i64"))], Some("i64"));
    assert_eq!(
        seal(&physical),
        Err(CallableIndexSealErrorV1::PhysicalSymbolSpellingInSource)
    );
}

#[test]
fn private_draft_rejects_zero_and_duplicate_rows() {
    assert_eq!(
        CallableIndexDraftV1::default().seal_one(),
        Err(CallableIndexSealErrorV1::IndexCardinality { actual: 0 })
    );

    let tree = function("f", &[("x", Some("i64"))], Some("i64"));
    let view = CallableHeaderSyntaxViewV1::from_function_ast(&tree).unwrap();
    let header = seal_exact_i64_header(owner(), view).unwrap();
    let mut draft = CallableIndexDraftV1::default();
    draft.insert(header.clone()).unwrap();
    assert_eq!(
        draft.insert(header),
        Err(CallableIndexSealErrorV1::DuplicateSourceKey)
    );

    let first_tree = function("f", &[("x", Some("i64"))], Some("i64"));
    let second_tree = function("g", &[("x", Some("i64"))], Some("i64"));
    let mut two_rows = CallableIndexDraftV1::default();
    for tree in [&first_tree, &second_tree] {
        two_rows
            .insert(
                seal_exact_i64_header(
                    owner(),
                    CallableHeaderSyntaxViewV1::from_function_ast(tree).unwrap(),
                )
                .unwrap(),
            )
            .unwrap();
    }
    assert_eq!(
        two_rows.seal_one(),
        Err(CallableIndexSealErrorV1::IndexCardinality { actual: 2 })
    );
}
