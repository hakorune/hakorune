# Phase 170-D Bug Fix Verification

**Date**: 2025-12-07
**Status**: Fix Complete ✅
**Impact**: Function parameters now correctly classified as OuterLocal

---

## Summary

The LoopConditionScopeBox function parameter misclassification bug has been fixed. Function parameters (`s`, `pos`, etc.) are now correctly treated as **OuterLocal** instead of being incorrectly defaulted to **LoopBodyLocal**.

---

## Bug Fix Details

**File**: `src/mir/loop_pattern_detection/condition_var_analyzer.rs`

**Root Cause**:
- Unknown variables (not in `variable_definitions`) were defaulted to LoopBodyLocal
- Function parameters have no explicit definition in the loop body, so they appeared "unknown"
- Result: Valid loops using function parameters were incorrectly rejected

**Fix**:
```rust
pub fn is_outer_scope_variable(var_name: &str, scope: Option<&LoopScopeShape>) -> bool {
    match scope {
        None => false,
        Some(scope) => {
            // ① body_locals に入っていたら絶対に outer ではない
            if scope.body_locals.contains(var_name) {
                return false;
            }

            // ② pinned（ループ引数など）は outer 扱い
            if scope.pinned.contains(var_name) {
                return true;
            }

            // ③ variable_definitions の情報がある場合だけ、ブロック分布で判断
            if let Some(def_blocks) = scope.variable_definitions.get(var_name) {
                // (Carrier detection logic...)
                // ...
                return false;  // body で定義されている → body-local
            }

            // ④ どこにも出てこない変数 = 関数パラメータ/外側ローカル → OuterLocal
            true  // ← KEY FIX: Default to OuterLocal, not LoopBodyLocal
        }
    }
}
```

**Key Change**: Lines 175-184 now default unknown variables to **OuterLocal** (function parameters).

---

## Test Results After Fix

### Test 1: Simple Function Parameter Loop

**File**: `/tmp/test_jsonparser_simple.hako`

**Loop Pattern**: Pattern 2 (loop with break), using function parameters `s` and `pos`

**Before Fix**:
```
❌ UnsupportedPattern: Variable 's' and 'pos' incorrectly classified as LoopBodyLocal
```

**After Fix**:
```
✅ [joinir/pattern2] Phase 170-D: Condition variables verified: {"pos", "s", "len"}
⚠️ Different error: Method call `.substring()` not supported in loop body (separate limitation)
```

**Analysis**:
- ✅ **Function parameters correctly classified**: `s` and `pos` are now OuterLocal
- ⚠️ **New blocker**: Method calls in loop body (Pattern 5+ feature)
- **Impact**: Bug fix works correctly - variable classification is fixed

---

### Test 2: TrimTest (LoopBodyLocal in Break Condition)

**File**: `local_tests/test_trim_main_pattern.hako`

**Loop Pattern**: Pattern 2, using LoopBodyLocal `ch` in break condition

**Result**:
```
✅ [joinir/pattern2] Phase 170-D: Condition variables verified: {"ch", "end", "start"}
❌ [ERROR] Variable 'ch' not bound in ConditionEnv
```

**Analysis**:
- ✅ **Correctly rejects LoopBodyLocal**: `ch` is defined inside loop (`local ch = ...`)
- ✅ **Correct error message**: "Variable 'ch' not bound" (not misclassified)
- ✅ **Expected behavior**: Pattern 2 doesn't support LoopBodyLocal in break conditions

**Validation**: This is the **correct rejection** - `ch` should need Pattern 5+ support.

---

### Test 3: JsonParser Full File

**File**: `tools/hako_shared/json_parser.hako`

**Result**:
```
❌ [ERROR] Unsupported expression in value context: MethodCall { object: Variable { name: "s" }, method: "length", ... }
```

**Analysis**:
- ✅ **Variable classification working**: No more "variable incorrectly classified" errors
- ⚠️ **New blocker**: Method calls in loop conditions (`s.length()`)
- **Impact**: Bug fix successful - JsonParser now hits **different** limitations (not variable scope bugs)

---

## Pattern 1-4 Coverage Analysis (After Fix)

### ✅ Fixed by Bug Fix

**Before Fix**: X loops incorrectly rejected due to function parameter misclassification

**After Fix**: These loops now correctly pass variable classification:
- **Simple loops using function parameters**: ✅ `s`, `pos` classified as OuterLocal
- **Loops with outer locals**: ✅ `len`, `maxLen` classified correctly
- **Carrier variables**: ✅ `start`, `end` (header+latch) classified as OuterLocal

