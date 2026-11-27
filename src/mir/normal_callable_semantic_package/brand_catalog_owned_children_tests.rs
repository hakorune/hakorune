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
        claim.core().destruction(),
        crate::mir::function::ObjectDestructionDispositionV1::ARRAY_FIELDS_NO_HOOK
    );
    let ordinals: Vec<u32> = claim
        .core().children()
        .expect("proven owned residences")
        .iter()
        .map(|child| {
            assert!(matches!(
                child.kind,
                crate::mir::normal_callable_semantic_package::OwnedFieldChildKindV1::Residence(crate::mir::instruction::FieldResidenceEndModeV1::ArrayResidence)
            ));
            child.field.declaration_ordinal()
        })
        .collect();
    assert_eq!(ordinals, [1, 2], "declaration order is the sealed order");
}

#[test]
fn owned_child_claim_requires_unambiguous_residence_evidence() {
    for (case_name, class, preamble, filter_class, destruction, reason, members_cases) in [
                ("ordinary_new_owned_array_children_stay_unproven_on_ambiguous_writes", "Page", "", false, crate::mir::function::ObjectDestructionDispositionV1::ARRAY_FIELDS_NO_HOOK, "an unproven residence stays unsealed", &[
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
    ][..]),
("ordinary_new_owned_object_children_stay_unproven_on_rejected_evidence", "Parent", "box Child { v: i64 = 0\nbirth() { } }\n            box Other { v: i64 = 0\nbirth() { } }\n            box DeepChild { inner: Child = new Child()\nbirth() { } }\n            box HalfChild { items: ArrayBox\nbirth() { } }\n            ", true, crate::mir::function::ObjectDestructionDispositionV1::OBJECT_FIELDS_NO_HOOK, "rejected residence evidence stays unsealed", &[
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
    ][..]),
            ] {
                println!("case: {case_name}");
                    for &(label, members) in members_cases {

        let source = format!(
            "{preamble}box {class} {{ {members} }}
            static box Main {{ main() {{ local item = new {class}() return 0 }} }}"
        );
        let package = issue_with_brand_catalog(&source)
            .unwrap_or_else(|issue| panic!("{label} package: {issue:?}"));
        let rows = package
            .ordinary_new_claim_ledger
            .pending_claims_for_test();
        let claims: Vec<_> = rows
            .values()
            .filter(|claim| !filter_class || claim.core().class() == class)
            .collect();
        let [claim] = claims.as_slice() else {
            panic!("{label}: one claim, got {claims:?}")
        };
        assert_eq!(
            claim.core().destruction(),
            destruction,
            "{label}"
        );
        assert!(
            claim.core().children().is_none(),
            "{label}: {reason}"
        );

    }
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
        .filter(|claim| claim.core().class() == "Parent")
        .collect();
    let [claim] = claims.as_slice() else {
        panic!("one Parent claim, got {claims:?}")
    };
    assert_eq!(
        claim.core().destruction(),
        crate::mir::function::ObjectDestructionDispositionV1::no_hook_fields(true, true, false)
    );
    let children = claim.core().children().expect("proven owned residences");
    assert_eq!(children.len(), 2);
    assert_eq!(children[0].field.declaration_ordinal(), 0);
    assert!(
        matches!(children[0].kind, OwnedFieldChildKindV1::Residence(crate::mir::instruction::FieldResidenceEndModeV1::ArrayResidence)),
        "array child keeps the residence-release kind"
    );
    assert_eq!(children[1].field.declaration_ordinal(), 1);
    let OwnedFieldChildKindV1::Object(child) = children[1].kind else {
        panic!("user-object field must seal an Object child, got {:?}", children[1])
    };
    assert_ne!(
        child, claim.core().object(),
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
        .filter(|claim| claim.core().class() == "Node")
        .collect();
    let [claim] = claims.as_slice() else {
        panic!("one Node claim, got {claims:?}")
    };
    assert_eq!(
        claim.core().destruction(),
        crate::mir::function::ObjectDestructionDispositionV1::OBJECT_FIELDS_NO_HOOK
    );
    let children = claim.core().children().expect("self-referential residences seal");
    assert_eq!(children.len(), 2);
    for (ordinal, child) in children.iter().enumerate() {
        assert_eq!(child.field.declaration_ordinal() as usize, ordinal);
        let OwnedFieldChildKindV1::Object(inner) = child.kind else {
            panic!("self-referential field seals an Object child, got {child:?}")
        };
        assert_eq!(
            inner,
            claim.core().object(),
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
        .filter(|claim| claim.core().class() == "Top")
        .collect();
    let [claim] = claims.as_slice() else {
        panic!("one Top claim, got {claims:?}")
    };
    let children = claim.core().children().expect("nested object residences seal");
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
        .filter(|claim| claim.core().class() == "Mid")
        .collect();
    let [mid] = mid_claims.as_slice() else {
        panic!("one Mid claim, got {mid_claims:?}")
    };
    assert!(
        mid.core().children().is_some(),
        "the nested child's own inventory seals through the same census"
    );
}

#[test]
fn owned_object_children_remain_unsealed_without_complete_evidence() {
    for (case_name, source_text, description, box_name, claim_error, unsealed_error) in [
        (
            "ordinary_new_owned_nested_object_child_stays_unproven_on_rejected_evidence",
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
            "unproven nested package",
            "Top",
            "one Top claim",
            "an unproven nested child leaves the inventory unsealed",
        ),
        (
            "ordinary_new_owned_object_children_decline_non_self_cycles",
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
            "mutual-ownership package",
            "Parent",
            "one Parent claim",
            "a non-self ownership cycle stays unsealed",
        ),
    ] {
        println!("case: {case_name}");
        let package = issue_with_brand_catalog(source_text).expect(description);
        let rows = package.ordinary_new_claim_ledger.pending_claims_for_test();
        let claims: Vec<_> = rows
            .values()
            .filter(|claim| claim.core().class() == box_name)
            .collect();
        let [claim] = claims.as_slice() else {
            panic!("{claim_error}, got {claims:?}")
        };
        assert!(claim.core().children().is_none(), "{unsealed_error}");
    }
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
        .filter(|claim| claim.core().class() == "Parent")
        .collect();
    let [claim] = claims.as_slice() else {
        panic!("one Parent claim, got {claims:?}")
    };
    let children = claim.core().children().expect("proven owned residences");
    let [child] = children.as_ref() else {
        panic!("one Object child, got {children:?}")
    };
    assert!(
        matches!(child.kind, OwnedFieldChildKindV1::Object(_)),
        "owned-array child seals as an Object child, got {child:?}"
    );
}
