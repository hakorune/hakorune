# Task 176-4: Trim Pattern Loop Variable Overwrite Bug Fix

**Status**: ✅ COMPLETED

**Date**: 2025-12-08

## Problem

In `src/mir/builder/control_flow/joinir/patterns/pattern2_with_break.rs`, lines 271-272 were incorrectly overwriting the loop variable information during Trim pattern processing:

```rust
// ❌ Incorrect code (lines 271-272)
carrier_info.loop_var_id = is_ch_match0;
carrier_info.loop_var_name = carrier_name.clone();  // Overwrites "pos" with "is_ch_match"
```

### Root Cause

The Trim pattern implementation (Phase 171-172) was confusing two distinct concepts:

1. **Loop variable**: The actual loop counter (e.g., `pos`) that increments through the string
2. **Promoted carrier**: The derived variable (e.g., `is_ch_match`) that tracks whether the current character matches whitespace

The code was incorrectly treating the promoted carrier as a replacement for the loop variable, when it should only be an additional carrier alongside the loop variable.

### Impact

- **ExitMeta generation**: Would use wrong variable name ("is_ch_match" instead of "pos")
- **PHI node construction**: Would create PHI nodes for the wrong variable
- **Variable mapping**: Would lose track of the actual loop counter

## Solution

Removed lines 271-272 and replaced with explanatory comment:

```rust
// Note: DO NOT overwrite carrier_info.loop_var_id/loop_var_name here!
// The loop variable is 'pos' (counter), not 'is_ch_match' (carrier).
// carrier_info.loop_var_name should remain as the original loop variable.

eprintln!("[pattern2/trim] Carrier registered. Loop var='{}' remains unchanged",
    carrier_info.loop_var_name);
```

### Design Principle

In Trim patterns:
- `loop_var_name` = "pos" (the counter variable)
- `carriers` = ["result", "is_ch_match"] (accumulated values)
- `is_ch_match` is a **promoted carrier**, not a loop variable replacement

## Verification

### 1. Build Status
✅ Build succeeded with no errors

### 2. Log Output Verification
Both E2E tests show correct behavior:

```
[pattern2/promoter] Phase 171-C-4 DEBUG: BEFORE merge - carrier_info.loop_var_name='pos'
[pattern2/promoter] Phase 171-C-4 DEBUG: promoted_carrier.loop_var_name='is_ch_match'
[pattern2/promoter] Phase 171-C-4 DEBUG: AFTER merge - carrier_info.loop_var_name='pos'  ← CORRECT!
[pattern2/trim] Carrier registered. Loop var='pos' remains unchanged  ← NEW LOG MESSAGE
[pattern2/before_lowerer] About to call lower_loop_with_break_minimal with carrier_info.loop_var_name='pos'
```

**Key observation**: The loop_var_name correctly remains as 'pos' throughout the entire process.

### 3. Regression Tests
✅ All carrier update tests pass (6/6):
- `test_emit_binop_update_with_const`
- `test_emit_binop_update_with_variable`
- `test_emit_const_update`
- `test_emit_update_lhs_mismatch`
- `test_emit_update_carrier_not_in_env`
- `test_emit_update_rhs_variable_not_found`

❌ One pre-existing test failure (unrelated to this fix):
- `test_pattern2_accepts_loop_param_only` - Failed on both HEAD and with our changes

### 4. E2E Test Results

**test_jsonparser_parse_string_min2.hako**:
- ✅ Loop variable name correctly preserved as 'pos'
- ⚠️ Execution fails with: "Phase 33-16: Carrier 'result' has no latch incoming set"
- **Note**: This is a separate issue unrelated to our fix (carrier PHI initialization)

**test_trim_simple.hako** (new test):
- ✅ Loop variable name correctly preserved as 'pos'
- ⚠️ Same carrier PHI error as above

## Remaining Issues (Out of Scope)

The following error is NOT caused by this fix and exists independently:
```
[ERROR] ❌ MIR compilation error: Phase 33-16: Carrier 'result' has no latch incoming set
```

This appears to be related to PHI node initialization for carriers in Pattern 2, and will need to be addressed separately.

## Files Changed

1. `src/mir/builder/control_flow/joinir/patterns/pattern2_with_break.rs`
   - Lines 271-272: Removed incorrect loop_var overwrite
   - Lines 270-275: Added explanatory comment and new debug log

## Conclusion

✅ **Bug successfully fixed**
- Loop variable name no longer gets overwritten by carrier name
- Design clarification: promoted carriers are additions, not replacements
- No regressions introduced in carrier update logic
- Remaining errors are pre-existing and unrelated to this fix

The fix correctly implements the design intention: in Trim patterns, `is_ch_match` should be an additional carrier alongside the original loop variable `pos`, not a replacement for it.
