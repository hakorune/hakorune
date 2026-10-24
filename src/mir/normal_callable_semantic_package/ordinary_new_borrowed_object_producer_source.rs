//! Original read producers borrow existing Array and returned-Field provenance.
//! Selection grants no Home, Completion, entry or executable result.
use super::*;
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::field_write_claim::OrdinaryNewFieldWriteClaimsV1;
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::result_class_claim::{
    ObjectCallSourceLoanV1, OrdinaryNewResultClassClaimsV1,
};

fn same_source(a: &ObjectCallSourceLoanV1, b: &ObjectCallSourceLoanV1) -> bool {
    a.source() == b.source()
        && a.result_class() == b.result_class()
        && a.roots().len() == b.roots().len()
        && a.roots()
            .iter()
            .zip(b.roots())
            .all(|(a, b)| Rc::ptr_eq(a, b))
}
impl PreparedBorrowedFormalIngressV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn original_field_producer_source_v1(
        &self,
        target: &LexicalInstanceCallSourceTargetV1,
    ) -> Result<Option<(&ObjectCallSourceLoanV1, BindingRefV1)>, String> {
        let mut found = None;
        for row in self
            .original_formal_seeds
            .rows
            .iter()
            .filter(|row| row.legacy)
        {
            for seed in row.seeds.iter() {
                let OriginalFormalActualSeedV1::ReturnedField(capsule) = seed else {
                    continue;
                };
                for (loan, destination) in capsule.acquisition_sources_v1() {
                    if loan.source().call_site() != target.call_site() {
                        continue;
                    }
                    if !self
                        .original_formal_seeds
                        .corroborate_original_v1(&self.source_incoming)
                        || loan.source() != target
                        || destination.owner() != target.call_site().owner()
                        || self.object_source_target_at_v1(target.call_site())? != Some(target)
                    {
                        return Err(freeze("field-producer/original-source-drift"));
                    }
                    if found.is_some_and(|(previous, binding)| {
                        !same_source(previous, loan) || binding != destination
                    }) {
                        return Err(freeze("field-producer/original-source-ambiguity"));
                    }
                    found = Some((loan, destination));
                }
            }
        }
        Ok(found)
    }

    /// Before Home observation, recheck every capsule lending this acquisition.
    pub(in crate::mir::normal_callable_semantic_package) fn recheck_field_producer_source_v1(
        &self,
        target: &LexicalInstanceCallSourceTargetV1,
        classes: &OrdinaryNewResultClassClaimsV1,
        fields: &OrdinaryNewFieldWriteClaimsV1,
    ) -> Result<Option<(&ObjectCallSourceLoanV1, BindingRefV1)>, String> {
        let source = self.original_field_producer_source_v1(target)?;
        if source.is_none() {
            return Ok(None);
        }
        for row in self
            .original_formal_seeds
            .rows
            .iter()
            .filter(|row| row.legacy)
        {
            for seed in row.seeds.iter() {
                let OriginalFormalActualSeedV1::ReturnedField(capsule) = seed else {
                    continue;
                };
                if !capsule
                    .acquisition_sources_v1()
                    .any(|(loan, _)| loan.source().call_site() == target.call_site())
                {
                    continue;
                }
                classes
                    .recheck_returned_field_graph_v1(capsule, fields, &mut |site| {
                        self.object_source_target_at_v1(site)
                            .map_err(|_| "field-producer/current-target")
                    })
                    .map_err(freeze)?;
            }
        }
        Ok(source)
    }

    /// Field cohort membership selects input only; each LOCAL owns its acquisition.
    pub(in crate::mir::normal_callable_semantic_package) fn recheck_field_producer_callee_v1(
        &self,
        target: &LexicalInstanceCallSourceTargetV1,
        classes: &OrdinaryNewResultClassClaimsV1,
        fields: &OrdinaryNewFieldWriteClaimsV1,
    ) -> Result<bool, String> {
        if !self.original_object_producer_callee_v1(target)? {
            return Ok(false);
        }
        let mut selected = false;
        for call in self
            .source_incoming
            .exact_rows()
            .filter(|row| row.callee == target.callee_owner())
        {
            let original = call.source.require_instance()?;
            selected |= self
                .recheck_field_producer_source_v1(original, classes, fields)?
                .is_some();
        }
        Ok(selected)
    }

    /// Common input selection; each original source retains its own read law.
    pub(in crate::mir::normal_callable_semantic_package) fn original_object_producer_source_v1(
        &self,
        target: &LexicalInstanceCallSourceTargetV1,
    ) -> Result<Option<&ObjectCallSourceLoanV1>, String> {
        let array = self.original_array_producer_source_v1(target)?;
        let field = self
            .original_field_producer_source_v1(target)?
            .map(|(loan, _)| loan);
        if let (Some(a), Some(b)) = (array, field) {
            if !same_source(a, b) {
                return Err(freeze("object-producer/original-source-ambiguity"));
            }
        }
        Ok(array.or(field))
    }

    pub(in crate::mir::normal_callable_semantic_package) fn original_object_producer_callee_v1(
        &self,
        target: &LexicalInstanceCallSourceTargetV1,
    ) -> Result<bool, String> {
        let mut selected = false;
        for call in self
            .source_incoming
            .exact_rows()
            .filter(|row| row.callee == target.callee_owner())
        {
            let original = call.source.require_instance()?;
            if original.target() != target.target()
                || original.target_batch_slot() != target.target_batch_slot()
                || original.call_site() != &call.call
                || original.callee_owner() != call.callee
            {
                return Err(freeze("object-producer/original-callee-drift"));
            }
            selected |= self.original_object_producer_source_v1(original)?.is_some();
        }
        Ok(selected)
    }

    /// Field producers and consumers borrow the existing original parameter frame.
    pub(in crate::mir::normal_callable_semantic_package) fn original_returned_field_participant_request_v1(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<Option<OwnedExprSiteV1>, String> {
        let consumer = self.original_returned_field_input_request_v1(owner)?;
        let mut producer = None;
        for call in self
            .source_incoming
            .exact_rows()
            .filter(|row| row.callee == owner)
        {
            let Some(target) = call.source.instance() else {
                continue;
            };
            if self.original_field_producer_source_v1(target)?.is_some() {
                producer.get_or_insert_with(|| call.call.clone());
            }
        }
        Ok(consumer.or(producer))
    }
}
