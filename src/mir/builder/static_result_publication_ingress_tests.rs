use super::*;
use crate::mir::builder::{CanonicalSameModuleCallableKeyV1, RawSourceLocatorV1};
use crate::mir::resolved_semantics::{SourceNodeSiteV1, SourcePathV1};

fn site() -> SourceNodeSiteV1 {
    SourcePathV1::function_body().node()
}

fn caller() -> CanonicalSameModuleCallableKeyV1 {
    CanonicalSameModuleCallableKeyV1::test_static_box_method("Caller", "run", 0)
}

fn instance_caller() -> CanonicalSameModuleCallableKeyV1 {
    CanonicalSameModuleCallableKeyV1::test_instance_box_method("Caller", "run", 0)
}

#[test]
fn state_vocabulary_keeps_unavailable_and_no_exact_target_distinct() {
    assert_ne!(
        StaticResultPublicationIngressV1::Unavailable,
        StaticResultPublicationIngressV1::NoExactStaticTarget
    );
}

#[test]
fn source_loss_has_a_typed_error() {
    assert_eq!(
        StaticResultPublicationIngressErrorV1::SourceLocationLost.to_string(),
        "[freeze:contract][static-result-ingress/source-location-lost]"
    );
}

#[test]
fn source_classification_enumerates_all_ingress_boundaries() {
    let absent = None;
    assert_eq!(
        classify_source_context_v1(None, false, absent, "Owner", "m", 0).unwrap(),
        StaticResultPublicationSourceClassV1::Unavailable
    );
    let compatibility = RawInvocationSourceContextV1::UnlocatedCompatibility {
        reason: super::super::raw_invocation_source_transport::RawUnlocatedPortalV1::CallObject,
        expected_lineage: None,
    };
    assert_eq!(
        classify_source_context_v1(Some(&compatibility), false, absent, "Owner", "m", 0).unwrap(),
        StaticResultPublicationSourceClassV1::Unavailable
    );
    let lost = RawInvocationSourceContextV1::UnlocatedCompatibility {
        reason: super::super::raw_invocation_source_transport::RawUnlocatedPortalV1::CallObject,
        expected_lineage: Some(RawInvocationRootLineageV1::Cataloged(caller())),
    };
    assert_eq!(
        classify_source_context_v1(Some(&lost), true, absent, "Owner", "m", 0),
        Err(StaticResultPublicationIngressErrorV1::SourceLocationLost)
    );
    let located = RawInvocationSourceContextV1::Located {
        root: RawInvocationRootLineageV1::Cataloged(caller()),
        site: site(),
        body_kind: None,
    };
    assert!(matches!(
        classify_source_context_v1(Some(&located), true, absent, "Owner", "m", 0).unwrap(),
        StaticResultPublicationSourceClassV1::Cataloged { .. }
    ));
    let foreign = RawInvocationSourceContextV1::Located {
        root: RawInvocationRootLineageV1::ScriptRoot,
        site: site(),
        body_kind: None,
    };
    assert_eq!(
        classify_source_context_v1(Some(&foreign), false, absent, "Owner", "m", 0).unwrap(),
        StaticResultPublicationSourceClassV1::Unavailable
    );
    assert_eq!(
        classify_source_context_v1(None, true, absent, "Owner", "m", 0),
        Err(StaticResultPublicationIngressErrorV1::SourceContextMissing)
    );
    assert_eq!(
        classify_source_context_v1(Some(&foreign), true, absent, "Owner", "m", 0),
        Err(StaticResultPublicationIngressErrorV1::ForeignLineage)
    );
}

fn main_declarations() -> VerifiedSameModuleCallableDeclarationCatalogV1 {
    let root = crate::parser::NyashParser::parse_from_string(
        "static box Main { main(args) { return 0 } sample() { return \"x\" } }",
    )
    .expect("fixture program must parse");
    VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root)
        .expect("fixture declaration catalog must seal")
}

