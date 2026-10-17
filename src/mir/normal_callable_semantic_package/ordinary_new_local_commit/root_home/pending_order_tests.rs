//! Reuse the source-issued three-Home fixture before any root publication.
use super::*;

type Origins = [(RootHomeReleaseOriginV1, BasicBlockId, MirInstruction)];
type Bindings = [(BasicBlockId, MirInstruction)];

impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn check_pending_nonempty_home_order_for_test(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceStmtSiteV1,
        origins: &Origins,
        bindings: &Bindings,
        physical: &MirFunction,
    ) {
        assert_eq!(origins.len(), 3);
        let assert_unpublished = || {
            assert!(self
                .root_exits
                .borrow()
                .get(&(owner, (site.clone()).into()))
                .is_none_or(|progress| matches!(progress, RootHomeExitProgress::Unprepared)));
        };
        let prepare = |origins: &Origins, bindings: &Bindings| {
            let plan = self.prepared_root_home_graph_plan_v1(owner, site).unwrap();
            self.prepare_selected_root_exit_v1(
                owner,
                site.clone(),
                plan.order.order,
                origins.to_vec(),
                bindings.to_vec(),
                RootHomeExitEntry::Plain {
                    local_bindings: Vec::new(),
                },
            )
        };
        let pending = prepare(origins, bindings).unwrap();
        self.validate_root_cleanup_shape_rows(owner, physical, &[&pending])
            .unwrap();
        drop(pending);
        assert_unpublished();

        // Source origin order cannot be replaced by the emitted graph's order.
        let mut reversed = origins.to_vec();
        reversed.reverse();
        for changed in [&reversed[..], &origins[1..]] {
            let error = prepare(changed, bindings)
                .err()
                .expect("origin drift refused");
            assert!(error.contains("root-exit-origin-order"), "{error}");
            assert_unpublished();
        }
        // The entry binding is ordered: another valid node cannot replace it.
        let mut reordered = bindings.to_vec();
        let last = reordered.len() - 1;
        reordered.swap(0, last);
        let pending = prepare(origins, &reordered).unwrap();
        assert!(self
            .validate_root_cleanup_shape_rows(owner, physical, &[&pending])
            .is_err());
        drop(pending);
        assert_unpublished();

        // Check physical Normal and Fault traversal independently of origins.
        for fault_path in [false, true] {
            let mut changed = physical.clone();
            let mut changed_bindings = bindings.to_vec();
            let first = origins[0].1;
            let second = origins[1].1;
            for (id, instruction) in &mut changed_bindings {
                if *id != first {
                    continue;
                }
                let MirInstruction::Invoke {
                    normal_landing,
                    fault_landing,
                    ..
                } = instruction
                else {
                    panic!("source Home release")
                };
                if fault_path {
                    *fault_landing = second;
                } else {
                    *normal_landing = origins[2].1;
                }
                changed.blocks.get_mut(id).unwrap().terminator = Some(instruction.clone());
            }
            let pending = prepare(origins, &changed_bindings).unwrap();
            let error = self
                .validate_root_cleanup_shape_rows(owner, &changed, &[&pending])
                .unwrap_err();
            assert!(error.contains("ordered-path/"), "{error}");
            drop(pending);
            assert_unpublished();
        }
    }
}
