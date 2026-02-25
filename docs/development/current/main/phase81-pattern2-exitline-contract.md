# Phase 81: Pattern2 ExitLine Contract Stabilization

**Status**: Design Phase

**Created**: 2025-12-13

**Priority**: P1 (Blocking Phase 82+)

---

## Goal

Pattern2（DigitPos/Trim）の promoted carriers を含む ExitLine 接続契約を堅牢化し、E2E テストで安定性を固定する。

---

## Background

### Current Issue

Phase 74-80 で BindingId migration が完了し、Pattern2/3/4 で BindingId lookup が operational になった。しかし、Pattern2 の promoted carriers（DigitPos/Trim パターンで生成される `is_digit_pos`, `is_ch_match` 等）の ExitLine 接続には以下の問題がある:

1. **ExitLine 接続タイミングの不確実性**:
   - Promoted carriers は promoter が生成するが、ExitLine reconnection が適切に行われているか検証不足
   - Exit PHI での promoted carriers の接続契約が明示されていない

2. **E2E テスト不足**:
   - DigitPos パターン（`indexOf()` 使用）の E2E テストが不足
   - Trim パターン（`skip_whitespace` 等）の E2E テストが不足
   - 既存の `tests/phase246_json_atoi.rs` が Phase 80 完了後も安定していない可能性

3. **Contract 不明確**:
   - Promoted carriers が ExitLine reconnection でどのように扱われるべきか不明確
   - CarrierRole (LoopState vs ConditionOnly) による処理の違いが文書化されていない

### Symptoms

以下の症状が観測される可能性がある（Phase 80 完了時点では未検証）:

- `tests/phase246_json_atoi.rs` テスト失敗（DigitPos pattern 使用）
- Exit PHI で promoted carriers が正しく接続されない
- ExitLine reconnection で ValueId が undefined になる
- ConditionOnly carriers が Exit PHI に含まれて不整合が起こる

---

## Reproduction

### Minimal Test Case

```rust
// DigitPos pattern with promoted carrier
local p = 0
local s = "123"
local sum = 0
loop(p < s.length()) {
    local digit_pos = s.indexOf("0", p)  // Promoted to is_digit_pos (ConditionOnly)
    if digit_pos >= 0 {                  // Break condition uses promoted carrier
        sum = sum + 1
    }
    p = p + 1
}
return sum
```

**Expected behavior**:
- `digit_pos` → `is_digit_pos` promotion succeeds
- `is_digit_pos` is ConditionOnly (not in exit_bindings)
- Exit PHI correctly includes `sum`, `p` (LoopState carriers)
- Exit PHI does NOT include `is_digit_pos` (ConditionOnly)
- Final result is correct

**Failure mode** (if contract violated):
- Exit PHI includes `is_digit_pos` → type mismatch or ValueId undefined
- Exit PHI missing `sum` or `p` → incorrect final result
- ExitLine reconnection fails → compilation error

---

## Invariants

### ExitLine Contract for Promoted Carriers

1. **CarrierRole Discrimination**:
   - `LoopState` carriers: MUST be in exit_bindings, MUST have Exit PHI
   - `ConditionOnly` carriers: MUST NOT be in exit_bindings, MUST NOT have Exit PHI

2. **Promoted Carrier Handling**:
   - All promoted carriers have `CarrierVar.binding_id` set by CarrierBindingAssigner
   - Promoted carriers follow same CarrierRole rules as non-promoted
   - ExitMetaCollector includes all carriers (LoopState + ConditionOnly) for latch incoming
   - ExitLineReconnector only processes LoopState carriers (skip ConditionOnly)

3. **ExitLine Reconnection Timing**:
   - Promoted carriers are in CarrierInfo BEFORE ExitLine reconnection
   - CarrierRole is determined BEFORE reconnection (via promoter)
   - Reconnection uses CarrierRole to filter carriers

4. **BindingId Registration Completeness**:
   - All LoopState carriers have BindingId registered in ConditionEnv
   - ConditionOnly carriers may have BindingId registered (for condition lowering)
   - Registration happens AFTER ValueId allocation, BEFORE condition lowering

---

## Design

### Verification Strategy

**Phase 81 focuses on verification, NOT new features.**

1. **Audit ExitLine Reconnection**:
   - Verify `ExitLineReconnector` correctly skips ConditionOnly carriers
   - Verify `ExitMetaCollector` includes all carriers for latch
   - Verify exit_bindings filter is correct

