//! PUBLICATION-ADAPTER0: erase the private Raw publication owner once.
//!
//! Publication has already completed every fallible check.  This module only
//! moves the published module and exact Raw verification result into the
//! existing compatibility result, while retaining the complete evidence in a
//! private envelope until its sole consuming erasure terminal.

use super::module_postprocess::{ModulePostprocessScheduleV1, ModuleVerificationEvidenceV1};
use super::raw_root_postprocess::{RawPostprocessEvidenceV1, RawPostprocessRouteEvidenceV1};
use super::raw_root_publication::{
    RawAppPublishedInvocationV1, RawPublishedInvocationCoreV1, RawPublishedInvocationV1,
    RawScriptPublishedInvocationV1,
};
use super::MirCompileResult;
use crate::mir::builder::{
    RawDrainWitnessV1, RawFinalizationParitySealV1, RawPostprocessParitySealV1,
    RawPostprocessProgressV1,
};

#[derive(Debug)]
pub(in crate::mir) struct RawPublicationCompatibilityEnvelopeV1 {
    result: MirCompileResult,
    _evidence: RawPublicationToResultEvidenceV1,
    _seal: RawPublicationCompatibilityEnvelopeSealV1,
}

#[derive(Debug)]
struct RawPublicationCompatibilityEnvelopeSealV1;

#[derive(Debug)]
struct RawPublicationToResultEvidenceV1 {
    _route: RawPostprocessRouteEvidenceV1,
    _witness: RawDrainWitnessV1,
    _finalization_parity: RawFinalizationParitySealV1,
    _postprocess_parity: RawPostprocessParitySealV1,
    _schedule: ModulePostprocessScheduleV1,
    _progress: RawPostprocessProgressV1,
    _publication: super::raw_root_publication::RawPublicationSealV1,
    _verification_projection: RawVerificationProjectionSealV1,
}

#[derive(Debug)]
struct RawVerificationProjectionSealV1 {
    _disposition: RawVerificationDispositionV1,
    _error_count: usize,
}

#[derive(Debug)]
enum RawVerificationDispositionV1 {
    Passed,
    ReportableFailure,
}

impl RawPublishedInvocationV1 {
    pub(in crate::mir) fn into_compatibility_envelope(
        self,
    ) -> RawPublicationCompatibilityEnvelopeV1 {
        match self {
            Self::Script(RawScriptPublishedInvocationV1 { core })
            | Self::App(RawAppPublishedInvocationV1 { core }) => envelope_from_core(core),
        }
    }
}

impl RawPublicationCompatibilityEnvelopeV1 {
    /// The sole authority-erasure terminal for the Raw compatibility result.
    pub(in crate::mir) fn into_compatibility(self) -> MirCompileResult {
        self.result
    }
}

fn envelope_from_core(core: RawPublishedInvocationCoreV1) -> RawPublicationCompatibilityEnvelopeV1 {
    let RawPublishedInvocationCoreV1 {
        _token: _,
        module,
        evidence,
        publication,
    } = core;
    let RawPostprocessEvidenceV1 {
        route,
        schedule,
        verification,
        progress,
        witness,
        finalization_parity,
        postprocess_parity,
    } = evidence;
    let (verification_result, verification_projection) = project_verification(verification);
    let result = MirCompileResult {
        module: module.into_compatibility_module(),
        verification_result,
    };
    let evidence = RawPublicationToResultEvidenceV1 {
        _route: route,
        _witness: witness,
        _finalization_parity: finalization_parity,
        _postprocess_parity: postprocess_parity,
        _schedule: schedule,
        _progress: progress,
        _publication: publication,
        _verification_projection: verification_projection,
    };
    RawPublicationCompatibilityEnvelopeV1 {
        result,
        _evidence: evidence,
        _seal: RawPublicationCompatibilityEnvelopeSealV1,
    }
}

fn project_verification(
    verification: ModuleVerificationEvidenceV1,
) -> (
    Result<(), Vec<crate::mir::verification_types::VerificationError>>,
    RawVerificationProjectionSealV1,
) {
    match verification {
        ModuleVerificationEvidenceV1::Raw { pre_transform } => match pre_transform {
            Ok(()) => (
                Ok(()),
                RawVerificationProjectionSealV1 {
                    _disposition: RawVerificationDispositionV1::Passed,
                    _error_count: 0,
                },
            ),
            Err(errors) => {
                let error_count = errors.len();
                (
                    Err(errors.into_vec()),
                    RawVerificationProjectionSealV1 {
                        _disposition: RawVerificationDispositionV1::ReportableFailure,
                        _error_count: error_count,
                    },
                )
            }
        },
        ModuleVerificationEvidenceV1::Canonical { .. } => {
            unreachable!("Raw publication cannot carry canonical verification evidence")
        }
    }
}
