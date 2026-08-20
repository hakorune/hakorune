//! Disconnected NormalFile-to-source-plan consuming boundary.
//!
//! This owner moves one already parsed source into the source-family
//! classifier. It retains the sealed entry profile and read/parse receipt,
//! but does not inspect the profile or connect a compiler/runtime route.

use super::{
    NormalFileSourceReceiptV1, PreparedNormalFileSourceSealV1, PreparedNormalFileSourceV1,
    SealedNormalEntryProfileV1,
};
use crate::mir::normal_source_plan::{
    NormalSourcePlanClassifierV1, NormalSourcePlanErrorV1, NormalSourcePlanIdentityFieldV1,
    NormalSourcePlanStageV1, PreparedNormalSourcePlanInputV1, RejectedNormalSourcePlanV1,
    SealedNormalSourcePlanV1,
};
use crate::mir::{
    CanonicalCoreSourcePlanCompileRequestV1, NormalSourcePlanReceiptV1,
    VerifiedCanonicalCoreSourcePlanAdmissionV1,
};
use hakorune_frontend_parser::parser::GrammarProfile;

#[derive(Debug)]
pub(crate) struct PreparedNormalFileSourcePlanRequestV1 {
    input: PreparedNormalSourcePlanInputV1,
    profile: SealedNormalEntryProfileV1,
    receipt: NormalFileSourceReceiptV1,
    _seal: PreparedNormalFileSourcePlanRequestSealV1,
}

#[derive(Debug)]
struct PreparedNormalFileSourcePlanRequestSealV1;

#[derive(Debug)]
pub(crate) struct ClassifiedNormalFileSourcePlanV1 {
    plan: SealedNormalSourcePlanV1,
    profile: SealedNormalEntryProfileV1,
    receipt: NormalFileSourceReceiptV1,
    _seal: ClassifiedNormalFileSourcePlanSealV1,
}

#[derive(Debug)]
struct ClassifiedNormalFileSourcePlanSealV1;

#[derive(Debug)]
pub(crate) struct RejectedNormalFileSourcePlanningV1 {
    rejected: RejectedNormalSourcePlanV1,
    profile: SealedNormalEntryProfileV1,
    receipt: NormalFileSourceReceiptV1,
}

/// The canonical-core front door rejected a classified plan before compiler
/// dispatch. The original classified owner remains intact for inspection or
/// one consuming discard; this terminal never retries another profile.
#[derive(Debug)]
pub(crate) struct RejectedCanonicalCoreSourcePlanHandoffV1 {
    owner: ClassifiedNormalFileSourcePlanV1,
    error: CanonicalCoreSourcePlanHandoffErrorV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CanonicalCoreSourcePlanHandoffErrorV1 {
    ProfileExcludesCanonicalCore,
}

impl PreparedNormalFileSourceV1 {
    pub(crate) fn prepare_source_plan_request(self) -> PreparedNormalFileSourcePlanRequestV1 {
        let Self {
            source_file,
            parser_source_handoff,
            _seal: PreparedNormalFileSourceSealV1,
        } = self;
        let display_identity = source_file.to_string_lossy().into_owned().into_boxed_str();
        let (callable_source, profile, receipt) = parser_source_handoff.into_parts();
        PreparedNormalFileSourcePlanRequestV1 {
            input: PreparedNormalSourcePlanInputV1::from_parser_callable_source(
                callable_source,
                display_identity,
            ),
            profile,
            receipt,
            _seal: PreparedNormalFileSourcePlanRequestSealV1,
        }
    }
}

impl PreparedNormalFileSourcePlanRequestV1 {
    pub(crate) fn classify(
        self,
    ) -> Result<ClassifiedNormalFileSourcePlanV1, RejectedNormalFileSourcePlanningV1> {
        let Self {
            input,
            profile,
            receipt,
            _seal: _,
        } = self;
        if let Err(error) = validate_parser_identity(&input, &receipt) {
            return Err(RejectedNormalFileSourcePlanningV1 {
                rejected: RejectedNormalSourcePlanV1::new(input, error),
                profile,
                receipt,
            });
        }
        match NormalSourcePlanClassifierV1::seal(input) {
            Ok(plan) => Ok(ClassifiedNormalFileSourcePlanV1 {
                plan,
                profile,
                receipt,
                _seal: ClassifiedNormalFileSourcePlanSealV1,
            }),
            Err(rejected) => Err(RejectedNormalFileSourcePlanningV1 {
                rejected,
                profile,
                receipt,
            }),
        }
    }
}

fn validate_parser_identity(
    input: &PreparedNormalSourcePlanInputV1,
    receipt: &NormalFileSourceReceiptV1,
) -> Result<(), NormalSourcePlanErrorV1> {
    if !input.is_parser_source_backed() {
        return Err(if input.parser_lineage().is_some() {
            NormalSourcePlanErrorV1::CompatibilitySourceUnavailable
        } else {
            NormalSourcePlanErrorV1::SourceAuthorityUnavailable
        });
    }
    let Some(lineage) = input.parser_lineage() else {
        return Err(NormalSourcePlanErrorV1::SourceLineageUnavailable);
    };
    if lineage.source_identity() != receipt.source_identity.as_ref() {
        return Err(NormalSourcePlanErrorV1::SourceIdentityMismatch {
            field: NormalSourcePlanIdentityFieldV1::SourceIdentity,
        });
    }
    if lineage.source_digest() != receipt.source_digest {
        return Err(NormalSourcePlanErrorV1::SourceIdentityMismatch {
            field: NormalSourcePlanIdentityFieldV1::Digest,
        });
    }
    if lineage.grammar_profile() != GrammarProfile::Canonical {
        return Err(NormalSourcePlanErrorV1::SourceIdentityMismatch {
            field: NormalSourcePlanIdentityFieldV1::GrammarProfile,
        });
    }
    if lineage.utf8_len() != receipt.utf8_len {
        return Err(NormalSourcePlanErrorV1::SourceIdentityMismatch {
            field: NormalSourcePlanIdentityFieldV1::Utf8Length,
        });
    }
    let (read_count, parse_count) = lineage.receipt_counts();
    if read_count != receipt.read_count {
        return Err(NormalSourcePlanErrorV1::SourceIdentityMismatch {
            field: NormalSourcePlanIdentityFieldV1::ReadCount,
        });
    }
    if parse_count != receipt.parse_count {
        return Err(NormalSourcePlanErrorV1::SourceIdentityMismatch {
            field: NormalSourcePlanIdentityFieldV1::ParseCount,
        });
    }
    Ok(())
}

impl ClassifiedNormalFileSourcePlanV1 {
    pub(crate) fn plan(&self) -> &SealedNormalSourcePlanV1 {
        &self.plan
    }

