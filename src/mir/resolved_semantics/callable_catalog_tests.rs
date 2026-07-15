use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl, Span};
use crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1;

use super::*;

fn function(name: &str, parameter_count: usize) -> ASTNode {
    let params = (0..parameter_count)
        .map(|index| format!("p{index}"))
        .collect::<Vec<_>>();
    ASTNode::FunctionDeclaration {
        param_decls: params
            .iter()
            .map(|name| ParamDecl {
                name: name.clone(),
                declared_type_name: Some("i64".to_string()),
            })
            .collect(),
        name: name.to_string(),
        params,
        return_type_name: Some("i64".to_string()),
        body: Vec::new(),
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn owner_free(functions: Vec<ASTNode>) -> VerifiedOwnerFreeCallableCatalogSourceUnitV1 {
    let source = VerifiedCallableHeaderSourceUnitV1::seal_header_surface(ASTNode::Program {
        statements: functions,
        span: Span::unknown(),
    })
    .unwrap();
    VerifiedOwnerFreeCallableCatalogSourceUnitV1::seal(source).unwrap()
}

#[test]
fn reserves_all_top_level_origins_and_owners_before_body_resolution() {
    let outcome = CallableCatalogSealOutcomeV1::seal(
        owner_free(vec![function("first", 1), function("second", 2)]),
        7,
    )
    .unwrap();
    let (unit, _) = outcome.into_parts();

    assert_eq!(unit.catalog().len(), 2);
    let mut compilation_brand = None;
    for (index, &site) in unit.declaration_sites().iter().enumerate() {
        let declaration = unit.catalog().declaration(site).unwrap();
        assert_eq!(declaration.site(), site);
        assert_eq!(declaration.origin().compilation_unit_ordinal(), 7);
        assert_eq!(declaration.origin().function_ordinal(), index as u32);
        assert_eq!(declaration.callable().owner().slot(), index as u32);
        match compilation_brand {
            Some(expected) => {
                assert_eq!(declaration.callable().owner().compilation_brand(), expected)
            }
            None => compilation_brand = Some(declaration.callable().owner().compilation_brand()),
        }
        let header = unit
            .catalog()
            .index()
            .header_for_callable(declaration.callable())
            .unwrap();
        assert_eq!(
            header.source_key().name(),
            if index == 0 { "first" } else { "second" }
        );
        assert_eq!(
            unit.catalog().index().header_for_symbol(header.symbol()),
            Ok(header)
        );
    }
}

#[test]
fn continuation_preserves_next_origin_and_same_owner_brand() {
    let outcome = CallableCatalogSealOutcomeV1::seal(
        owner_free(vec![function("first", 1), function("second", 1)]),
        11,
    )
    .unwrap();
    let (unit, continuation) = outcome.into_parts();
    let first_site = unit.declaration_sites()[0];
    let first_owner = unit
        .catalog()
        .declaration(first_site)
        .unwrap()
        .callable()
        .owner();
    let mut resolver = continuation.into_resolver();
    let (next_origin, next_owner) = resolver.issue_owner().unwrap();

    assert_eq!(next_origin.compilation_unit_ordinal(), 11);
    assert_eq!(next_origin.function_ordinal(), 2);
    assert_eq!(next_owner.slot(), 2);
    assert_eq!(
        next_owner.compilation_brand(),
        first_owner.compilation_brand()
    );
}

#[test]
fn one_entry_catalog_preserves_exact_callable_header_contract() {
    let outcome =
        CallableCatalogSealOutcomeV1::seal(owner_free(vec![function("only", 1)]), 0).unwrap();
    let (unit, _) = outcome.into_parts();
    let header = unit.catalog().index().sole_header().unwrap();

    assert_eq!(header.source_key().name(), "only");
    assert_eq!(header.source_key().arity(), 1);
    assert_eq!(header.symbol().as_mir_name(), "only/1");
    assert_eq!(header.signature().params(), &[ExactTrivialScalarAbiV1::I64]);
    assert_eq!(header.signature().result(), ExactTrivialScalarAbiV1::I64);
}

#[test]
fn co_sealed_unit_retains_exact_program_header_membership() {
    let outcome = CallableCatalogSealOutcomeV1::seal(
        owner_free(vec![function("f", 1), function("f", 2), function("g", 1)]),
        0,
    )
    .unwrap();
    let (unit, _) = outcome.into_parts();

    assert_eq!(unit.declaration_sites().len(), 3);
    for &site in unit.declaration_sites() {
        assert!(unit.located_header(site).is_some());
        assert!(unit.catalog().declaration(site).is_some());
    }
}

#[test]
fn normalized_catalog_is_independent_of_declaration_order_and_owner_brand() {
    let first = CallableCatalogSealOutcomeV1::seal(
        owner_free(vec![function("zeta", 2), function("alpha", 1)]),
        3,
    )
    .unwrap();
    let second = CallableCatalogSealOutcomeV1::seal(
        owner_free(vec![function("alpha", 1), function("zeta", 2)]),
        91,
    )
    .unwrap();
    let (first, _) = first.into_parts();
    let (second, _) = second.into_parts();
    let first_normalized = NormalizedCallableCatalogV1::from_catalog(first.catalog());
    let second_normalized = NormalizedCallableCatalogV1::from_catalog(second.catalog());

    assert_eq!(first_normalized, second_normalized);
    assert_eq!(first_normalized.rows().len(), 2);
    assert_eq!(first_normalized.rows()[0].name(), "alpha");
    assert_eq!(first_normalized.rows()[1].name(), "zeta");
    assert_eq!(first_normalized.rows()[0].arity(), 1);
    assert_eq!(first_normalized.rows()[1].arity(), 2);
    assert_eq!(first_normalized.rows()[0].symbol(), "alpha/1");
    assert_eq!(first_normalized.rows()[1].symbol(), "zeta/2");
    assert_eq!(
        first_normalized.rows()[0].namespace(),
        CallableNamespaceV1::FreeStatic
    );
    assert_eq!(
        first_normalized.rows()[0].params(),
        &[ExactTrivialScalarAbiV1::I64]
    );
    assert_eq!(
        first_normalized.rows()[0].result(),
        ExactTrivialScalarAbiV1::I64
    );
}

#[test]
fn declaration_reorder_preserves_exact_lookup_results() {
    for functions in [
        vec![function("left", 1), function("right", 2)],
        vec![function("right", 2), function("left", 1)],
    ] {
        let outcome = CallableCatalogSealOutcomeV1::seal(owner_free(functions), 0).unwrap();
        let (unit, _) = outcome.into_parts();
        let left = unit
            .catalog()
            .index()
            .resolve_free_static_source_call("left", 1)
            .unwrap();
        let right = unit
            .catalog()
            .index()
            .resolve_free_static_source_call("right", 2)
            .unwrap();
        assert_eq!(left.symbol().as_mir_name(), "left/1");
        assert_eq!(right.symbol().as_mir_name(), "right/2");
    }
}
