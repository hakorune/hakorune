//! Verify ordered Normal and Fault cleanup with the real recorded root entry.
use super::*;
use super::super::root_cleanup_graph::{ordered_paths, ordered_structure};

impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn check_recorded_home_order_for_test(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceStmtSiteV1,
        physical: &MirFunction,
    ) {
        self.validate_root_cleanup_shape(owner, physical).unwrap();
        let exits = self.root_exits.borrow();
        let RootHomeExitProgress::Emitted {
            order, origins, entry, bindings,
        } = &exits[&(owner, site.clone().into())] else {
            panic!("the common root writer recorded this exit");
        };
        assert_eq!(origins.len(), 3);
        let mut reordered = bindings.clone();
        let last = reordered.len() - 1;
        reordered.swap(0, last);
        let wrapper = self.main_arguments_release_suffix_v1(owner, physical).unwrap();
        assert!(ordered_paths::validate(physical, &reordered, entry, order, None).is_err()
            || ordered_structure::validate_original_with_wrapper(
                physical, &reordered, entry, order.ingress_result_for_entry(entry), &wrapper,
                order.borrowed_expression_fault_v1(),
            ).is_err());

        // Mutate the recorded row together with MIR so ordered-path validation
        // must reject the actual cleanup traversal, not just a row mismatch.
        for fault_path in [false, true] {
            let mut changed = physical.clone();
            let mut changed_bindings = bindings.clone();
            let first = origins[0].block;
            let (_, instruction) = changed_bindings.iter_mut().find(|(id, _)| *id == first).unwrap();
            let MirInstruction::Invoke { normal_landing, fault_landing, .. } = instruction else {
                panic!("source Home release");
            };
            if fault_path {
                *fault_landing = origins[1].block;
            } else {
                *normal_landing = origins[2].block;
            }
            changed.blocks.get_mut(&first).unwrap().terminator = Some(instruction.clone());
            let error = ordered_paths::validate(&changed, &changed_bindings, entry, order, None)
                .unwrap_err();
            assert!(error.contains("ordered-path/"), "{error}");
        }
    }
}
