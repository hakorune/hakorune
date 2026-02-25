//! Phase 141 P1.5: Known Intrinsic Registry (SSOT)
//!
//! ## Responsibility
//!
//! - Centralize known intrinsic metadata (method_name, arity, return_type_hint)
//! - Provide lookup API: method name + arity → KnownIntrinsic
//! - Single responsibility: eliminate string literal scattered matching
//!
//! ## Design
//!
//! - `KnownIntrinsicSpec`: Metadata struct for each intrinsic
//! - `KnownIntrinsicRegistryBox`: Lookup SSOT (all_specs() static array)
//! - Adding new intrinsics: (1) enum variant, (2) registry entry only
//!
//! ## Non-Goals
//!
//! - Effects/typing system (Phase 141 P2+ with IntrinsicEffects trait)
//! - Runtime intrinsic dispatch (this is compile-time metadata only)

use super::expr_lowering_contract::KnownIntrinsic;

/// Phase 141 P1.5: Known intrinsic specification
#[derive(Debug, Clone, Copy)]
pub struct KnownIntrinsicSpec {
    pub intrinsic: KnownIntrinsic,
    pub method_name: &'static str,
    pub arity: usize,
    pub return_type_hint: Option<&'static str>,
}

/// Phase 141 P1.5: Known intrinsic registry SSOT
pub struct KnownIntrinsicRegistryBox;

impl KnownIntrinsicRegistryBox {
    /// Lookup intrinsic by method name and arity
    ///
    /// ## Contract
    ///
    /// - Input: method name + arity
    /// - Output: Some(KnownIntrinsic) if matched, None otherwise
    /// - Use case: Pattern matching in expr_lowerer_box.rs
    ///
    /// ## Example
    ///
    /// ```
    /// // receiver.length() with 0 args
    /// let intrinsic = KnownIntrinsicRegistryBox::lookup("length", 0);
    /// assert_eq!(intrinsic, Some(KnownIntrinsic::Length0));
    /// ```
    pub fn lookup(method: &str, arity: usize) -> Option<KnownIntrinsic> {
        Self::all_specs()
            .iter()
            .find(|spec| spec.method_name == method && spec.arity == arity)
            .map(|spec| spec.intrinsic)
    }

    /// Get spec for a known intrinsic
    ///
    /// ## Contract
    ///
    /// - Input: KnownIntrinsic enum variant
    /// - Output: KnownIntrinsicSpec
    /// - Panics if intrinsic not registered (design invariant)
    ///
    /// ## Example
    ///
    /// ```
    /// let spec = KnownIntrinsicRegistryBox::get_spec(KnownIntrinsic::Length0);
    /// assert_eq!(spec.method_name, "length");
    /// assert_eq!(spec.arity, 0);
    /// ```
    pub fn get_spec(intrinsic: KnownIntrinsic) -> KnownIntrinsicSpec {
        Self::all_specs()
            .iter()
            .find(|spec| spec.intrinsic == intrinsic)
            .expect("KnownIntrinsic not registered in all_specs() - design invariant violated")
            .clone()
    }

    /// All registered intrinsics (SSOT)
    ///
    /// ## Extending
    ///
    /// To add a new intrinsic:
    /// 1. Add enum variant to KnownIntrinsic (in expr_lowering_contract.rs)
    /// 2. Add entry to this array
    ///
    /// No other files need editing!
    fn all_specs() -> &'static [KnownIntrinsicSpec] {
        static SPECS: &[KnownIntrinsicSpec] = &[
            // Phase 141 P1: receiver.length() with 0 args
            KnownIntrinsicSpec {
                intrinsic: KnownIntrinsic::Length0,
                method_name: "length",
                arity: 0,
                return_type_hint: Some("integer"),
            },
            // Future intrinsics here...
            // Example: receiver.toUpperCase() with 0 args
            // KnownIntrinsicSpec {
            //     intrinsic: KnownIntrinsic::ToUpperCase0,
            //     method_name: "toUpperCase",
            //     arity: 0,
            //     return_type_hint: Some("string".to_string()),
            // },
        ];
        SPECS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup_length0_success() {
        let result = KnownIntrinsicRegistryBox::lookup("length", 0);
        assert_eq!(result, Some(KnownIntrinsic::Length0));
    }

    #[test]
    fn test_lookup_wrong_arity_returns_none() {
        // length() with 1 arg should not match Length0 (arity 0)
        let result = KnownIntrinsicRegistryBox::lookup("length", 1);
        assert_eq!(result, None);
    }

    #[test]
    fn test_lookup_unknown_method_returns_none() {
        let result = KnownIntrinsicRegistryBox::lookup("unknown_method", 0);
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_spec_length0() {
        let spec = KnownIntrinsicRegistryBox::get_spec(KnownIntrinsic::Length0);
        assert_eq!(spec.method_name, "length");
        assert_eq!(spec.arity, 0);
        assert_eq!(
            spec.return_type_hint,
            Some("integer")
        );
    }

    #[test]
    fn test_lookup_deterministic() {
        // Ensure lookup is deterministic (same input = same output)
        for _ in 0..10 {
            let result = KnownIntrinsicRegistryBox::lookup("length", 0);
            assert_eq!(result, Some(KnownIntrinsic::Length0));
        }
    }

    #[test]
    fn test_all_specs_non_empty() {
        let specs = KnownIntrinsicRegistryBox::all_specs();
        assert!(!specs.is_empty(), "all_specs() should have at least one entry");
    }
}