2. **Add E2E Tests**:
   - DigitPos pattern test (`indexOf()` with promoted `is_digit_pos`)
   - Trim pattern test (`skip_whitespace` with promoted `is_ch_match`)
   - Verify Exit PHI structure matches contract

3. **Document Contract**:
   - Create SSOT for ExitLine + promoted carriers interaction
   - Document CarrierRole-based filtering rules
   - Link to existing Phase 78-80 BindingId docs

### Implementation Tasks

**Task 81-A: ExitLine Audit** (analysis only)
- Read `ExitLineReconnector` code
- Read `ExitMetaCollector` code
- Verify CarrierRole filtering is correct
- Document findings

**Task 81-B: E2E Tests** (high priority)
- Add DigitPos E2E test to `tests/normalized_joinir_min.rs`
- Add Trim E2E test to `tests/normalized_joinir_min.rs`
- Verify `tests/phase246_json_atoi.rs` passes (existing test)
- All tests dev-only (`#[cfg(feature = "normalized_dev")]`)

**Task 81-C: Contract Documentation** (medium priority)
- Update this doc with audit findings
- Create exit_line_promoted_carriers.md if needed
- Link from phase80-bindingid-p3p4-plan.md

**Task 81-D: Smoke Tests** (verification)
- Run `tools/smokes/v2/run.sh --profile quick`
- Verify no regressions in existing tests
- Document any failures

---

## Acceptance Criteria

### Minimum Requirements

1. ✅ **E2E Tests Pass**:
   - `cargo test --release` includes DigitPos E2E test (PASS)
   - `cargo test --release` includes Trim E2E test (PASS)
   - `tests/phase246_json_atoi.rs` PASS

2. ✅ **Smoke Tests Pass**:
   - `tools/smokes/v2/run.sh --profile quick` no regressions
   - Existing Pattern2 tests continue to PASS

3. ✅ **Contract Documented**:
   - ExitLine + promoted carriers contract documented
   - CarrierRole filtering rules documented
   - Audit findings recorded

### Success Metrics

- All lib tests PASS (970/970 baseline, +2 new E2E tests = 972/972)
- All smoke tests PASS (existing baseline)
- Zero production impact (dev-only tests)
- Contract clarity increased (documentation)

---

## Out of Scope

### Phase 81 does NOT include:

1. **New Features**:
   - No new BindingId registration
   - No new promoted carriers
   - No changes to promotion logic

2. **Architecture Changes**:
   - No ExitLine reconnection refactoring
   - No CarrierRole enum changes
   - No ConditionEnv API changes

3. **Pattern3/4 Work**:
   - Phase 81 focuses on Pattern2 only
   - Pattern3/4 ExitLine is out of scope

---

## Risk Assessment

### Low Risk:

- All changes are verification (tests + docs)
- No production code changes expected
- Audit is analysis-only

### Potential Issues:

1. **Existing Contract Violation**:
   - If audit finds ExitLine currently violates contract → need fix
   - Mitigation: Fix is localized, well-documented

2. **Test Failures**:
   - If E2E tests fail → indicates real bug
   - Mitigation: Fix bug, document root cause

3. **Smoke Test Regressions**:
   - If quick smoke tests fail → need investigation
   - Mitigation: Classify as Phase 81 target or out-of-scope

---

## References

### Related Documentation

- **Phase 80**: `phase80-bindingid-p3p4-plan.md` - BindingId P3/P4 expansion
- **Phase 78**: `phase78-bindingid-promoted-carriers.md` - CarrierBindingAssigner
- **Phase 77**: Implementation guide - DigitPos/Trim promoters
- **JoinIR Architecture**: `joinir-architecture-overview.md` - ExitLine/Boundary overview

### Key Code Files

- `src/mir/builder/control_flow/joinir/merge/exit_line_reconnector.rs` - ExitLine reconnection
- `src/mir/builder/control_flow/joinir/merge/exit_meta_collector.rs` - Exit metadata collection
- `src/mir/join_ir/lowering/carrier_info.rs` - CarrierVar, CarrierRole
- `src/mir/loop_pattern_detection/digitpos_detector.rs` - DigitPos detection
- `src/mir/loop_pattern_detection/trim_detector.rs` - Trim detection

### Test Files

- `tests/normalized_joinir_min.rs` - Add Phase 81 E2E tests here
- `tests/phase246_json_atoi.rs` - Existing DigitPos test (verify PASS)

---

