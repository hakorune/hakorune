//! ExprLoweringContract: Pure/Impure boundary SSOT (Phase 141 P0)
//!
//! ## Responsibility
//!
//! Define the contract surface for `NormalizedExprLowererBox` so that Phase 141+
//! can introduce Call/MethodCall lowering without changing existing behavior.
//!
//! ## Key Policy (Phase 141 P0)
//!
//! - Current lowering is **PureOnly**.
//! - Call/MethodCall and other impure constructs remain **out-of-scope** and must
//!   return `Ok(None)`, preserving route-decline behavior.
//!
//! ## Phase 141 P1 (incremental)
//!
//! - `WithImpure(KnownIntrinsicOnly)` allows a *small, explicit allowlist* of
//!   "known intrinsic" method calls (still pure, fixed arity, stable type_hint).
//! - Everything else stays out-of-scope and must return `Ok(None)`.

/// Scope selector for expression lowering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExprLoweringScope {
    /// Pure expressions only (Phase 140 baseline).
    PureOnly,
    /// Placeholder for future impure support (Phase 141+).
    WithImpure(ImpurePolicy),
}

/// Phase 141+ policies (effects/ordering/typing).
///
/// Phase 141 P1 uses `KnownIntrinsicOnly` as a safe on-ramp.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImpurePolicy {
    /// Allow a small explicit allowlist of intrinsic calls only.
    ///
    /// Still must be semantics-preserving: if the intrinsic is not applicable at runtime,
    /// it should decline the route with the same observable behavior as the baseline path.
    KnownIntrinsicOnly,
}

impl Default for ImpurePolicy {
    fn default() -> Self {
        Self::KnownIntrinsicOnly
    }
}

/// Phase 141 P1: marker for method calls allowed under `KnownIntrinsicOnly`.
///
/// ## Metadata Owner
///
/// `KnownIntrinsic` carries only the semantic variant. Method name, arity, and
/// return type metadata live in `KnownIntrinsicRegistryBox`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KnownIntrinsic {
    /// `receiver.length()` with 0 args, expected to return Integer.
    ///
    /// Notes:
    /// - Receiver type is *not* resolved here; this is a structural allowlist.
    /// - Return type hint is used to prevent downstream type inference noise.
    Length0,
}

/// Minimal classification for "why did we return Ok(None)?" (Phase 141 P0).
///
/// ## Phase 141 P1.5: Added IntrinsicNotWhitelisted
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutOfScopeReason {
    Call,
    MethodCall,
    /// Phase 141 P1.5: Known intrinsic but not in current allowlist
    /// (Better diagnostic than generic MethodCall)
    IntrinsicNotWhitelisted,
    ImpureExpression,
    UnsupportedLiteral,
    UnsupportedOperator,
    MissingEnvVar,
}