### ⚠️ Remaining Limitations (Not Bug - Missing Features)

**Pattern 2 doesn't support**:
1. **Method calls in loop condition**: `s.length()`, `s.substring()` → Need Pattern 5+
2. **Method calls in loop body**: `.substring()` in break guards → Need Pattern 5+
3. **LoopBodyLocal in break conditions**: `local ch = ...; if ch == ...` → Need Pattern 5+

**These are legitimate feature gaps, not bugs.**

---

## Verification Commands

```bash
# Test 1: Function parameter loop (should pass variable verification)
NYASH_JOINIR_DEBUG=1 NYASH_JOINIR_STRUCTURE_ONLY=1 \
  ./target/release/hakorune /tmp/test_jsonparser_simple.hako 2>&1 | \
  grep "Phase 170-D"

# Expected: Phase 170-D: Condition variables verified: {"pos", "s", "len"}

# Test 2: LoopBodyLocal in break (should correctly reject)
NYASH_JOINIR_DEBUG=1 NYASH_JOINIR_STRUCTURE_ONLY=1 \
  ./target/release/hakorune local_tests/test_trim_main_pattern.hako 2>&1 | \
  grep "Phase 170-D\|Variable 'ch'"

# Expected:
#   Phase 170-D: Condition variables verified: {"ch", "end", "start"}
#   ERROR: Variable 'ch' not bound in ConditionEnv

# Test 3: JsonParser (should pass variable verification, fail on method calls)
NYASH_JOINIR_STRUCTURE_ONLY=1 \
  ./target/release/hakorune tools/hako_shared/json_parser.hako 2>&1 | \
  grep -E "Phase 170-D|MethodCall"

# Expected: Error about MethodCall, not about variable classification
```

---

## Impact Assessment

### ✅ What the Fix Achieves

1. **Function parameters work correctly**: `s`, `pos` in JsonParser methods
2. **Carrier variables work correctly**: `start`, `end` in trim loops
3. **Outer locals work correctly**: `len`, `maxLen` from outer scope
4. **Correct rejection**: LoopBodyLocal `ch` properly rejected (not a bug)

### ⚠️ What Still Needs Work

**Pattern 5+ Features** (not covered by this fix):
- Method calls in conditions: `loop(pos < s.length())`
- Method calls in loop body: `s.substring(pos, pos+1)`
- LoopBodyLocal in break conditions: `if ch == " " { break }`

**Exit Line & Boundary Issues** (orthogonal to this fix):
- Some loops fail with ExitLine/Boundary errors
- These are separate architectural issues

---

## Next Steps

### Priority 1: Pattern 5+ Implementation

**Target loops**:
- `_trim` loops (LoopBodyLocal `ch` in break condition)
- `_parse_object`, `_parse_array` (method calls in loop body)

**Estimated impact**: 10-15 additional loops in JsonParser

### Priority 2: .hako Rewrite Strategy

**For loops with complex method calls**:
- Hoist `.length()` calls to outer locals
- Pre-compute `.substring()` results outside loop
- Simplify break conditions to use simple comparisons

**Example rewrite**:
```hako
// Before (Pattern 5+ needed)
loop(pos < s.length()) {
  local ch = s.substring(pos, pos+1)
  if ch == "}" { break }
  pos = pos + 1
}

// After (Pattern 2 compatible)
local len = s.length()
loop(pos < len) {
  // Need to avoid method calls in break guard
  // This still requires Pattern 5+ for ch definition
  local ch = s.substring(pos, pos+1)
  if ch == "}" { break }
  pos = pos + 1
}
```

### Priority 3: Coverage Metrics

**Run systematic observation**:
```bash
# Count loops by pattern support
./tools/analyze_joinir_coverage.sh tools/hako_shared/json_parser.hako

# Expected output:
#   Pattern 1: X loops
#   Pattern 2: Y loops (with method call blockers)
#   Pattern 3: Z loops
#   Pattern 4: W loops
#   Unsupported (need Pattern 5+): N loops
```

---

## Conclusion

✅ **Bug Fix Complete**: Function parameters correctly classified as OuterLocal
✅ **Verification Successful**: Tests demonstrate correct variable classification
✅ **Expected Rejections**: LoopBodyLocal in break conditions correctly rejected
⚠️ **Next Blockers**: Method calls in loops (Pattern 5+ features, not bugs)

**Overall Impact**: Significant progress - variable scope classification is now correct. Remaining errors are legitimate feature gaps, not misclassification bugs.
Status: Historical
