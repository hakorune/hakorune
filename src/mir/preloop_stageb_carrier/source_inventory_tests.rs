use crate::mir::builder::VerifiedSameModuleCallableDeclarationCatalogV1;
use crate::mir::callable_result_representation::{
    project_static_exact_i64_requirement_v1, StaticExactI64RequirementErrorV1,
    VerifiedSameModuleCallableResultCatalogV1,
};
use crate::mir::source_call_target::{
    VerifiedStaticImportAliasViewV1, VerifiedWholeSourceStaticCallTargetInventoryV1,
};
use crate::parser::NyashParser;

use super::{
    inventory_preloop_stageb_candidates_v1, source_inventory::PreloopStageBCandidateCardinalityV1,
    PreloopStageBCandidateIdentityV1, VerifiedPreloopStageBCandidateInventoryV1,
};

const ZERO: &str = r#"
static box Carrier {
  keep(left, right) { return right }
}
"#;

const ONE_DIRECT: &str = r#"
static box Carrier {
  keep(left, right) { return right }
}
box Caller {
  inner(value) { return 1 }
  run(text, pos) { pos = Carrier.keep(text, me.inner(pos)) }
}
"#;

const ONE_ALIAS: &str = r#"
static box Carrier {
  keep(left, right) { return right }
}
box Caller {
  inner(value) { return 1 }
  run(text, pos) { pos = Alias.keep(text, me.inner(pos)) }
}
"#;

const MANY: &str = r#"
static box Carrier {
  keep(left, right) { return right }
}
box Caller {
  inner(value) { return 1 }
  first(text, pos) { pos = Carrier.keep(text, me.inner(pos)) }
  second(text, pos) { pos = Carrier.keep(text, me.inner(pos)) }
}
"#;

const UNRELATED: &str = r#"
static box Carrier {
  keep(left, right) { return left }
}
box Caller {
  inner(value) { return 1 }
  run(text, pos) { pos = Carrier.keep(text, me.inner(pos)) }
}
"#;

const GENERAL_ALREADY_AVAILABLE: &str = r#"
static box Carrier {
  keep(left, right) { return right }
}
static box Caller {
  exact(value) { return Carrier.keep(1, value) }
}
"#;

fn catalog(source: &str) -> Box<VerifiedSameModuleCallableDeclarationCatalogV1> {
    let ast = NyashParser::parse_from_string(source).expect("Stage-B source inventory fixture");
    Box::new(
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&ast)
            .expect("Stage-B declaration catalog"),
    )
}

fn inventory(
    source: &str,
    aliases: &[(&str, &str)],
) -> (
    Box<VerifiedSameModuleCallableDeclarationCatalogV1>,
    VerifiedPreloopStageBCandidateInventoryV1,
) {
    let declarations = catalog(source);
    let imports = VerifiedStaticImportAliasViewV1::seal(
        declarations.as_ref(),
        aliases
            .iter()
            .map(|(alias, owner)| ((*alias).to_string(), (*owner).to_string())),
    )
    .expect("valid alias snapshot");
    let calls =
        VerifiedWholeSourceStaticCallTargetInventoryV1::verify(declarations.as_ref(), &imports)
            .expect("complete MethodCall inventory");
    let candidates =
        inventory_preloop_stageb_candidates_v1(&calls).expect("complete Stage-B inventory");
    assert!(candidates.is_branded_by(declarations.as_ref()));
    (declarations, candidates)
}

fn identities(source: &str) -> Vec<PreloopStageBCandidateIdentityV1> {
    let (_catalog, inventory) = inventory(source, &[]);
    inventory.candidate_identities().cloned().collect()
}

#[test]
fn exact_candidate_cardinality_is_zero_one_or_many() {
    let (_, zero) = inventory(ZERO, &[]);
    assert_eq!(zero.observed_declaration_count(), 1);
    assert_eq!(zero.observed_method_call_count(), 0);
    assert_eq!(zero.candidate_count(), 0);

    let (_, direct) = inventory(ONE_DIRECT, &[]);
    assert_eq!(direct.observed_declaration_count(), 3);
    assert_eq!(direct.observed_method_call_count(), 2);
    assert_eq!(direct.candidate_count(), 1);

    let (_, alias_missing) = inventory(ONE_ALIAS, &[]);
    assert_eq!(alias_missing.candidate_count(), 0);

    let (_, alias) = inventory(ONE_ALIAS, &[("Alias", "Carrier")]);
    assert_eq!(alias.candidate_count(), 1);

    let (_, many) = inventory(MANY, &[]);
    assert_eq!(many.observed_declaration_count(), 4);
    assert_eq!(many.observed_method_call_count(), 4);
    assert_eq!(many.candidate_count(), 2);
}