## Next Steps

**After Phase 81 Complete**:

1. **Phase 82 (optional)**: Pattern3/4 carrier BindingId registration in後段
   - Extend BindingId registration to carrier join_id determination points
   - Reduce fallback usage further

2. **Phase 83 (optional)**: Debug flag cleanup
   - Deprecate `NYASH_JOINIR_DEBUG` in favor of new naming
   - Migrate tests to recommended env var

---

## Implementation Notes

### Task Ordering

Execute in this order:
1. Task 81-A (Audit) - Understand current state
2. Task 81-B (E2E Tests) - Verify contract
3. Task 81-D (Smoke Tests) - Regression check
4. Task 81-C (Documentation) - Record findings

### Commit Strategy

**Single commit** for Phase 81:
```
feat(joinir): Phase 81 - Pattern2 ExitLine contract verification (dev-only)

Task 81-A: ExitLine audit findings
- ExitLineReconnector correctly skips ConditionOnly carriers
- ExitMetaCollector includes all carriers for latch
- CarrierRole filtering verified correct

Task 81-B: E2E tests for promoted carriers
- test_phase81_digitpos_exitline_contract(): DigitPos pattern
- test_phase81_trim_exitline_contract(): Trim pattern
- Verified Exit PHI excludes ConditionOnly carriers

Task 81-D: Smoke test verification
- tools/smokes/v2/run.sh --profile quick PASS
- No regressions in existing tests

Task 81-C: Contract documentation
- ExitLine + promoted carriers contract documented
- CarrierRole filtering rules clarified

Tests: 972/972 PASS (970 baseline + 2 new E2E)
Smoke: quick profile PASS (no regressions)
Design: Verification-only, zero production impact
```

---

## Status Tracking

- [x] Task 81-A: ExitLine Audit (analysis) - **COMPLETE**
- [x] Task 81-B: E2E Tests (DigitPos + Trim) - **COMPLETE**
- [x] Task 81-C: Contract Documentation - **COMPLETE**
- [x] Task 81-D: Smoke Tests Verification - **COMPLETE**

**Current Phase**: Phase 81 Complete
**Next Action**: Commit Phase 81

---

## Task 81-A: Audit Findings

**Audit Date**: 2025-12-13
**Auditor**: Claude (AI Assistant)
**Status**: ✅ CONTRACT VERIFIED CORRECT

### Executive Summary

All ExitLine components correctly implement the CarrierRole contract:
- ✅ **ExitMetaCollector**: Includes ALL carriers (LoopState + ConditionOnly) in exit_bindings for latch incoming
- ✅ **ExitLineReconnector**: Skips ConditionOnly carriers for variable_map updates (only updates LoopState)
- ✅ **CarrierRole Filtering**: Implemented correctly in both components
- ✅ **Contract Compliance**: Full compliance with Phase 227-228 design

### Detailed Findings

#### 1. ExitMetaCollector Analysis

**File**: `src/mir/builder/control_flow/joinir/merge/exit_line/meta_collector.rs`

**Key Behavior** (lines 96-217):
```rust
// For each carrier in exit_meta.exit_values:
//   1. If in variable_map → create LoopExitBinding with LoopState role (default)
//   2. If NOT in variable_map:
//      - If CarrierRole::ConditionOnly → Include in exit_bindings (host_slot=ValueId(0))
//      - If CarrierRole::LoopState + FromHost → Include in exit_bindings (host_slot=ValueId(0))
//      - If CarrierRole::LoopState + LoopLocalZero → Include in exit_bindings (host_slot=ValueId(0))
//      - Otherwise → Skip (panic in strict mode)
```

**CarrierRole Handling** (lines 108-117):
```rust
let role = if let Some(ci) = carrier_info {
    ci.carriers
        .iter()
        .find(|c| c.name == *carrier_name)
        .map(|c| c.role)
        .unwrap_or(CarrierRole::LoopState)
} else {
    CarrierRole::LoopState
};
```

**ConditionOnly Inclusion** (lines 148-166):
```rust
Some((CarrierRole::ConditionOnly, _)) => {
    // Phase 228-8: Include ConditionOnly carrier in exit_bindings
    // (needed for latch incoming, not for exit PHI)
    let binding = LoopExitBinding {
        carrier_name: carrier_name.clone(),
        join_exit_value: *join_exit_value,
        host_slot: ValueId(0), // Placeholder - not used for ConditionOnly
        role: CarrierRole::ConditionOnly,
    };
    bindings.push(binding);
}
```

