//! Checked BindingRef-to-ValueId access for the callable semantic owner.
//!
//! Exact reads consume source sites, never the reusable binding value. Read
//! preparation checks source and current provenance without consuming a site;
//! a complete batch is rechecked before any commit. No value class, receiver
//! selection, call target or runtime ownership is issued here.

use std::fmt;

use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1};
use crate::mir::ValueId;

use super::CallableSemanticLoweringState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum ExactBindingValueErrorV1 {
    OwnerMismatch,
    ForeignBinding,
    EntryNotInstalled,
    ValueUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum ExactReceiverValueErrorV1 {
    OwnerMismatch,
    ReceiverBindingMismatch,
    ReceiverSiteUnavailable,
    SiteBindingMismatch,
    AlreadyTaken,
    EntryNotInstalled,
    ValueUnavailable,
    DuplicateReadSite,
    ReadSnapshotChanged,
}

/// The original exact lexical read, retained for physical corroboration.
/// Only this state module can construct it; it issues no source meaning.
#[derive(Debug, PartialEq)]
pub(in crate::mir) struct ExactLexicalReadV1 {
    site: crate::mir::resolved_semantics::SourceNodeSiteV1,
    binding_value: BindingValueObservationV2,
}

/// An immutable observation of the existing value slot, without a source read
/// site. It cannot consume a read or issue a source/physical Loop role.
#[derive(Debug, Clone, PartialEq)]
pub(in crate::mir) struct BindingValueObservationV2 {
    owner: FunctionOwnerIdV1,
    binding: BindingRefV1,
    value: ValueId,
    provenance: Option<super::materialized_values::LocalProvenanceV1>,
}

/// Non-consuming current-state snapshot. Only a whole validated commit turns
/// this into the one-shot physical observation; it grants no value class.
#[derive(Debug)]
pub(in crate::mir::builder) struct PreparedExactLexicalReadV1 {
    read: ExactLexicalReadV1,
}
impl PreparedExactLexicalReadV1 {
    /// Immutable pre-commit correspondence; lending it consumes no read site.
    pub(in crate::mir::builder) fn observation_v1(&self) -> &ExactLexicalReadV1 { &self.read }

    pub(in crate::mir::builder) fn loan_local_initializer_v1(&self, root: BindingRefV1)
        -> Result<(ValueId, &[(crate::mir::BasicBlockId, crate::mir::MirInstruction)]), String> {
        self.read.loan_local_initializer_v1(root)
    }
    pub(in crate::mir::builder) fn value_for(
        &self, owner: FunctionOwnerIdV1,
        site: &crate::mir::resolved_semantics::SourceNodeSiteV1, binding: BindingRefV1,
    ) -> Result<ValueId, ExactReceiverValueErrorV1> {
        self.read.value_for(owner, site, binding)
    }
}

impl BindingValueObservationV2 {
    pub(in crate::mir) fn binding_value(&self) -> (BindingRefV1, ValueId) { (self.binding, self.value) }
    pub(in crate::mir) fn loan_local_initializer_v1(&self, root: BindingRefV1)
        -> Result<(ValueId, &[(crate::mir::BasicBlockId, crate::mir::MirInstruction)]), String> {
        if root.owner() != self.owner { return Err("[freeze:contract][local-read/root-owner]".into()); }
        self.provenance.as_ref().ok_or_else(|| "[freeze:contract][local-read/initializer-missing]".to_owned())?
            .loan_initializer_v1(root, self.value)
    }
    pub(in crate::mir) fn loan_checked_rebind_v1(&self,
        frame: &std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedBindingRebindFrameV1>,
    ) -> Result<(ValueId, &[(crate::mir::BasicBlockId, crate::mir::MirInstruction)]), String> {
        if frame.target().owner() != self.owner { return Err("[freeze:contract][local-read/rebind-owner]".into()); }
        self.provenance.as_ref().ok_or_else(|| "[freeze:contract][local-read/rebind-missing]".to_owned())?
            .loan_checked_rebind_v1(frame, self.value)
    }
    pub(in crate::mir) fn original_copy_dependencies_v1(&self)
        -> Result<&[(crate::mir::BasicBlockId, crate::mir::MirInstruction)], String> {
        match &self.provenance {
            Some(source) => source.original_copy_dependencies_v1(self.value),
            None => Ok(&[]),
        }
    }
    pub(in crate::mir) fn proves_forwarded_entry(
        &self,
        formal: BindingRefV1,
        value: ValueId,
    ) -> bool {
        self.loan_forwarded_copies(formal, value).is_ok()
    }

    pub(in crate::mir) fn loan_forwarded_copies(
        &self,
        formal: BindingRefV1,
        value: ValueId,
    ) -> Result<&[(crate::mir::BasicBlockId, crate::mir::MirInstruction)], String> {
        if formal.owner() != self.owner {
            return Err("[freeze:contract][forwarded-copy/owner]".into());
        }
        if self.binding == formal && self.value == value {
            return Ok(&[]);
        }
        self.provenance
            .as_ref()
            .ok_or_else(|| "[freeze:contract][forwarded-copy/proof-missing]".to_string())?
            .loan_copies(formal, value, self.value)
    }

    pub(in crate::mir) fn value_for(&self, owner: FunctionOwnerIdV1, binding: BindingRefV1)
        -> Result<ValueId, ExactReceiverValueErrorV1> {
        if self.owner != owner || binding.owner() != owner { return Err(ExactReceiverValueErrorV1::OwnerMismatch); }
        if self.binding != binding { return Err(ExactReceiverValueErrorV1::SiteBindingMismatch); }
        Ok(self.value)
    }
}

impl ExactLexicalReadV1 {
    pub(in crate::mir) fn binding_observation_v2(&self) -> &BindingValueObservationV2 { &self.binding_value }
    pub(in crate::mir) fn loan_local_initializer_v1(&self, root: BindingRefV1)
        -> Result<(ValueId, &[(crate::mir::BasicBlockId, crate::mir::MirInstruction)]), String> {
        self.binding_value.loan_local_initializer_v1(root)
    }
    pub(in crate::mir) fn loan_checked_rebind_v1(&self,
        frame: &std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedBindingRebindFrameV1>,
    ) -> Result<(ValueId, &[(crate::mir::BasicBlockId, crate::mir::MirInstruction)]), String> {
        self.binding_value.loan_checked_rebind_v1(frame)
    }
    pub(in crate::mir) fn original_copy_dependencies_v1(&self)
        -> Result<&[(crate::mir::BasicBlockId, crate::mir::MirInstruction)], String> {
        self.binding_value.original_copy_dependencies_v1()
    }
    pub(in crate::mir) fn source_value(&self) -> (crate::mir::resolved_semantics::OwnedExprSiteV1, BindingRefV1, ValueId) {
        (crate::mir::resolved_semantics::OwnedExprSiteV1::new(self.binding_value.owner,
            crate::mir::resolved_semantics::SourceExprSiteV1::from_node(self.site.clone())), self.binding_value.binding, self.binding_value.value)
    }
    pub(in crate::mir) fn proves_forwarded_entry(&self, formal: BindingRefV1, value: ValueId) -> bool {
        self.binding_value.proves_forwarded_entry(formal, value)
    }
    pub(in crate::mir) fn loan_forwarded_copies(&self, formal: BindingRefV1, value: ValueId)
        -> Result<&[(crate::mir::BasicBlockId, crate::mir::MirInstruction)], String> {
        self.binding_value.loan_forwarded_copies(formal, value)
    }
    pub(in crate::mir) fn value_for(&self, owner: FunctionOwnerIdV1,
        site: &crate::mir::resolved_semantics::SourceNodeSiteV1, binding: BindingRefV1,
    ) -> Result<ValueId, ExactReceiverValueErrorV1> {
        // Preserve owner-before-site diagnostic precedence.
        self.binding_value.value_for(owner, binding)?;
        if &self.site != site { return Err(ExactReceiverValueErrorV1::SiteBindingMismatch); }
        Ok(self.binding_value.value)
    }
}

impl fmt::Display for ExactReceiverValueErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let reason = match self {
            Self::OwnerMismatch => "exact-receiver-owner-mismatch",
            Self::ReceiverBindingMismatch => "exact-receiver-binding-mismatch",
            Self::ReceiverSiteUnavailable => "exact-receiver-site-unavailable",
            Self::SiteBindingMismatch => "exact-receiver-site-binding-mismatch",
            Self::AlreadyTaken => "exact-receiver-already-taken",
            Self::EntryNotInstalled => "exact-receiver-entry-not-installed",
            Self::ValueUnavailable => "exact-receiver-value-unavailable",
            Self::DuplicateReadSite => "exact-lexical-read-duplicate-site",
            Self::ReadSnapshotChanged => "exact-lexical-read-snapshot-changed",
        };
        write!(
            formatter,
            "[freeze:contract][callable-semantic-lowering/{reason}]"
        )
    }
}