#[test]
fn consuming_cardinality_keeps_zero_one_and_many_evidence_separate() {
    let (_, zero) = inventory(ZERO, &[]);
    let PreloopStageBCandidateCardinalityV1::Zero(zero) = zero.classify() else {
        panic!("zero candidates must retain the complete zero inventory");
    };
    assert_eq!(zero.candidate_count(), 0);

    let (_, one) = inventory(ONE_DIRECT, &[]);
    let PreloopStageBCandidateCardinalityV1::One { identity, rows } = one.classify() else {
        panic!("one candidate must split one paired identity and row");
    };
    assert_eq!(identity.caller(), rows.caller());
    assert_eq!(identity.outer_call_site(), rows.outer_call_site());
    assert_eq!(
        identity.selected_argument_index(),
        rows.selected_argument_index()
    );
    assert_eq!(identity.inner_call_site(), rows.inner_call_site());
    assert_eq!(identity.outer_target(), rows.outer_target());

    let (_, many) = inventory(MANY, &[]);
    let PreloopStageBCandidateCardinalityV1::Many(many) = many.classify() else {
        panic!("many candidates must retain the complete ambiguous inventory");
    };
    assert_eq!(many.candidate_count(), 2);
}

#[test]
fn unrelated_nested_call_remains_a_complete_noncandidate() {
    let (_, inventory) = inventory(UNRELATED, &[]);
    assert_eq!(inventory.observed_method_call_count(), 2);
    assert_eq!(inventory.candidate_count(), 0);
}

#[test]
fn general_static_result_remains_owned_by_the_existing_result_catalog() {
    let declarations = catalog(GENERAL_ALREADY_AVAILABLE);
    let imports = VerifiedStaticImportAliasViewV1::seal(declarations.as_ref(), std::iter::empty())
        .expect("empty alias view");
    let calls =
        VerifiedWholeSourceStaticCallTargetInventoryV1::verify(declarations.as_ref(), &imports)
            .expect("complete MethodCall inventory");
    let results =
        VerifiedSameModuleCallableResultCatalogV1::verify(declarations.as_ref(), calls.targets())
            .expect("existing result catalog");
    let ((caller, site), _) = calls.targets().rows().next().expect("one static target");
    let projected = project_static_exact_i64_requirement_v1(
        declarations.as_ref(),
        caller,
        site,
        calls.targets(),
        &results,
    );
    assert!(
        matches!(
            projected,
            Err(StaticExactI64RequirementErrorV1::GeneralCallResultAlreadyAvailable)
        ),
        "existing general result must retain authority: {projected:?}"
    );
    let candidates =
        inventory_preloop_stageb_candidates_v1(&calls).expect("complete Stage-B inventory");
    assert_eq!(candidates.candidate_count(), 0);
}

#[test]
fn selected_candidate_retains_exact_source_and_target_identity() {
    let (_, inventory) = inventory(ONE_DIRECT, &[]);
    let identity = inventory
        .candidate_identities()
        .next()
        .expect("one exact candidate");
    assert_eq!(identity.caller().owner(), "Caller");
    assert_eq!(identity.caller().name(), "run");
    assert_eq!(identity.outer_call_site().node().segments().len(), 2);
    assert_eq!(identity.selected_argument_index(), 1);
    assert_eq!(identity.inner_call_site().node().segments().len(), 3);
    assert_eq!(identity.outer_target().owner(), "Carrier");
    assert_eq!(identity.outer_target().name(), "keep");
}

#[test]
fn equal_looking_foreign_catalog_cannot_brand_the_candidate_inventory() {
    let (primary, inventory) = inventory(ONE_DIRECT, &[]);
    let foreign = catalog(ONE_DIRECT);
    assert!(inventory.is_branded_by(primary.as_ref()));
    assert!(!inventory.is_branded_by(foreign.as_ref()));
}

#[test]
fn declaration_reorder_preserves_normalized_candidate_identity() {
    let reordered = r#"
box Caller {
  inner(value) { return 1 }
  run(text, pos) { pos = Carrier.keep(text, me.inner(pos)) }
}
static box Carrier {
  keep(left, right) { return right }
}
"#;
    assert_eq!(identities(ONE_DIRECT), identities(reordered));
}

#[test]
fn many_candidate_reorder_preserves_the_full_identity_vector() {
    let reordered = r#"
box Caller {
  second(text, pos) { pos = Carrier.keep(text, me.inner(pos)) }
  inner(value) { return 1 }
  first(text, pos) { pos = Carrier.keep(text, me.inner(pos)) }
}
static box Carrier {
  keep(left, right) { return right }
}
"#;
    let original = identities(MANY);
    let reordered = identities(reordered);
    assert_eq!(original.len(), 2);
    assert_eq!(original, reordered);
}
