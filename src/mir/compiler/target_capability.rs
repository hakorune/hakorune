//! Compile-invocation target capability for the pinned Text residence lane.
//!
//! The outer LLVM runner selects the closed profile.  This module owns only
//! the opaque, move-only capability that is carried into the MIR close; it
//! does not inspect JSON, host layout, or an LLVM TargetMachine.

use std::num::NonZeroU64;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_COMPILE_INVOCATION_ORDINAL: AtomicU64 = AtomicU64::new(1);

/// Closed target catalog for the first pinned-Text backend row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PinnedTextCompileTargetProfileV1 {
    NyRtTextResidencePtr64As0V1,
}

impl PinnedTextCompileTargetProfileV1 {
    pub const fn profile_id(self) -> &'static str {
        match self {
            Self::NyRtTextResidencePtr64As0V1 => "nyrt-text-residence-ptr64-as0-v1",
        }
    }

    pub const fn target_triple(self) -> &'static str {
        match self {
            Self::NyRtTextResidencePtr64As0V1 => "x86_64-pc-linux-gnu",
        }
    }

    pub const fn data_layout(self) -> &'static str {
        match self {
            Self::NyRtTextResidencePtr64As0V1 => {
                "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
            }
        }
    }

    pub const fn little_endian(self) -> bool {
        true
    }

    pub const fn address_space_zero_pointer_width(self) -> u16 {
        64
    }

    pub const fn address_space_zero_abi_alignment(self) -> u16 {
        8
    }

    pub const fn consumer_abi_revision(self) -> &'static str {
        match self {
            Self::NyRtTextResidencePtr64As0V1 => "hako-llvmc-pure-first-v2",
        }
    }

    /// The contract-bound emitter is deliberately pinned to the LLVM 18 C
    /// API row.  These are catalog values, never host or environment probes.
    pub const fn llvm_c_api_abi_revision(self) -> &'static str {
        match self {
            Self::NyRtTextResidencePtr64As0V1 => "llvm-c-api-18-v1",
        }
    }

    pub const fn object_cpu(self) -> &'static str {
        match self {
            Self::NyRtTextResidencePtr64As0V1 => "",
        }
    }

    pub const fn object_features(self) -> &'static str {
        match self {
            Self::NyRtTextResidencePtr64As0V1 => "",
        }
    }

    pub const fn object_codegen_opt_level(self) -> u8 {
        match self {
            Self::NyRtTextResidencePtr64As0V1 => 3,
        }
    }

    pub const fn object_relocation_model(self) -> u8 {
        match self {
            Self::NyRtTextResidencePtr64As0V1 => 0,
        }
    }

    pub const fn object_code_model(self) -> u8 {
        match self {
            Self::NyRtTextResidencePtr64As0V1 => 0,
        }
    }

    pub const fn residence_abi_revision(self) -> &'static str {
        match self {
            Self::NyRtTextResidencePtr64As0V1 => "text-formal-residence-v1",
        }
    }

    pub const fn max_root_count(self) -> u32 {
        1024
    }

    pub const fn max_private_frame_bytes(self) -> u32 {
        65_536
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinnedTextCompileTargetCapabilityIssueV1 {
    InvocationOrdinalExhausted,
}

impl std::fmt::Display for PinnedTextCompileTargetCapabilityIssueV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "[freeze:contract][pinned-text/compile-target] {self:?}")
    }
}

impl std::error::Error for PinnedTextCompileTargetCapabilityIssueV1 {}

/// Move-only capability for one explicit compile invocation.
///
/// The profile is catalog-owned.  The ordinal is an invocation brand; it is
/// not a source identity, a MIR value, a JSON field, or an LLVM realization
/// receipt.  No public constructor or `Clone` implementation exists.
#[derive(Debug, PartialEq, Eq)]
pub struct PinnedTextCompileTargetCapabilityV1 {
    profile: PinnedTextCompileTargetProfileV1,
    invocation_ordinal: NonZeroU64,
}

impl PinnedTextCompileTargetCapabilityV1 {
    pub const fn profile(&self) -> PinnedTextCompileTargetProfileV1 {
        self.profile
    }

    pub const fn invocation_ordinal(&self) -> NonZeroU64 {
        self.invocation_ordinal
    }

    pub fn same_invocation(&self, other: &Self) -> bool {
        self.profile == other.profile && self.invocation_ordinal == other.invocation_ordinal
    }
}

/// Sole Rust-side issuer for the target capability.
pub struct PinnedTextCompileTargetCapabilityIssuerV1;

impl PinnedTextCompileTargetCapabilityIssuerV1 {
    pub fn issue(
        profile: PinnedTextCompileTargetProfileV1,
    ) -> Result<PinnedTextCompileTargetCapabilityV1, PinnedTextCompileTargetCapabilityIssueV1>
    {
        let ordinal = NEXT_COMPILE_INVOCATION_ORDINAL
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
                current.checked_add(1)
            })
            .map_err(|_| PinnedTextCompileTargetCapabilityIssueV1::InvocationOrdinalExhausted)?;
        let invocation_ordinal = NonZeroU64::new(ordinal)
            .ok_or(PinnedTextCompileTargetCapabilityIssueV1::InvocationOrdinalExhausted)?;
        Ok(PinnedTextCompileTargetCapabilityV1 {
            profile,
            invocation_ordinal,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_row_is_explicit_and_stable() {
        let profile = PinnedTextCompileTargetProfileV1::NyRtTextResidencePtr64As0V1;
        assert_eq!(profile.profile_id(), "nyrt-text-residence-ptr64-as0-v1");
        assert_eq!(profile.target_triple(), "x86_64-pc-linux-gnu");
        assert_eq!(profile.address_space_zero_pointer_width(), 64);
        assert_eq!(profile.address_space_zero_abi_alignment(), 8);
        assert!(profile.little_endian());
        assert_eq!(profile.residence_abi_revision(), "text-formal-residence-v1");
        assert_eq!(profile.max_root_count(), 1024);
        assert_eq!(profile.max_private_frame_bytes(), 65_536);
    }

    #[test]
    fn issuer_brands_each_capability_once() {
        let profile = PinnedTextCompileTargetProfileV1::NyRtTextResidencePtr64As0V1;
        let first = PinnedTextCompileTargetCapabilityIssuerV1::issue(profile).unwrap();
        let second = PinnedTextCompileTargetCapabilityIssuerV1::issue(profile).unwrap();
        assert_ne!(first.invocation_ordinal(), second.invocation_ordinal());
        assert!(!first.same_invocation(&second));
    }
}
