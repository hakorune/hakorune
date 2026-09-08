//! Physical consumption state inside the existing New ledger, not a new issuer.
//! Installed is visible to later-New cleanup before whole-emission validation.
use super::*;

#[derive(Debug)]
pub(super) enum NewEmissionProgress {
    Unprepared,
    RetainedUnavailable {
        progress: UnavailableLocalProgress,
    },
    Prepared {
        operands: Vec<InvokeOperation>,
        reclaim: Option<ReclaimUnpublishedOriginV1>,
    },
    Emitting,
    Emitted {
        result: ValueId,
        arguments: Box<[EmittedNewArgumentV1]>,
        reclaim: Option<ReclaimUnpublishedEmissionV1>,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
        progress: EmittedLocalProgress,
    },
}

#[derive(Debug)]
pub(super) enum EmittedLocalProgress {
    PendingExpression,
    ExpressionCompleted,
    Installed { local: ValueId },
    Checked { local: ValueId },
}

#[derive(Debug)]
pub(super) enum UnavailableLocalProgress {
    PendingExpression,
    ExpressionCompleted { initializer: ValueId },
    Installed { local: ValueId },
}

impl NewEmissionProgress {
    pub(super) fn local(&self) -> Option<ValueId> {
        match self {
            Self::Emitted {
                progress:
                    EmittedLocalProgress::Installed { local } | EmittedLocalProgress::Checked { local },
                ..
            }
            | Self::RetainedUnavailable {
                progress: UnavailableLocalProgress::Installed { local },
            } => Some(*local),
            _ => None,
        }
    }

    pub(super) fn is_complete(&self) -> bool {
        matches!(
            self,
            Self::Emitted {
                progress: EmittedLocalProgress::Checked { .. },
                ..
            } | Self::RetainedUnavailable {
                progress: UnavailableLocalProgress::Installed { .. }
            }
        )
    }

    pub(super) fn completed_initializer(&self) -> Option<ValueId> {
        match self {
            Self::Emitted {
                result,
                progress: EmittedLocalProgress::ExpressionCompleted,
                ..
            } => Some(*result),
            Self::RetainedUnavailable {
                progress: UnavailableLocalProgress::ExpressionCompleted { initializer },
            } => Some(*initializer),
            _ => None,
        }
    }

    pub(super) fn complete_expression(&mut self, value: ValueId) -> Result<(), String> {
        if self.completed_initializer().is_some() || self.local().is_some() {
            return Err(freeze("duplicate-expression-completion"));
        }
        match self {
            Self::Emitted {
                result, progress, ..
            } if *result == value => {
                *progress = EmittedLocalProgress::ExpressionCompleted;
            }
            Self::RetainedUnavailable { progress } => {
                *progress = UnavailableLocalProgress::ExpressionCompleted { initializer: value };
            }
            _ => return Err(freeze("expression-before-emission-or-result-drift")),
        }
        Ok(())
    }

    /// Called only after every local in the statement has passed preflight.
    pub(super) fn install(&mut self, local: ValueId) {
        match self {
            Self::Emitted { progress, .. }
                if matches!(progress, EmittedLocalProgress::ExpressionCompleted) =>
            {
                *progress = EmittedLocalProgress::Installed { local };
            }
            Self::RetainedUnavailable { progress }
                if matches!(
                    progress,
                    UnavailableLocalProgress::ExpressionCompleted { .. }
                ) =>
            {
                *progress = UnavailableLocalProgress::Installed { local };
            }
            _ => unreachable!("local batch preflight establishes expression completion"),
        }
    }

    /// All emitted rows have been validated before this non-fallible batch update.
    pub(super) fn mark_checked(&mut self) {
        if let Self::Emitted { progress, .. } = self {
            let local = match *progress {
                EmittedLocalProgress::Installed { local }
                | EmittedLocalProgress::Checked { local } => local,
                _ => unreachable!("emission validation establishes local installation"),
            };
            *progress = EmittedLocalProgress::Checked { local };
        }
    }
}
