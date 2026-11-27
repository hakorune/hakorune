//! Join the original entry owner to completed call actuals.
use super::owner_view::BorrowedEntryOwnerViewV1;
use super::*;

/// Canonical source correspondence does not grant an executable actual loan.
#[derive(Debug)]
pub(super) enum BorrowedEntryActualsV1<'a> {
    Executable(Box<[(&'a OwnedExprSiteV1, &'a [PreparedBorrowedFormalActualV1])]>),
    SelectedTagged,
}

impl OrdinaryNewClaimLedgerV1 {
    pub(super) fn entry_actual_loans_v1<'a>(
        &'a self,
        source: &'a PreparedBorrowedFormalIngressV1,
        owner: FunctionOwnerIdV1,
        view: &BorrowedEntryOwnerViewV1<'a>,
    ) -> Result<BorrowedEntryActualsV1<'a>, String> {
        if view.mixed {
            if view.incoming.is_empty() {
                return Ok(BorrowedEntryActualsV1::SelectedTagged);
            }
            return self
                .checked_mixed_instance_actuals_v1(source, owner, view)
                .map(BorrowedEntryActualsV1::Executable);
        }
        if view.tagged {
            return Ok(BorrowedEntryActualsV1::SelectedTagged);
        }
        if self
            .selected_instance_input_protocol_v1(source, owner)?
            .is_some()
        {
            let (selected, signatures) = self
                .original_instance_input_issuance
                .as_ref()
                .ok_or_else(|| freeze("borrowed-entry/original-issuance-missing"))?;
            let catalog = self
                .original_parameter_contracts.as_ref();
            let first = view
                .incoming
                .first()
                .ok_or_else(|| freeze("borrowed-entry/incoming-missing"))?;
            if self
                .checked_completed_original_instance_target_arguments_v1(
                    first.source.require_instance()?,
                    selected,
                    catalog,
                    signatures,
                )?
                .is_none()
            {
                let uses = source.original_formal_uses_v1(owner)?;
                if uses.dynamic_return_requested_v1()
                    && uses.dynamic_return_roots_v1().any(|formal| {
                        source
                            .source_value_sum_v1(formal)
                            .is_some_and(|sum| !sum.finite_input())
                    })
                {
                    return Err(freeze("dynamic-result/unsupported-input-domain"));
                }
                return Err(format!(
                    "{}: {}",
                    freeze("borrowed-entry/original-input-not-ready"),
                    self.typed_object_input_completion_diagnostic_v1(owner)?
                ));
            }
            let mut rows = Vec::new();
            for call in &view.incoming {
                if call.callee != owner {
                    return Err(freeze("borrowed-entry/incoming-identity"));
                }
                let actuals = self
                    .borrowed_formal_actuals
                    .get(&call.call)
                    .ok_or_else(|| freeze("borrowed-entry/actuals-missing"))?
                    .as_ref()
                    .map_err(Clone::clone)?;
                actuals.ordered_arguments_for_v1(call)?;
                self.check_entry_actual_domains_v1(
                    source,
                    view.definition,
                    &actuals.opaque_actuals,
                )?;
                rows.push((&call.call, actuals.opaque_actuals.as_ref()));
            }
            return Ok(BorrowedEntryActualsV1::Executable(rows.into_boxed_slice()));
        }
        self.checked_borrowed_entry_incoming(source, owner)
            .map(BorrowedEntryActualsV1::Executable)
    }
}
