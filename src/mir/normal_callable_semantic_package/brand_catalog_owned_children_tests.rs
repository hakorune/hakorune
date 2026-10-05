/// A declaration default `items: ArrayBox = new ArrayBox()` desugars to a
/// birth-side `me.items = new ArrayBox()` provider store; every declared
/// ArrayBox field with that sealed store lands in `array_children` in
/// declaration order under the `OwnedArrayFieldsNoHook` disposition.
#[test]
fn ordinary_new_owned_array_children_seal_in_declaration_order() {
    let package = issue_with_brand_catalog(
        "box Page {
            left: i64
            items: ArrayBox = new ArrayBox()
            children: ArrayBox = new ArrayBox()
            birth() { }
        }
        static box Main { main() { local item = new Page() return 0 } }",
    )
    .expect("owned-array package");
    let rows = package
        .ordinary_new_claim_ledger
        .pending_claims_for_test();
    let claims: Vec<_> = rows.values().collect();
    let [claim] = claims.as_slice() else {
        panic!("one claim, got {claims:?}")
    };
    assert_eq!(
        claim.destruction(),
        crate::mir::function::ObjectDestructionDispositionV1::OwnedArrayFieldsNoHook
    );
    let ordinals: Vec<u32> = claim
        .children()
        .expect("proven owned residences")
        .iter()
        .map(|child| {
            assert!(matches!(
                child.kind,
                crate::mir::normal_callable_semantic_package::OwnedFieldChildKindV1::Array
            ));
            child.field.declaration_ordinal()
        })
        .collect();
    assert_eq!(ordinals, [1, 2], "declaration order is the sealed order");
}

/// The residence proof stays fail-closed: no provider store, a second
/// `me.` store, or a store outside the constructor path each leave
/// `array_children` empty on an `OwnedArrayFieldsNoHook` claim — the
/// lifecycle gate then refuses the teardown instead of releasing plain.
#[test]
fn ordinary_new_owned_array_children_stay_unproven_on_ambiguous_writes() {
    for (label, members) in [
        // No birth-side provider store at all.
        ("missing", "items: ArrayBox\nbirth() { }"),
        // A second `me.` store — even of `new ArrayBox()` — is re-assignment.
        (
            "rewritten",
            "items: ArrayBox = new ArrayBox()\nbirth() { me.items = new ArrayBox() }",
        ),
        // A store outside the constructor path is not a birth provider.
        (
            "non-birth",
            "items: ArrayBox = new ArrayBox()\nbirth() { }\ntouch() { me.items = new ArrayBox() }",
        ),
    ] {
        let source = format!(
            "box Page {{ {members} }}
            static box Main {{ main() {{ local item = new Page() return 0 }} }}"
        );
        let package = issue_with_brand_catalog(&source)
            .unwrap_or_else(|issue| panic!("{label} package: {issue:?}"));
        let rows = package
            .ordinary_new_claim_ledger
            .pending_claims_for_test();
        let claims: Vec<_> = rows.values().collect();
        let [claim] = claims.as_slice() else {
            panic!("{label}: one claim, got {claims:?}")
        };
        assert_eq!(
            claim.destruction(),
            crate::mir::function::ObjectDestructionDispositionV1::OwnedArrayFieldsNoHook,
            "{label}"
        );
        assert!(
            claim.children().is_none(),
            "{label}: an unproven residence stays unsealed"
        );
    }
}

/// A declaration default `child: Child = new Child()` on a user-object field
/// resolves through canonical membership into a typed `Object` child under
/// the `OwnedObjectFieldsNoHook` disposition — declared order, with the
/// child identity distinct from the parent's.
#[test]
fn ordinary_new_owned_object_children_seal_in_declaration_order() {
    use crate::mir::normal_callable_semantic_package::OwnedFieldChildKindV1;
    let package = issue_with_brand_catalog(
        "box Child {
            v: i64 = 0
            birth() { }
        }
        box Parent {
            items: ArrayBox = new ArrayBox()
            child: Child = new Child()
            birth() { }
        }
        static box Main { main() { local item = new Parent() return 0 } }",
    )
    .expect("owned-object package");
    let rows = package
        .ordinary_new_claim_ledger
        .pending_claims_for_test();
    let claims: Vec<_> = rows
        .values()
        .filter(|claim| claim.class() == "Parent")
        .collect();
    let [claim] = claims.as_slice() else {
        panic!("one Parent claim, got {claims:?}")
    };
    assert_eq!(
        claim.destruction(),
        crate::mir::function::ObjectDestructionDispositionV1::OwnedObjectFieldsNoHook
    );
    let children = claim.children().expect("proven owned residences");
    assert_eq!(children.len(), 2);
    assert_eq!(children[0].field.declaration_ordinal(), 0);
    assert!(
        matches!(children[0].kind, OwnedFieldChildKindV1::Array),
        "array child keeps the residence-release kind"
    );
    assert_eq!(children[1].field.declaration_ordinal(), 1);
    let OwnedFieldChildKindV1::Object(child) = children[1].kind else {
        panic!("user-object field must seal an Object child, got {:?}", children[1])
    };
    assert_ne!(
        child, claim.object(),
        "the sealed child identity is the declared class, not the parent"
    );
}

