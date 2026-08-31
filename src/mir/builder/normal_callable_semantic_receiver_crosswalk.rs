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
pub(super) enum ExactBindingValueErrorV1 {
    OwnerMismatch,
    ForeignBinding,
    EntryNotInstalled,
    ValueUnavailable,
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
    pub(super) fn value_for_exact_binding(
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
}

#[cfg(test)]
#[path = "normal_callable_semantic_receiver_crosswalk_tests.rs"]
mod tests;
