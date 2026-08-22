//! Key-free required-argument facts issued during the one A observation.

use std::collections::BTreeSet;

use crate::mir::builder::normal_script_direct_static_join_handoff::{
    RequiredArgumentProofArgumentV1, ScriptDirectStaticRequiredArgumentProofDispositionV1,
};
use crate::mir::builder::normal_script_direct_static_join_handoff::issue_node;
use crate::mir::resolved_semantics::VerifiedResolvedScriptV1;
use crate::mir::source_call_target::VerifiedScriptDirectStaticCallLookupRowV1;

use super::model::{CanonicalScriptAIncompleteV1, CanonicalScriptAIssueV1};

pub(super) fn issue_required_argument_source(
    product: &VerifiedResolvedScriptV1,
    lookup: &VerifiedScriptDirectStaticCallLookupRowV1,
) -> Result<ScriptDirectStaticRequiredArgumentProofDispositionV1, CanonicalScriptAIssueV1> {
    if lookup.required_callee_i64_arguments().is_empty() {
        return Ok(ScriptDirectStaticRequiredArgumentProofDispositionV1::ExactI64Empty);
    }

    let mut seen_ordinals = BTreeSet::new();
    let mut seen_sites = BTreeSet::new();
    let mut rows = Vec::with_capacity(lookup.required_callee_i64_arguments().len());
    for ordinal in lookup.required_callee_i64_arguments() {
        if !seen_ordinals.insert(*ordinal) {
            return Err(CanonicalScriptAIssueV1::IntegrityInvalid(
                super::model::CanonicalScriptAIntegrityInvalidV1::DuplicateRequiredArgument(
                    lookup.site().clone(),
                ),
            ));
        }
        let Some(site) = lookup.argument_sites().get(*ordinal as usize) else {
            return Err(CanonicalScriptAIssueV1::Incomplete(
                CanonicalScriptAIncompleteV1::RequiredArgumentOutsideI0 {
                    site: lookup.site().clone(),
                    ordinal: *ordinal,
                    reason: "required argument ordinal is outside the source call".into(),
                },
            ));
        };
        let tree = issue_node(product.expression_source(), site, &mut seen_sites).map_err(
            |reason| {
                CanonicalScriptAIssueV1::Incomplete(
                    CanonicalScriptAIncompleteV1::RequiredArgumentOutsideI0 {
                        site: lookup.site().clone(),
                        ordinal: *ordinal,
                        reason: format!("{reason:?}").into(),
                    },
                )
            },
        )?;
        rows.push(RequiredArgumentProofArgumentV1::from_canonical_source(
            *ordinal,
            site.clone(),
            tree,
        ));
    }
    Ok(
        ScriptDirectStaticRequiredArgumentProofDispositionV1::ExactI64Required(
            rows.into_boxed_slice(),
        ),
    )
}