impl fmt::Display for ExactBindingValueErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let reason = match self {
            Self::OwnerMismatch => "exact-binding-owner-mismatch",
            Self::ForeignBinding => "exact-binding-foreign",
            Self::EntryNotInstalled => "exact-binding-entry-not-installed",
            Self::ValueUnavailable => "exact-binding-value-unavailable",
        };
        write!(
            formatter,
            "[freeze:contract][callable-semantic-lowering/{reason}]"
        )
    }
}

impl CallableSemanticLoweringState {
    /// Retain the SAME one-shot variable read, including its actual reaching
    /// provenance. Assignment-target reads keep read_variable's existing law.
    pub(in crate::mir::builder) fn read_variable_with_source_v1(
        &mut self, site: &crate::mir::resolved_semantics::SourceNodeSiteV1,
    ) -> Result<ExactLexicalReadV1, String> {
        let binding = self.source_read_binding(site)?;
        let value = self.read_variable(site)?;
        Ok(ExactLexicalReadV1 { site: site.clone(), binding_value: BindingValueObservationV2 {
            owner: self.owner, binding, value, provenance: self.values.provenance(&binding) } })
    }

    /// Read an already-materialized value by its owner-branded binding.
    ///
    /// The operation is observational and deliberately reusable.  The caller
    /// must supply the exact owner and binding relation; no name, position,
    /// AST, or receiver inference is performed here.
    pub(in crate::mir::builder) fn value_for_exact_binding(
        &self,
        expected_owner: FunctionOwnerIdV1,
        expected_binding: BindingRefV1,
    ) -> Result<ValueId, ExactBindingValueErrorV1> {
        if self.owner != expected_owner {
            return Err(ExactBindingValueErrorV1::OwnerMismatch);
        }
        if expected_binding.owner() != self.owner {
            return Err(ExactBindingValueErrorV1::ForeignBinding);
        }
        if !self.entry_installed {
            return Err(ExactBindingValueErrorV1::EntryNotInstalled);
        }
        self.values
            .get(&expected_binding)
            .copied()
            .ok_or(ExactBindingValueErrorV1::ValueUnavailable)
    }