fn main_located(method: &str, arity: usize) -> RawInvocationSourceContextV1 {
    RawInvocationSourceContextV1::Located {
        root: RawInvocationRootLineageV1::Main(RawSourceLocatorV1::for_test(
            0,
            "Main",
            method,
            "ignored.symbol/0",
            arity,
        )),
        site: site(),
        body_kind: None,
    }
}

#[test]
fn main_lineage_resolves_caller_through_sealed_declaration() {
    let declarations = main_declarations();
    for (method, arity) in [("main", 1usize), ("sample", 0)] {
        let located = main_located(method, arity);
        let class = classify_source_context_v1(
            Some(&located),
            true,
            Some(&declarations),
            "LayoutBox",
            "class_size",
            0,
        )
        .unwrap();
        let StaticResultPublicationSourceClassV1::Cataloged { caller, .. } = class else {
            panic!("Main root must classify as Cataloged");
        };
        assert_eq!(
            caller,
            CanonicalSameModuleCallableKeyV1::test_static_box_method("Main", method, arity),
            "the caller key must come from the sealed declaration, not the call target"
        );
    }
}

#[test]
fn main_lineage_requires_declaration_catalog() {
    let located = main_located("main", 1);
    assert_eq!(
        classify_source_context_v1(Some(&located), true, None, "LayoutBox", "class_size", 0),
        Err(StaticResultPublicationIngressErrorV1::DeclarationCatalogUnavailable),
        "without the catalog the locator cannot be verified"
    );
}

#[test]
fn main_lineage_rejects_undeclared_static_method() {
    let declarations = main_declarations();
    let located = main_located("ghost", 0);
    assert_eq!(
        classify_source_context_v1(
            Some(&located),
            true,
            Some(&declarations),
            "LayoutBox",
            "class_size",
            0,
        ),
        Err(StaticResultPublicationIngressErrorV1::ForeignLineage),
        "a locator that names no declared static method stays foreign"
    );
}

#[test]
fn main_lineage_stays_unavailable_without_ledger() {
    let located = main_located("main", 1);
    assert_eq!(
        classify_source_context_v1(Some(&located), false, None, "LayoutBox", "class_size", 0)
            .unwrap(),
        StaticResultPublicationSourceClassV1::Unavailable,
        "the ledger-less raw-root producer never reaches the catalog probe"
    );
}

fn json_line_declarations() -> VerifiedSameModuleCallableDeclarationCatalogV1 {
    let root = crate::parser::NyashParser::parse_from_string(
        "static box JsonLine { stringField(a, b) { return \"x\" } }",
    )
    .expect("fixture program must parse");
    VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root)
        .expect("fixture declaration catalog must seal")
}

fn instance_caller_source() -> RawInvocationSourceContextV1 {
    RawInvocationSourceContextV1::Located {
        root: RawInvocationRootLineageV1::Cataloged(instance_caller()),
        site: site(),
        body_kind: None,
    }
}

#[test]
fn instance_caller_is_admitted_for_declaration_resolved_static_target() {
    let declarations = json_line_declarations();
    let located = instance_caller_source();

    assert!(
            matches!(
                classify_source_context_v1(
                    Some(&located),
                    true,
                    Some(&declarations),
                    "JsonLine",
                    "stringField",
                    2,
                )
                .unwrap(),
                StaticResultPublicationSourceClassV1::Cataloged { .. }
            ),
            "a qualified same-module static call inside an instance method must reach the publication owner"
        );
}

#[test]
fn instance_caller_declines_non_declaration_targets() {
    let declarations = json_line_declarations();
    let located = instance_caller_source();

    assert_eq!(
        classify_source_context_v1(
            Some(&located),
            true,
            Some(&declarations),
            "Math",
            "floor",
            1,
        )
        .unwrap(),
        StaticResultPublicationSourceClassV1::Unavailable,
        "builtin owners stay on the compatibility lane"
    );
    assert_eq!(
        classify_source_context_v1(
            Some(&located),
            true,
            Some(&declarations),
            "JsonLine",
            "undeclared",
            0,
        )
        .unwrap(),
        StaticResultPublicationSourceClassV1::Unavailable,
        "undeclared same-name methods keep the retired-fallback terminal"
    );
}

