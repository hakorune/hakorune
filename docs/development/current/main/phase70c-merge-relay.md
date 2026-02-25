# Phase 70-C: Merge Relay (Multiple Inner Loops → Same Owner)

**Status**: ✅ Completed
**Date**: 2025-12-13

---

## Overview

Phase 70-C implements detection and validation support for "merge relay" patterns where **multiple inner loops update the same owner-owned variable**.

This builds upon Phase 70-B's simple passthrough multihop support by handling the case where a single owner scope must merge updates from multiple relay sources.

---

## Merge Relay Pattern

### Definition

**Merge Relay**: Multiple inner loops update the same ancestor-owned variable, requiring the owner to merge all relay updates at its exit PHI.

### Example

```nyash
loop L1 {
    local total = 0    // owned by L1
    loop L2_A {
        total++        // L2_A → L1 relay
    }
    loop L2_B {
        total += 10    // L2_B → L1 relay
    }
}
// L1 exit: merge both L2_A and L2_B's updates to 'total'
```

### OwnershipPlans

**L2_A Plan**:
```rust
OwnershipPlan {
    scope_id: ScopeId(2),
    relay_writes: [
        RelayVar {
            name: "total",
            owner_scope: ScopeId(1),      // L1
            relay_path: [ScopeId(2)],     // Single hop
        }
    ],
    owned_vars: [],  // L2_A doesn't own total
}
```

**L2_B Plan**:
```rust
OwnershipPlan {
    scope_id: ScopeId(3),
    relay_writes: [
        RelayVar {
            name: "total",
            owner_scope: ScopeId(1),      // L1 (same owner)
            relay_path: [ScopeId(3)],     // Single hop
        }
    ],
    owned_vars: [],  // L2_B doesn't own total
}
```

**L1 Plan**:
```rust
OwnershipPlan {
    scope_id: ScopeId(1),
    owned_vars: [
        ScopeOwnedVar {
            name: "total",
            is_written: true,  // Owner accepts relays
        }
    ],
    relay_writes: [],  // No further relay (L1 is owner)
}
```

---

## Design Decisions

### 1. PERMIT with Owner Merge

**Decision**: Merge relay is **PERMITTED** (not rejected) because:
- Multiple inner loops updating a shared variable is a **legitimate pattern**
- Owner scope can merge all updates at its exit PHI
- Each inner loop has a valid single-hop relay to the same owner

### 2. Validation Strategy

**OwnershipPlanValidator accepts merge relay if**:
1. Each relay has valid `relay_path` (non-empty, first hop is current scope)
2. No self-conflict (scope doesn't both own and relay the same variable)
3. All relays point to the same owner scope (validated separately per relay)

**Note**: Validator validates **individual plans** - it doesn't check cross-plan consistency (e.g., "two different scopes relay to the same owner"). This is intentional - each relay is independently valid.

### 3. Runtime Support

**Phase 70-C Status**: Detection and validation only (dev-only)

**Runtime Implementation** (Phase 70-D+):
- Owner scope exit PHI must merge values from all relay sources
- Each relay source appears as a separate carrier in owner's loop_step
- PHI merges: `total_final = phi(total_init, total_from_L2_A, total_from_L2_B)`

---

## Implementation

### Files Modified

1. **tests/normalized_joinir_min.rs**
   - `test_phase70c_merge_relay_multiple_inner_loops_detected()`: AST → OwnershipPlan detection
   - `test_phase70c_merge_relay_same_owner_accepted()`: Validator accepts merge relay

### Test Coverage

#### Test 1: AST Detection
- **Input**: 3-level nested loop (L1 owns, L2_A and L2_B relay)
- **Verification**:
  - 2 relay plans detected
  - Both relay to the same owner (L1)
  - Single-hop relay paths
  - Owner plan marks variable as written

#### Test 2: Validator Acceptance
- **Input**: Two OwnershipPlans with single-hop relay to same owner
- **Verification**:
  - Both plans pass `OwnershipPlanValidator::validate_relay_support()`
  - No runtime_unsupported errors

---

## Test Results

```bash
$ NYASH_JOINIR_NORMALIZED_DEV_RUN=1 cargo test --features normalized_dev --test normalized_joinir_min test_phase70c

running 2 tests
test test_phase70c_merge_relay_same_owner_accepted ... ok
test test_phase70c_merge_relay_multiple_inner_loops_detected ... ok

test result: ok. 2 passed; 0 failed; 0 ignored
```

**Regression Check**:
- normalized_dev: 54/54 PASS ✅
- lib tests: 950/950 PASS ✅
- Zero regressions

---

## Acceptance Criteria

- [x] Merge relay pattern fixture created (2 inner loops → 1 owner)
- [x] AST analyzer correctly detects merge relay (2 relay plans)
- [x] OwnershipPlanValidator accepts merge relay (no runtime_unsupported error)
- [x] Tests pass with normalized_dev feature
- [x] No regressions in lib tests

---

## Future Work (Phase 70-D+)

### Runtime Execution Support

**Required for full merge relay execution**:

1. **Exit PHI Generation**:
   - Owner scope must generate PHI with N+1 inputs (init + each relay source)
   - Example: `phi(total_init, total_from_L2_A, total_from_L2_B)`

2. **Carrier Propagation**:
   - Each relay source must appear in owner's loop_step signature
   - Owner boundary must collect all relay values

3. **Integration Testing**:
   - E2E test: 2 inner loops → owner → verify final merged value
   - Combination: Multihop + Merge (3-layer nested with multiple relays)

4. **Edge Cases**:
   - More than 2 relay sources
   - If-branches with relay (conditional merge)
   - Multihop + Merge (nested relay with multiple sources)

---

## Related Documents

- [Phase 65: Multihop Design](phase65-ownership-relay-multihop-design.md#merge-relay-の意味論)
- [Phase 70-A: Runtime Guard](phase70-relay-runtime-guard.md)
- [Phase 70-B: Simple Passthrough](phase70-relay-runtime-guard.md)
- [Phase 56: Ownership-Relay Architecture](phase56-ownership-relay-design.md)

---

## Changelog

- **2025-12-13**: Phase 70-C completed - Merge relay detection and validation
  - 2 tests added (AST detection, validator acceptance)
  - 54/54 normalized_dev tests pass
  - 950/950 lib tests pass (zero regressions)
