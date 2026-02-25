# Phase 132: LLVM Exit PHI=0 Bug Investigation & Fix

**Date**: 2025-12-15
**Status**: ✅ Fixed
**Impact**: Critical - Exit PHIs from loops were returning 0 instead of correct values

## Problem Statement

LLVM backend was returning 0 for exit PHI values in simple while loops, while VM backend correctly returned the final loop variable value.

### Test Case
```nyash
static box Main {
  main() {
    local i = 0
    loop(i < 3) { i = i + 1 }
    return i  // Should return 3, was returning 0 in LLVM
  }
}
```

**Expected**: Result: 3 (matches VM)
**Actual (before fix)**: Result: 0
**MIR**: Correct (`bb3: %1 = phi [%3, bb6]; ret %1`)
**LLVM IR (before fix)**: `ret i64 0` (wrong!)
**LLVM IR (after fix)**: `ret i64 %"phi_1"` (correct!)

## Root Cause Analysis

この不具合は「2層」にまたがっていました:

1. JoinIR/Boundary 層で exit 値が境界を通っていない（VM でも 0 になり得る）
2. LLVM Python 層で PHI の SSOT が壊れていて exit PHI が 0 になる（VM は正常でも LLVM が壊れる）

このページは主に (2) の LLVM Python 層の根治を記録します。  
(1) の修正は Phase 132 の一部として別途コード側で入っています（修正ファイル一覧を参照）。

### Investigation Path

1. **PHI filtering issue in vmap_cur initialization**
   - ✅ Confirmed: Filter relied on `phi.basic_block` attribute
   - Issue: llvmlite sets `phi.basic_block = None` until IR finalization
   - Filter at block_lower.py:323-365 was silently dropping ALL PHIs

2. **builder.vmap overwrite issue**
   - ✅ Confirmed: The real root cause!

### The Actual Bug

**Two separate issues** combined to cause the bug:

#### Issue 1: Unreliable PHI.basic_block Attribute
- llvmlite's PHI instructions have `basic_block = None` when created
- Filter logic at block_lower.py:326-340 relied on `phi.basic_block.name` comparison
- Since `basic_block` was always None, filter excluded ALL PHIs from vmap_cur
- **Fix**: Use `predeclared_ret_phis` dict instead of `basic_block` attribute

#### Issue 2: builder.vmap PHI Overwrites (Critical!)
At block_lower.py:437-448, Pass A syncs created values to builder.vmap:

```python
# Phase 131-7: Sync ALL created values to global vmap
for vid in created_ids:
    val = vmap_cur.get(vid)
    if val is not None:
        builder.vmap[vid] = val  # ❌ Unconditional overwrite!
```

**The Fatal Sequence**:
1. Pass A setup: Creates PHI v1 for bb3, stores in `builder.vmap[1]`
2. Pass A processes bb0:
   - vmap_cur filters out v1 PHI (not from bb0)
   - const v1 instruction writes to vmap_cur[1]
   - **Line 444: Syncs vmap_cur[1] → builder.vmap[1], overwriting PHI!**
3. Pass A processes bb3:
   - vmap_cur initialized from builder.vmap
   - builder.vmap[1] is now the const (not PHI!)
   - return v1 uses const 0 instead of PHI

## The Fix

### Fix 1: PHI Filtering (block_lower.py:320-347)

**Before** (unreliable basic_block check):
```python
if hasattr(_val, 'add_incoming'):
    bb_of = getattr(getattr(_val, 'basic_block', None), 'name', None)
    bb_name = getattr(bb, 'name', None)
    keep = (bb_of == bb_name)  # ❌ Always False! bb_of is None
```

**After** (use predeclared_ret_phis dict):
```python
if hasattr(_val, 'add_incoming'):  # Is it a PHI?
    phi_key = (int(bid), int(_vid))
    if phi_key in predecl_phis:
        keep = True  # ✅ Reliable tracking
    else:
        keep = False  # Avoid namespace collision
```

