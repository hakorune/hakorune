//! Disconnected NormalFile-to-source-plan consuming boundary.
//!
//! This owner moves one already parsed source into the source-family
//! classifier. It retains the sealed entry profile and read/parse receipt,
//! but does not inspect the profile or connect a compiler/runtime route.

use super::script_source_input::CanonicalScriptSourceInputDispositionV1;
use super::{
    NormalFileSourceReceiptV1, PreparedNormalFileParsedRouteV1, PreparedNormalFileSourceV1,
    SealedNormalEntryProfileV1,
};
use crate::mir::normal_source_plan::{
    NormalSourcePlanClassifierV1, NormalSourcePlanErrorV1, NormalSourcePlanIdentityFieldV1,
    NormalSourcePlanStageV1, RejectedNormalSourcePlanV1, SealedNormalSourcePlanV1,
};
use crate::mir::{
    CanonicalCoreSourcePlanCompileRequestV1, CanonicalScriptSourceAInputTransportV1,
    NormalSourcePlanReceiptV1, VerifiedCanonicalCoreSourcePlanAdmissionV1,
};
use crate::parser::callable_parameter_source::ParsedProgramWithCallableParameterSourceV1;
use crate::parser::{
    NormalParserSourceLineageErrorV1, NormalParserSourceLineageV1,
    ParserNormalRootSourcePlanConsumeErrorV1, ParserNormalRootSourcePlanConsumerV1,
    RejectedParserNormalRootSourcePlanConsumptionV1, SourcePlanBoundNormalCallableSourceV1,
};
use hakorune_frontend_parser::parser::GrammarProfile;

#[derive(Debug)]
pub(crate) struct PreparedNormalFileSourcePlanRequestV1 {
    input: PreparedNormalFileSourcePlanAuthorityV1,
    script_input: CanonicalScriptSourceInputDispositionV1,
    profile: SealedNormalEntryProfileV1,
    receipt: NormalFileSourceReceiptV1,
    _seal: PreparedNormalFileSourcePlanRequestSealV1,
}

#[derive(Debug)]
struct PreparedNormalFileSourcePlanRequestSealV1;

#[derive(Debug)]
enum PreparedNormalFileSourcePlanAuthorityV1 {
    ParserBound(SourcePlanBoundNormalCallableSourceV1),
    ParserRejected(RejectedParserNormalRootSourcePlanConsumptionV1),
    LineageRejected(ParsedProgramWithCallableParameterSourceV1),
}

#[derive(Debug)]
pub(crate) struct ClassifiedNormalFileSourcePlanV1 {
    plan: SealedNormalSourcePlanV1,
    script_input: CanonicalScriptSourceInputDispositionV1,
    profile: SealedNormalEntryProfileV1,
    receipt: NormalFileSourceReceiptV1,
    _seal: ClassifiedNormalFileSourcePlanSealV1,
}

#[derive(Debug)]
struct ClassifiedNormalFileSourcePlanSealV1;

#[derive(Debug)]
pub(crate) struct RejectedNormalFileSourcePlanningV1 {
    owner: RejectedNormalFileSourcePlanningOwnerV1,
    script_input: CanonicalScriptSourceInputDispositionV1,
    profile: SealedNormalEntryProfileV1,
    receipt: NormalFileSourceReceiptV1,
}

#[derive(Debug)]
enum RejectedNormalFileSourcePlanningOwnerV1 {
    Policy(RejectedNormalSourcePlanV1),
    Parser {
        rejected: RejectedParserNormalRootSourcePlanConsumptionV1,
        stage: NormalSourcePlanStageV1,
        error: NormalSourcePlanErrorV1,
    },
    Lineage {
        source: ParsedProgramWithCallableParameterSourceV1,
        stage: NormalSourcePlanStageV1,
        error: NormalSourcePlanErrorV1,
    },
}

