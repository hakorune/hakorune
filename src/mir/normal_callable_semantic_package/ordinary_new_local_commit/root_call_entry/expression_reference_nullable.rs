//! Original nullable producer dispatch; neither source lends a payload view.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::NullableResultReferenceV1;

impl CallValidationContextV1<'_> {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn nullable_result_read_v1(
        self, source: &NullableResultReferenceV1, read: &ExactLexicalReadV1,
        function: &MirFunction, finishing: Option<&Finished>,
    ) -> Result<(ValueId, [Binding; 2]), String> {
        let pair = match source {
            NullableResultReferenceV1::StoredMapGet(frame) =>
                return self.stored_map_get_read_v1(frame, read, function, finishing),
            NullableResultReferenceV1::DeclaredFrom(view) => {
                self.validate_declared_read_transport_v1(view.source(), read.binding_observation_v2(), function, finishing)?;
                self.declared_result_read_v1(view, read)?
            },
        };
        if let Some(finishing) = finishing {
            finishing.required_effect_binding(function, &pair.1[0])?;
            finishing.relation_binding(function, &pair.1[1])?;
            for original in read.original_copy_dependencies_v1()? {
                finishing.relation_binding(function, original)?;
            }
        }
        Ok(pair)
    }
    pub(in crate::mir) fn declared_result_read_v1(self,
        view: &crate::mir::resolved_semantics::home_new_prefix::BorrowedResultReadV1,
        read: &ExactLexicalReadV1,
    ) -> Result<(ValueId, [Binding; 2]), String> {
        let value = read.value_for(view.read().owner(), view.read().site().node(), view.binding())
            .map_err(|_| freeze("expression/reference-source-node"))?;
        if let Some(point) = read.binding_observation_v2().source_read_branch_v1()? {
            if !point.source().matches_read_v1(view) { return Err(freeze("expression/reference-source-node")); }
        }
        let (current, pair) = self.declared_result_binding_v1(view.source(), read.binding_observation_v2())?;
        if value != current { return Err(freeze("expression/reference-source-node")); }
        Ok((value, pair))
    }
    pub(in crate::mir) fn declared_result_binding_v1(self,
        loan: &crate::mir::normal_callable_semantic_package::BorrowedNullableLocalSourceLoanV1,
        read: &BindingValueObservationV2,
    ) -> Result<(ValueId, [Binding; 2]), String> {
        let value = read.value_for(loan.call().owner(), read.binding_value().0)
            .map_err(|_| freeze("expression/reference-source-node"))?;
        if let Some(point) = read.source_read_branch_v1()? {
            return Ok((value, point.original_pair_v1(loan, self)?));
        }
        if let Some(frame) = read.source_declared_read_rebind_v1()? {
            return self.with_original_rebind_v1(&frame, |packet| {
                let rhs = frame.declared_read_v1().ok_or_else(|| freeze("expression/reference-source-node"))?;
                if rhs.source() != loan || !std::rc::Rc::ptr_eq(packet.frame(), &frame) {
                    return Err(freeze("expression/reference-source-node"));
                }
                let original = packet.reference_read_v1(0).ok_or_else(|| freeze("expression/reference-source-node"))?;
                let (rhs_value, pair) = self.declared_result_read_v1(rhs, original.exact_read_v1())?;
                let (assigned, _) = read.loan_checked_rebind_v1(&frame)?;
                if assigned != packet.value() || assigned != rhs_value { return Err(freeze("expression/reference-source-node")); }
                Ok((value, pair))
            });
        }
        let original = self.ledger().borrowed_nullable_call_source_v1(loan.call())
            .ok_or_else(|| freeze("expression/reference-source-node"))?;
        if original.borrowed_nullable() != Some(loan)
            || original.local_binding().map(|(_, binding)| binding) != Some(loan.destination()) {
            return Err(freeze("expression/reference-source-node"));
        }
        self.with_taken_packet_v1(loan.call(), |packet| {
            let row = packet.original_row()?;
            if row.source_target().borrowed_nullable_local_source() != Some(loan)
                || self.ledger().corroborate_borrowed_nullable_call_v1(row)? != loan.result_kind_v1() {
                return Err(freeze("expression/reference-source-node"));
            }
            let original_value = packet.value_in_context_v1(original.owner(), original.arguments(), self)?;
            read.loan_forwarded_initializer_v1(Some(original_value))?;
            let (invoke, normal) = packet.outer_bindings()?;
            Ok((value, [invoke.clone(), normal.clone()]))
        })
    }

    pub(in crate::mir) fn validate_declared_read_transport_v1(self,
        loan: &crate::mir::normal_callable_semantic_package::BorrowedNullableLocalSourceLoanV1,
        read: &BindingValueObservationV2, function: &MirFunction, finishing: Option<&Finished>,
    ) -> Result<(), String> {
        self.declared_result_binding_v1(loan, read)?;
        if let Some(point) = read.source_read_branch_v1()? { point.validate_v1(self, function, finishing)?; }
        if let Some(frame) = read.source_declared_read_rebind_v1()? {
            self.with_original_rebind_v1(&frame, |packet| {
                let rhs = frame.declared_read_v1().ok_or_else(|| freeze("expression/reference-source-node"))?;
                let original = packet.reference_read_v1(0).ok_or_else(|| freeze("expression/reference-source-node"))?;
                self.validate_declared_read_transport_v1(rhs.source(), original.exact_read_v1().binding_observation_v2(), function, finishing)
            })?;
        }
        for original in read.original_copy_dependencies_v1()? {
            if let Some(finishing) = finishing { finishing.relation_binding(function, original)?; }
            else if !super::super::super::physical_boundary::check_binding(function, None, original.0, &original.1)? {
                return Err(freeze("expression/reference-dependency-missing"));
            }
        }
        Ok(())
    }

}
