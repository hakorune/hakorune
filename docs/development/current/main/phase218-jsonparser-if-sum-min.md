# Phase 218: JsonParser If-Sum Mini Test - Pattern Recognition Gap Found

## Overview

Phase 218 attempts to apply Pattern 3 if-sum infrastructure (Phase 213-217) to JsonParser-style conditional accumulation pattern.

**Status**: 🔍 **INVESTIGATION COMPLETE** - Pattern recognition gap identified

## Test Case Created

### File: `apps/tests/phase218_json_if_sum_min.hako`

**Pattern**: JsonParser-style conditional accumulation
```hako
static box JsonIfSumTest {
    sum_digits(data) {
        local sum = 0
        local i = 0
        local len = 5

        loop(i < len) {
            // JsonParser pattern: if digit > 0 { sum = sum + digit }
            if i > 0 {
                sum = sum + i  // ← Variable-based accumulation (not literal!)
                print(sum)
            } else {
                print(0)
            }
            i = i + 1
        }
        return sum
    }

    main() {
        local result = JsonIfSumTest.sum_digits(0)
        return result
    }
}
```

**Expected**: RC=10 (sum = 1 + 2 + 3 + 4)
**Actual**: RC=0 (legacy lowerer used, wrong logic)

## Investigation Results

### Observation 1: Legacy Lowerer Triggered

Debug output shows:
```
[joinir/pattern3] Generated JoinIR for Loop with If-Else PHI (Phase 195: multi-carrier)
[joinir/pattern3] Carriers: i (counter), sum (accumulator), count (counter) [Phase 195]
[joinir/pattern3] If-Else PHI in loop body:
[joinir/pattern3]   sum_new = (i % 2 == 1) ? sum+i : sum+0
```

**Finding**: Legacy template lowerer is used (hardcoded `i % 2 == 1` condition)

### Observation 2: AST-Based Lowerer NOT Called

Expected debug messages from `loop_with_if_phi_if_sum.rs`:
```
[joinir/pattern3/if-sum] Starting AST-based if-sum lowering   ← NOT SEEN
[joinir/pattern3/if-sum] Loop condition: ...                  ← NOT SEEN
```

**Finding**: `ctx.is_if_sum_pattern()` returns **false**, so AST-based lowerer is not triggered

### Observation 3: Pattern vs Literal Difference

| Test | Update Pattern | Lowerer Used | Result |
|------|---------------|--------------|--------|
| phase212_if_sum_min.hako | `sum = sum + 1` | Legacy ❌ | RC=0 |
| phase218_json_if_sum_min.hako | `sum = sum + i` | Legacy ❌ | RC=0 |

**Finding**: Both use legacy lowerer, suggesting either:
1. AST-based lowerer was never fully integrated, OR
2. There's a regression since Phase 216 documentation

### Observation 4: Phase 216 Claims Success

Phase 216 documentation states:
```markdown
**Expected**: RC=2 (sum=1 at i=1, sum=2 at i=2)
**Actual**: **RC=2** ✅
```

But current execution of `phase212_if_sum_min.hako` returns **RC=0**, not RC=2.

**Finding**: Either a regression occurred, OR Phase 216 tests were never actually run successfully

## Root Cause Analysis

### Pattern Detection Logic

File: `src/mir/builder/control_flow/joinir/patterns/pattern_pipeline.rs`
```rust
pub fn is_if_sum_pattern(&self) -> bool {
    // Check if loop_body has if statement
    let has_if = self.loop_body.as_ref().map_or(false, |body| {
        body.iter().any(|stmt| matches!(stmt, ASTNode::If { .. }))
    });

    if !has_if {
        return false;
    }

    // Check carrier pattern using name heuristics
    let summary = analyze_loop_updates(&all_names);
    summary.is_simple_if_sum_pattern()  // ← Returns false
}
```

### Why Detection Fails

Carrier detection reports:
```
Carriers: i (counter), sum (accumulator), count (counter)
```

**Issue**: A phantom `count` carrier is detected as `CounterLike` instead of not existing.

This causes `is_simple_if_sum_pattern()` to fail:
```rust
pub fn is_simple_if_sum_pattern(&self) -> bool {
    if self.counter_count() != 1 { return false; }  // ← Fails here (2 counters: i, count)
    if self.accumulation_count() < 1 { return false; }
    if self.accumulation_count() > 2 { return false; }
    true
}
```