#[test]
fn me_call_probe_keeps_instance_callers_outside_static_ingress() {
    let declarations = json_line_declarations();
    let located = instance_caller_source();

    assert_eq!(
            classify_source_context_v1(
                Some(&located),
                true,
                Some(&declarations),
                "<source-owned>",
                "stringField",
                2,
            )
            .unwrap(),
            StaticResultPublicationSourceClassV1::Unavailable,
            "the me.method probe never resolves <source-owned>, so the DeclaredInstance sibling keeps the site"
        );
}

#[test]
fn instance_caller_declines_when_declarations_are_unavailable() {
    let located = instance_caller_source();

    assert_eq!(
        classify_source_context_v1(Some(&located), true, None, "JsonLine", "stringField", 2,)
            .unwrap(),
        StaticResultPublicationSourceClassV1::Unavailable,
        "an unresolvable target stays outside this ingress rather than guessing"
    );
}

fn top_level_declarations() -> VerifiedSameModuleCallableDeclarationCatalogV1 {
    let root = crate::parser::NyashParser::parse_from_string(
        "function helper() { return JsonLine.stringField(\"a\", \"b\") }\n\
             static box JsonLine { stringField(a, b) { return \"x\" } }",
    )
    .expect("fixture program must parse");
    VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root)
        .expect("fixture declaration catalog must seal")
}

fn top_level_located(name: &str, arity: usize) -> RawInvocationSourceContextV1 {
    RawInvocationSourceContextV1::Located {
        root: RawInvocationRootLineageV1::TopLevel(
            super::super::callable_declaration_catalog::SelectedTopLevelFunctionKeyV1::new(
                0, name, arity,
            ),
        ),
        site: site(),
        body_kind: None,
    }
}

#[test]
fn top_level_lineage_admits_declared_caller_for_static_target() {
    let declarations = top_level_declarations();
    let located = top_level_located("helper", 0);

    let class = classify_source_context_v1(
        Some(&located),
        true,
        Some(&declarations),
        "JsonLine",
        "stringField",
        2,
    )
    .unwrap();
    let StaticResultPublicationSourceClassV1::Cataloged { caller, .. } = class else {
        panic!("a declared TopLevel caller with a static target must classify Cataloged");
    };
    assert_eq!(
        caller,
        CanonicalSameModuleCallableKeyV1::free_function("helper", 0),
        "the caller key must come from the sealed declaration row"
    );
}

#[test]
fn top_level_lineage_requires_declaration_catalog() {
    let located = top_level_located("helper", 0);
    assert_eq!(
        classify_source_context_v1(Some(&located), true, None, "JsonLine", "stringField", 2,),
        Err(StaticResultPublicationIngressErrorV1::DeclarationCatalogUnavailable),
        "without the catalog the top-level caller cannot be verified"
    );
}

#[test]
fn top_level_lineage_rejects_unrowed_caller() {
    let declarations = top_level_declarations();
    let located = top_level_located("ghost", 0);
    assert_eq!(
        classify_source_context_v1(
            Some(&located),
            true,
            Some(&declarations),
            "JsonLine",
            "stringField",
            2,
        ),
        Err(StaticResultPublicationIngressErrorV1::ForeignLineage),
        "a top-level caller with no sealed declaration row stays foreign"
    );
}

#[test]
fn top_level_lineage_declines_non_declaration_targets() {
    let declarations = top_level_declarations();
    let located = top_level_located("helper", 0);
    assert_eq!(
        classify_source_context_v1(
            Some(&located),
            true,
            Some(&declarations),
            "Math",
            "floor",
            1,
        )
        .unwrap(),
        StaticResultPublicationSourceClassV1::Unavailable,
        "a resolved caller still keeps non-declaration targets on sibling lanes"
    );
}

#[test]
fn top_level_lineage_stays_unavailable_without_ledger() {
    let located = top_level_located("helper", 0);
    assert_eq!(
        classify_source_context_v1(Some(&located), false, None, "JsonLine", "stringField", 2,)
            .unwrap(),
        StaticResultPublicationSourceClassV1::Unavailable
    );
}

