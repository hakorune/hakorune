# Phase 45: JoinIR Mode Unification

**Status**: ✅ Implemented
**Date**: 2025-12-12
**Goal**: Unify JoinIR routing logic through a single `JoinIrMode` enum

---

## Overview

Phase 45 consolidates fragmented feature flags and environment variables into a unified `JoinIrMode` enum, making JoinIR routing decisions centralized and maintainable.

### Before (Fragmented)
```rust
// Scattered checks across codebase
if normalized_dev_enabled() { /* ... */ }
if cfg!(feature = "normalized_dev") { /* ... */ }
if env::var("NYASH_JOINIR_NORMALIZED_DEV_RUN").is_ok() { /* ... */ }
```

### After (Unified)
```rust
match current_joinir_mode() {
    JoinIrMode::NormalizedDev => { /* dev path */ }
    JoinIrMode::StructuredOnly => { /* default path */ }
    JoinIrMode::NormalizedCanonical => { /* future canonical */ }
}
```

---

## JoinIrMode Enum

```rust
pub enum JoinIrMode {
    /// Default: Structured→MIR direct (no Normalized layer)
    StructuredOnly,

    /// Dev mode: Structured→Normalized→MIR(direct) for supported shapes
    /// Requires: --features normalized_dev + NYASH_JOINIR_NORMALIZED_DEV_RUN=1
    NormalizedDev,

    /// Future: All canonical shapes use Normalized→MIR(direct)
    /// Reserved for Phase 46+ canonical migration
    NormalizedCanonical,
}
```

---

## Mode Determination Logic

```
┌─────────────────────────────────────────────────┐
│ current_joinir_mode()                           │
├─────────────────────────────────────────────────┤
│                                                 │
│  #[cfg(feature = "normalized_dev")]             │
│    ├─ NYASH_JOINIR_NORMALIZED_DEV_RUN=1         │
│    │   → NormalizedDev                          │
│    └─ else                                      │
│        → StructuredOnly                         │
│                                                 │
│  #[cfg(not(feature = "normalized_dev"))]        │
│    └─ StructuredOnly                            │
│                                                 │
└─────────────────────────────────────────────────┘
```

---

## Routing Logic

### Bridge (bridge.rs)

```rust
let mode = current_joinir_mode();
let shapes = shape_guard::classify(&module);

// 1. Canonical P2-Core: always Normalized→MIR(direct) (mode-independent)
if !canonical_shapes.is_empty() {
    return lower_via_normalized_direct(...);
}

// 2. Mode-based routing
match mode {
    JoinIrMode::NormalizedDev => {
        if !dev_shapes.is_empty() {
            return lower_via_normalized_dev(...);
        } else {
            return lower_structured(...);
        }
    }
    JoinIrMode::StructuredOnly | JoinIrMode::NormalizedCanonical => {
        return lower_structured(...);
    }
}
```

**Canonical P2-Core shapes** (Phase 41):
- `Pattern2Mini`
- `JsonparserSkipWsMini/Real`
- `JsonparserAtoiMini/Real`
- `JsonparserParseNumberReal`

### Runner (join_ir_runner.rs)

```rust
match current_joinir_mode() {
    JoinIrMode::NormalizedDev => {
        // Dev roundtrip: Structured→Normalized→Structured
        run_joinir_function_normalized_dev(...)
    }
    _ => {
        // Structured-only path (default)
        run_joinir_function_structured(...)
    }
}
```

---

## Implementation Summary

### Files Modified

1. **`src/config/env/joinir_dev.rs`**:
   - Added `JoinIrMode` enum (3 variants)
   - Added `current_joinir_mode()` function
   - Refactored `normalized_dev_enabled()` as thin wrapper

2. **`src/mir/join_ir_vm_bridge/bridge.rs`**:
   - Replaced `normalized_dev_enabled()` with `current_joinir_mode()`
   - Updated routing logic to use pattern matching
   - Preserved canonical P2-Core special handling

3. **`src/mir/join_ir_runner.rs`**:
   - Replaced boolean check with mode pattern matching
   - Updated comments to reference `JoinIrMode::NormalizedDev`

### Tests

- ✅ 937/937 tests pass (no regression)
- ✅ Both configurations verified:
  - Default (without `--features normalized_dev`)
  - With `--features normalized_dev`

---

## Future Work (Phase 46+)

**NormalizedCanonical mode** is reserved for future use when:
- All Pattern1/Pattern2/Pattern3 shapes become canonical
- Normalized→MIR(direct) becomes the default for all supported shapes
- `StructuredOnly` mode becomes legacy/fallback only

**Migration path**:
```
Phase 45 (now):   StructuredOnly (default) + NormalizedDev (opt-in)
Phase 46+:        NormalizedCanonical (default) + StructuredOnly (legacy)
```

---

## Related Documentation

- **JoinIR Architecture**: [joinir-architecture-overview.md](joinir-architecture-overview.md)
- **Shape Guard**: `src/mir/join_ir/normalized/shape_guard.rs`
- **Normalized Bridge**: `src/mir/join_ir_vm_bridge/normalized_bridge.rs`
