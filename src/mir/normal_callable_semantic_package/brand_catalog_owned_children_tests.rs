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

/// A self-referential provided field is the admitted recursion:
/// `left: Node`/`right: Node` seal `Object` children whose canonical
/// identity is the enclosing object itself — the emitted per-class
/// teardown calls its own helper on each live instance, so the census
/// admits the type-level cycle while the release graph bottoms out on
/// real children.
#[test]
fn ordinary_new_owned_self_referential_children_seal() {
    use crate::mir::normal_callable_semantic_package::OwnedFieldChildKindV1;
    let package = issue_with_brand_catalog(
        "box Node {
            left: Node
            right: Node
            value: i64
            birth(left, right, value) {
                me.left = left
                me.right = right
                me.value = value
            }
        }
        static box Main {
            main() { local item = new Node(null, null, 1) return 0 }
        }",
    )
    .expect("self-referential package");
    let rows = package
        .ordinary_new_claim_ledger
        .pending_claims_for_test();
    let claims: Vec<_> = rows
        .values()
        .filter(|claim| claim.class() == "Node")
        .collect();
    let [claim] = claims.as_slice() else {
        panic!("one Node claim, got {claims:?}")
    };
    assert_eq!(
        claim.destruction(),
        crate::mir::function::ObjectDestructionDispositionV1::OwnedObjectFieldsNoHook
    );
    let children = claim.children().expect("self-referential residences seal");
    assert_eq!(children.len(), 2);
    for (ordinal, child) in children.iter().enumerate() {
        assert_eq!(child.field.declaration_ordinal() as usize, ordinal);
        let OwnedFieldChildKindV1::Object(inner) = child.kind else {
            panic!("self-referential field seals an Object child, got {child:?}")
        };
        assert_eq!(
            inner,
            claim.object(),
            "the sealed child identity is the enclosing object itself"
        );
    }
}

/// A non-self `OwnedObjectFieldsNoHook` child resolves through the same
/// recursion: `mid: Mid` seals while `Mid`'s own inventory seals —
/// nesting depth is a runtime property of the emitted teardown call
/// graph, never a census bound.
#[test]
fn ordinary_new_owned_nested_object_child_seals() {
    use crate::mir::normal_callable_semantic_package::OwnedFieldChildKindV1;
    let package = issue_with_brand_catalog(
        "box Inner {
            value: i64
            birth(value) { me.value = value }
        }
        box Mid {
            inner: Inner
            birth(inner) { me.inner = inner }
        }
        box Top {
            mid: Mid
            birth(mid) { me.mid = mid }
        }
        static box Main {
            main() {
                local inner = new Inner(7)
                local mid = new Mid(inner)
                local top = new Top(mid)
                return 0
            }
        }",
    )
    .expect("nested object package");
    let rows = package
        .ordinary_new_claim_ledger
        .pending_claims_for_test();
    let claims: Vec<_> = rows
        .values()
        .filter(|claim| claim.class() == "Top")
        .collect();
    let [claim] = claims.as_slice() else {
        panic!("one Top claim, got {claims:?}")
    };
    let children = claim.children().expect("nested object residences seal");
    let [child] = children.as_ref() else {
        panic!("one Object child, got {children:?}")
    };
    assert!(
        matches!(child.kind, OwnedFieldChildKindV1::Object(_)),
        "nested object child seals as an Object child, got {child:?}"
    );
    // The nested inventory lands in the ledger too — `inner`'s own
    // children are the sealed rows the teardown walks at runtime.
    let mid_claims: Vec<_> = rows
        .values()
        .filter(|claim| claim.class() == "Mid")
        .collect();
    let [mid] = mid_claims.as_slice() else {
        panic!("one Mid claim, got {mid_claims:?}")
    };
    assert!(
        mid.children().is_some(),
        "the nested child's own inventory seals through the same census"
    );
}

/// The nested recursion stays fail-closed: a `OwnedObjectFields` child
/// whose own inventory cannot seal — an unproven object-field residence
/// — leaves the enclosing children unsealed rather than guessing a
/// release shape.
#[test]
fn ordinary_new_owned_nested_object_child_stays_unproven_on_rejected_evidence() {
    let package = issue_with_brand_catalog(
        "box Inner { value: i64\nbirth(value) { me.value = value } }
        box Mid { inner: Inner\nbirth() { } }
        box Top { mid: Mid\nbirth(mid) { me.mid = mid } }
        static box Main {
            main() {
                local mid = new Mid()
                local top = new Top(mid)
                return 0
            }
        }",
    )
    .expect("unproven nested package");
    let rows = package
        .ordinary_new_claim_ledger
        .pending_claims_for_test();
    let claims: Vec<_> = rows
        .values()
        .filter(|claim| claim.class() == "Top")
        .collect();
    let [claim] = claims.as_slice() else {
        panic!("one Top claim, got {claims:?}")
    };
    assert!(
        claim.children().is_none(),
        "an unproven nested child leaves the inventory unsealed"
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
/// birth stores, a wrong-class store, or a child whose own nested
/// residences never proved each leave `children` unsealed — the lifecycle
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
        // A child whose own `ArrayBox` residence never proved a birth
        // provider keeps the whole nested claim unsealed.
        (
            "unproven-nested",
            "child: HalfChild = new HalfChild()\nbirth() { }",
        ),
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

/// A non-self ownership cycle stays declined: `PeerA -> PeerB -> PeerA`
/// revisits an in-progress ancestor, so the `visiting` set declines the
/// nested seal and the enclosing claim keeps `children` unsealed. Mutual
/// ownership is source-unconstructible — each binding moves once — and
/// its teardown could never bottom out, so the census refuses the type
/// shape rather than inventing a release order.
#[test]
fn ordinary_new_owned_object_children_decline_non_self_cycles() {
    let package = issue_with_brand_catalog(
        "box PeerA {
            peer: PeerB
            birth(peer) { me.peer = peer }
        }
        box PeerB {
            peer: PeerA
            birth(peer) { me.peer = peer }
        }
        box Parent {
            child: PeerA
            birth(child) { me.child = child }
        }
        static box Main {
            main() { local item = new Parent(null) return 0 }
        }",
    )
    .expect("mutual-ownership package");
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
    assert!(
        claim.children().is_none(),
        "a non-self ownership cycle stays unsealed"
    );
}
