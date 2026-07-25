//! Explicit `ny_main` capability boundary for normalized process status.
//!
//! This adapter is intentionally disconnected from native/LLVM entry code.
//! Its only input is an already-projected `ProcessExitCodeV1`; source values,
//! objects, and legacy status rules cannot cross this boundary.

use super::source_entry_result::ProcessExitCodeV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) struct NyMainStatusV1 {
    code: ProcessExitCodeV1,
    _seal: NyMainStatusSealV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NyMainStatusSealV1;

pub(in crate::mir) struct NyMainCapabilityAdapterV1;

impl NyMainCapabilityAdapterV1 {
    /// Accept only normalized status; no source-result or backend lookup occurs.
    pub(in crate::mir) const fn accept(code: ProcessExitCodeV1) -> NyMainStatusV1 {
        NyMainStatusV1 {
            code,
            _seal: NyMainStatusSealV1,
        }
    }
}

impl NyMainStatusV1 {
    /// The normalized status consumed by a later native ABI adapter.
    pub(in crate::mir) const fn normalized_i64(self) -> i64 {
        self.code.normalized_i64()
    }

    pub(in crate::mir) const fn code(self) -> ProcessExitCodeV1 {
        self.code
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adapter_accepts_only_normalized_zero_and_byte_status() {
        let zero = NyMainCapabilityAdapterV1::accept(ProcessExitCodeV1::zero());
        let max = NyMainCapabilityAdapterV1::accept(ProcessExitCodeV1::from_byte(255));
        assert_eq!(zero.normalized_i64(), 0);
        assert_eq!(max.normalized_i64(), 255);
        assert_eq!(max.code(), ProcessExitCodeV1::from_byte(255));
    }
}
