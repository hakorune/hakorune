use super::{GlobalCallShapeBlocker, GlobalCallTargetFacts};

fn direct_unknown_global_target_blocker(
    name: &str,
    target: &GlobalCallTargetFacts,
) -> GlobalCallShapeBlocker {
    GlobalCallShapeBlocker {
        symbol: target.symbol().unwrap_or(name).to_string(),
        reason: target.shape_reason(),
    }
}

pub(super) fn propagated_unknown_global_target_blocker(
    name: &str,
    target: &GlobalCallTargetFacts,
) -> GlobalCallShapeBlocker {
    if let Some(symbol) = target.shape_blocker_symbol() {
        return GlobalCallShapeBlocker {
            symbol: symbol.to_string(),
            reason: target
                .shape_blocker_reason()
                .or_else(|| target.shape_reason()),
        };
    }
    direct_unknown_global_target_blocker(name, target)
}
