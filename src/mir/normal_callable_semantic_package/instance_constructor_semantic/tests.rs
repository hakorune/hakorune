use super::*;

#[test]
fn definition_transfer_rejects_foreign_context_and_missing_empty_payload() {
    use super::super::NormalCallableSemanticPackageInstallIssueV1;
    use crate::mir::builder::CompilationContext;
    let issue =
        || super::super::brand_catalog_tests::issue_with_brand_catalog("box Empty {}").unwrap();
    let mut own_context = CompilationContext::new();
    let own = issue().prepare_install(&mut own_context).unwrap().commit();
    let mut foreign_context = CompilationContext::new();
    let _foreign = issue()
        .prepare_install(&mut foreign_context)
        .unwrap()
        .commit();
    let mut port = own.begin_lowering(&own_context).unwrap();
    assert!(port
        .take_object_definitions(&foreign_context)
        .unwrap_err()
        .contains("object-definitions/foreign-package"));
    assert!(own.instance_constructors().has_pending_object_definitions());
    let payload = port.take_object_definitions(&own_context).unwrap();
    assert_eq!(payload.len(), 1);
    assert!(payload[0].fields().is_empty());
    assert!(port.take_object_definitions(&own_context).is_err());
    port.complete().unwrap();

    let mut pending_context = CompilationContext::new();
    let pending = issue()
        .prepare_install(&mut pending_context)
        .unwrap()
        .commit();
    assert_eq!(
        pending.begin_lowering(&pending_context).unwrap().complete(),
        Err(NormalCallableSemanticPackageInstallIssueV1::ObjectDefinitionsNotConsumed)
    );
}

#[test]
fn object_identity_covers_distinct_boxes_and_rejects_foreign_same_index() {
    let text = "box First { value: i64\nbirth(x) { me.value = x } }\nbox Second { value: i64\nbirth(x) { me.value = x } }\nbox Empty {}";
    let own = super::super::brand_catalog_tests::issue_with_brand_catalog(text).unwrap();
    let foreign = super::super::brand_catalog_tests::issue_with_brand_catalog(text).unwrap();
    let batch = &own.instance_constructors;
    let mut ids = std::collections::BTreeSet::new();
    for (index, name) in ["First", "Second", "Empty"].into_iter().enumerate() {
        let source = batch.box_sources.row_for(name).unwrap().unwrap();
        let id = batch.object_for(source).unwrap();
        assert_eq!(id.declaration_index() as usize, index);
        assert!(ids.insert(id));
        let arity = if name == "Empty" { 0 } else { 1 };
        let plan = batch
            .construction_for(source, arity)
            .unwrap()
            .as_ref()
            .unwrap();
        assert_eq!(plan.object(), id);
        assert!(plan.stores().iter().all(|(_, field)| field.object() == id));
        let foreign_source = foreign
            .instance_constructors
            .box_sources
            .row_for(name)
            .unwrap()
            .unwrap();
        assert_eq!(
            foreign
                .instance_constructors
                .object_for(foreign_source)
                .unwrap(),
            id,
            "the raw number is module-local, not a membership proof"
        );
        assert_eq!(
            batch.object_for(foreign_source),
            Err(InstanceConstructorBirthLookupErrorV1::ParentSourceMismatch)
        );
        assert_eq!(
            batch.with_source_object_definition(foreign_source, |_, _| ()),
            Err(InstanceConstructorBirthLookupErrorV1::ParentSourceMismatch)
        );
    }
    assert_eq!(ids.len(), 3);
    let definitions = batch.take_object_definitions().unwrap();
    assert_eq!(definitions.len(), 3);
    assert_eq!(definitions[0].diagnostic_name(), "First");
    assert_eq!(
        definitions[1].fields()[0].declared_type_name.as_deref(),
        Some("i64")
    );
    assert!(definitions[2].fields().is_empty());
    assert!(batch.take_object_definitions().is_none());
    let source = batch.box_sources.row_for("First").unwrap().unwrap();
    assert_eq!(
        batch.with_source_object_definition(source, |_, _| ()),
        Err(InstanceConstructorBirthLookupErrorV1::ObjectDefinitionsTransferred)
    );
    assert_eq!(
        batch.object_sources.len(),
        3,
        "taking payload preserves exact claim linkage"
    );
}

