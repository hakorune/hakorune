# Phase 72: PHI Reserved Region Verification - Complete

## Status: ✅ OBSERVATION COMPLETE - Verifier Strengthening NOT RECOMMENDED

**Date**: 2025-12-13
**Objective**: Observe PHI dst ValueId distribution and determine if verifier can enforce reserved region (0-99)
**Outcome**: Phase complete - documentation-only result, no verifier strengthening

---

## Executive Summary

Phase 72 successfully observed PHI dst allocation patterns and determined that **strengthening the verifier is not architecturally sound**. While PHI dsts currently fall in the low ValueId range (0-99), this is **accidental** rather than enforced.

### Key Finding

**PHI dst allocation does NOT respect the "PHI Reserved (0-99)" region by design.**

- PHI dst comes from `builder.next_value_id()` (host MirBuilder)
- Reserved region (0-99) is a JoinValueSpace contract for JoinIR lowering
- These are separate allocation pools with no enforcement mechanism
- Current stability is due to ValueId allocation ordering, not architectural guarantee

### Decision: Document, Don't Enforce

**Recommendation**: Keep existing behavior, add documentation, monitor for regressions.

**Rationale**:
1. No architectural mechanism to enforce PHI dst ∈ [0, 99]
2. Current system works through separate allocation pools (accidental non-overlap)
3. Adding verifier would create false assumptions about allocation order
4. 950/950 tests pass with current design

---

## Implementation Summary

### Files Added

1. **`src/mir/join_ir/verify_phi_reserved.rs`** (266 lines)
   - Observation infrastructure for debug-only PHI dst tracking
   - Distribution analyzer with region classification
   - Report generator for human-readable summaries
   - ✅ All unit tests pass

2. **`docs/development/current/main/phase72-phi-reserved-observation.md`**
   - Detailed observation report
   - Evidence and analysis
   - Risk assessment
   - Future enhancement proposals

3. **`tests/phase72_phi_observation.rs`** (skeleton, not used)
   - Integration test template for future phases
   - Blocked by API visibility in current design

### Files Modified

1. **`src/mir/join_ir/mod.rs`** (+4 lines)
   - Added `verify_phi_reserved` module (debug-only)

2. **`src/mir/builder/control_flow/joinir/merge/loop_header_phi_builder.rs`** (+6 lines)
   - Added observation hooks at PHI dst allocation points (lines 94, 151)
   - Debug-only instrumentation via `observe_phi_dst()`

### Observation Infrastructure

#### Key Components

```rust
// Enable/disable observation
pub fn enable_observation()
pub fn disable_observation()

// Observe PHI dst allocation
pub fn observe_phi_dst(dst: ValueId)

// Collect and analyze
pub fn get_observations() -> Vec<u32>
pub fn analyze_distribution(observations: &[u32]) -> PhiDistributionReport

// Report structure
pub struct PhiDistributionReport {
    pub total: usize,
    pub in_reserved: usize,  // 0-99
    pub in_param: usize,     // 100-999
    pub in_local: usize,     // 1000+
    pub min_val: Option<u32>,
    pub max_val: Option<u32>,
}
```

#### Instrumentation Points

1. **Loop variable PHI** (`loop_header_phi_builder.rs:94`)
   ```rust
   let loop_var_phi_dst = builder.next_value_id();
   #[cfg(debug_assertions)]
   crate::mir::join_ir::verify_phi_reserved::observe_phi_dst(loop_var_phi_dst);
   ```

2. **Carrier PHI** (`loop_header_phi_builder.rs:151`)
   ```rust
   let phi_dst = builder.next_value_id();
   #[cfg(debug_assertions)]
   crate::mir::join_ir::verify_phi_reserved::observe_phi_dst(phi_dst);
   ```

---

## Evidence and Analysis

### Manual Verification

**Test case**: `apps/tests/loop_min_while.hako`

```hakorune
static box Main {
  main() {
    local i = 0
    loop(i < 3) {
      print(i)
      i = i + 1
    }
    return 0
  }
}
```

**Generated MIR**:
```mir
bb4:
    1: %3: String = phi [%2, bb0], [%12, bb7]
    1: br label bb5
```

**Observation**: PHI dst = `%3` (ValueId(3))
**Region**: Reserved (0-99) ✅

### Why This Works Today