    /// Move a canonical-core classified plan into the compiler without
    /// inspecting whether it is Script, Main, or a callable module.
    pub(crate) fn into_canonical_core_compile_request(
        self,
    ) -> Result<CanonicalCoreSourcePlanCompileRequestV1, RejectedCanonicalCoreSourcePlanHandoffV1>
    {
        if !self.profile.is_canonical_core() {
            return Err(RejectedCanonicalCoreSourcePlanHandoffV1 {
                owner: self,
                error: CanonicalCoreSourcePlanHandoffErrorV1::ProfileExcludesCanonicalCore,
            });
        }
        let Self {
            plan,
            profile: _,
            receipt,
            _seal: _,
        } = self;
        let receipt = NormalSourcePlanReceiptV1::one_read_one_parse(
            receipt.source_identity,
            receipt.source_digest,
            receipt.utf8_len,
            receipt.read_count,
            receipt.parse_count,
        );
        Ok(CanonicalCoreSourcePlanCompileRequestV1::new(
            plan,
            VerifiedCanonicalCoreSourcePlanAdmissionV1::seal_from_frontdoor_profile(),
            receipt,
        ))
    }

    #[cfg(test)]
    fn receipt_counts(&self) -> (u8, u8) {
        (self.receipt.read_count, self.receipt.parse_count)
    }

    #[cfg(test)]
    fn retained_source_identity(&self) -> &str {
        &self.receipt.source_identity
    }

    #[cfg(test)]
    fn source_digest(&self) -> crate::mir::CanonicalSourceBytesDigestV1 {
        self.receipt.source_digest
    }

    #[cfg(test)]
    pub(crate) fn is_canonical_core_profile_for_test(&self) -> bool {
        self.profile.is_canonical_core()
    }

    #[cfg(test)]
    fn retains_parser_postpass(&self) -> bool {
        self.plan.has_parser_postpass()
    }
}

impl RejectedCanonicalCoreSourcePlanHandoffV1 {
    pub(crate) const fn error(&self) -> CanonicalCoreSourcePlanHandoffErrorV1 {
        self.error
    }

    pub(crate) fn discard(self) {
        drop(self);
    }
}

impl RejectedNormalFileSourcePlanningV1 {
    pub(crate) fn stage(&self) -> &NormalSourcePlanStageV1 {
        self.rejected.stage()
    }

    pub(crate) fn error(&self) -> &NormalSourcePlanErrorV1 {
        self.rejected.error()
    }

    pub(crate) fn discard(self) {
        let Self {
            rejected,
            profile: _,
            receipt: _,
        } = self;
        rejected.discard();
    }

    #[cfg(test)]
    fn receipt_counts(&self) -> (u8, u8) {
        (self.receipt.read_count, self.receipt.parse_count)
    }
}

#[cfg(test)]
#[path = "source_plan_input_tests.rs"]
mod tests;
