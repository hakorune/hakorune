//! JoinIR merge dev logging helpers (SSOT)

use crate::config::env::joinir_dev_enabled;

/// Dev logging is enabled when explicit debug is on or NYASH_JOINIR_DEV is set.
pub(super) fn dev_enabled(debug: bool) -> bool {
    debug || joinir_dev_enabled()
}
