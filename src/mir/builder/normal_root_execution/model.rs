use super::super::PreparedAdmittedNormalRootExpansionV1;
use crate::parser::{ParserNormalRootExecutionSourceV1, VerifiedFinalCallableProgramSourceV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum AdmittedNormalRootExecutionModeV1 {
    App,
    ProgramRuntime,
}

/// Affine pre-effect owner for one parser-preserved normal root.
#[derive(Debug)]
pub(in crate::mir) struct PreparedNormalRootExecutionConsumptionV1 {
    source: VerifiedFinalCallableProgramSourceV1,
    mode: AdmittedNormalRootExecutionModeV1,
    root_execution: PreparedAdmittedNormalRootExpansionV1,
    _seal: PreparedNormalRootExecutionConsumptionSealV1,
}

#[derive(Debug)]
struct PreparedNormalRootExecutionConsumptionSealV1;

/// Final callable source after the sole normal-root consumer has admitted it.
///
/// Production semantic-package issuance accepts this affine owner instead of
/// a raw final parser product, so root admission cannot be silently bypassed.
#[derive(Debug)]
pub(in crate::mir) struct ConsumedNormalRootCallableSourceV1 {
    source: VerifiedFinalCallableProgramSourceV1,
    mode: AdmittedNormalRootExecutionModeV1,
    root_execution: PreparedAdmittedNormalRootExpansionV1,
    _seal: ConsumedNormalRootCallableSourceSealV1,
}

#[derive(Debug)]
struct ConsumedNormalRootCallableSourceSealV1;

impl PreparedNormalRootExecutionConsumptionV1 {
    pub(super) fn issue(
        source: VerifiedFinalCallableProgramSourceV1,
        mode: AdmittedNormalRootExecutionModeV1,
        root_execution: PreparedAdmittedNormalRootExpansionV1,
    ) -> Self {
        Self {
            source,
            mode,
            root_execution,
            _seal: PreparedNormalRootExecutionConsumptionSealV1,
        }
    }

    pub(in crate::mir) const fn mode(&self) -> AdmittedNormalRootExecutionModeV1 {
        self.mode
    }

    pub(in crate::mir) fn into_consumed_source(self) -> ConsumedNormalRootCallableSourceV1 {
        ConsumedNormalRootCallableSourceV1 {
            source: self.source,
            mode: self.mode,
            root_execution: self.root_execution,
            _seal: ConsumedNormalRootCallableSourceSealV1,
        }
    }

    #[cfg(test)]
    pub(in crate::mir) fn consume_at_named_test_terminal(
        self,
    ) -> AdmittedNormalRootExecutionModeV1 {
        let Self {
            source,
            mode,
            root_execution,
            _seal,
        } = self;
        root_execution.discard_unconnected();
        source.discard_at_named_root_execution_terminal();
        mode
    }
}

impl ConsumedNormalRootCallableSourceV1 {
    pub(in crate::mir) const fn mode(&self) -> AdmittedNormalRootExecutionModeV1 {
        self.mode
    }

    pub(in crate::mir) fn source(&self) -> &VerifiedFinalCallableProgramSourceV1 {
        &self.source
    }

    pub(in crate::mir) fn root_source(&self) -> &ParserNormalRootExecutionSourceV1 {
        self.source
            .normal_root_execution()
            .ready_source()
            .expect("consumed normal-root source stores only preserved Ready")
    }

    /// Sole move boundary into the callable semantic-package issuer.
    ///
    /// The callback receives the intact final source and its one pre-effect
    /// root projection together. No independent getter can orphan or re-pair
    /// either side.
    pub(in crate::mir) fn consume_into_semantic_package<R>(
        self,
        consume: impl FnOnce(
            VerifiedFinalCallableProgramSourceV1,
            PreparedAdmittedNormalRootExpansionV1,
        ) -> R,
    ) -> R {
        consume(self.source, self.root_execution)
    }
}