/// The canonical-core front door rejected a classified plan before compiler
/// dispatch. The original classified owner remains intact for inspection or
/// one consuming discard; this terminal never retries another profile.
#[derive(Debug)]
pub(crate) struct RejectedCanonicalCoreSourcePlanHandoffV1 {
    owner: ClassifiedNormalFileSourcePlanV1,
    error: CanonicalCoreSourcePlanHandoffErrorV1,
}

#[derive(Debug)]
pub(crate) struct RejectedNormalFileSourcePlanRouteV1 {
    owner: PreparedNormalFileSourceV1,
    error: NormalFileSourcePlanRouteErrorV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NormalFileSourcePlanRouteErrorV1 {
    ProfileExcludesCanonicalCore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CanonicalCoreSourcePlanHandoffErrorV1 {
    ProfileExcludesCanonicalCore,
}

impl PreparedNormalFileSourceV1 {
    pub(crate) fn prepare_source_plan_request(
        self,
    ) -> Result<PreparedNormalFileSourcePlanRequestV1, RejectedNormalFileSourcePlanRouteV1> {
        let Self { route, _seal } = self;
        match route {
            PreparedNormalFileParsedRouteV1::Canonical {
                source_file,
                source,
            } => {
                drop((source_file, _seal));
                Ok(source.into_source_plan_request())
            }
            PreparedNormalFileParsedRouteV1::Raw {
                source_file,
                source,
            } => Err(RejectedNormalFileSourcePlanRouteV1 {
                owner: PreparedNormalFileSourceV1 {
                    route: PreparedNormalFileParsedRouteV1::Raw {
                        source_file,
                        source,
                    },
                    _seal,
                },
                error: NormalFileSourcePlanRouteErrorV1::ProfileExcludesCanonicalCore,
            }),
        }
    }
}

impl RejectedNormalFileSourcePlanRouteV1 {
    pub(crate) const fn error(&self) -> NormalFileSourcePlanRouteErrorV1 {
        self.error
    }

    pub(crate) fn discard(self) {
        let Self { owner, error } = self;
        match error {
            NormalFileSourcePlanRouteErrorV1::ProfileExcludesCanonicalCore => {}
        }
        owner.discard_at_named_terminal();
    }
}

impl PreparedNormalFileSourcePlanRequestV1 {
    pub(super) fn from_parser_product(
        source: ParsedProgramWithCallableParameterSourceV1,
        script_input: CanonicalScriptSourceInputDispositionV1,
        profile: SealedNormalEntryProfileV1,
        receipt: NormalFileSourceReceiptV1,
    ) -> Self {
        let lineage = NormalParserSourceLineageV1::issue(
            receipt.source_identity.clone(),
            receipt.source_digest,
            GrammarProfile::Canonical,
            receipt.utf8_len,
            receipt.read_count,
            receipt.parse_count,
        );
        let input = match lineage {
            Ok(lineage) => {
                match ParserNormalRootSourcePlanConsumerV1::consume_once(source, lineage) {
                    Ok(source) => PreparedNormalFileSourcePlanAuthorityV1::ParserBound(source),
                    Err(rejected) => {
                        PreparedNormalFileSourcePlanAuthorityV1::ParserRejected(rejected)
                    }
                }
            }
            Err(error) => {
                discard_lineage_issue_at_named_terminal(error);
                PreparedNormalFileSourcePlanAuthorityV1::LineageRejected(source)
            }
        };
        Self {
            input,
            script_input,
            profile,
            receipt,
            _seal: PreparedNormalFileSourcePlanRequestSealV1,
        }
    }

    pub(crate) fn classify(
        self,
    ) -> Result<ClassifiedNormalFileSourcePlanV1, RejectedNormalFileSourcePlanningV1> {
        let Self {
            input,
            script_input,
            profile,
            receipt,
            _seal: _,
        } = self;
        let input = match input {
            PreparedNormalFileSourcePlanAuthorityV1::ParserBound(source) => source,
            PreparedNormalFileSourcePlanAuthorityV1::ParserRejected(rejected) => {
                let error = map_parser_consume_error(rejected.error());
                return Err(RejectedNormalFileSourcePlanningV1 {
                    owner: RejectedNormalFileSourcePlanningOwnerV1::Parser {
                        rejected,
                        stage: error.stage(),
                        error,
                    },
                    script_input,
                    profile,
                    receipt,
                });
            }
            PreparedNormalFileSourcePlanAuthorityV1::LineageRejected(source) => {
                let error = NormalSourcePlanErrorV1::SourceLineageUnavailable;
                return Err(RejectedNormalFileSourcePlanningV1 {
                    owner: RejectedNormalFileSourcePlanningOwnerV1::Lineage {
                        source,
                        stage: error.stage(),
                        error,
                    },
                    script_input,
                    profile,
                    receipt,
                });
            }
        };
        if let Err(error) = validate_parser_identity(&input, &receipt) {
            return Err(RejectedNormalFileSourcePlanningV1 {
                owner: RejectedNormalFileSourcePlanningOwnerV1::Policy(
                    NormalSourcePlanClassifierV1::reject_parser_bound(input, error),
                ),
                script_input,
                profile,
                receipt,
            });
        }
        match NormalSourcePlanClassifierV1::seal_parser_bound(input) {
            Ok(plan) => Ok(ClassifiedNormalFileSourcePlanV1 {
                plan,
                script_input,
                profile,
                receipt,
                _seal: ClassifiedNormalFileSourcePlanSealV1,
            }),
            Err(rejected) => Err(RejectedNormalFileSourcePlanningV1 {
                owner: RejectedNormalFileSourcePlanningOwnerV1::Policy(rejected),
                script_input,
                profile,
                receipt,
            }),
        }
    }
}

fn validate_parser_identity(
    input: &SourcePlanBoundNormalCallableSourceV1,
    receipt: &NormalFileSourceReceiptV1,
) -> Result<(), NormalSourcePlanErrorV1> {
    let lineage = input.lineage();
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

fn map_parser_consume_error(
    error: ParserNormalRootSourcePlanConsumeErrorV1,
) -> NormalSourcePlanErrorV1 {
    match error {
        ParserNormalRootSourcePlanConsumeErrorV1::CompatibilitySourceUnavailable => {
            NormalSourcePlanErrorV1::CompatibilitySourceUnavailable
        }
        ParserNormalRootSourcePlanConsumeErrorV1::SourceAuthorityUnavailable => {
            NormalSourcePlanErrorV1::SourceAuthorityUnavailable
        }
        ParserNormalRootSourcePlanConsumeErrorV1::Incomplete => {
            NormalSourcePlanErrorV1::ParserSourceIncomplete
        }
        ParserNormalRootSourcePlanConsumeErrorV1::IntegrityInvalid => {
            NormalSourcePlanErrorV1::ParserSourceIntegrityInvalid
        }
    }
}

impl ClassifiedNormalFileSourcePlanV1 {
    pub(crate) fn plan(&self) -> &SealedNormalSourcePlanV1 {
        &self.plan
    }

    pub(crate) fn script_input(&self) -> &CanonicalScriptSourceInputDispositionV1 {
        &self.script_input
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
            script_input,
            profile,
            receipt,
            _seal,
        } = self;
        profile.discard_after_canonical_admission();
        drop(_seal);
        let script_input: CanonicalScriptSourceAInputTransportV1 =
            script_input.into_compiler_transport();
        let receipt = receipt.into_source_plan_receipt();
        Ok(CanonicalCoreSourcePlanCompileRequestV1::new(
            plan,
            VerifiedCanonicalCoreSourcePlanAdmissionV1::seal_from_frontdoor_profile(),
            receipt,
            script_input,
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
        let Self { owner, error } = self;
        discard_canonical_handoff_error_at_named_terminal(error);
        owner.discard_before_a_consumer();
    }
}

impl ClassifiedNormalFileSourcePlanV1 {
    fn discard_before_a_consumer(self) {
        let Self {
            plan,
            script_input,
            profile,
            receipt,
            _seal,
        } = self;
        plan.discard_before_dispatch();
        script_input.discard_before_a_consumer();
        profile.discard_after_source_plan_terminal();
        receipt.discard_after_source_plan_terminal();
        drop(_seal);
    }
}

impl RejectedNormalFileSourcePlanningV1 {
    pub(crate) fn stage(&self) -> &NormalSourcePlanStageV1 {
        match &self.owner {
            RejectedNormalFileSourcePlanningOwnerV1::Policy(rejected) => rejected.stage(),
            RejectedNormalFileSourcePlanningOwnerV1::Parser { stage, .. }
            | RejectedNormalFileSourcePlanningOwnerV1::Lineage { stage, .. } => stage,
        }
    }

    pub(crate) fn error(&self) -> &NormalSourcePlanErrorV1 {
        match &self.owner {
            RejectedNormalFileSourcePlanningOwnerV1::Policy(rejected) => rejected.error(),
            RejectedNormalFileSourcePlanningOwnerV1::Parser { error, .. }
            | RejectedNormalFileSourcePlanningOwnerV1::Lineage { error, .. } => error,
        }
    }

    pub(crate) fn discard(self) {
        let Self {
            owner,
            script_input,
            profile,
            receipt,
        } = self;
        script_input.discard_before_a_consumer();
        profile.discard_after_source_plan_terminal();
        receipt.discard_after_source_plan_terminal();
        match owner {
            RejectedNormalFileSourcePlanningOwnerV1::Policy(rejected) => rejected.discard(),
            RejectedNormalFileSourcePlanningOwnerV1::Parser {
                rejected,
                stage,
                error,
            } => {
                discard_source_plan_rejection_observation(stage, error);
                rejected.discard();
            }
            RejectedNormalFileSourcePlanningOwnerV1::Lineage {
                source,
                stage,
                error,
            } => {
                discard_source_plan_rejection_observation(stage, error);
                source.discard_after_source_plan_rejection();
            }
        }
    }

    #[cfg(test)]
    fn receipt_counts(&self) -> (u8, u8) {
        (self.receipt.read_count, self.receipt.parse_count)
    }
}

impl SealedNormalEntryProfileV1 {
    fn discard_after_canonical_admission(self) {
        drop(self);
    }

    pub(super) fn discard_after_source_plan_terminal(self) {
        drop(self);
    }
}

impl NormalFileSourceReceiptV1 {
    fn into_source_plan_receipt(self) -> NormalSourcePlanReceiptV1 {
        let Self {
            source_identity,
            source_digest,
            utf8_len,
            read_count,
            parse_count,
            _seal,
        } = self;
        drop(_seal);
        NormalSourcePlanReceiptV1::one_read_one_parse(
            source_identity,
            source_digest,
            utf8_len,
            read_count,
            parse_count,
        )
    }

    pub(super) fn discard_after_source_plan_terminal(self) {
        drop(self);
    }
}

fn discard_lineage_issue_at_named_terminal(error: NormalParserSourceLineageErrorV1) {
    match error {
        NormalParserSourceLineageErrorV1::InvalidReadParseReceipt
        | NormalParserSourceLineageErrorV1::EmptySourceIdentity => {}
    }
}

fn discard_canonical_handoff_error_at_named_terminal(error: CanonicalCoreSourcePlanHandoffErrorV1) {
    match error {
        CanonicalCoreSourcePlanHandoffErrorV1::ProfileExcludesCanonicalCore => {}
    }
}

fn discard_source_plan_rejection_observation(
    stage: NormalSourcePlanStageV1,
    error: NormalSourcePlanErrorV1,
) {
    drop((stage, error));
}

#[cfg(test)]
#[path = "source_plan_input_tests.rs"]
mod tests;
