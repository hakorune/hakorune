# Phase 216: Selfhost If-Sum Production Test

## Overview

Phase 216 validates that Pattern 3 if-sum implementation (Phase 213-215) works correctly with actual selfhost test cases.

**Status**: ✅ **COMPLETE** - All production tests passing with correct RC values

## Test Results

### Primary Target: `phase212_if_sum_min.hako`

**File**: `apps/tests/phase212_if_sum_min.hako`
**Test Case**: Loop with conditional accumulator (i=0..2, sum counts i>0)

```hako
static box IfSumTest {
    sum_def_count(defs) {
        local sum = 0
        local i = 0
        local len = 3

        loop(i < len) {
            if i > 0 {
                sum = sum + 1      // Conditional update
                print(sum)
            } else {
                print(0)
            }
            i = i + 1
        }
        return sum
    }

    main() {
        local result = IfSumTest.sum_def_count(0)
        return result
    }
}
```

**Expected**: RC=2 (sum=1 at i=1, sum=2 at i=2)
**Actual**: **RC=2** ✅

### Regression Tests

All existing loop patterns still passing:

| Test File | Expected | Actual | Status |
|-----------|----------|--------|--------|
| `loop_if_phi.hako` (P3) | RC=2 | RC=2 | ✅ PASS |
| `loop_min_while.hako` (P1) | RC=2 | RC=2 | ✅ PASS |
| `joinir_min_loop.hako` (P2) | RC=2 | RC=2 | ✅ PASS |

## Design Validation

### Pattern 3 If-Sum Flow

Phase 216 confirms the complete data flow for Pattern 3 if-sum:

```
Source Code (phase212_if_sum_min.hako)
    ↓
Parser (Nyash/Hakorune)
    ↓
AST with Loop + If Statement
    ↓
Pattern Detection: Pattern3IfPhi
    ↓
JoinIR Lowerer (loop_with_if_phi_if_sum.rs)
    ├─ Extracts: loop condition, if condition, updates
    └─ Creates: JoinFragmentMeta with expr_result
    ↓
JoinInlineBoundary (with_expr_result)
    ↓
JoinIRConversionPipeline
    ├─ Merges blocks
    ├─ Creates exit PHI for sum
    └─ Returns merge_result (exit PHI ValueId)
    ↓
Pattern3 Dispatcher
    └─ Returns expr_result (not Void)
    ↓
Final MIR Function
    ├─ Loop produces ValueId (sum)
    └─ Return statement uses that ValueId
    ↓
VM Execution
    └─ RC=2 (actual sum value, not 0)
```

## Architecture Verification

### Phase 215-2 Fixes Confirmed

✅ **Fix 1 Verified**: JoinIR lowerer creates expr_result
- `loop_with_if_phi_if_sum.rs` uses `with_expr_result(sum_final, exit_meta)`
- expr_result properly carries sum's exit PHI ValueId

✅ **Fix 2 Verified**: Boundary builder passes expr_result
- `pattern3_with_if_phi.rs` calls `.with_expr_result(Some(expr_id))`
- expr_result flows to ExitLineReconnector

✅ **Fix 3 Verified**: Final return uses merge result
- Pattern dispatcher returns `expr_val` instead of Void
- Loop result (RC=2) propagates to main's return statement

### ExprResult Exit Contract

Pattern 3 now matches Pattern 2's behavior:
- **Expr-position loops**: Return actual computation result
- **Statement-position loops**: Return Void, update variable_map only
- **Unified contract**: Consistent with Pattern 2 (Phase 172/190)

## Implementation Quality

### Code Cleanliness
- No special casing for expr_result in final return
- Conditional return based on merge_result (Option<ValueId>)
- Follows established Pattern 2 pattern

### No Regressions
- All existing tests still passing
- Pattern 1/2/4 behavior unchanged
- Legacy P3 mode unaffected

### Documentation
- Phase 215 design doc complete
- Phase 212 doc updated with RC=2 success
- This Phase 216 report documents production validation

## Lessons Learned

1. **ExprResult Contract Importance**: Phase 215 fixes were essential
   - Without proper expr_result propagation, RC=0 problem was unfixable
   - Pattern 2 reference implementation was key to understanding correct flow

2. **AST-Based Lowering Works**: if-sum pattern extraction reliable
   - Loop condition extraction works
   - If condition extraction works
   - Update extraction works
   - Exit PHI handling correct

3. **Minimal Changes Approach**: Phase 216 needed no fixes
   - All work was in Phase 213-215
   - Production validation just confirmed Phase 215 sufficiency
   - Risk minimal, confidence high

## Future Work

### Phase 217: Multi-Carrier If-Sum
- Extend to sum+count pattern (2 accumulators)
- Pattern already supports (Phase 214 dynamic join_inputs)
- Test file candidate: `Candidate 2` from Phase 211 search

### Phase 218: Nested If-Else
- Complex conditional patterns
- Test file candidate: `esc_json()` from Phase 211 search
- Validates nested if handling inside loops

### Phase 219: JsonParser Integration
- Apply if-sum pattern to actual JsonParser loops
- First target: numeric processing loop
- Validates selfhost compiler real use case

## Files Modified

**No source code changes** in Phase 216 - all fixes were in Phase 215.

## Summary

Phase 216 successfully validates that Pattern 3 if-sum implementation is production-ready:
- ✅ Target test (`phase212_if_sum_min.hako`) returns RC=2
- ✅ All regression tests passing
- ✅ ExprResult contract verified
- ✅ No additional bugs found

The complete Phase 213-215 work (AST-based if-sum lowering + ExprResult exit contract) is now validated on production test case. Ready to proceed with multi-carrier and nested patterns (Phase 217+).
Status: Active  
Scope: selfhost If-sum 本番適用（JoinIR v2）
