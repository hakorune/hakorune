# Phase 131-6: MIR SSA Dominance Diagnosis

## Update (Phase 131-10)

Case B (`apps/tests/loop_min_while.hako`) は **LLVM AOT でも `0,1,2` を出して終了**するところまで復旧済み。

この文書は Phase 131-6 時点の「切り分けログ（歴史）」として残し、最終的な到達点と修正の全体像は次を SSOT とする：
- `docs/development/current/main/phase131-3-llvm-lowering-inventory.md`

## Executive Summary

**Status (Phase 131-6 時点)**: ❌ LLVM Backend Bug Confirmed
**Severity**: P0 - Breaks basic loop functionality
**Root Cause (Phase 131-6 時点の仮説)**: PHI node incoming values not properly wired in LLVM IR generation

## Evidence Chain

### 1. Test Case SSOT

**File**: `/tmp/simple_add.hako`
```nyash
static box Main {
    main() {
        local i
        i = 0
        i = i + 1
        print(i)
        return 0
    }
}
```

**Expected**: Prints `1`
**Actual (VM)**: ✅ Prints `1`
**Actual (LLVM)**: ✅ Prints `1`

**File**: `apps/tests/loop_min_while.hako`
```nyash
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

**Expected**: Prints `0\n1\n2`
**Actual (VM)**: ✅ Prints `0\n1\n2`
**Actual (LLVM)**: ❌ Prints `0` infinitely (infinite loop)

### 2. MIR Verification

**Command**: `./target/release/hakorune --dump-mir apps/tests/loop_min_while.hako`

**MIR Output** (relevant blocks):
```mir
define i64 @main() {
bb0:
    %1 = const 0
    %2 = copy %1
    br label bb4

bb3:
    %17 = const 0
    ret %17

bb4:
    %3 = phi [%2, bb0], [%12, bb7]   // ← PHI node for loop variable
    br label bb5

bb5:
    %8 = const 3
    %9 = icmp Lt %3, %8
    %10 = Not %9
    br %10, label bb6, label bb7

bb6:
    br label bb3

bb7:
    extern_call env.console.log(%3)   // ← Prints %3 (should increment each iteration)
    %11 = const 1
    %12 = %3 Add %11                  // ← %12 = %3 + 1 (updated value)
    br label bb4                      // ← Jumps back with %12
}
```

**Analysis**:
- ✅ SSA form is correct
- ✅ All values defined before use within each block
- ✅ PHI node properly declares incoming values: `[%2, bb0]` (initial) and `[%12, bb7]` (loop update)
- ✅ No use-before-def violations

### 3. VM Execution Verification

**Command**: `timeout 2 ./target/release/hakorune apps/tests/loop_min_while.hako`

**Output**:
```
0
1
2
RC: 0
```

**Conclusion**: ✅ MIR is correct, VM interprets it correctly

### 4. LLVM Execution Failure

**Build Command**: `bash tools/build_llvm.sh apps/tests/loop_min_while.hako -o /tmp/loop_test`

**Build Result**: ✅ Success (no errors)

**Run Command**: `/tmp/loop_test`

**Output** (truncated):
```
0
0
0
0
... (repeats infinitely)
```

**Conclusion**: ❌ LLVM backend bug - PHI node not working

## Root Cause Analysis

### Affected Component
**File**: `/home/tomoaki/git/hakorune-selfhost/src/llvm_py/llvm_builder.py`
**Function**: `finalize_phis()` (lines 601-735+)

### Bug Mechanism

The PHI node `%3 = phi [%2, bb0], [%12, bb7]` should:
1. On first iteration: Use %2 (value 0 from bb0)
2. On subsequent iterations: Use %12 (updated value from bb7)

**What's happening**:
- %3 always resolves to 0 (initial value from %2)
- The incoming value from bb7 (%12) is not being properly connected
- Loop variable never increments → infinite loop

### Suspected Code Location

In `finalize_phis()` around lines 670-688:
```python
chosen: Dict[int, ir.Value] = {}
for (b_decl, v_src) in incoming:
    try:
        bd = int(b_decl); vs = int(v_src)
    except Exception:
        continue
    pred_match = nearest_pred_on_path(bd)
    if pred_match is None:
        continue
    # If self-carry is specified (vs == dst_vid), map to init_src_vid when available
    if vs == int(dst_vid) and init_src_vid is not None:
        vs = int(init_src_vid)   # ← SUSPICIOUS: May cause %12 to be ignored
    try:
        val = self.resolver._value_at_end_i64(vs, pred_match, self.preds, self.block_end_values, self.vmap, self.bb_map)
    except Exception:
        val = None
    if val is None:
        val = ir.Constant(self.i64, 0)   # ← Falls back to 0
    chosen[pred_match] = val
```

### Hypothesis

The self-carry logic (lines 679-681) or value resolution (line 683) may be incorrectly mapping or failing to retrieve %12 from bb7, causing the PHI to always use the fallback value of 0.

## Next Steps

### Immediate Action Required

1. **Add Trace Logging**:
   - Enable `NYASH_CLI_VERBOSE=1` or similar PHI-specific tracing
   - Log what values are being wired to each PHI incoming edge

2. **Minimal Fix Verification**:
   - Verify `_value_at_end_i64(12, 7, ...)` returns the correct LLVM value
   - Check if `nearest_pred_on_path()` correctly identifies bb7 as predecessor of bb4

3. **Test Matrix**:
   - Simple Add: ✅ (already passing)
   - Loop Min While: ❌ (currently failing)
   - Case A/B2 from previous phases: (regression check needed)

### Long-term Solution

Implement structural dominance verification:
- MIR verifier pass to check SSA properties
- LLVM IR verification before object emission
- Automated test for PHI node correctness

## Acceptance Criteria

### Must Pass
1. ✅ `tools/build_llvm.sh apps/tests/loop_min_while.hako -o /tmp/loop_test && /tmp/loop_test` outputs `0\n1\n2` and exits
2. ✅ Simple Add still works: `/tmp/simple_add` outputs `1`
3. ✅ No regression in existing LLVM smoke tests

### Documentation
1. ✅ This diagnosis added to `docs/development/current/main/`
2. ✅ Fix explanation added to phase131-3-llvm-lowering-inventory.md
3. ✅ Test case added to prevent regression

## Files Modified (To Be Updated)

- `src/llvm_py/llvm_builder.py` - PHI wiring logic
- `docs/development/current/main/phase131-3-llvm-lowering-inventory.md` - Add Phase 131-6 section
- (potential) Test case addition

## Timeline

- **Diagnosis**: 2025-12-14 (Complete)
- **Fix**: TBD
- **Verification**: TBD
