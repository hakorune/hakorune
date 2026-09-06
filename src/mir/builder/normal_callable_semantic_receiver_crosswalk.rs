//! Checked BindingRef-to-ValueId access for the callable semantic owner.
//!
//! This is a behavior-neutral view of the existing lowering ledger.  It does
//! not consume a binding, choose a receiver, or issue any call target.  The
//! later DeclaredInstance crosswalk may borrow this accessor after it has
//! independently proven the exact source relation.

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
pub(in crate::mir::builder) enum ExactReceiverValueErrorV1 {
    OwnerMismatch,
    ReceiverBindingMismatch,
    ReceiverSiteUnavailable,
    SiteBindingMismatch,
    AlreadyTaken,
    EntryNotInstalled,
    ValueUnavailable,
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

    /// Consume one exact receiver source site and read its already-materialized
    /// value. The binding/value itself stays reusable so repeated `me.method`
    /// calls remain legal; only the source-site observation is one-shot.
    pub(in crate::mir::builder) fn take_exact_receiver_value(
        &mut self,
        expected_owner: FunctionOwnerIdV1,
        receiver_site: &crate::mir::resolved_semantics::SourceNodeSiteV1,
        expected_binding: BindingRefV1,
    ) -> Result<ValueId, ExactReceiverValueErrorV1> {
        if self.owner != expected_owner || expected_binding.owner() != self.owner {
            return Err(ExactReceiverValueErrorV1::OwnerMismatch);
        }
        if self.receiver != Some(expected_binding) {
            return Err(ExactReceiverValueErrorV1::ReceiverBindingMismatch);
        }
        let Some(binding) = self.variables.get(receiver_site).copied() else {
            return Err(ExactReceiverValueErrorV1::ReceiverSiteUnavailable);
        };
        if binding != expected_binding {
            return Err(ExactReceiverValueErrorV1::SiteBindingMismatch);
        }
        if self.consumed_variables.contains(receiver_site) {
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
        if !self.consumed_variables.insert(receiver_site.clone()) {
            return Err(ExactReceiverValueErrorV1::AlreadyTaken);
        }
        Ok(value)
    }
}

#[cfg(test)]
#[path = "normal_callable_semantic_receiver_crosswalk_tests.rs"]
mod tests;
