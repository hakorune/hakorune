# Phase 131-12-P1: vmap Object Identity Trace Analysis

## Executive Summary

**Status**: ⚠️ Hypothesis C (Object Identity Problem) - **PARTIALLY CONFIRMED**

### Key Findings

1. **vmap_ctx identity changes between blocks**:
   - bb1: `vmap_ctx id=140506427346368` (creation)
   - bb1 ret: `vmap_ctx id=140506427248448` (DIFFERENT!)
   - bb2: `vmap_ctx id=140506427351808` (new object)

2. **Trace stopped early** - execution crashed before reaching critical bb3/exit blocks

3. **No v17 writes detected** - the problematic value was never written

## Detailed Trace Analysis

### Block 1 Trace Sequence

```
[vmap/id] bb1 vmap_cur id=140506427346368 keys=[0]          # ← Block creation
[vmap/id] instruction op=const vmap_ctx id=140506427346368  # ← Same object ✅
[vmap/id] const dst=1 vmap id=140506427346368 before_write  # ← Same object ✅
[vmap/write] dst=1 written, vmap.keys()=[0, 1]              # ← Write successful ✅
[vmap/id] instruction op=ret vmap_ctx id=140506427248448    # ← DIFFERENT OBJECT! ❌
```

**Problem Found**: The `vmap_ctx` object changed identity **within the same block**!
- Creation: `140506427346368`
- Terminator: `140506427248448`

### Block 2 Trace Sequence

```
[vmap/id] bb2 vmap_cur id=140506427351808 keys=[]           # ← New block (expected)
[vmap/id] instruction op=const vmap_ctx id=140506427351808  # ← Consistent ✅
[vmap/write] dst=1 written, vmap.keys()=[1]                 # ← Write successful ✅
[vmap/id] instruction op=const vmap_ctx id=140506427351808  # ← Still consistent ✅
[vmap/write] dst=2 written, vmap.keys()=[1, 2]              # ← Write successful ✅
[vmap/id] instruction op=binop vmap_ctx id=140506427351808  # ← Still consistent ✅
# CRASH - execution stopped here
```

Block 2 shows **good consistency** - same object throughout.

## Root Cause Hypothesis

### Hypothesis A (Timing): ❌ REJECTED
- Writes are successful and properly sequenced
- No evidence of post-instruction sync reading from wrong location

### Hypothesis B (PHI Collision): ⚠️ POSSIBLE
- Cannot verify - trace stopped before PHI blocks
- Need to check if existing PHIs block safe_vmap_write

### Hypothesis C (Object Identity): ✅ **CONFIRMED**
- **Critical evidence**: `vmap_ctx` changed identity during bb1 terminator instruction
- This suggests `getattr(owner, '_current_vmap', owner.vmap)` is returning a **different object**

## Source Code Analysis

### Terminator Lowering Path

The identity change happens during `ret` instruction. Checking the code:

**File**: `src/llvm_py/builders/block_lower.py`

Line 236-240:
```python
# Phase 131-4 Pass A: DEFER terminators until after PHI finalization
# Store terminators for Pass C (will be lowered in lower_terminators)
if not hasattr(builder, '_deferred_terminators'):
    builder._deferred_terminators = {}
if term_ops:
    builder._deferred_terminators[bid] = (bb, term_ops)
```

**Smoking Gun**: Terminators are deferred! When `ret` is lowered in Pass C (line 270+), the `_current_vmap` may have been **deleted**:

Line 263-267:
```python
builder.block_end_values[bid] = snap
try:
    delattr(builder, '_current_vmap')  # ← DELETED BEFORE PASS C!
except Exception:
    pass
```

**Problem**: 
1. Pass A creates `_current_vmap` for block (line 168)
2. Pass A defers terminators (line 240)
3. Pass A **deletes** `_current_vmap` (line 265)
4. Pass C lowers terminators → `getattr(owner, '_current_vmap', owner.vmap)` falls back to `owner.vmap`
5. **Result**: Different object! ❌

## Recommended Fix (3 Options)

### Option 1: Preserve vmap_cur for Pass C (Quick Fix)

```python
# Line 263 in block_lower.py
builder.block_end_values[bid] = snap
# DON'T delete _current_vmap yet! Pass C needs it!
# try:
#     delattr(builder, '_current_vmap')
# except Exception:
#     pass
```

Then delete it in `lower_terminators()` after all terminators are done.

### Option 2: Use block_end_values in Pass C (SSOT)

```python
# In lower_terminators() line 282
for bid, (bb, term_ops) in deferred.items():
    # Use snapshot from Pass A as SSOT
    vmap_ctx = builder.block_end_values.get(bid, builder.vmap)
    builder._current_vmap = vmap_ctx  # Restore for consistency
    # ... lower terminators ...
```

### Option 3: Store vmap_cur in deferred_terminators (Explicit)

```python
# Line 240
if term_ops:
    builder._deferred_terminators[bid] = (bb, term_ops, vmap_cur)  # ← Add vmap_cur

# Line 282 in lower_terminators
for bid, (bb, term_ops, vmap_ctx) in deferred.items():  # ← Unpack vmap_ctx
    builder._current_vmap = vmap_ctx  # Restore
    # ... lower terminators ...
```

## Next Steps (Recommended Order)

1. **Verify hypothesis** with simpler test case:
   ```bash
   # Create minimal test without loop complexity
   echo 'static box Main { main() { return 42 } }' > /tmp/minimal.hako
   NYASH_LLVM_VMAP_TRACE=1 NYASH_LLVM_USE_HARNESS=1 \
     ./target/release/hakorune --backend llvm /tmp/minimal.hako 2>&1 | grep vmap/id
   ```

2. **Apply Option 1** (quickest to verify):
   - Comment out `delattr(builder, '_current_vmap')` in Pass A
   - Add it to end of `lower_terminators()` in Pass C

3. **Re-run full test**:
   ```bash
   NYASH_LLVM_VMAP_TRACE=1 NYASH_LLVM_USE_HARNESS=1 \
     ./target/release/hakorune --backend llvm apps/tests/llvm_stage3_loop_only.hako
   ```

4. **Check if bb3/exit blocks now show consistent vmap_ctx IDs**

## Architecture Feedback (Box-First Principle)

**Problem**: Multi-pass architecture (A → B → C) with mutable state (`_current_vmap`) creates temporal coupling.

**Recommendation**: Apply SSOT principle from CLAUDE.md:
- `block_end_values` should be the **single source of truth** for post-block state
- Pass C should **read** from SSOT, not rely on ephemeral `_current_vmap`
- This matches "箱理論" - `block_end_values` is the persistent "box", `_current_vmap` is a working buffer

**Fail-Fast Opportunity**:
```python
# In lower_instruction() line 33
vmap_ctx = getattr(owner, '_current_vmap', None)
if vmap_ctx is None:
    # Fail-Fast instead of silent fallback!
    raise RuntimeError(
        f"[LLVM_PY] _current_vmap not set for instruction {op}. "
        f"This indicates Pass A/C timing issue. Check block_lower.py multi-pass logic."
    )
```

## Appendix: Environment Variables Used

```bash
NYASH_LLVM_VMAP_TRACE=1    # Our new trace flag
NYASH_LLVM_USE_HARNESS=1   # Enable llvmlite harness
NYASH_LLVM_DUMP_IR=<path>  # Save LLVM IR (for later analysis)
```
