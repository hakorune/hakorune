use super::model::{GlobalCallShapeBlocker, GlobalCallTargetShapeReason};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct GenericPureStringReject {
    pub(super) reason: GlobalCallTargetShapeReason,
    pub(super) blocker: Option<GlobalCallShapeBlocker>,
}

impl GenericPureStringReject {
    pub(super) fn new(reason: GlobalCallTargetShapeReason) -> Self {
        Self {
            reason,
            blocker: None,
        }
    }

    pub(super) fn with_blocker(
        reason: GlobalCallTargetShapeReason,
        symbol: impl Into<String>,
        blocker_reason: Option<GlobalCallTargetShapeReason>,
    ) -> Self {
        Self {
            reason,
            blocker: Some(GlobalCallShapeBlocker {
                symbol: symbol.into(),
                reason: blocker_reason,
            }),
        }
    }

    pub(super) fn with_shape_blocker(
        reason: GlobalCallTargetShapeReason,
        blocker: GlobalCallShapeBlocker,
    ) -> Self {
        Self {
            reason,
            blocker: Some(blocker),
        }
    }
}
