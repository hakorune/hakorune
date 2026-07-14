//! Disconnected pure ownership-transition planning for canonical local bindings.

#![allow(dead_code, unused_imports)]

mod assignment;
mod error;
mod scope_exit;
mod value;

#[cfg(test)]
mod tests;

use assignment::{
    plan_assignment, plan_declaration, AssignmentOwnershipPlanV1, InstallBindingOwnershipPlanV1,
    ReplaceBindingOwnershipPlanV1,
};
use error::OwnershipTransitionErrorV1;
use scope_exit::{
    plan_function_exit, plan_scope_close, plan_unpublished_draft_discard,
    FunctionExitOwnershipPlanV1, FunctionTerminalOwnershipV1, FunctionTerminalResultPlanV1,
    OwnedBindingAtCloseV1, ScopeCloseOwnershipPlanV1, ScopeResultOwnershipPlanV1,
    ScopeTailOwnershipV1, UnpublishedDraftDiscardOwnershipPlanV1,
};
use value::{
    LocalBindingClassV1, LocalBindingSubjectV1, LoweredValueOwnershipV1, NextBindingValuePlanV1,
    OwnedValueIdV1,
};