### Fix 2: Protect builder.vmap PHIs (block_lower.py:437-455)

**Before** (unconditional overwrite):
```python
for vid in created_ids:
    val = vmap_cur.get(vid)
    if val is not None:
        builder.vmap[vid] = val  # ❌ Overwrites PHIs!
```

**After** (PHI protection):
```python
for vid in created_ids:
    val = vmap_cur.get(vid)
    if val is not None:
        existing = builder.vmap.get(vid)
        # Don't overwrite existing PHIs - SSOT principle
        if existing is not None and hasattr(existing, 'add_incoming'):
            continue  # ✅ Skip sync, preserve PHI
        builder.vmap[vid] = val
```

## Verification

### Test Results
```bash
# ✅ LLVM matches VM
NYASH_LLVM_USE_HARNESS=1 NYASH_LLVM_STRICT=1 ./target/release/hakorune --backend llvm /tmp/p1_return_i.hako
# Output: Result: 3

# ✅ VM baseline
./target/release/hakorune --backend vm /tmp/p1_return_i.hako
# Output: RC: 3
```

### Generated LLVM IR Comparison

**Before** (wrong):
```llvm
bb3:
  %"phi_1" = phi  i64 [%"phi_3", %"bb6"]
  ret i64 0  ; ❌ Hardcoded 0!
```

**After** (correct):
```llvm
bb3:
  %"phi_1" = phi  i64 [%"phi_3", %"bb6"]
  ret i64 %"phi_1"  ; ✅ Uses PHI value!
```

## Design Lessons

### The SSOT Principle

**builder.vmap is the Single Source of Truth for PHI nodes**:
- PHIs are created once in setup_phi_placeholders
- PHIs must NEVER be overwritten by later instructions
- vmap_cur is per-block and must filter PHIs correctly

### PHI Ownership Tracking

**llvmlite limitation**: PHI.basic_block is None until finalization
**Solution**: Explicit tracking via `predeclared_ret_phis: Dict[(block_id, value_id), PHI]`

### Fail-Fast vs Silent Failures

The original filter silently dropped PHIs via broad exception handling:
```python
except Exception:
    keep = False  # ❌ Silent failure!
```

**Better approach**: Explicit checks with trace logging for debugging.

## Related Issues

- Phase 131: Block_end_values SSOT system
- Phase 131-12: VMap snapshot investigation
- Phase 131-14-B: Jump-only block resolution

## Files Modified

### JoinIR/Boundary 層（exit 値の SSOT を境界で明示）

- `src/mir/join_ir/lowering/simple_while_minimal.rs`（`Jump(k_exit, [i_param])`）
- `src/mir/builder/control_flow/joinir/patterns/pattern1_minimal.rs`（`LoopExitBinding` を作って境界へ設定）

### LLVM Python 層（PHI SSOT の維持）

- `src/llvm_py/builders/block_lower.py`
  - PHI filtering を `predeclared_ret_phis` ベースへ変更（`phi.basic_block` 依存を排除）
  - `builder.vmap` へ sync する際、既存 PHI を上書きしない（PHI を SSOT として保護）

## Debug Environment Variables

```bash
NYASH_LLVM_STRICT=1           # Fail-fast on errors
NYASH_LLVM_TRACE_PHI=1        # PHI wiring traces
NYASH_LLVM_TRACE_VMAP=1       # VMap operation traces
NYASH_LLVM_DUMP_IR=/tmp/x.ll  # Dump generated IR
```

## Acceptance Criteria

✅ `/tmp/p1_return_i.hako` returns 3 in LLVM (was 0)
✅ STRICT mode enabled, no fallback to 0
✅ VM and LLVM results match
✅ No regression on Phase 131 test cases
✅ Generated LLVM IR uses `ret i64 %phi_1` not `ret i64 0`

## Next Steps

- [ ] Add regression test for exit PHI patterns
- [ ] Document PHI ownership model in the LLVM harness docs (SSOT: `phase131-3-llvm-lowering-inventory.md`)