#[test]
fn construction_plan_retains_declaration_order_and_source_store_cutpoints() {
    for (fields, body, expected) in [
        (
            "left: i64\nright: i64",
            "me.left = value\nme.right = 2",
            vec![0, 1],
        ),
        (
            "east: i64\nwest: i64",
            "me.west = 2\nme.east = value",
            vec![1, 0],
        ),
    ] {
        let source = format!("box Page {{ {fields}\nbirth(value) {{ {body} }} }}");
        let package = super::super::brand_catalog_tests::issue_with_brand_catalog(&source).unwrap();
        let batch = &package.instance_constructors;
        let parent = batch.box_sources.row_for("Page").unwrap().unwrap();
        let plan = batch.construction_for(parent, 1).unwrap().as_ref().unwrap();
        assert_eq!(
            plan.field_demands(),
            &[crate::mir::resolved_semantics::HomeDemandV1::Trivial; 2]
        );
        assert_eq!(
            plan.stores()
                .iter()
                .map(|(_, field)| field.declaration_ordinal())
                .collect::<Vec<_>>(),
            expected
        );
        assert_eq!(plan.object(), batch.object_for(parent).unwrap());
        assert!(plan
            .stores()
            .iter()
            .all(|(_, field)| field.object() == plan.object()));
        assert_ne!(
            plan.stores()[0].0.statement_site(),
            plan.stores()[1].0.statement_site()
        );
        assert!(plan.reclaims_unpublished_outer_storage());
        let row = batch.birth_for(parent, 1).unwrap().unwrap();
        assert_eq!(
            plan.constructor(),
            Some(&(row.source_id().clone(), row.forest.roots()[0]))
        );
    }
    let package =
        super::super::brand_catalog_tests::issue_with_brand_catalog("box Empty {}").unwrap();
    let batch = &package.instance_constructors;
    let parent = batch.box_sources.row_for("Empty").unwrap().unwrap();
    let plan = batch.construction_for(parent, 0).unwrap().as_ref().unwrap();
    assert!(plan.field_demands().is_empty() && plan.stores().is_empty());
    assert!(plan.constructor().is_none());
    assert!(
        plan.reclaims_unpublished_outer_storage(),
        "no fields is not no construction cleanup"
    );
}