/// A child owning sealed `ArrayBox` residences is the admitted bounded
/// nesting: `child: OwnChild = new OwnChild()` seals an `Object` child
/// while the child's own teardown inventory lands in the ledger —
/// releasing its slots is sound exactly while every residence is a
/// proven provider store.
#[test]
fn ordinary_new_owned_nested_array_child_seals() {
    use crate::mir::normal_callable_semantic_package::OwnedFieldChildKindV1;
    let package = issue_with_brand_catalog(
        "box OwnChild {
            items: ArrayBox = new ArrayBox()
            birth() { }
        }
        box Parent {
            child: OwnChild = new OwnChild()
            birth() { }
        }
        static box Main { main() { local item = new Parent() return 0 } }",
    )
    .expect("nested owned-array package");
    let rows = package
        .ordinary_new_claim_ledger
        .pending_claims_for_test();
    let claims: Vec<_> = rows
        .values()
        .filter(|claim| claim.class() == "Parent")
        .collect();
    let [claim] = claims.as_slice() else {
        panic!("one Parent claim, got {claims:?}")
    };
    let children = claim.children().expect("proven owned residences");
    let [child] = children.as_ref() else {
        panic!("one Object child, got {children:?}")
    };
    assert!(
        matches!(child.kind, OwnedFieldChildKindV1::Object(_)),
        "owned-array child seals as an Object child, got {child:?}"
    );
}

/// The user-object residence proof stays fail-closed: missing or ambiguous
/// birth stores, a wrong-class store, a child deeper than the bounded
/// owned-`ArrayBox` level or with unproven nested residences, and a
/// self-referential field each leave `children` unsealed — the lifecycle
/// gate refuses the teardown rather than guessing a release shape.
#[test]
fn ordinary_new_owned_object_children_stay_unproven_on_rejected_evidence() {
    for (label, members) in [
        // No birth-side provider store at all.
        ("missing", "child: Child\nbirth() { }"),
        // A second `me.` store — even of the same class — is re-assignment.
        (
            "rewritten",
            "child: Child = new Child()\nbirth() { me.child = new Child() }",
        ),
        // A store outside the constructor path is not a birth provider.
        (
            "non-birth",
            "child: Child = new Child()\nbirth() { }\ntouch() { me.child = new Child() }",
        ),
        // The sole birth store must write exactly the declared class.
        ("wrong-class", "child: Child = new Other()\nbirth() { }"),
        // A child owning a user-object field is deeper than the bounded
        // owned-`ArrayBox` nesting level.
        (
            "deeper-child",
            "child: DeepChild = new DeepChild()\nbirth() { }",
        ),
        // A child whose own `ArrayBox` residence never proved a birth
        // provider keeps the whole nested claim unsealed.
        (
            "unproven-nested",
            "child: HalfChild = new HalfChild()\nbirth() { }",
        ),
        // A field of the enclosing class can never terminate in S0.
        ("self", "child: Parent = new Parent()\nbirth() { }"),
    ] {
        let source = format!(
            "box Child {{ v: i64 = 0\nbirth() {{ }} }}
            box Other {{ v: i64 = 0\nbirth() {{ }} }}
            box DeepChild {{ inner: Child = new Child()\nbirth() {{ }} }}
            box HalfChild {{ items: ArrayBox\nbirth() {{ }} }}
            box Parent {{ {members} }}
            static box Main {{ main() {{ local item = new Parent() return 0 }} }}"
        );
        let package = issue_with_brand_catalog(&source)
            .unwrap_or_else(|issue| panic!("{label} package: {issue:?}"));
        let rows = package
            .ordinary_new_claim_ledger
            .pending_claims_for_test();
        let claims: Vec<_> = rows
            .values()
            .filter(|claim| claim.class() == "Parent")
            .collect();
        let [claim] = claims.as_slice() else {
            panic!("{label}: one Parent claim, got {claims:?}")
        };
        assert_eq!(
            claim.destruction(),
            crate::mir::function::ObjectDestructionDispositionV1::OwnedObjectFieldsNoHook,
            "{label}"
        );
        assert!(
            claim.children().is_none(),
            "{label}: rejected residence evidence stays unsealed"
        );
    }
}
