# Phase 131-11-G: PHI Type Inference Bug - Root Cause Report

**Date**: 2025-12-14
**Status**: Historical (Fixed in Phase 131-11-H)
**Severity**: High (Breaks loop carrier PHI type inference)

## Executive Summary

PHI nodes for loop carriers are getting incorrect `String` type instead of `Integer`, breaking type propagation throughout the loop. Investigation reveals a circular dependency in the type inference chain.

## Update (Phase 131-11-H): Fix Applied

**Fix**: Seed loop-carrier PHI type from the entry (init) value only, to break the cycle.

- File: `src/mir/builder/control_flow/joinir/merge/loop_header_phi_builder.rs`
- Change: when creating the loop-carrier PHI dst, copy the init value’s type into `value_types` (ignore backedge type at creation time).
- Result: MIR/VM observe `%phi` as `Integer` (expected) and the loop semantics are restored on VM.

Note:
- This document remains as the “why it broke” report. The current task should track the remaining LLVM mismatch separately.
- Environment variables introduced for this investigation are now documented in `docs/reference/environment-variables.md`.

## Bug Symptoms

```mir
bb4:
    1: %3: String = phi [%2, bb0], [%8, bb7]  ← Should be Integer!
    1: %8 = %3 Add %7                         ← No type assigned!
```

**Expected**: `%3: Integer` (loop counter)
**Actual**: `%3: String` (wrong!)

## Root Cause Chain

### 1. Initial Infection (Source TBD)

PHI %3 gets initial type `String` during JoinIR → MIR lowering, **before** lifecycle.rs runs.

**Evidence**:
```
[lifecycle/phi-scan] main PHI ValueId(3) existing type: Some(String)
```

This happens **before** PhiTypeResolver runs.

### 2. BinOp Type Assignment Failure

When `%8 = %3 + 1` is emitted (`ops.rs:189-221`):

**Code Path**:
```rust
// ops.rs:193-194
let lhs_type = self.classify_operand_type(lhs);  // %3 → String
let rhs_type = self.classify_operand_type(rhs);  // %7 → Integer

// ops.rs:210-213
(String, Integer) | (Integer, String) => {
    // Mixed types: leave as Unknown for use-site coercion
    // LLVM backend will handle string concatenation
}
```

**Result**: %8 gets **NO TYPE** in `value_types` map!

### 3. PhiTypeResolver Failure

**Trace Output**:
```
[phi/type] Resolving PHI dst=3 incoming=Phi([(bb0, ValueId(2)), (bb7, ValueId(8))])
[phi/type]   ValueId(8) is Base/None but NO TYPE in value_types!
[phi/type]   ValueId(2) is Copy -> ValueId(1)
[phi/type]   ValueId(1) is Phi with 1 inputs: [(BasicBlockId(6), ValueId(8))]
[phi_resolver] failed for ValueId(3): base_types = []
```

**Why It Fails**:
- Incoming `%2` → Copy → `%1` → PHI → `%8` (circular!)
- Incoming `%8` has **NO TYPE**
- Cannot find any base types → returns `None`

### 4. BinOp Re-propagation Ineffective

**Code** (`lifecycle.rs:605-672`):
```rust
// Tries to fix %8 type
let lhs_type = self.value_types.get(lhs);  // %3 → still String!
let rhs_type = self.value_types.get(rhs);  // %7 → Integer

match (lhs_class, rhs_class) {
    (String, Integer) | (Integer, String) => None,  // No update!
}
```

**Result**: Still no type for %8, circular dependency persists.

## Technical Analysis

### Circular Dependency Diagram

```
PHI %3 type (String ❌)
    ↓
BinOp %8 = %3 + 1
    ↓ (mixed String + Integer)
NO TYPE ASSIGNED
    ↓
PHI %3 incoming [%2, %8]
    ↓ (%8 has no type)
PhiTypeResolver FAILS
    ↓
BinOp re-propagation
    ↓ (%3 still String)
NO UPDATE
    ↓
STUCK IN LOOP!
```

### Why Current Architecture Fails

1. **PHI gets wrong initial type** (before lifecycle.rs)
2. **BinOp depends on operand types** (correct design, but fails with wrong PHI type)
3. **PhiTypeResolver depends on incoming types** (correct design, but %8 is untyped)
4. **Re-propagation can't break cycle** (depends on %3 type, which is wrong)

### SSOT Violation

**TypeFacts SSOT** says:
> Types are determined by **definitions only**, not usage

But PHI %3 gets initial type from **somewhere**, violating this principle.

## Debug Traces Added

### 1. PhiTypeResolver Debug (`NYASH_PHI_TYPE_DEBUG=1`)

**File**: `src/mir/phi_core/phi_type_resolver.rs`