    /// Observe the existing binding slot without creating/consuming a read site.
    pub(in crate::mir::builder) fn observe_binding_value_v2(&self, binding: BindingRefV1)
        -> Result<BindingValueObservationV2, ExactBindingValueErrorV1> {
        let value = self.value_for_exact_binding(self.owner, binding)?;
        Ok(BindingValueObservationV2 { owner: self.owner, binding, value,
            provenance: self.values.provenance(&binding) })
    }

    /// Consume one exact receiver source site and read its already-materialized
    /// value. The binding/value itself stays reusable so repeated `me.method`
    /// calls remain legal; only the source-site observation is one-shot.
    pub(in crate::mir::builder) fn take_exact_receiver_value(
        &mut self,
        expected_owner: FunctionOwnerIdV1,
        receiver_site: &crate::mir::resolved_semantics::SourceNodeSiteV1,
        expected_binding: BindingRefV1,
    ) -> Result<ValueId, ExactReceiverValueErrorV1> {
        self.take_exact_receiver_read(expected_owner, receiver_site, expected_binding)?
            .value_for(expected_owner, receiver_site, expected_binding)
    }

    /// Keep the same one-shot read for the downstream physical call packet.
    pub(in crate::mir::builder) fn take_exact_receiver_read(
        &mut self,
        expected_owner: FunctionOwnerIdV1,
        receiver_site: &crate::mir::resolved_semantics::SourceNodeSiteV1,
        expected_binding: BindingRefV1,
    ) -> Result<ExactLexicalReadV1, ExactReceiverValueErrorV1> {
        if self.owner != expected_owner || expected_binding.owner() != self.owner {
            return Err(ExactReceiverValueErrorV1::OwnerMismatch);
        }
        if self.receiver != Some(expected_binding) {
            return Err(ExactReceiverValueErrorV1::ReceiverBindingMismatch);
        }
        self.take_exact_lexical_read(expected_owner, receiver_site, expected_binding)
    }

