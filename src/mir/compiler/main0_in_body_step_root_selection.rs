//! Caller-zero pre-wrapper selection for the Main0 in-body-step profile.
//!
//! The installed package lends the App Main root input through a one-shot
//! loan. Route selection must classify that source before the legacy
//! wrapper `main` function opens, without consuming the loan: this module
//! borrows the same input through the observation API, runs the
//! facts/map/co-seal chain, and returns the owned semantic-program product
//! when the exact profile is admitted. Whichever downstream branch runs
//! still performs the single loan consumption.
//!
//! A facts decline is the only quiet outcome: it means the source is simply
//! not this profile, and the legacy path keeps its unconsumed loan. Once
//! the facts admit the shape, any later disagreement between the resolver
//! ledger and the admitted facts is a hard contract failure and rejects.

use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::normal_callable_semantic_package::{
    NormalCallableSemanticPackageInstallIssueV1, NormalCallableSemanticPackagePortV1,
};
use crate::parser::CallableDeclarationIdentityV1;

use super::main0_in_body_step_recipe_coseal::{
    issue_main0_in_body_step_recipe_v1, Main0InBodyStepCoSealRejectV1,
    VerifiedMain0InBodyStepRecipeProductV1,
};
use super::main0_in_body_step_source_map::Main0InBodyStepSourceMapRejectV1;
use super::main0_in_body_step_source_map_issue::issue_main0_in_body_step_source_map_v1;
use super::main0_in_body_step_syntax_facts::issue_main0_in_body_step_syntax_facts_from_ledger_v1;

/// Result of observing the installed App Main root for the Main0
/// in-body-step profile.
#[derive(Debug)]
pub(crate) enum Main0InBodyStepSourceSelectionV1 {
    /// The exact profile was admitted and the complete co-sealed semantic
    /// program product is owned by the selected canonical-root path.
    Selected(VerifiedMain0InBodyStepRecipeProductV1),
    /// The installed App Main root is not this profile. The one-shot loan is
    /// still unconsumed; the existing wrapper path keeps it.
    Unselected,
}

/// Hard selection failures. Profile mismatch is not an error; these cover
/// loan mechanics and post-admission contract disagreements only.
#[derive(Debug)]
pub(crate) enum Main0InBodyStepSourceSelectionRejectV1 {
    RootLoan(NormalCallableSemanticPackageInstallIssueV1),
    SourceMap(Main0InBodyStepSourceMapRejectV1),
    CoSeal(Main0InBodyStepCoSealRejectV1),
}

/// Classify the installed App Main root source for the Main0 in-body-step
/// profile without consuming the one-shot loan.
///
/// `expected_key` and `expected_identity` are the catalog's source-backed
/// App Main relation rows; the port re-validates them against the batch so
/// a foreign or mismatched root rejects instead of silently declining.
pub(crate) fn select_main0_in_body_step_source_v1(
    port: &NormalCallableSemanticPackagePortV1<'_>,
    expected_key: &CanonicalSameModuleCallableKeyV1,
    expected_identity: &CallableDeclarationIdentityV1,
) -> Result<Main0InBodyStepSourceSelectionV1, Main0InBodyStepSourceSelectionRejectV1> {
    port.observe_app_main_root_source_v1(expected_key, expected_identity, |input, _identity| {
        let Ok(ledger) = input.forest().callable_source_ledger(input.owner()) else {
            // No resolver ledger means this source cannot be the selected
            // profile; leave the loan unconsumed for the existing path.
            return Ok(Main0InBodyStepSourceSelectionV1::Unselected);
        };
        let facts = match issue_main0_in_body_step_syntax_facts_from_ledger_v1(input, &ledger) {
            Ok(facts) => facts,
            Err(_) => return Ok(Main0InBodyStepSourceSelectionV1::Unselected),
        };
        let map = issue_main0_in_body_step_source_map_v1(&ledger, facts)
            .map_err(Main0InBodyStepSourceSelectionRejectV1::SourceMap)?;
        let product = issue_main0_in_body_step_recipe_v1(&ledger, map)
            .map_err(Main0InBodyStepSourceSelectionRejectV1::CoSeal)?;
        Ok(Main0InBodyStepSourceSelectionV1::Selected(product))
    })
    .map_err(Main0InBodyStepSourceSelectionRejectV1::RootLoan)?
}