#[test]
fn construction_plan_keeps_unavailable_dependencies_out_of_empty_cleanup() {
    use super::super::instance_construction::ConstructionUnavailableV1 as U;
    let mut failures = Vec::new();
    for (source, expected) in [
        ("box Page { value: i64 }", U::InitializationContractMissing),
        (
            "box Page { value: i64\nbirth() {} }",
            U::InitializationContractMissing,
        ),
        (
            "box Page { value: i64\nbirth() { return } }",
            U::InitializationContractMissing,
        ),
        (
            "box Page { value\nbirth() { me.value = 1 } }",
            U::FieldContractUnsupported,
        ),
        (
            "box Page { value: i64 = 1\nbirth() {} }",
            U::FieldContractUnsupported,
        ),
        ("box Page { value: i64 = 1 }", U::FieldContractUnsupported),
        (
            "box Page { value: i64\nbirth() { me.value = [1] } }",
            U::BodyCoverageUnsupported,
        ),
        (
            "box Page { value: i64\nbirth(other) { other.value = 1 } }",
            U::BodyCoverageUnsupported,
        ),
        (
            "box Page { value: i64\nbirth() { me.value = new Page() } }",
            U::BodyCoverageUnsupported,
        ),
        (
            "box Page { value: i64\nbirth() { me.value = fn() { return 1 } } }",
            U::BodyCoverageUnsupported,
        ),
        (
            "box Page { value: i64\nbirth() { me.value = 1 + 2 } }",
            U::BodyCoverageUnsupported,
        ),
        (
            "box Page { value: i64\nbirth() { me.value += 1 } }",
            U::BodyCoverageUnsupported,
        ),
        (
            "box Page { value: i64\nbirth() { me.value = 1\nme.value = 2 } }",
            U::BodyCoverageUnsupported,
        ),
        (
            "box Page { value: i64\nbirth() { local x = 1\nme.value = x } }",
            U::BodyCoverageUnsupported,
        ),
        (
            "box Page { value: i64\nbirth() { me.other = 1 } }",
            U::SourceRelationMissing,
        ),
    ] {
        let package = match super::super::brand_catalog_tests::issue_with_brand_catalog(source) {
            Ok(package) => package,
            Err(error) => {
                failures.push(format!("source {source}: {error:?}"));
                continue;
            }
        };
        let batch = &package.instance_constructors;
        let parent = batch.box_sources.row_for("Page").unwrap().unwrap();
        let arity = batch
            .rows
            .iter()
            .find(|row| row.kind() == ConstructorSourceKindV1::Birth)
            .map_or(0, |row| row.source_arity() as usize);
        let actual = batch.construction_for(parent, arity).unwrap();
        if actual != &Err(expected) {
            failures.push(format!("{source}: expected {expected:?}, got {actual:?}"));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn construction_plan_rejects_foreign_parent_and_birth_arity_not_as_no_birth() {
    let source = "box Page { value: i64\nbirth(value) { me.value = value } }";
    let own = super::super::brand_catalog_tests::issue_with_brand_catalog(source).unwrap();
    let foreign = super::super::brand_catalog_tests::issue_with_brand_catalog(source).unwrap();
    let batch = &own.instance_constructors;
    let foreign_parent = foreign
        .instance_constructors
        .box_sources
        .row_for("Page")
        .unwrap()
        .unwrap();
    assert_eq!(
        batch.construction_for(foreign_parent, 1),
        Err(InstanceConstructorBirthLookupErrorV1::ParentSourceMismatch)
    );
    let parent = batch.box_sources.row_for("Page").unwrap().unwrap();
    assert_eq!(
        batch.construction_for(parent, 0),
        Err(InstanceConstructorBirthLookupErrorV1::BirthArityMismatch)
    );
}

#[test]
fn constructor_lookup_rejects_foreign_or_mismatched_parent_not_as_no_birth() {
    for source in ["box Page { birth() {} }", "box Page {}"] {
        let own = super::super::brand_catalog_tests::issue_with_brand_catalog(source).unwrap();
        let foreign = super::super::brand_catalog_tests::issue_with_brand_catalog(source).unwrap();
        let row = foreign
            .batch()
            .ordinary_box_coverage()
            .row_for("Page")
            .unwrap()
            .unwrap();
        assert!(matches!(
            own.instance_constructors.birth_for(row, 0),
            Err(InstanceConstructorBirthLookupErrorV1::ParentSourceMismatch)
        ));
    }
    let mut package = super::super::brand_catalog_tests::issue_with_brand_catalog(
        "box Page { birth() {} } box Other { birth() {} }",
    )
    .unwrap();
    let batch = &mut package.instance_constructors;
    let page = batch.box_sources.row_for("Page").unwrap().unwrap().clone();
    let other = batch.box_sources.row_for("Other").unwrap().unwrap().clone();
    batch.rows[0].box_source = other;
    assert!(matches!(
        batch.birth_for(&page, 0),
        Err(InstanceConstructorBirthLookupErrorV1::ParentSourceMismatch)
    ));
}

#[test]
fn ordinary_new_constructor_lookup_reports_missing_nonzero_birth() {
    let package =
        super::super::brand_catalog_tests::issue_with_brand_catalog("box Page {}").unwrap();
    let batch = &package.instance_constructors;
    let parent = batch.box_sources.row_for("Page").unwrap().unwrap();
    assert!(matches!(batch.birth_for(parent, 1), Ok(None)));
}

#[test]
fn ordinary_new_constructor_lookup_rejects_source_arity_overflow() {
    let package =
        super::super::brand_catalog_tests::issue_with_brand_catalog("box Page {}").unwrap();
    let batch = &package.instance_constructors;
    let parent = batch.box_sources.row_for("Page").unwrap().unwrap();
    assert!(matches!(
        batch.birth_for(parent, usize::MAX),
        Err(InstanceConstructorBirthLookupErrorV1::SourceArityOverflow)
    ));
}

#[test]
fn constructor_loan_rejects_lost_shape_and_completion() {
    for fault in [
        "missing-shape",
        "foreign-shape",
        "missing-completion",
        "foreign-completion",
    ] {
        let mut package = super::super::brand_catalog_tests::issue_with_brand_catalog(
            "box Page { birth(value) { local saved = value } }
                 box Other { birth(value) { return 1 } }",
        )
        .unwrap();
        let (first, rest) = package.instance_constructors.rows.split_at_mut(1);
        let row = &mut first[0];
        let foreign = &mut rest[0];
        assert!(!row.birth_completion().unwrap().returns_value());
        assert!(foreign.birth_completion().unwrap().returns_value());
        let expected = match fault {
            "missing-shape" => {
                row.body_shapes.clear();
                "body-shape"
            }
            "foreign-shape" => {
                let shape = foreign
                    .body_shapes
                    .remove(&foreign.forest.roots()[0])
                    .unwrap();
                row.body_shapes.insert(row.forest.roots()[0], shape);
                "body-shape-owner"
            }
            "missing-completion" => {
                row.birth_completion = None;
                "completion-owner"
            }
            "foreign-completion" => {
                row.birth_completion = foreign.birth_completion.take();
                "completion-owner"
            }
            _ => unreachable!(),
        };
        package
            .with_normal_program_source_loan(|loan| {
                let error = package.instance_constructors().rows()[0]
                    .lowering_input(loan.program())
                    .unwrap_err();
                assert_eq!(
                    error,
                    format!("[freeze:contract][mir/instance-constructor-semantic/{expected}]")
                );
            })
            .unwrap();
    }
}