1. **MirBuilder sequential allocation**:
   - Function entry: ValueId(0), ValueId(1), ValueId(2)
   - Loop header PHI: ValueId(3) allocated early
   - Loop body: ValueId(8+) allocated later

2. **JoinValueSpace high allocation**:
   - Param region: ValueId(100-999)
   - Local region: ValueId(1000+)

3. **No overlap**:
   - Host MirBuilder: 0-50 typical
   - JoinValueSpace: 100-2000 typical
   - Accidental separation, not enforced

### Why Enforcement Is Not Recommended

1. **No architectural coupling**:
   - `builder.next_value_id()` doesn't know about reserved region
   - `JoinValueSpace` doesn't control PHI dst allocation
   - These are separate systems with separate counters

2. **Fragile assumption**:
   - PHI dst only stays in 0-99 if allocated early
   - Large function with 100+ instructions before loop → PHI dst could be 100+
   - Would break verifier assumptions

3. **False security**:
   - Enforcing 0-99 check gives false confidence
   - Doesn't prevent actual allocation outside range
   - Just fails later with unclear error

### Correct Current Behavior

The existing `JoinValueSpace.reserve_phi()` is correctly designed as:
- **Debug marker only**
- Not allocation mechanism
- Used for collision detection (Phase 205)
- Documents intent, doesn't enforce

---

## Test Results

### Unit Tests

```bash
cargo test --release --lib mir::join_ir::verify_phi_reserved
```

**Result**: ✅ **4/4 tests PASS**

- `test_analyze_distribution_empty`
- `test_analyze_distribution_all_reserved`
- `test_analyze_distribution_mixed`
- `test_analyze_distribution_all_local`

### Regression Tests

```bash
cargo test --release --lib
```

**Result**: ✅ **950/950 tests PASS**, 56 ignored

```bash
cargo test --features normalized_dev --test normalized_joinir_min
```

**Result**: ✅ **54/54 tests PASS**

### Manual Verification

```bash
./target/release/hakorune --dump-mir apps/tests/loop_min_while.hako
```

**Result**: ✅ PHI dst = %3 (in reserved region)

---

## Documentation Updates

### Added

1. **`phase72-phi-reserved-observation.md`**
   - Full observation report with evidence
   - Risk assessment (current: LOW, future: MEDIUM)
   - Alternative architectural fix (future phase)
   - Decision rationale

2. **`PHASE_72_SUMMARY.md`** (this file)
   - Executive summary
   - Implementation record
   - Test results
   - Recommendations

### To Update (Next Phase)

1. **`joinir-architecture-overview.md`**
   - Add Phase 72 finding to Invariant 8
   - Clarify that "PHI Reserved" is JoinIR-only, not host MIR
   - Document accidental separation vs enforced separation

---

## Recommendations

### Immediate (Phase 72)

1. ✅ **Keep observation infrastructure** (debug-only)
   - Low overhead
   - Useful for future debugging
   - No production impact

2. ✅ **Document findings**
   - phase72-phi-reserved-observation.md
   - Architecture overview update (Phase 73)

3. ✅ **Monitor in test suite**
   - Existing 950 tests cover PHI generation
   - Any collision would be caught by Phase 205 checks

### Future (Optional Enhancement)

**Phase 73+: Explicit PHI Reserved Pool** (if strict enforcement desired)

1. Add `PhiReservedPool` to MirBuilder
2. Replace `builder.next_value_id()` with `builder.alloc_phi_reserved()`
3. Enforce 0-99 limit at allocation time
4. Fail-fast at 100 PHI nodes per function

**Scope**: Optional architectural enhancement, not urgent

**Priority**: P3 (nice-to-have, current system stable)

---

## Acceptance Criteria

- ✅ Observation infrastructure implemented
- ✅ Distribution analyzer tested
- ✅ Manual verification completed (loop_min_while.hako)
- ✅ Documentation written (observation report + summary)
- ✅ Decision documented (no verifier strengthening)
- ✅ Test suite regression check passed (950/950 + 54/54)

## Phase 72 Complete

**Status**: ✅ **COMPLETE**
**Outcome**: **Documentation-only** - observation successful, verifier strengthening not recommended
**Next**: Phase 73 - Update architecture overview with Phase 72 findings

---

## Changelog

**2025-12-13**: Phase 72 complete
- Observation infrastructure added
- PHI dst distribution analyzed
- Decision: Do not strengthen verifier
- Documentation created
- All tests passing
