//! Original receiver observation membership and exact role refusal.
use super::*;

#[test]
fn receiver_source_handle_role_requires_original_object_observation() {
    for nullable in [false, true] {
        let body = if nullable {
            "if size == 0 { return null } return new Token()"
        } else {
            "return new Token()"
        };
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
            &format!("box Token {{}} box Maker {{ make(size: i64) {{ {body} }} relay(size: i64) {{ local item = me.make(size) return item }} }} static box Main {{ main() {{ return 0 }} }}")
        ).unwrap();
        let ledger = &package.ordinary_new_claim_ledger;
        let (site, observation) = ledger
            .receiver_call_observations
            .iter()
            .find(|(_, row)| row.callee().owner() == "Maker" && row.callee().name() == "make")
            .unwrap();
        assert_eq!(
            has_object_receiver_call_at_v1(&ledger.receiver_call_observations, site),
            !nullable
        );
        assert!(!has_object_receiver_call_at_v1(&BTreeMap::new(), site));
        assert_eq!(observation.arguments().len(), 1);
        let mut missing = ledger.receiver_call_observations.clone();
        missing.remove(site).unwrap();
        assert!(!has_object_receiver_call_at_v1(&missing, site));
    }
}