    /// Consume one exact lexical source read without requiring the callable's
    /// entry receiver slot.  Static roots use this for a local object receiver;
    /// instance bodies continue to use `take_exact_receiver_value` above.
    pub(in crate::mir::builder) fn take_exact_lexical_value(
        &mut self,
        expected_owner: FunctionOwnerIdV1,
        source_site: &crate::mir::resolved_semantics::SourceNodeSiteV1,
        expected_binding: BindingRefV1,
    ) -> Result<ValueId, ExactReceiverValueErrorV1> {
        self.take_exact_lexical_read(expected_owner, source_site, expected_binding)?
            .value_for(expected_owner, source_site, expected_binding)
    }

    pub(in crate::mir::builder) fn prepare_exact_lexical_read(
        &self,
        expected_owner: FunctionOwnerIdV1,
        source_site: &crate::mir::resolved_semantics::SourceNodeSiteV1,
        expected_binding: BindingRefV1,
    ) -> Result<PreparedExactLexicalReadV1, ExactReceiverValueErrorV1> {
        if self.owner != expected_owner || expected_binding.owner() != self.owner {
            return Err(ExactReceiverValueErrorV1::OwnerMismatch);
        }
        let Some(binding) = self.variables.get(source_site).copied() else {
            return Err(ExactReceiverValueErrorV1::ReceiverSiteUnavailable);
        };
        if binding != expected_binding {
            return Err(ExactReceiverValueErrorV1::SiteBindingMismatch);
        }
        if self.consumed_variables.contains(source_site) {
            return Err(ExactReceiverValueErrorV1::AlreadyTaken);
        }
        let value = self
            .value_for_exact_binding(expected_owner, expected_binding)
            .map_err(|error| match error {
                ExactBindingValueErrorV1::EntryNotInstalled => {
                    ExactReceiverValueErrorV1::EntryNotInstalled
                }
                ExactBindingValueErrorV1::ValueUnavailable => {
                    ExactReceiverValueErrorV1::ValueUnavailable
                }
                ExactBindingValueErrorV1::OwnerMismatch => ExactReceiverValueErrorV1::OwnerMismatch,
                ExactBindingValueErrorV1::ForeignBinding => {
                    ExactReceiverValueErrorV1::ReceiverBindingMismatch
                }
            })?;
        Ok(PreparedExactLexicalReadV1 { read: ExactLexicalReadV1 {
            site: source_site.clone(),
            binding_value: BindingValueObservationV2 { owner: expected_owner,
                binding: expected_binding, value, provenance: self.values.provenance(&expected_binding) },
        } })
    }

    /// All checks finish before the first consumption. A late unavailable,
    /// changed or repeated read cannot leave an earlier site consumed.
    pub(in crate::mir::builder) fn commit_exact_lexical_reads(
        &mut self, prepared: Vec<PreparedExactLexicalReadV1>,
    ) -> Result<Vec<ExactLexicalReadV1>, ExactReceiverValueErrorV1> {
        let mut sites = std::collections::BTreeSet::new();
        for pending in &prepared {
            let read = &pending.read;
            if !sites.insert(&read.site) {
                return Err(ExactReceiverValueErrorV1::DuplicateReadSite);
            }
            let current = self.prepare_exact_lexical_read(read.binding_value.owner, &read.site, read.binding_value.binding)?;
            if current.read != *read {
                return Err(ExactReceiverValueErrorV1::ReadSnapshotChanged);
            }
        }
        Ok(prepared.into_iter().map(|pending| self.commit_prepared_read(pending)).collect())
    }

    pub(in crate::mir::builder) fn take_exact_lexical_read(
        &mut self, expected_owner: FunctionOwnerIdV1,
        source_site: &crate::mir::resolved_semantics::SourceNodeSiteV1,
        expected_binding: BindingRefV1,
    ) -> Result<ExactLexicalReadV1, ExactReceiverValueErrorV1> {
        let pending = self.prepare_exact_lexical_read(expected_owner, source_site, expected_binding)?;
        // No state mutation or callback separates this prepare from commit.
        Ok(self.commit_prepared_read(pending))
    }

    fn commit_prepared_read(&mut self, pending: PreparedExactLexicalReadV1) -> ExactLexicalReadV1 {
        let inserted = self.consumed_variables.insert(pending.read.site.clone());
        debug_assert!(inserted, "complete exact-read preflight precedes every commit");
        pending.read
    }

}

#[cfg(test)]
#[path = "normal_callable_semantic_receiver_crosswalk_tests.rs"]
mod tests;
