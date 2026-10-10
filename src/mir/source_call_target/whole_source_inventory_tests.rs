use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, NormalRootExecutionConsumerV1,
    VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::resolved_semantics::{ShadowMethodCallReceiverV0, SourceExprSiteV1};
use crate::parser::{NyashParser, ParserBuildConfig};

use super::{
    CurrentOwnerStaticCallTargetErrorV1, StaticImportAliasViewErrorV1,
    VerifiedStaticImportAliasViewV1, VerifiedWholeSourceStaticCallTargetInventoryV1,
    WholeSourceStaticCallTargetInventoryErrorV1,
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

const LEXICALLY_BOUND_ALIAS: &str = r#"
static box Carrier {
  keep(left, right) { return right }
}
box Caller {
  inner(value) { return 1 }
  run(carrier_alias, text, pos) {
    pos = carrier_alias.keep(text, me.inner(pos))
  }
}
"#;

const UNAVAILABLE_THEN_EXACT: &str = r#"
static box AUnsupported {
  bad() { return me }
}
static box Carrier {
  keep(left, right) { return right }
}
box Caller {
  inner(value) { return 1 }
  run(text, pos) { pos = Carrier.keep(text, me.inner(pos)) }
}
"#;

fn catalog(source: &str) -> VerifiedSameModuleCallableDeclarationCatalogV1 {
    let ast = NyashParser::parse_from_string(source).expect("whole-source inventory fixture");
    VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&ast)
        .expect("whole-source declaration catalog")
}

fn app_catalog(source: &str) -> VerifiedSameModuleCallableDeclarationCatalogV1 {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
    .expect("closed App source");
    let transformed = crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        crate::r#macro::transform_normal_callable_program_v1(parsed)
            .expect("exact App callable transform")
    });
    let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed else {
        panic!("closed App must remain source-backed")
    };
    let source = NormalRootExecutionConsumerV1::consume_once(source)
        .expect("App root execution")
        .into_consumed_source();
    crate::mir::builder::issue_source_backed_same_module_callable_catalog_v1(&source)
        .expect("source-backed App catalog")
        .into_catalog()
}

fn app_omissions(source: &str) -> std::collections::BTreeSet<CanonicalSameModuleCallableKeyV1> {
    let declarations = app_catalog(source);
    let imports = VerifiedStaticImportAliasViewV1::seal(
        &declarations,
        std::iter::empty::<(String, String)>(),
    )
    .expect("closed App import view");
    let inventory = VerifiedWholeSourceStaticCallTargetInventoryV1::verify(&declarations, &imports)
        .expect("closed App MethodCall inventory");
    inventory.closed_app_zero_incoming_static_keys(
        declarations
            .source_backed_app_main()
            .expect("App Main co-seal"),
    )
}

#[test]
fn closed_app_static_omission_distinguishes_exact_other_from_uncertain_incoming() {
    let target = CanonicalSameModuleCallableKeyV1::static_box_method("LayoutBox", "accepts", 1);
    let exact_other = "static box LayoutBox { accepts(size) { return 1 } }
        static box Other { accepts(size) { return 1 } use(size) { return me.accepts(size) } }
        static box Main { main() { return 0 } }";
    assert!(app_omissions(exact_other).contains(&target));

    let exact_incoming = "static box LayoutBox { accepts(size) { return 1 } }
        static box Main { main() { local x = LayoutBox.accepts(1) return 0 } }";
    assert!(!app_omissions(exact_incoming).contains(&target));

    let uncertain = "static box LayoutBox { accepts(size) { return 1 } }
        box Caller { use(x) { return x.accepts(1) } }
        static box Main { main() { return 0 } }";
    assert!(!app_omissions(uncertain).contains(&target));

    let observation_gap = "static box LayoutBox { accepts(size) { return 1 } }
        static box AUnsupported { bad() { return me } }
        static box Main { main() { return 0 } }";
    assert!(app_omissions(observation_gap).is_empty());

    let public_method = "static box LayoutBox {
            @rune Public
            accepts(size) { return 1 }
        }
        static box Main { main() { return 0 } }";
    assert!(!app_omissions(public_method).contains(&target));
}

#[test]
fn closed_app_static_omission_rejects_foreign_app_brand() {
    let source = "static box LayoutBox { accepts(size) { return 1 } }
        static box Main { main() { return 0 } }";
    let declarations = app_catalog(source);
    let foreign = app_catalog(source);
    let imports = VerifiedStaticImportAliasViewV1::seal(
        &declarations,
        std::iter::empty::<(String, String)>(),
    )
    .unwrap();
    let inventory =
        VerifiedWholeSourceStaticCallTargetInventoryV1::verify(&declarations, &imports).unwrap();
    assert!(inventory
        .closed_app_zero_incoming_static_keys(foreign.source_backed_app_main().unwrap())
        .is_empty());
}

fn inventory_counts(source: &str, aliases: &[(&str, &str)]) -> (usize, usize, usize, usize) {
    let declarations = catalog(source);
    let imports = VerifiedStaticImportAliasViewV1::seal(
        &declarations,
        aliases
            .iter()
            .map(|(alias, owner)| ((*alias).to_string(), (*owner).to_string())),
    )
    .expect("valid alias view");
    let inventory = VerifiedWholeSourceStaticCallTargetInventoryV1::verify(&declarations, &imports)
        .expect("complete whole-source inventory");
    assert!(inventory.is_branded_by(&declarations));
    (
        inventory.observed_declaration_count(),
        inventory.len(),
        inventory.target_len(),
        inventory.noncandidate_len(),
    )
}

