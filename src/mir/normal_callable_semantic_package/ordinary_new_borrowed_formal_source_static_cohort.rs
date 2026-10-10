//! Complete selected Static incoming identity before executable actuals.

use super::*;
use crate::mir::normal_callable_semantic_package::qualified_static_call_claim::incoming_source::StaticIncomingSourceV1;
use std::rc::Rc;

impl PreparedBorrowedFormalIngressV1 {
    /// The selected-caller inventory's whole-callee projection retains vetoes.
    /// This receipt is source-only: no actual, carrier or call coordinate.
    pub(in crate::mir::normal_callable_semantic_package) fn static_incoming_cohort_v1(
        &self,
        selected: &Rc<StaticIncomingSourceV1>,
    ) -> Result<Box<[Rc<StaticIncomingSourceV1>]>, String> {
        let rows = self
            .source_incoming
            .project(&BTreeSet::from([selected.callee_owner()]))
            .map_err(|error| format!("{}: {error:?}", freeze("static-cohort/incoming-veto")))?;
        let mut sites = BTreeSet::new();
        let mut includes_selected = false;
        let mut original = Vec::with_capacity(rows.len());
        for row in rows {
            let BorrowedIncomingSourceV1::Static(source) = row.source else {
                return Err(freeze("static-cohort/nonstatic-incoming"));
            };
            if row.callee != selected.callee_owner()
                || source.callee_owner() != row.callee
                || source.call_site() != &row.call
                || source.target() != selected.target()
                || source.target_batch_slot() != selected.target_batch_slot()
                || source.argument_sites().len() != selected.target().arity() as usize
                || !sites.insert(row.call)
            {
                return Err(freeze("static-cohort/original-drift"));
            }
            includes_selected |= Rc::ptr_eq(&source, selected);
            original.push(source);
        }
        if !includes_selected {
            return Err(freeze("static-cohort/selected-missing"));
        }
        Ok(original.into_boxed_slice())
    }
}
