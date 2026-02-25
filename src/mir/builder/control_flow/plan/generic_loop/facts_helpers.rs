//! Utility helpers for generic loop facts extraction

use crate::mir::builder::control_flow::plan::planner::Freeze;

/// Returns `Ok(None)` or `Err(Freeze)` based on strict mode
pub(super) fn reject_or_none<T>(strict: bool, message: &str) -> Result<Option<T>, Freeze> {
    if strict {
        Err(Freeze::ambiguous(message))
    } else {
        Ok(None)
    }
}

/// Returns `Ok(false)` or `Err(Freeze)` based on strict mode
pub(super) fn reject_or_false(strict: bool, message: &str) -> Result<bool, Freeze> {
    if strict {
        Err(Freeze::unsupported(message))
    } else {
        Ok(false)
    }
}
