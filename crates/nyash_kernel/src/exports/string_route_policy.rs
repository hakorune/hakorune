//! Route policy helpers for string exports.
//!
//! Export files own ABI symbols. This module owns small route toggles used by
//! those symbols and by `string_helpers`.

#[cfg(not(test))]
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::OnceLock;

#[inline(always)]
pub(crate) fn compat_fallback_allowed() -> bool {
    nyash_rust::config::env::vm_compat_fallback_allowed()
}

#[inline(always)]
pub(crate) fn substring_view_enabled() -> bool {
    static SUBSTRING_VIEW_ENABLED: OnceLock<bool> = OnceLock::new();
    crate::env_flags::flag_default_on_cached(&SUBSTRING_VIEW_ENABLED, "NYASH_LLVM_FAST")
}

#[derive(Clone, Copy)]
pub(crate) struct SubstringRoutePolicy {
    pub(crate) view_enabled: bool,
    pub(crate) fallback_allowed: bool,
}

#[cfg(not(test))]
static SUBSTRING_ROUTE_POLICY_CACHE: AtomicU8 = AtomicU8::new(0);

#[inline(always)]
pub(crate) fn substring_route_policy() -> SubstringRoutePolicy {
    #[cfg(test)]
    {
        return SubstringRoutePolicy {
            view_enabled: substring_view_enabled(),
            fallback_allowed: compat_fallback_allowed(),
        };
    }
    #[cfg(not(test))]
    {
        match SUBSTRING_ROUTE_POLICY_CACHE.load(Ordering::Relaxed) {
            0 => {
                let policy = SubstringRoutePolicy {
                    view_enabled: substring_view_enabled(),
                    fallback_allowed: compat_fallback_allowed(),
                };
                SUBSTRING_ROUTE_POLICY_CACHE.store(
                    0b100 | (policy.view_enabled as u8) | ((policy.fallback_allowed as u8) << 1),
                    Ordering::Relaxed,
                );
                policy
            }
            raw => SubstringRoutePolicy {
                view_enabled: raw & 0b001 != 0,
                fallback_allowed: raw & 0b010 != 0,
            },
        }
    }
}