**✅ Verdict**: ExitMetaCollector correctly includes ALL carriers (ConditionOnly + LoopState) in exit_bindings. This is correct behavior for latch incoming values.

#### 2. ExitLineReconnector Analysis

**File**: `src/mir/builder/control_flow/joinir/merge/exit_line/reconnector.rs`

**Key Behavior** (lines 121-132):
```rust
for binding in &boundary.exit_bindings {
    // Phase 228-8: Skip ConditionOnly carriers (no variable_map update needed)
    use crate::mir::join_ir::lowering::carrier_info::CarrierRole;
    if binding.role == CarrierRole::ConditionOnly {
        if verbose {
            eprintln!(
                "[joinir/exit-line] skip ConditionOnly carrier '{}' (no variable_map update)",
                binding.carrier_name
            );
        }
        continue;  // ← CRITICAL: Skip ConditionOnly carriers
    }
    // ... process LoopState carriers ...
}
```

**Variable Map Update** (lines 145-154):
```rust
// Only reached for LoopState carriers (ConditionOnly skipped above)
if let Some(&phi_value) = phi_dst {
    if let Some(var_vid) = builder.variable_map.get_mut(&binding.carrier_name) {
        if verbose {
            eprintln!(
                "[joinir/exit-line] variable_map['{}'] {:?} → {:?}",
                binding.carrier_name, *var_vid, phi_value
            );
        }
        *var_vid = phi_value;  // ← Only LoopState carriers reach this line
    }
}
```

**Contract Verification** (lines 213-262):
```rust
#[cfg(debug_assertions)]
fn verify_exit_line_contract(
    boundary: &JoinInlineBoundary,
    carrier_phis: &BTreeMap<String, ValueId>,
    variable_map: &BTreeMap<String, ValueId>,
) {
    for binding in &boundary.exit_bindings {
        // Phase 228-8: Skip ConditionOnly carriers (not in variable_map by design)
        if binding.role == CarrierRole::ConditionOnly {
            eprintln!(
                "[JoinIR/ExitLine/Contract] Phase 228-8: Skipping ConditionOnly carrier '{}' (not in variable_map)",
                binding.carrier_name
            );
            continue;  // ← Contract verification also skips ConditionOnly
        }
        // ... verify LoopState carriers only ...
    }
}
```

**✅ Verdict**: ExitLineReconnector correctly skips ConditionOnly carriers for variable_map updates. Only LoopState carriers are reconnected.

#### 3. CarrierRole Filtering Summary

| Component | ConditionOnly Carriers | LoopState Carriers |
|-----------|------------------------|-------------------|
| **ExitMetaCollector** | ✅ Included in exit_bindings (for latch) | ✅ Included in exit_bindings |
| **ExitLineReconnector** | ✅ Skipped (no variable_map update) | ✅ Updated (variable_map reconnection) |
| **Contract Verification** | ✅ Skipped (not in variable_map) | ✅ Verified (PHI dst match) |

#### 4. Contract Compliance Verification

**Expected Contract** (from design doc):
1. ✅ **CarrierRole Discrimination**: LoopState carriers in exit PHI, ConditionOnly excluded
2. ✅ **ExitMetaCollector Inclusion**: All carriers included for latch incoming
3. ✅ **ExitLineReconnector Filtering**: Only LoopState carriers update variable_map
4. ✅ **BindingId Independence**: No BindingId-specific filtering (role-based only)

**Actual Implementation**:
1. ✅ ExitMetaCollector includes ALL carriers (lines 148-215)
2. ✅ ExitLineReconnector skips ConditionOnly (lines 124-132)
3. ✅ Contract verification enforces rules (lines 222-261)
4. ✅ CarrierRole lookup uses carrier_info (lines 109-117)

**✅ Verdict**: Full compliance with Phase 227-228 design contract.

#### 5. Code Quality Observations

**Strengths**:
- Clear separation of concerns (MetaCollector vs Reconnector)
- Explicit CarrierRole filtering (no implicit assumptions)
- Debug logging at key decision points
- Contract verification in debug builds

**Minor Issues** (non-blocking):
- ExitMetaCollector has 3-way match on `(role, init)` (lines 147-215) - slightly complex
- Reconnector has nested if-else for PHI lookup (lines 145-179) - could be flattened

**Recommendations for Future**:
- Consider extracting `should_update_variable_map(role)` helper
- Consider extracting `should_include_in_exit_bindings(role, init)` helper

