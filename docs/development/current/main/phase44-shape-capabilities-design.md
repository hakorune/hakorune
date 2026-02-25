# Phase 44: Shape Capabilities Design

**Status**: Implemented
**Date**: 2025-12-12

## Overview

Phase 44 converts function-name-based shape detection to capability-based ShapeCapability model.

## Motivation

The previous approach used direct shape enum matching throughout the codebase:

```rust
// Old approach: if explosion when adding new shapes
if shape == Pattern2Mini || shape == JsonparserSkipWsMini || ... {
    // canonical handling
}
```

This led to:
- **If explosion**: Every new shape required updates to multiple match expressions
- **Unclear intent**: Shape names don't express purpose/capability
- **Hard to extend**: Adding new shape variants required widespread changes

## Solution: Capability-Based Architecture

Phase 44 introduces a two-level architecture:

1. **Shape Level** (`NormalizedDevShape`): Concrete implementation patterns
2. **Capability Level** (`ShapeCapability`): Abstract capabilities/features

```rust
// New approach: capability-based filtering
let cap = capability_for_shape(&shape);
if is_canonical_p2_core(&cap) {
    // canonical handling
}
```

## ShapeCapabilityKind Mapping

| Kind | Shapes | Description |
|------|--------|-------------|
| `P2CoreSimple` | Pattern2Mini, Pattern1Mini | Simple P2 mini patterns (i/acc/n) |
| `P2CoreSkipWs` | JsonparserSkipWsMini, JsonparserSkipWsReal | skip_whitespace loops |
| `P2CoreAtoi` | JsonparserAtoiMini, JsonparserAtoiReal | _atoi number parsing |
| `P2MidParseNumber` | JsonparserParseNumberReal | _parse_number with num_str |

### Future Extensibility

Capability struct designed for future extensions:

```rust
pub struct ShapeCapability {
    pub kind: ShapeCapabilityKind,
    // Future fields (not yet used):
    // pub pattern_kind: LoopPatternKind,
    // pub loop_param_count: usize,
    // pub carrier_roles: Vec<CarrierRole>,
    // pub method_calls: Vec<MethodCallSignature>,
}
```

## Canonical vs Dev Support

### Canonical P2-Core (Phase 41 Definition)

**Always use Normalized→MIR direct path** (mode-independent):
- Pattern2Mini (P2CoreSimple)
- JsonparserSkipWsMini, JsonparserSkipWsReal (P2CoreSkipWs)
- JsonparserAtoiMini (P2CoreAtoi)

**Excluded from canonical** (future expansion candidates):
- Pattern1Mini (also P2CoreSimple, but minimal fallback pattern)
- JsonparserAtoiReal (P2CoreAtoi, but not yet canonical)
- JsonparserParseNumberReal (P2MidParseNumber, mid-tier pattern)

### Supported by NormalizedDev

**All P2-Core capabilities** (canonical + dev):
- P2CoreSimple, P2CoreSkipWs, P2CoreAtoi, P2MidParseNumber

## API Design

### Core Functions

```rust
/// Map shape to capability (primary mapping)
pub fn capability_for_shape(shape: &NormalizedDevShape) -> ShapeCapability

/// Check if shape is canonical (shape-level, exact)
pub fn is_canonical_shape(shape: &NormalizedDevShape) -> bool

/// Check if capability is in P2-Core family (capability-level, broad)
pub fn is_p2_core_capability(cap: &ShapeCapability) -> bool

/// Check if capability is supported by Normalized dev
pub fn is_supported_by_normalized(cap: &ShapeCapability) -> bool
```

### Why Both Shape-Level and Capability-Level?

**Shape-level** (`is_canonical_shape`):
- **Granular control**: Pattern1Mini vs Pattern2Mini (both P2CoreSimple)
- **Exact filtering**: Phase 41 canonical set definition
- **Backward compatible**: Preserves existing behavior exactly

**Capability-level** (`is_p2_core_capability`):
- **Future expansion**: Easy to add new capability kinds
- **Intent clarity**: P2CoreSimple vs P2MidParseNumber
- **Extensibility**: Prepare for carrier roles, method signatures, etc.

## Implementation Notes

### Backward Compatibility

Phase 44 is **pure refactoring** - zero behavioral changes:

1. **`canonical_shapes()`**: Still returns exact same shapes
   - Uses `is_canonical_shape()` internally (shape-level check)
   - Capability mapping is internal implementation detail

2. **All tests pass**: 937/937 tests (zero regression)

3. **Bridge routing unchanged**: Mode-based routing logic preserved

### Key Files Modified

1. **`src/mir/join_ir/normalized/shape_guard.rs`**:
   - Added `ShapeCapability`, `ShapeCapabilityKind`
   - Added capability mapping functions
   - Updated `canonical_shapes()` to use `is_canonical_shape()`

