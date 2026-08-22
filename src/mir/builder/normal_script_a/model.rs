//! Private linear products for canonical Script A and C.

use std::collections::BTreeMap;

use crate::mir::builder::normal_script_direct_static_join_handoff::ScriptDirectStaticRequiredArgumentProofDispositionV1;
use crate::mir::builder::normal_script_root_demand_window::PreparedScriptRootAdmissionV1;
use crate::mir::builder::normal_script_semantic_lowering_input::{
    CanonicalScriptANonDirectRowV1, CanonicalScriptCNoDirectClaimsV1,
};
use crate::mir::builder::normal_script_semantic_source::ScriptSemanticSourcePreEffectPartsV1;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, SourceExprSiteV1};
use crate::mir::source_call_target::VerifiedScriptDirectStaticCallLookupRowV1;
use crate::parser::ParserInvocationWitnessV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum CanonicalScriptAIncompleteV1 {
    ScriptRootMissing,
    ScriptRootNotScript,
    MethodRowMissing(SourceExprSiteV1),
    ContinuationRowMissing(SourceExprSiteV1),
    LookupRowMissing(SourceExprSiteV1),
    ResultOutsideExactI64(SourceExprSiteV1),
    RequiredArgumentOutsideI0 {
        site: SourceExprSiteV1,
        ordinal: u32,
        reason: Box<str>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum CanonicalScriptAIntegrityInvalidV1 {
    ForeignInvocation,
    RootOwnerMismatch,
    CoverageCardinalityMismatch,
    LookupCardinalityMismatch,
    CallSiteMismatch(SourceExprSiteV1),
    ReceiverMismatch(SourceExprSiteV1),
    ArgumentMismatch(SourceExprSiteV1),
    ResultMismatch(SourceExprSiteV1),
    TargetMismatch(SourceExprSiteV1),
    LookupRowUnexpected(SourceExprSiteV1),
    DuplicateRequiredArgument(SourceExprSiteV1),
    TerminalMismatch(SourceExprSiteV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum CanonicalScriptAIssueV1 {
    Incomplete(CanonicalScriptAIncompleteV1),
    IntegrityInvalid(CanonicalScriptAIntegrityInvalidV1),
}

#[derive(Debug)]
pub(in crate::mir::builder) struct CanonicalScriptADirectRowsV1 {
    source_owner: FunctionOwnerIdV1,
    observed_method_calls: usize,
    lookup_rows: BTreeMap<SourceExprSiteV1, VerifiedScriptDirectStaticCallLookupRowV1>,
    non_direct_rows: BTreeMap<SourceExprSiteV1, CanonicalScriptANonDirectRowV1>,
    required_argument_rows: BTreeMap<
        SourceExprSiteV1,
        ScriptDirectStaticRequiredArgumentProofDispositionV1,
    >,
}

impl CanonicalScriptADirectRowsV1 {
    pub(in crate::mir::builder) fn new(
        source_owner: FunctionOwnerIdV1,
        observed_method_calls: usize,
        lookup_rows: BTreeMap<SourceExprSiteV1, VerifiedScriptDirectStaticCallLookupRowV1>,
        non_direct_rows: BTreeMap<SourceExprSiteV1, CanonicalScriptANonDirectRowV1>,
        required_argument_rows: BTreeMap<
            SourceExprSiteV1,
            ScriptDirectStaticRequiredArgumentProofDispositionV1,
        >,
    ) -> Self {
        Self {
            source_owner,
            observed_method_calls,
            lookup_rows,
            non_direct_rows,
            required_argument_rows,
        }
    }

    pub(in crate::mir::builder) fn into_parts(
        self,
    ) -> (
        FunctionOwnerIdV1,
        BTreeMap<SourceExprSiteV1, VerifiedScriptDirectStaticCallLookupRowV1>,
        BTreeMap<SourceExprSiteV1, CanonicalScriptANonDirectRowV1>,
        BTreeMap<
            SourceExprSiteV1,
            ScriptDirectStaticRequiredArgumentProofDispositionV1,
        >,
    ) {
        (
            self.source_owner,
            self.lookup_rows,
            self.non_direct_rows,
            self.required_argument_rows,
        )
    }

    #[cfg(test)]
    pub(super) fn candidate_count(&self) -> usize {
        self.lookup_rows.len()
    }

    #[cfg(test)]
    pub(super) const fn observed_method_calls(&self) -> usize {
        self.observed_method_calls
    }

    #[cfg(test)]
    pub(super) fn non_direct_count(&self) -> usize {
        self.non_direct_rows.len()
    }
}

#[derive(Debug)]
pub(in crate::mir::builder) enum CanonicalScriptCDispositionV1 {
    NonDirect(CanonicalScriptCNoDirectClaimsV1),
    DirectStatic(CanonicalScriptADirectRowsV1),
}

#[derive(Debug)]
pub(in crate::mir::builder) struct CanonicalScriptCTransportV1 {
    source_window: PreparedScriptRootAdmissionV1,
    invocation: ParserInvocationWitnessV1,
    parts: ScriptSemanticSourcePreEffectPartsV1,
    disposition: CanonicalScriptCDispositionV1,
    _seal: CanonicalScriptCTransportSealV1,
}

#[derive(Debug)]
struct CanonicalScriptCTransportSealV1;

impl CanonicalScriptCTransportV1 {
    pub(in crate::mir::builder) fn new(
        source_window: PreparedScriptRootAdmissionV1,
        invocation: ParserInvocationWitnessV1,
        parts: ScriptSemanticSourcePreEffectPartsV1,
        disposition: CanonicalScriptCDispositionV1,
    ) -> Self {
        Self {
            source_window,
            invocation,
            parts,
            disposition,
            _seal: CanonicalScriptCTransportSealV1,
        }
    }

    pub(in crate::mir::builder) fn split_for_work_plan(
        self,
    ) -> (
        PreparedScriptRootAdmissionV1,
        CanonicalScriptCPostWindowTransportV1,
    ) {
        let Self {
            source_window,
            invocation,
            parts,
            disposition,
            ..
        } = self;
        (
            source_window,
            CanonicalScriptCPostWindowTransportV1 {
                invocation,
                parts,
                disposition,
            },
        )
    }

    #[cfg(test)]
    pub(super) fn disposition_counts(&self) -> (usize, usize) {
        match &self.disposition {
            CanonicalScriptCDispositionV1::NonDirect(witness) => {
                (0, witness.non_direct_count())
            }
            CanonicalScriptCDispositionV1::DirectStatic(rows) => {
                (rows.candidate_count(), rows.non_direct_count())
            }
        }
    }
}

#[derive(Debug)]
pub(in crate::mir::builder) struct CanonicalScriptCPostWindowTransportV1 {
    invocation: ParserInvocationWitnessV1,
    parts: ScriptSemanticSourcePreEffectPartsV1,
    disposition: CanonicalScriptCDispositionV1,
}

impl CanonicalScriptCPostWindowTransportV1 {
    pub(in crate::mir::builder) fn into_parts(
        self,
    ) -> (
        ParserInvocationWitnessV1,
        ScriptSemanticSourcePreEffectPartsV1,
        CanonicalScriptCDispositionV1,
    ) {
        (self.invocation, self.parts, self.disposition)
    }
}