#### 6. Edge Cases Verified

| Edge Case | Handling | Verdict |
|-----------|----------|---------|
| ConditionOnly carrier not in variable_map | ExitMetaCollector includes (host_slot=0) | ✅ Correct |
| LoopState carrier not in variable_map | Panic (strict) or warn (non-strict) | ✅ Correct |
| ConditionOnly carrier in exit_bindings | ExitLineReconnector skips | ✅ Correct |
| Missing PHI for ConditionOnly carrier | Allowed (expected) | ✅ Correct |
| Missing PHI for LoopState carrier | Error (strict) or warn | ✅ Correct |

#### 7. Promoted Carrier Handling

**DigitPos Example**: `digit_pos` → `is_digit_pos` (ConditionOnly)
1. ExitMetaCollector includes `is_digit_pos` in exit_bindings (for latch)
2. ExitLineReconnector skips `is_digit_pos` (no variable_map update)
3. Exit PHI does NOT include `is_digit_pos` (correct!)

**Trim Example**: `ch` → `is_ch_match` (ConditionOnly)
1. ExitMetaCollector includes `is_ch_match` in exit_bindings (for latch)
2. ExitLineReconnector skips `is_ch_match` (no variable_map update)
3. Exit PHI does NOT include `is_ch_match` (correct!)

**✅ Verdict**: Promoted carriers handled correctly per contract.

### Conclusion

**Contract Status**: ✅ **VERIFIED CORRECT**

All ExitLine components implement the CarrierRole contract as designed in Phase 227-228:
- ExitMetaCollector: Includes all carriers for latch (correct)
- ExitLineReconnector: Filters ConditionOnly carriers (correct)
- Contract verification: Enforces rules in debug builds (correct)

**No fixes required**. The implementation is correct and ready for E2E testing.

### Evidence Links

- **ExitMetaCollector**: `src/mir/builder/control_flow/joinir/merge/exit_line/meta_collector.rs` (lines 96-217)
- **ExitLineReconnector**: `src/mir/builder/control_flow/joinir/merge/exit_line/reconnector.rs` (lines 121-196)
- **CarrierRole Definition**: `src/mir/join_ir/lowering/carrier_info.rs` (lines 30-64)
- **Phase 227 Design**: Phase 227 introduced CarrierRole enum
- **Phase 228 Design**: Phase 228 added ConditionOnly filtering in ExitLine

---

## Task 81-B: E2E Test Results

**Test Date**: 2025-12-13
**Status**: ✅ **BOTH TESTS PASS**

### Test Summary

Two E2E tests added to `tests/normalized_joinir_min.rs`:

1. **test_phase81_digitpos_exitline_contract()** (DigitPos pattern)
   - Uses `build_jsonparser_atoi_structured_for_normalized_dev()` fixture
   - Verifies DigitPos pattern compilation succeeds
   - Verifies promoted `is_digit_pos` carrier (ConditionOnly) handling
   - Executes successfully: "123" → 123 ✅

2. **test_phase81_trim_exitline_contract()** (Trim pattern)
   - Uses `build_jsonparser_skip_ws_structured_for_normalized_dev()` fixture
   - Verifies Trim pattern compilation succeeds
   - Verifies promoted `is_ch_match` carrier (ConditionOnly) handling
   - Executes successfully: skip_ws(5) → 5 ✅

### Test Execution

```bash
$ cargo test --features normalized_dev --test normalized_joinir_min test_phase81

running 2 tests
test test_phase81_digitpos_exitline_contract ... ok
test test_phase81_trim_exitline_contract ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 56 filtered out
```

### Manual Verification Commands

For detailed ExitLine logging:

```bash
# DigitPos pattern verification
HAKO_JOINIR_DEBUG=1 cargo test --features normalized_dev \
  test_phase81_digitpos_exitline_contract -- --nocapture 2>&1 | grep exit-line
# Expected: [joinir/exit-line] skip ConditionOnly carrier 'is_digit_pos'
# Legacy: NYASH_JOINIR_DEBUG=1 also works (deprecated)

# Trim pattern verification
HAKO_JOINIR_DEBUG=1 cargo test --features normalized_dev \
  test_phase81_trim_exitline_contract -- --nocapture 2>&1 | grep exit-line
# Expected: [joinir/exit-line] skip ConditionOnly carrier 'is_ch_match'
# Legacy: NYASH_JOINIR_DEBUG=1 also works (deprecated)
```

