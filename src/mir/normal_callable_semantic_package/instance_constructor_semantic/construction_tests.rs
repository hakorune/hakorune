//! Constructor scalar-read and child-provider regression owner.
use super::*;
use crate::mir::normal_callable_semantic_package::ConstructionStoreRhsV1;

#[test]
fn construction_plan_resolves_me_field_reads_to_prior_scalar_stores() {
    let package = super::super::brand_catalog_tests::issue_with_brand_catalog(
        "box Page { a: i64\nb: i64\nc: i64
        birth() { me.a = 4\nme.b = me.a + 1\nme.c = me.a } }",
    )
    .unwrap();
    let batch = &package.instance_constructors;
    let parent = batch.box_sources.row_for("Page").unwrap().unwrap();
    let plan = batch
        .construction_for(parent, 0)
        .unwrap()
        .as_ref()
        .expect("computed scalar stores seal");
    let rhs: Vec<_> = plan
        .stores()
        .iter()
        .map(|store| store.rhs().clone())
        .collect();
    assert_eq!(
        rhs,
        vec![
            ConstructionStoreRhsV1::LiteralI64(4),
            ConstructionStoreRhsV1::LiteralI64(5),
            ConstructionStoreRhsV1::LiteralI64(4),
        ]
    );
    // Each `me.<field>` read registers its `me` receiver site for
    // consumption; the literal-only store carries none.
    assert!(plan.stores()[0].me_reads().is_empty());
    assert_eq!(plan.stores()[1].me_reads().len(), 1);
    assert_eq!(plan.stores()[2].me_reads().len(), 1);
}

#[test]
fn construction_plan_declines_forward_and_parameter_field_reads() {
    use super::super::instance_construction::ConstructionUnavailableV1 as U;
    for (label, source) in [
        (
            "read-before-store",
            "box Page { a: i64\nb: i64\nbirth() { me.b = me.a + 1\nme.a = 4 } }",
        ),
        (
            "unwritten-field-read",
            "box Page { a: i64\nb: i64\nc: i64\nbirth() { me.b = me.c\nme.a = 4 } }",
        ),
        (
            "parameter-store-read",
            "box Page { a: i64\nb: i64\nbirth(x) { me.a = x\nme.b = me.a } }",
        ),
        (
            "parameter-in-arithmetic",
            "box Page { a: i64\nb: i64\nbirth(x) { me.b = x + 1\nme.a = 4 } }",
        ),
        (
            "object-field-read",
            "box Child { birth() {} }
            box Page { a: i64\ninner: Child\nb: i64
            birth() { me.inner = new Child()\nme.b = me.inner\nme.a = 4 } }",
        ),
    ] {
        let package =
            super::super::brand_catalog_tests::issue_with_brand_catalog(source).unwrap();
        let batch = &package.instance_constructors;
        let name = if source.contains("Page") { "Page" } else { "Child" };
        let parent = batch.box_sources.row_for(name).unwrap().unwrap();
        let arity = if source.contains("birth(x)") { 1 } else { 0 };
        let error = batch
            .construction_for(parent, arity)
            .unwrap()
            .as_ref()
            .expect_err(&format!("{label} stays declined"));
        assert_eq!(
            error,
            &U::BodyCoverageUnsupported,
            "{label} fails closed"
        );
    }
}

#[test]
fn construction_plan_marks_nobirth_provider_child_zero_init() {
    let package = super::super::brand_catalog_tests::issue_with_brand_catalog(
        "box Leaf {}
        box Born { birth() {} }
        box Page { leaf: Leaf\nborn: Born
        birth() { me.leaf = new Leaf()\nme.born = new Born() } }",
    )
    .unwrap();
    let batch = &package.instance_constructors;
    let parent = batch.box_sources.row_for("Page").unwrap().unwrap();
    let plan = batch
        .construction_for(parent, 0)
        .unwrap()
        .as_ref()
        .expect("provider children seal");
    let leaf = plan.stores()[0].rhs();
    let ConstructionStoreRhsV1::ProviderConstruction {
        object:
            Some(crate::mir::normal_callable_semantic_package
                ::ProviderConstructionChildV1::NoBirthZero(child)),
        arguments,
        ..
    } = leaf
    else {
        panic!("a fieldless no-birth `new` seals NoBirthZero: {leaf:?}");
    };
    let leaf_object = batch
        .object_for(batch.box_sources.row_for("Leaf").unwrap().unwrap())
        .unwrap();
    assert_eq!(*child, leaf_object);
    assert!(arguments.is_empty());
    let born = plan.stores()[1].rhs();
    let ConstructionStoreRhsV1::ProviderConstruction {
        object:
            Some(crate::mir::normal_callable_semantic_package
                ::ProviderConstructionChildV1::BirthIndexed(child)),
        ..
    } = born
    else {
        panic!("a `birth` child stays BirthIndexed: {born:?}");
    };
    let born_object = batch
        .object_for(batch.box_sources.row_for("Born").unwrap().unwrap())
        .unwrap();
    assert_eq!(*child, born_object);
}