fn birth_declarations() -> VerifiedSameModuleCallableDeclarationCatalogV1 {
    let root = crate::parser::NyashParser::parse_from_string(
        "box Holder { birth() { return me } }\n\
             static box LayoutBox { class_size(i) { return i } }",
    )
    .expect("fixture program must parse");
    VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root)
        .expect("fixture declaration catalog must seal")
}

fn constructor_located(
    published: Option<CanonicalSameModuleCallableKeyV1>,
) -> RawInvocationSourceContextV1 {
    RawInvocationSourceContextV1::Located {
            root: RawInvocationRootLineageV1::InstanceConstructor(
                super::super::normal_instance_constructor_admission::NormalInstanceConstructorSourceKeyV1::from_physical_source(
                    crate::parser::ConstructorSourceIdV1::test_new(0),
                    published,
                    0,
                    "Holder",
                    "birth/0",
                ),
            ),
            site: site(),
            body_kind: None,
        }
}

#[test]
fn instance_constructor_admits_declared_birth_caller_for_static_target() {
    let declarations = birth_declarations();
    let located = constructor_located(Some(CanonicalSameModuleCallableKeyV1::birth_constructor(
        "Holder", 0,
    )));

    let class = classify_source_context_v1(
        Some(&located),
        true,
        Some(&declarations),
        "LayoutBox",
        "class_size",
        1,
    )
    .unwrap();
    let StaticResultPublicationSourceClassV1::Cataloged { caller, .. } = class else {
        panic!("a declared birth caller with a static target must classify Cataloged");
    };
    assert_eq!(
        caller,
        CanonicalSameModuleCallableKeyV1::birth_constructor("Holder", 0),
        "the caller key must come from the sealed declaration row"
    );
}

#[test]
fn instance_constructor_requires_declaration_catalog() {
    let located = constructor_located(Some(CanonicalSameModuleCallableKeyV1::birth_constructor(
        "Holder", 0,
    )));
    assert_eq!(
        classify_source_context_v1(Some(&located), true, None, "LayoutBox", "class_size", 1,),
        Err(StaticResultPublicationIngressErrorV1::DeclarationCatalogUnavailable),
    );
}

#[test]
fn instance_constructor_without_birth_key_stays_foreign() {
    let declarations = birth_declarations();
    let located = constructor_located(None);
    assert_eq!(
        classify_source_context_v1(
            Some(&located),
            true,
            Some(&declarations),
            "LayoutBox",
            "class_size",
            1,
        ),
        Err(StaticResultPublicationIngressErrorV1::ForeignLineage),
        "init/pack constructors carry no canonical caller key"
    );
}

#[test]
fn instance_constructor_rejects_unrowed_birth_caller() {
    let declarations = birth_declarations();
    let located = constructor_located(Some(CanonicalSameModuleCallableKeyV1::birth_constructor(
        "Ghost", 0,
    )));
    assert_eq!(
        classify_source_context_v1(
            Some(&located),
            true,
            Some(&declarations),
            "LayoutBox",
            "class_size",
            1,
        ),
        Err(StaticResultPublicationIngressErrorV1::ForeignLineage),
        "a published birth key with no sealed declaration row stays foreign"
    );
}

#[test]
fn instance_constructor_declines_non_declaration_targets() {
    let declarations = birth_declarations();
    let located = constructor_located(Some(CanonicalSameModuleCallableKeyV1::birth_constructor(
        "Holder", 0,
    )));
    assert_eq!(
        classify_source_context_v1(
            Some(&located),
            true,
            Some(&declarations),
            "Math",
            "floor",
            1,
        )
        .unwrap(),
        StaticResultPublicationSourceClassV1::Unavailable,
    );
}

#[test]
fn instance_constructor_stays_unavailable_without_ledger() {
    let located = constructor_located(Some(CanonicalSameModuleCallableKeyV1::birth_constructor(
        "Holder", 0,
    )));
    assert_eq!(
        classify_source_context_v1(Some(&located), false, None, "LayoutBox", "class_size", 1,)
            .unwrap(),
        StaticResultPublicationSourceClassV1::Unavailable
    );
}