```rust
if debug {
    eprintln!("[phi/type] Resolving PHI dst={} incoming={:?}", ...);
    eprintln!("[phi/type]   {:?} is Copy -> {:?}", v, src);
    eprintln!("[phi/type]   {:?} is Phi with {} inputs: {:?}", ...);
    eprintln!("[phi/type]   {:?} is Base with type {:?}", v, ty);
    eprintln!("[phi/type]   {:?} is Base/None but NO TYPE in value_types!", v);
}
```

### 2. PHI Metadata Propagation Debug (`NYASH_PHI_META_DEBUG=1`)

**File**: `src/mir/builder/origin/phi.rs`

```rust
if debug {
    eprintln!("[phi/meta] propagate_phi_meta dst={:?} inputs={:?}", ...);
    eprintln!("[phi/meta]   incoming {:?} has type {:?}", v, t);
    eprintln!("[phi/meta]   NO TYPE COPIED (ty_agree=false)");
}
```

### 3. Existing Debug Flags

- `NYASH_PHI_GLOBAL_DEBUG=1` - Global PHI re-inference (lifecycle.rs)
- `NYASH_BINOP_REPROP_DEBUG=1` - BinOp re-propagation (lifecycle.rs)
- `NYASH_PHI_RESOLVER_DEBUG=1` - PhiTypeResolver summary

## Next Steps (Phase 131-11-H)

### Immediate Tasks

1. **Find Initial String Type Source**
   - Add traces to all PHI creation sites in JoinIR lowering
   - Check `emit_phi` in merge modules
   - Check loop pattern lowering (Pattern 1-4)

2. **Fix PHI Initial Type Assignment**
   - Loop carrier PHI should start as `Unknown` or use init value type **only**
   - Do NOT use backedge type for initial assignment
   - Let PhiTypeResolver handle multi-path inference

3. **Fix BinOp Mixed-Type Handling**
   - `(String, Integer)` case should check if String is actually a loop carrier
   - Fallback to `Integer` if one operand is Unknown PHI

### Architectural Fix Options

#### Option A: Remove Initial PHI Typing
```rust
// In PHI emission
// DO NOT set dst type during emission
// Let PhiTypeResolver handle it later
```

**Pros**: Simple, follows SSOT
**Cons**: More values stay Unknown longer

#### Option B: Smart Initial Typing
```rust
// In PHI emission for loops
if is_loop_carrier {
    // Use init value type only (ignore backedge)
    if let Some(init_type) = get_init_value_type() {
        value_types.insert(dst, init_type);
    }
}
```

**Pros**: Fewer Unknown values
**Cons**: Requires loop structure awareness

#### Option C: BinOp Fallback for Unknown
```rust
// In BinOp emission
(String, Integer) | (Integer, String) => {
    // Check if "String" operand is actually Unknown PHI
    if lhs_is_unknown_phi || rhs_is_unknown_phi {
        value_types.insert(dst, MirType::Integer);  // Assume numeric
    }
    // else: leave Unknown for true string concat
}
```

**Pros**: Breaks circular dependency
**Cons**: Heuristic, might mistype some cases

### Recommended Fix: **Option B** (Smart Initial Typing)

**Rationale**:
1. Preserves SSOT (init value is a definition)
2. Prevents circular dependency (backedge ignored initially)
3. PhiTypeResolver still validates and corrects if needed
4. Minimal code changes (confined to loop lowering)

## Test Case

**File**: `apps/tests/llvm_stage3_loop_only.hako`

```nyash
static box Main {
  main() {
    local counter = 0
    loop (true) {
      counter = counter + 1  // ← This should be Integer type!
      if counter == 3 { break }
      continue
    }
    print("Result: " + counter)
    return 0
  }
}
```

**Expected MIR**:
```mir
bb4:
    1: %3: Integer = phi [%2, bb0], [%8, bb7]  ← Integer, not String!
    1: %8: Integer = %3 Add %7                 ← Should have type!
```

## References

- **Phase 131-11-E**: TypeFacts/TypeDemands separation
- **Phase 131-11-F**: MIR JSON metadata output
- **Phase 131-9**: Global PHI type inference
- **Phase 84-3**: PhiTypeResolver box design

## Files Modified (Debug Traces)

1. `src/mir/phi_core/phi_type_resolver.rs` - Added detailed PHI resolution traces
2. `src/mir/builder/origin/phi.rs` - Added metadata propagation traces

## Environment Variables Reference

```bash
# Complete PHI type debugging
NYASH_PHI_TYPE_DEBUG=1 \
NYASH_PHI_META_DEBUG=1 \
NYASH_PHI_GLOBAL_DEBUG=1 \
NYASH_BINOP_REPROP_DEBUG=1 \
./target/release/hakorune --dump-mir apps/tests/llvm_stage3_loop_only.hako

# Quick diagnosis
NYASH_PHI_TYPE_DEBUG=1 ./target/release/hakorune --dump-mir test.hako 2>&1 | grep "\[phi/type\]"
```

---

**Status**: Ready for Phase 131-11-H implementation (Fix PHI initial typing)
