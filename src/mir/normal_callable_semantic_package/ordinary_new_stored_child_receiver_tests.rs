//! Exact stored receiver identity stays shared by incoming and result proofs.
use super::*;

#[test]
fn stored_child_receiver_corroboration_rejects_identity_drift() {
    let text = "box Item { value: i64 birth() { me.value = 5 } }
        box Leaf { flag: i64 birth() { me.flag = 0 }
            read(p: Item) { if p == null { return 7 } return p.value } }
        box Parent { left: Leaf right: Leaf
            birth() { me.left = new Leaf() me.right = new Leaf() }
            first(p: Item) { return me.left.read(p) }
            second(p: Item) { return me.right.read(p) } }
        static box Main { main() { local parent = new Parent() local item = new Item()
            local ignored = parent.second(item) return parent.first(item) } }";
    for change in 0..6 {
        let mut package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(text)
            .expect("healthy source-issued stored receiver");
        let ledger = std::rc::Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        let borrowed = ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap();
        let mut prepared: Vec<_> = borrowed
            .incoming
            .iter()
            .map(|row| Ok(Some(row.source.require_instance().unwrap().clone())))
            .collect();
        borrowed.corroborate_source_targets(&prepared).unwrap();
        let indices: Vec<_> = prepared
            .iter()
            .enumerate()
            .filter_map(|(i, row)| {
                row.as_ref()
                    .unwrap()
                    .as_ref()
                    .unwrap()
                    .stored_receiver()
                    .map(|_| i)
            })
            .collect();
        assert_eq!(indices.len(), 2, "both original stored terminals");
        let sibling = prepared[indices[1]]
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap()
            .clone();
        let target = prepared[indices[0]].as_mut().unwrap().as_mut().unwrap();
        let owner = target.call_site().owner();
        let LexicalInstanceCallReceiverV1::StoredOwnedChild {
            parent_binding,
            parent_site,
            field,
            child,
        } = &mut target.receiver
        else {
            panic!("stored receiver")
        };
        let (other_binding, _, other_field, _) = sibling.stored_receiver().unwrap();
        match change {
            0 => *field = other_field,
            1 => *child = field.object(),
            2 => *parent_binding = other_binding,
            3 => *parent_site = target.receiver_site.clone(),
            4 => target.receiver_site = target.argument_sites[0].clone(),
            _ => target.target_batch_slot += 1,
        }
        assert!(
            borrowed
                .corroborate_source_targets(&prepared)
                .unwrap_err()
                .contains("borrowed-formal/final-incoming-drift"),
            "change={change}"
        );
        ledger.corroborate_borrowed_result_cohort_v1(&prepared, &package.result_contracts);
        assert!(
            ledger.borrowed_i64_results[&owner]
                .as_ref()
                .unwrap_err()
                .contains("borrowed-result/result-contract-mismatch"),
            "change={change}"
        );
    }
}