fn normalized_inventory(
    source: &str,
) -> (
    Vec<crate::mir::builder::CanonicalSameModuleCallableKeyV1>,
    Vec<(
        crate::mir::builder::CanonicalSameModuleCallableKeyV1,
        SourceExprSiteV1,
        ShadowMethodCallReceiverV0,
        Option<crate::mir::builder::CanonicalSameModuleCallableKeyV1>,
    )>,
) {
    let declarations = catalog(source);
    let imports = VerifiedStaticImportAliasViewV1::seal(
        &declarations,
        std::iter::empty::<(String, String)>(),
    )
    .expect("empty alias view");
    let inventory = VerifiedWholeSourceStaticCallTargetInventoryV1::verify(&declarations, &imports)
        .expect("complete whole-source inventory");
    let callers = inventory.observed_callers().cloned().collect::<Vec<_>>();
    let rows = inventory
        .calls()
        .map(|row| {
            let call = row.call();
            (
                call.caller().clone(),
                call.site().clone(),
                row.receiver(),
                inventory
                    .target(call.caller(), call.site())
                    .map(|target| target.target().clone()),
            )
        })
        .collect::<Vec<_>>();
    (callers, rows)
}

#[test]
fn complete_inventory_distinguishes_targets_from_observed_noncandidates() {
    assert_eq!(inventory_counts(ZERO, &[]), (1, 0, 0, 0));
    assert_eq!(inventory_counts(ONE_DIRECT, &[]), (3, 2, 1, 1));
    assert_eq!(inventory_counts(ONE_ALIAS, &[]), (3, 2, 0, 2));
    assert_eq!(
        inventory_counts(ONE_ALIAS, &[("Alias", "Carrier")]),
        (3, 2, 1, 1)
    );
    assert_eq!(inventory_counts(MANY, &[]), (4, 4, 2, 2));
}

#[test]
fn invalid_alias_authority_rejects_before_inventory() {
    let declarations = catalog(ONE_ALIAS);
    assert!(matches!(
        VerifiedStaticImportAliasViewV1::seal(
            &declarations,
            [("Alias".to_string(), "Missing".to_string())],
        ),
        Err(StaticImportAliasViewErrorV1::TargetOwnerOutsideCatalog { .. })
    ));
}

#[test]
fn supplied_alias_precedes_equal_named_lexical_binding() {
    assert_eq!(
        inventory_counts(LEXICALLY_BOUND_ALIAS, &[("carrier_alias", "Carrier")]),
        (3, 2, 1, 1)
    );
}

#[test]
fn equal_looking_foreign_alias_view_rejects() {
    let left = catalog(ONE_ALIAS);
    let right = catalog(ONE_ALIAS);
    let imports = VerifiedStaticImportAliasViewV1::seal(
        &right,
        [("Alias".to_string(), "Carrier".to_string())],
    )
    .expect("foreign alias view");
    assert!(matches!(
        VerifiedWholeSourceStaticCallTargetInventoryV1::verify(&left, &imports),
        Err(WholeSourceStaticCallTargetInventoryErrorV1::ImportCatalogMismatch)
    ));
}

#[test]
fn declaration_reorder_preserves_normalized_inventory() {
    let reordered = r#"
box Caller {
  inner(value) { return 1 }
  run(text, pos) { pos = Carrier.keep(text, me.inner(pos)) }
}
static box Carrier {
  keep(left, right) { return right }
}
"#;
    assert_eq!(
        normalized_inventory(ONE_DIRECT),
        normalized_inventory(reordered)
    );
}

#[test]
fn bounded_observation_unavailability_does_not_hide_a_later_exact_target() {
    let declarations = catalog(UNAVAILABLE_THEN_EXACT);
    let imports = VerifiedStaticImportAliasViewV1::seal(
        &declarations,
        std::iter::empty::<(String, String)>(),
    )
    .expect("empty alias view");
    let inventory = VerifiedWholeSourceStaticCallTargetInventoryV1::verify(&declarations, &imports)
        .expect("bounded observation gaps are retained without aborting inventory");

    let unavailable = inventory
        .first_method_observation_unavailability()
        .expect("one bounded observation gap");
    assert_eq!(unavailable.caller().owner(), "AUnsupported");
    assert!(matches!(
        unavailable.cause(),
        crate::mir::resolved_semantics::ShadowResolveErrorV0::UnsupportedExpression {
            kind: "Me",
            ..
        }
    ));
    assert_eq!(inventory.target_len(), 1);
    assert_eq!(inventory.len(), 2);
}

#[test]
fn missing_static_current_owner_target_is_a_typed_inventory_error() {
    let declarations = catalog("static box Helpers { call(x) { return me.absent(x) } }");
    let imports = VerifiedStaticImportAliasViewV1::seal(
        &declarations,
        std::iter::empty::<(String, String)>(),
    )
    .expect("empty alias view");

    assert!(matches!(
        VerifiedWholeSourceStaticCallTargetInventoryV1::verify(&declarations, &imports),
        Err(
            WholeSourceStaticCallTargetInventoryErrorV1::CurrentOwnerTarget(
                CurrentOwnerStaticCallTargetErrorV1::TargetOutsideCatalog {
                    method,
                    arity: 1,
                    ..
                },
            ),
        ) if &*method == "absent"
    ));
}