#[test]
fn construction_plan_seals_parameter_provided_object_child() {
    let package = super::super::brand_catalog_tests::issue_with_brand_catalog(
        "box Node { value: i64\nbirth(v) { me.value = v } }
        box Holder { inner: Node\nbirth(inner) { me.inner = inner } }",
    )
    .unwrap();
    let batch = &package.instance_constructors;
    let parent = batch.box_sources.row_for("Holder").unwrap().unwrap();
    let plan = batch
        .construction_for(parent, 1)
        .unwrap()
        .as_ref()
        .expect("provided object param store seals");
    let [store] = plan.stores() else {
        panic!("one store: {:?}", plan.stores());
    };
    let ConstructionStoreRhsV1::Parameter { provided, .. } = store.rhs() else {
        panic!("param store: {:?}", store.rhs());
    };
    let node_object = batch
        .object_for(batch.box_sources.row_for("Node").unwrap().unwrap())
        .unwrap();
    assert_eq!(*provided, Some(node_object));
    let row = package
        .instance_constructors()
        .rows()
        .iter()
        .find(|row| row.box_name() == "Holder")
        .unwrap();
    let [contract] = row.formal_contracts() else {
        panic!("one formal contract");
    };
    assert_eq!(
        contract.declaration(),
        BirthFormalDeclarationClassV1::Unannotated
    );
    assert!(matches!(
        contract.uses(),
        BirthFormalUseCoverageV1::ObjectFieldStores { sites } if sites.len() == 1
    ));
    assert_eq!(
        contract.disposition(),
        BirthFormalPhysicalDispositionV1::DeferredActualBinding
    );
}

#[test]
fn construction_plan_keeps_mixed_param_uses_closed_admits_self_reference() {
    // A formal feeding both a scalar and an object-typed field has no
    // single wire lane — the mixture stays uncovered, never guessed.
    let mixed = super::super::brand_catalog_tests::issue_with_brand_catalog(
        "box Node { value: i64\nbirth(v) { me.value = v } }
        box Mix { inner: Node\nvalue: i64\nbirth(v) { me.inner = v\nme.value = v } }",
    )
    .unwrap();
    let contract = &mixed
        .instance_constructors()
        .rows()
        .iter()
        .find(|row| row.box_name() == "Mix")
        .unwrap()
        .formal_contracts()[0];
    assert!(matches!(
        contract.uses(),
        BirthFormalUseCoverageV1::UncoveredSelectedBody
    ));
    // A self-referential provided field resolves: Node's own declared
    // child class is itself — `provided` seals the canonical identity
    // and the releasable `OwnedObjectFields` disposition admits it (the
    // emitted teardown call graph recurses on the instance, not the
    // type).
    let package = super::super::brand_catalog_tests::issue_with_brand_catalog(
        "box Node { left: Node\nbirth(left) { me.left = left } }",
    )
    .unwrap();
    let batch = &package.instance_constructors;
    let parent = batch.box_sources.row_for("Node").unwrap().unwrap();
    let plan = batch
        .construction_for(parent, 1)
        .unwrap()
        .as_ref()
        .expect("self-referential provided store issues");
    let [store] = plan.stores() else {
        panic!("one store: {:?}", plan.stores());
    };
    let ConstructionStoreRhsV1::Parameter { provided, .. } = store.rhs() else {
        panic!("param store: {:?}", store.rhs());
    };
    let node_object = batch.object_for(parent).unwrap();
    assert_eq!(*provided, Some(node_object));
}

#[test]
fn construction_plan_declines_nobirth_provider_variants() {
    use super::super::instance_construction::ConstructionUnavailableV1 as U;
    for (label, source) in [
        (
            "arity-over-zero-no-birth",
            "box Leaf {}
            box Page { leaf: Leaf
            birth() { me.leaf = new Leaf(7) } }",
        ),
        (
            "non-fieldless-no-birth",
            "box Leaf { v: i64 }
            box Page { leaf: Leaf
            birth() { me.leaf = new Leaf() } }",
        ),
        (
            "birth-arity-mismatch-not-nobirth",
            "box Leaf { birth(x) {} }
            box Page { leaf: Leaf
            birth() { me.leaf = new Leaf() } }",
        ),
    ] {
        let package =
            super::super::brand_catalog_tests::issue_with_brand_catalog(source).unwrap();
        let batch = &package.instance_constructors;
        let parent = batch.box_sources.row_for("Page").unwrap().unwrap();
        let error = batch
            .construction_for(parent, 0)
            .unwrap()
            .as_ref()
            .expect_err(&format!("{label} stays declined"));
        assert_eq!(
            error,
            &U::FieldContractUnsupported,
            "{label} fails closed"
        );
    }
}
