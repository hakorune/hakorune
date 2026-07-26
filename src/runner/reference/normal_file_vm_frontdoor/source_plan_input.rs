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
    NormalSourcePlanClassifierV1, NormalSourcePlanErrorV1, NormalSourcePlanStageV1,
    PreparedNormalSourcePlanInputV1, RejectedNormalSourcePlanV1, SealedNormalSourcePlanV1,
};

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

impl PreparedNormalFileSourceV1 {
    pub(crate) fn prepare_source_plan_request(self) -> PreparedNormalFileSourcePlanRequestV1 {
        let Self {
            source_file,
            ast,
            profile,
            receipt,
            _seal: PreparedNormalFileSourceSealV1,
        } = self;
        let display_identity = source_file.to_string_lossy().into_owned().into_boxed_str();
        PreparedNormalFileSourcePlanRequestV1 {
            input: PreparedNormalSourcePlanInputV1::new(ast, display_identity),
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

impl ClassifiedNormalFileSourcePlanV1 {
    pub(crate) fn plan(&self) -> &SealedNormalSourcePlanV1 {
        &self.plan
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
    pub(crate) fn is_canonical_core_profile_for_test(&self) -> bool {
        self.profile.is_canonical_core()
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
