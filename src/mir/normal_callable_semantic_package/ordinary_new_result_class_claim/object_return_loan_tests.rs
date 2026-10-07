//! Source product membership and pure owned-call qualification boundaries.
use super::*;
const SOURCE: &str = "box Token { value: i64 birth(value) { me.value = value } } box Door { make(flag) { if flag == 0 { return null } return new Token(1) } direct(flag) { return me.make(flag) } localRelay(flag) { local item = me.make(flag) return item } borrowed(token: Token) { return token } borrowedRelay(token: Token) { return me.borrowed(token) } mixed(token: Token, flag) { if flag == 0 { return new Token(1) } return token } mixedRelay(token: Token, flag) { return me.mixed(token, flag) } } static box Main { main() { return 0 } }";
fn facts() -> OrdinaryNewResultClassClaimsV1 {
    super::super::super::source_result_facts_for_test(SOURCE)
}
fn key(name: &str, arity: u32) -> CanonicalSameModuleCallableKeyV1 {
    CanonicalSameModuleCallableKeyV1::instance_box_method("Door", name, arity)
}

#[test]
fn object_return_loan_retains_direct_and_received_source_witnesses_without_new_solver() {
    let facts = facts();
    for name in ["direct", "localRelay"] {
        let row = &facts.outcomes(&key(name, 1)).unwrap()[0];
        let loan = facts.object_return_qualification(row.site()).unwrap();
        assert_eq!(loan.value(), row.site());
        assert_eq!(loan.call().owner(), row.site().owner());
        assert_eq!(loan.key(), &key("make", 1));
        assert!(
            matches!(loan.class(), OrdinaryNewResultClassV1::NullableObject(class) if class.as_ref() == "Token")
        );
        assert_eq!(loan.witnesses().len(), row.witnesses().len());
        for (actual, original) in loan.witnesses().iter().zip(row.witnesses()) {
            assert!(Rc::ptr_eq(actual, original));
        }
        assert_eq!(loan, loan.clone());
        if name == "direct" {
            assert_eq!(loan.call(), loan.value());
        } else {
            assert_ne!(loan.call(), loan.value());
        }
    }
}

#[test]
fn object_return_loan_rejects_fresh_leaf_forwarded_borrowed_and_mixed_calls() {
    let facts = facts();
    for (name, arity) in [("make", 1), ("borrowedRelay", 1), ("mixedRelay", 2)] {
        for row in facts.outcomes(&key(name, arity)).unwrap() {
            assert!(
                facts.object_return_qualification(row.site()).is_none(),
                "{name}"
            );
        }
    }
    assert!(facts.get(&key("mixedRelay", 2)).is_none());
}

#[test]
fn object_return_loan_rejects_foreign_fact_product_and_missing_callee_membership() {
    let facts = facts();
    let other = super::super::super::source_result_facts_for_test(SOURCE);
    let row = &facts.outcomes(&key("direct", 1)).unwrap()[0];
    assert!(other.object_return_qualification(row.site()).is_none());
    let mut incomplete = OrdinaryNewResultClassClaimsV1::new();
    incomplete.insert(
        key("direct", 1),
        super::super::product::SourceResultRowV1 {
            exits: vec![ResultExitOriginV1 {
                site: row.site().clone(),
                alternatives: row.alternatives().clone(),
                witnesses: row.witnesses().iter().map(Rc::clone).collect(),
            }]
            .into_boxed_slice(),
            projection: facts.get(&key("direct", 1)).cloned(),
        },
    );
    assert!(incomplete.object_return_qualification(row.site()).is_none());
}

#[test]
fn object_producer_dependencies_use_exact_callee_owner_and_keep_leaf_lane_unmarked() {
    let facts = facts();
    for name in ["direct", "localRelay"] {
        let key = key(name, 1);
        let source = &facts.outcomes(&key).unwrap()[0];
        let dependencies = facts
            .object_return_dependencies(&key, source.site().owner())
            .unwrap();
        assert_eq!(dependencies.len(), 1);
        assert_eq!(
            dependencies[0],
            facts.object_return_qualification(source.site()).unwrap()
        );
        let foreign = super::super::super::source_result_facts_for_test(SOURCE);
        let foreign_owner = foreign.outcomes(&key).unwrap()[0].site().owner();
        assert!(facts
            .object_return_dependencies(&key, foreign_owner)
            .is_none());
    }
    for (name, arity) in [("make", 1), ("borrowedRelay", 1), ("mixedRelay", 2)] {
        let key = key(name, arity);
        let owner = facts.outcomes(&key).unwrap()[0].site().owner();
        assert!(
            facts.object_return_dependencies(&key, owner).is_none(),
            "{name}"
        );
    }
}

#[test]
fn object_producer_dependencies_reject_outer_class_disagreement_with_qualified_calls() {
    let mut facts = facts();
    let key = key("direct", 1);
    let row = &facts.outcomes(&key).unwrap()[0];
    let site = row.site().clone();
    let altered = super::super::product::SourceResultRowV1 {
        exits: vec![ResultExitOriginV1 {
            site: site.clone(),
            alternatives: row.alternatives().clone(),
            witnesses: row.witnesses().iter().map(Rc::clone).collect(),
        }]
        .into_boxed_slice(),
        projection: Some(OrdinaryNewResultClassV1::NullableObject("Other".into())),
    };
    facts.insert(key.clone(), altered);
    assert!(
        facts.object_return_qualification(&site).is_some(),
        "the original child-call qualification still exists"
    );
    assert!(facts
        .object_return_dependencies(&key, site.owner())
        .is_none());
}