2. **`src/mir/join_ir_vm_bridge/bridge.rs`**:
   - No changes needed (uses `canonical_shapes()` helper)

## Benefits

### 1. Extensibility
```rust
// Adding new capability kind:
enum ShapeCapabilityKind {
    P2CoreSimple,
    P2CoreSkipWs,
    P2CoreAtoi,
    P2MidParseNumber,
    P2HeavyString,  // NEW: Just add here
}

// Update mapping:
fn capability_for_shape(shape: &NormalizedDevShape) -> ShapeCapability {
    match shape {
        HeavyStringPattern => ShapeCapability::new(P2HeavyString),  // NEW
        // ... existing mappings
    }
}
```

### 2. Clarity
```rust
// Old: What does this mean?
if shape == JsonparserAtoiMini { ... }

// New: Intent is clear
let cap = capability_for_shape(&shape);
if cap.kind == P2CoreAtoi { ... }
```

### 3. Maintainability
```rust
// Old: Update multiple locations when adding shape
// bridge.rs: add to if expression
// shape_guard.rs: add to another if expression
// normalized.rs: add to yet another if expression

// New: Update one mapping function
fn capability_for_shape(shape: &NormalizedDevShape) -> ShapeCapability {
    // Add new shape here only
}
```

## Future Work

### Phase 45+: Capability-Based Routing

Once more patterns migrate to Normalized path:

```rust
// Current (Phase 44): Still uses shape-level filtering
pub(crate) fn canonical_shapes(module: &JoinModule) -> Vec<NormalizedDevShape> {
    detect_shapes(module).into_iter()
        .filter(|s| is_canonical_shape(s))  // Shape-level
        .collect()
}

// Future (Phase 46+): Pure capability-level filtering
pub(crate) fn canonical_shapes(module: &JoinModule) -> Vec<NormalizedDevShape> {
    detect_shapes(module).into_iter()
        .filter(|s| {
            let cap = capability_for_shape(s);
            cap.kind.is_canonical()  // Capability-level
        })
        .collect()
}
```

### Carrier Role Analysis

```rust
// Future: Carrier role detection
pub struct ShapeCapability {
    pub kind: ShapeCapabilityKind,
    pub carrier_roles: Vec<CarrierRole>,  // Enable this field
}

pub enum CarrierRole {
    LoopVar,        // i, pos
    Accumulator,    // sum, count
    HostReference,  // p (pointer to external state)
    StateCarrier,   // num_str (intermediate state)
}

// Automatic role detection
fn detect_carrier_roles(loop_func: &JoinFunction) -> Vec<CarrierRole> {
    // Analyze param usage patterns
}
```

### Method Call Signatures

```rust
// Future: Track required Box methods
pub struct ShapeCapability {
    pub kind: ShapeCapabilityKind,
    pub method_calls: Vec<MethodCallSignature>,  // Enable this field
}

pub struct MethodCallSignature {
    pub box_name: String,  // "StringBox"
    pub method: String,    // "get"
    pub arity: usize,      // 2 (self + index)
}
```

## Testing

### Verification Strategy

1. **Build test**: Zero compilation errors
2. **Regression test**: 937/937 library tests pass
3. **Canonical set verification**: Same shapes as Phase 41
4. **Smoke test**: Integration tests (if applicable)

### Test Results

```
cargo build --release
✓ Compiled successfully

cargo test --release --lib
✓ 937 passed; 0 failed; 56 ignored
```

## Lessons Learned

### Design Trade-offs

**Why not pure capability-level filtering immediately?**

Answer: **Gradual migration strategy**
- Phase 44: Introduce capability infrastructure (backward compatible)
- Phase 45+: Expand canonical set incrementally
- Phase 46+: Pure capability-level routing when migration complete

**Why keep shape-level API?**

Answer: **Multiple P2CoreSimple shapes**
- Pattern1Mini (minimal fallback)
- Pattern2Mini (canonical core)
- Both map to same capability, but different canonical status
- Shape-level check provides necessary granularity

### Anti-Patterns Avoided

❌ **Don't**: Rewrite all filtering logic at once
```rust
// Risky: Big-bang rewrite, hard to verify
pub(crate) fn canonical_shapes(...) {
    // Complete rewrite with new logic
}
```

✅ **Do**: Add capability layer, preserve existing behavior
```rust
// Safe: Capability-based implementation, same output
pub(crate) fn canonical_shapes(...) {
    shapes.filter(|s| is_canonical_shape(s))  // Uses capabilities internally
}
```

## References

- **Phase 41**: Canonical P2-Core definition
- **Phase 45**: JoinIrMode routing integration
- **JoinIR Architecture**: [joinir-architecture-overview.md](joinir-architecture-overview.md)
