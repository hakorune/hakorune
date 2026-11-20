//! Source-to-lowering handoff for the shared comparison writer.
use super::*;
use crate::mir::builder::ops::CompletedOrdinaryComparisonV1;
use crate::mir::resolved_semantics::{
    borrowed_integer_comparison::BorrowedIntegerComparisonSourceV1, OwnedExprSiteV1,
    SourceExprSiteV1,
};

impl RawInvocationChildPortV1<'_, '_> {
    pub(super) fn borrowed_integer_comparison_at_site(
        &self,
    ) -> Result<Option<BorrowedIntegerComparisonSourceV1>, String> {
        let (Some(ledger), Some(owner), Some(node)) = (
            self.ordinary_new_claim_ledger.as_ref(),
            self.callable_owner_v1(),
            self.current_source_site_v1(),
        ) else {
            return Ok(None);
        };
        if !ledger.has_borrowed_ordinary_entry_v1(owner) {
            return Ok(None);
        }
        let site = OwnedExprSiteV1::new(owner, SourceExprSiteV1::from_node(node));
        Ok(ledger
            .completion_for_owner(owner)
            .and_then(|completion| completion.cleanup().root_flow())
            .and_then(|flow| flow.comparison_prefix(&site))
            .map(|row| row.source.clone()))
    }

    pub(super) fn lower_borrowed_integer_comparison(
        &mut self,
        builder: &mut MirBuilder,
        source: BorrowedIntegerComparisonSourceV1,
        children: (ValueId, ValueId),
    ) -> Result<CompletedOrdinaryComparisonV1, String> {
        let ledger = self
            .ordinary_new_claim_ledger
            .as_ref()
            .ok_or_else(missing)?;
        let entry = ledger.borrowed_ordinary_entry_values_v1(source.site().owner())?;
        let tagged = entry
            .iter()
            .find_map(|(ordinal, formal, value)| {
                (*ordinal == source.ordinal() && *formal == source.formal()).then_some(*value)
            })
            .ok_or_else(missing)?;
        let (observed, other) = if source.tagged_on_left() {
            children
        } else {
            (children.1, children.0)
        };
        let function = builder
            .function_state
            .current_function
            .as_ref()
            .ok_or_else(missing)?;
        if !crate::mir::verification::borrowed_integer_comparison::is_original_read(
            function, tagged, observed,
        ) {
            return Err(missing());
        }

        let completion = ledger
            .completion_for_owner(source.site().owner())
            .ok_or_else(missing)?;
        let flow = completion.cleanup().root_flow().ok_or_else(missing)?;
        let prefix = flow
            .comparison_prefix(source.site())
            .filter(|row| row.source == source)
            .ok_or_else(missing)?;
        let unwind = ledger.prior_home_unwind_for(&prefix.homes)?;
        let state = self.source_ledger.callable().ok_or_else(missing)?;
        let frame = state.borrow_mut().borrow_fault_frame(builder)?;
        let owner = source.site().owner();
        let completed = builder
            .build_borrowed_integer_comparison(source, tagged, observed, other, frame, unwind)?;
        let function = builder
            .function_state
            .current_function
            .as_ref()
            .ok_or_else(missing)?;
        let frame_binding = state.borrow().fault_frame_binding(function)?;
        let check =
            crate::mir::verification::borrowed_integer_comparison::ComparisonBinding::for_result(
                function,
                completed.value,
            )
            .ok_or_else(missing)?;
        ledger.record_borrowed_integer_check(owner, check, frame_binding)?;
        Ok(completed)
    }
}

fn missing() -> String {
    "[freeze:contract][borrowed-integer/source-or-cleanup]".into()
}