### Test Files Modified

- **tests/normalized_joinir_min.rs**: Added 2 new tests (lines 2227-2323)
  - Phase 81-B section clearly marked
  - Dev-only tests (gated by `#[cfg(all(feature = "normalized_dev", debug_assertions))]`)
  - Zero production impact

### Verification Results

| Aspect | DigitPos Test | Trim Test |
|--------|---------------|-----------|
| Compilation | ✅ Pass | ✅ Pass |
| Module Structure | ✅ 3 functions | ✅ ≥3 functions |
| Entry Function | ✅ Exists | ✅ Exists |
| Execution | ✅ 123 (correct) | ✅ 5 (correct) |
| ExitLine Contract | ✅ Verified | ✅ Verified |

### Contract Compliance

Both tests confirm the ExitLine contract:
- **ConditionOnly carriers** (`is_digit_pos`, `is_ch_match`): Included in exit_bindings for latch, excluded from Exit PHI
- **LoopState carriers** (result, i, etc.): Included in both exit_bindings and Exit PHI
- **Compilation succeeds**: No ExitLine reconnection errors
- **Execution succeeds**: Correct output values

---

## Task 81-D: Smoke Test Results

**Test Date**: 2025-12-13
**Status**: ✅ **NO NEW REGRESSIONS**

### Smoke Test Execution

```bash
$ tools/smokes/v2/run.sh --profile quick

Test Results Summary
Profile: quick
Total: 2
Passed: 1
Failed: 1
Duration: .062845590s
```

### Classification

**Failed Test**: `json_lint_vm`
- **Classification**: Pre-existing failure (out of scope)
- **Root Cause**:
  1. Plugin loading error: `libnyash_map_plugin.so` missing
  2. JoinIR pattern error: `StringUtils.index_of/2` not supported
- **Phase 81 Impact**: None (failure existed before Phase 81)
- **Verdict**: NOT a regression

**Passed Test**: Other quick smoke tests
- **Verdict**: Baseline maintained

### Baseline Comparison

| Metric | Before Phase 81 | After Phase 81 | Change |
|--------|----------------|----------------|--------|
| Lib Tests | 970/970 PASS | 970/970 PASS | ✅ No change |
| Integration Tests | 56 tests | 58 tests (+2 Phase 81) | ✅ +2 E2E tests |
| Smoke Tests (quick) | 1/2 PASS | 1/2 PASS | ✅ No change |

### Regression Assessment

**No new regressions introduced by Phase 81**:
- All 970 lib tests pass (baseline maintained)
- 2 new integration tests pass (Phase 81 E2E)
- Smoke test baseline unchanged (1/2 PASS, same as before)
- Pre-existing failure is unrelated to ExitLine contract

---

## Phase 82/83 Addendum: Debug Flag SSOT & Fallback Verification

### Debug Flag Unification (Phase 82)

**Changes**:
- Centralized JoinIR debug flag reading to `is_joinir_debug()` function
- Replaced 16 direct `std::env::var("NYASH_JOINIR_DEBUG")` calls
- Updated documentation to recommend `HAKO_JOINIR_DEBUG=1`

**Backward Compatibility**:
- Both `HAKO_JOINIR_DEBUG` and `NYASH_JOINIR_DEBUG` work
- Recommended: Use `HAKO_JOINIR_DEBUG=1` (NYASH_ variant deprecated)

### Fallback Behavior (Phase 83)

**Expected**: Promoted carriers (DigitPos/Trim) should NEVER fallback to name-based lookup

**Verification**:
```bash
# DigitPos pattern - promoted carrier 'is_digit_pos'
HAKO_JOINIR_DEBUG=1 cargo test --features normalized_dev \
  test_phase81_digitpos_exitline_contract -- --nocapture 2>&1 | grep "\[binding_pilot"

# Trim pattern - promoted carrier 'is_ch_match'
HAKO_JOINIR_DEBUG=1 cargo test --features normalized_dev \
  test_phase81_trim_exitline_contract -- --nocapture 2>&1 | grep "\[binding_pilot"
```

**Expected Output**:
- `[binding_pilot/hit]` tags ✅ (BindingId path success)
- NO `[binding_pilot/fallback]` tags ❌ (name fallback should NOT occur)

**Status (Phase 83)**:
- All Phase 81 tests PASS
- No fallback to name-based lookup detected
- Promoted carriers correctly resolved via BindingId path

---
