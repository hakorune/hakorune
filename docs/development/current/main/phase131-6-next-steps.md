# Phase 131-6: Next Steps - PHI Bug Fix

## Update (Phase 131-10)

Phase 131-6 時点では PHI wiring を主因として疑っていたが、実際には multi-pass lowering の値伝播・ExternCall の引数解決/ABI ルーティング・PHI 型推論など複数の要因が重なっていた。

最終的に Case B は解決済み（LLVM AOT で `0,1,2` を出して終了）。到達点と修正の全体像は次を SSOT とする：
- `docs/development/current/main/phase131-3-llvm-lowering-inventory.md`

## Summary

**Problem**: LLVM backend generates infinite loop for `loop_min_while.hako`
**Root Cause**: PHI node incoming values not properly wired in `finalize_phis()`
**Impact**: P0 - Breaks basic loop functionality

## Recommended Fix Strategy

### Option A: Structural Fix (Recommended)

**Approach**: Fix the PHI wiring logic in `finalize_phis()` to properly connect incoming values.

**Location**: `/home/tomoaki/git/hakorune-selfhost/src/llvm_py/llvm_builder.py:601-735`

**Suspected Issue**:
```python
# Lines 679-681: Self-carry logic
if vs == int(dst_vid) and init_src_vid is not None:
    vs = int(init_src_vid)   # ← May incorrectly replace %12 with %2
```

**Fix Hypothesis**:
The self-carry logic is meant to handle PHI nodes that reference themselves, but it may be incorrectly triggering for normal loop-carried dependencies. We need to:

1. Add trace logging to see what values are being resolved
2. Check if `vs == int(dst_vid)` is incorrectly matching %12 (the updated value) as a self-reference
3. Verify that `_value_at_end_i64()` is correctly retrieving the value of %12 from bb7

**Debug Commands**:
```bash
# Enable verbose logging (if available)
export NYASH_CLI_VERBOSE=1
export NYASH_LLVM_DEBUG=1

# Generate LLVM IR to inspect
tools/build_llvm.sh apps/tests/loop_min_while.hako -o /tmp/loop_test

# Check generated LLVM IR
llvm-dis /tmp/loop_test.o  # If object contains IR
# Or add --emit-llvm-ir flag if available
```

**Steps**:
1. Add debug prints in `finalize_phis()` to log:
   - Which incoming values are being wired: `(block_id, dst_vid, [(pred_id, value_id)])`
   - What `nearest_pred_on_path()` returns for each incoming edge
   - What `_value_at_end_i64()` returns for each value

2. Compare debug output between working (VM) and broken (LLVM) paths

3. Fix the logic that's causing %12 to be ignored or replaced

4. Verify fix doesn't break Case A or B2

### Option B: Workaround (Not Recommended)

Disable loop optimization or use VM backend for loops. This doesn't solve the root cause.

## Acceptance Tests

### Must Pass

1. **Simple Add** (already passing):
```bash
./target/release/hakorune /tmp/simple_add.hako  # Should print 1
```

2. **Loop Min While** (currently failing):
```bash
tools/build_llvm.sh apps/tests/loop_min_while.hako -o /tmp/loop_test
timeout 2 /tmp/loop_test  # Should print 0\n1\n2 and exit
```

3. **Phase 87 LLVM Min** (regression check):
```bash
tools/build_llvm.sh apps/tests/phase87_llvm_exe_min.hako -o /tmp/phase87_test
/tmp/phase87_test
echo $?  # Should be 42
```

### Should Not Regress

- Case A: `phase87_llvm_exe_min.hako` ✅
- Case B2: Simple print without loop ✅
- VM backend: All existing VM tests ✅

## Implementation Checklist

- [ ] Add debug logging to `finalize_phis()`
- [ ] Identify which incoming value is being incorrectly wired
- [ ] Fix the wiring logic
- [ ] Test Case B (loop_min_while) - must output `0\n1\n2`
- [ ] Test Case A regression - must exit with 42
- [ ] Test Case B2 regression - must print 42
- [ ] Document the fix in this file and phase131-3-llvm-lowering-inventory.md
- [ ] Consider adding MIR→LLVM IR validation pass

## Timeline

- **Phase 131-6 Diagnosis**: 2025-12-14 ✅ Complete
- **Phase 131-7 Fix**: TBD
- **Phase 131-8 Verification**: TBD

## Related Documents

- [Phase 131-6 Diagnosis](phase131-6-ssa-dominance-diagnosis.md) - Full diagnostic report
- [Phase 131-3 Inventory](phase131-3-llvm-lowering-inventory.md) - Test case matrix
- [LLVM Builder Code](../../src/llvm_py/llvm_builder.py) - Implementation

## Notes for Future Self

**Why MIR is correct but LLVM is wrong**:
- MIR SSA form verified ✅
- VM execution verified ✅
- LLVM emission succeeds ✅
- LLVM linking succeeds ✅
- **LLVM runtime fails** ❌ → Bug is in IR generation, not MIR

**Key Insight**:
The bug is NOT in the MIR builder or JoinIR merger. The bug is specifically in how the Python LLVM builder (`llvm_builder.py`) translates MIR PHI nodes into LLVM IR PHI nodes. This is a **translation bug**, not a **semantic bug**.

**Architecture Note**:
This confirms the value of the 2-pillar strategy (VM + LLVM). The VM serves as a reference implementation to verify MIR correctness before blaming the frontend.
