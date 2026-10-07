//! Retained received-call source identity; no ownership or entry capability.
use super::*;

impl PrefixLocalFlow<'_> {
    /// Retain the source call that installed the received value. This is not
    /// a direct New acquisition or proof that the callee transferred a Home.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn install_received_handle(
        &mut self,
        binding: BindingRefV1,
        acquisition: &OwnedExprSiteV1,
    ) {
        self.store(
            binding,
            StoredLocal::ReceivedHandle {
                acquisition: acquisition.clone(),
            },
        );
    }

    /// Nullable received values keep their exact call source too. Null still
    /// carries no Home; the existing checked-release obligation is unchanged.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn install_received_nullable(
        &mut self,
        binding: BindingRefV1,
        acquisition: &OwnedExprSiteV1,
    ) {
        self.store(
            binding,
            StoredLocal::ReceivedNullable {
                acquisition: acquisition.clone(),
            },
        );
    }

    /// Exact received-call source, available only while the original binding
    /// remains live and still names this initializer in the same source unit.
    /// The boolean describes nullable source spelling, not ownership authority.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn received_call_acquisition(
        &self,
        binding: BindingRefV1,
    ) -> Option<(&OwnedExprSiteV1, bool)> {
        let (acquisition, nullable) = match self.locals.get(&binding)? {
            StoredLocal::ReceivedHandle { acquisition } => (acquisition, false),
            StoredLocal::ReceivedNullable { acquisition } => (acquisition, true),
            _ => return None,
        };
        if binding.owner() != self.input.owner() || acquisition.owner() != self.input.owner() {
            return None;
        }
        let mut initializers = self
            .input
            .function()
            .expression_source()
            .initializers()
            .filter(|row| row.binding() == binding);
        let row = initializers.next()?;
        if initializers.next().is_some() || row.initializer_site() != Some(acquisition.site()) {
            return None;
        }
        let is_call = self
            .input
            .function()
            .method_calls()
            .any(|(site, _)| site == acquisition.site())
            || self
                .input
                .function()
                .direct_call_observations()
                .any(|(site, _)| site == acquisition.site());
        is_call.then_some((acquisition, nullable))
    }

    /// Borrow the original acquisition row; do not reseal arguments at return.
    /// This joins source identity only. Entry-receiver typed arguments may live
    /// on a separate package row; an empty argument array is not arity proof.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn received_call_observation<'b>(
        &self,
        binding: BindingRefV1,
        calls: &'b [super::super::LocalCallObservationV1],
        covered: &std::collections::BTreeSet<OwnedExprSiteV1>,
    ) -> Option<&'b super::super::LocalCallObservationV1> {
        let (acquisition, nullable) = self.received_call_acquisition(binding)?;
        if !covered.contains(acquisition) {
            return None;
        }
        let expected = if nullable {
            super::super::LocalCallResultClassV1::Nullable
        } else {
            super::super::LocalCallResultClassV1::Handle
        };
        let crate::mir::resolved_semantics::BindingOriginV1::Source(
            declaration @ super::super::SourceBindingSiteV1::Local { statement, .. },
        ) = self.input.function().binding(binding)?.origin()
        else {
            return None;
        };
        let mut rows = calls.iter().filter(|row| {
            row.owner() == self.input.owner()
                && row.site() == acquisition
                && row.statement() == statement
                && row.local_binding().is_some_and(|(source, destination)| {
                    source == declaration && destination == binding
                })
        });
        let row = rows.next()?;
        if rows.next().is_some() || row.result() != expected {
            return None;
        }
        Some(row)
    }
}