**Root cause**: Name heuristic incorrectly classifies `count` as a counter based on its name alone.

## Implications for JsonParser Pattern

### Variable-Based Accumulation

JsonParser uses patterns like:
```hako
local digit = extract_digit(json, i)
if digit != "" {
    sum = sum + _str_to_int(digit)  // ← Variable, not literal!
}
```

This requires:
1. **Pattern recognition**: Detect `sum = sum + variable` as accumulation (not just `sum = sum + 1`)
2. **AST extraction**: Extract variable references (not just integer literals)

### Current Limitations

File: `src/mir/join_ir/lowering/loop_with_if_phi_if_sum.rs`

**extract_then_update()** only supports literal addends:
```rust
fn extract_then_update(if_stmt: &ASTNode) -> Result<(String, i64), String> {
    // ...
    match addend_expr {
        ASTNode::IntegerLiteral { value } => {
            Ok((var_name, *value))  // ← Only literals!
        }
        _ => Err("Then update addend must be integer literal".to_string())
    }
}
```

**Needed**: Support for variable references:
```rust
ASTNode::Variable { name } => {
    // Return variable name, lowerer generates Load instruction
    Ok((var_name, VariableRef(name.clone())))
}
```

## Task 218-3: Minimal Fix Strategy (80/20 Rule)

### Option A: Fix Pattern Detection (Recommended)

**File**: `src/mir/join_ir/lowering/loop_update_summary.rs`

**Issue**: Phantom `count` carrier detected
**Fix**: Improve name heuristic to not detect non-existent variables

### Option B: Support Variable Addends

**File**: `src/mir/join_ir/lowering/loop_with_if_phi_if_sum.rs`

**Issue**: `extract_then_update()` only supports literals
**Fix**: Support `ASTNode::Variable` in addend extraction

### Recommended Approach

**Start with Option A** (simpler):
1. Fix phantom `count` detection
2. Verify phase212 test passes with AST-based lowerer
3. If successful, phase218 test should also work (same pattern, different addend value)

**Only if needed, do Option B**:
1. Extend AST extraction for variable references
2. Update JoinIR generation to handle variable loads

## Files Created

1. `apps/tests/phase218_json_if_sum_min.hako` - JsonParser-style test case
2. `docs/development/current/main/phase218-jsonparser-if-sum-min.md` - This document

## Next Steps

### Immediate (Phase 219)

1. **Fix phantom carrier detection** (Option A)
   - Investigate why `count` is detected when it doesn't exist
   - Fix name heuristic or carrier enumeration logic
   - Verify phase212 test passes (RC=2)

2. **Regression check**
   - Run all Phase 212/217 tests
   - Verify RC values match Phase 216 documentation

### Future (Phase 220+)

1. **Variable addend support** (Option B)
   - Extend AST extraction for variable references
   - Update JoinIR lowerer to generate Load instructions
   - Target: JsonParser real-world pattern (`sum = sum + digit`)

2. **Method call results**
   - Support: `sum = sum + _str_to_int(digit)`
   - Requires: Expression lowering in JoinIR context

## Lessons Learned

### 1. Documentation vs Reality Gap

Phase 216 claims "RC=2 ✅" but current execution shows "RC=0 ❌".

**Learning**: Always verify documented success with actual execution. Documentation can become stale.

### 2. Legacy Code Path Persistence

Even with AST-based lowerer implemented, legacy path is still used.

**Learning**: Dual-mode architectures need clear activation conditions. If detection fails, system silently falls back.

### 3. Name Heuristics Are Fragile

Phantom `count` carrier detected based on name alone.

**Learning**: Name-based detection is a temporary solution. Phase 220+ should use AST-based analysis exclusively.

## Summary

**Phase 218 Goal**: Apply Pattern 3 if-sum to JsonParser-style loop
**Outcome**: Pattern recognition gap found (phantom carrier detection)
**Value**: Identified root cause blocking AST-based lowerer activation
**Next**: Fix carrier detection (Phase 219), then retry JsonParser pattern

**The investigation was successful - we found why AST-based lowerer doesn't activate!** 🔍
Status: Active  
Scope: JsonParser if-sum min ケース（JoinIR v2）
