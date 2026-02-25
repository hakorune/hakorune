# Phase 131-12-P1: vmap Object Identity Trace - Summary

## Status: ✅ Root Cause Identified

**Date**: 2025-12-14  
**Investigation**: vmap_cur object identity issue causing wrong values in LLVM backend  
**Result**: **Hypothesis C confirmed** - Object identity problem in Pass A→C temporal coupling

## Critical Discovery

### The Smoking Gun

```python
# Pass A (block_lower.py line 168)
builder._current_vmap = vmap_cur  # ← Create per-block vmap

# Pass A (block_lower.py line 240)  
builder._deferred_terminators[bid] = (bb, term_ops)  # ← Defer terminators

# Pass A (block_lower.py line 265)
delattr(builder, '_current_vmap')  # ← DELETE vmap_cur ❌

# Pass C (lower_terminators, line 282)
# When lowering deferred terminators:
vmap_ctx = getattr(owner, '_current_vmap', owner.vmap)  # ← Falls back to global vmap! ❌
```

**Problem**: Pass A deletes `_current_vmap` before Pass C runs, causing terminators to use the wrong vmap object.

### Trace Evidence

```
bb1 block creation:        vmap_ctx id=140506427346368  ← Creation
bb1 const instruction:     vmap_ctx id=140506427346368  ← Same (good)
bb1 ret terminator:        vmap_ctx id=140506427248448  ← DIFFERENT (bad!)
                                        ^^^^^^^^^^^^^^
                                        This is owner.vmap, not vmap_cur!
```

**Impact**: Values written to `vmap_cur` in Pass A are invisible to terminators in Pass C.

## The Bug Flow

1. **Pass A**: Create `vmap_cur` for block
2. **Pass A**: Lower body instructions → writes go to `vmap_cur`
3. **Pass A**: Store terminators for later
4. **Pass A**: **Delete `_current_vmap`** ← THE BUG
5. **Pass C**: Lower terminators → fallback to `owner.vmap` (different object!)
6. **Result**: Terminators read from wrong vmap, missing all Pass A writes

## Proof: Per-Block vs Global vmap

### Expected (Per-Block Context)
```python
vmap_cur = {...}  # Block-local SSA values
builder._current_vmap = vmap_cur
# All instructions in this block use the SAME object
```

### Actual (Broken State)
```python
vmap_cur = {...}  # Block-local SSA values
builder._current_vmap = vmap_cur  # Pass A body instructions use this

# Pass A ends
delattr(builder, '_current_vmap')  # DELETED!

# Pass C starts
vmap_ctx = owner.vmap  # Falls back to GLOBAL vmap (different object!)
# Terminators see different data than body instructions! ❌
```

## Fix Options (Recommended: Option 3)

### Option 1: Don't Delete Until Pass C Completes
- Quick fix but creates temporal coupling
- Harder to reason about state lifetime

### Option 2: Read from block_end_values SSOT
- Good: Uses snapshot as source of truth
- Issue: Requires restoring to builder state

### Option 3: Store vmap_cur in Deferred Data (RECOMMENDED)
```python
# Pass A (line 240)
builder._deferred_terminators[bid] = (bb, term_ops, vmap_cur)  # ← Add vmap_cur

# Pass C (line 282)
for bid, (bb, term_ops, vmap_ctx) in deferred.items():
    builder._current_vmap = vmap_ctx  # ← Restore exact context
    # Lower terminators with correct vmap
```

**Why Option 3?**
- Explicit ownership: vmap_cur is passed through deferred tuple
- No temporal coupling: Pass C gets exact context from Pass A
- SSOT principle: One source of vmap per block
- Fail-Fast: Type error if tuple structure changes

## Architecture Impact

### Current Problem
- **Temporal Coupling**: Pass C depends on Pass A's ephemeral state
- **Silent Fallback**: Wrong vmap used without error
- **Hidden Sharing**: Global vmap shared across blocks

### Fixed Architecture (Box-First)
```
Pass A: Create vmap_cur (per-block "box")
        ↓
        Store in deferred tuple (explicit ownership transfer)
        ↓
Pass C: Restore vmap_cur from tuple (unpack "box")
        ↓
        Use exact same object (SSOT)
```

**Aligns with CLAUDE.md principles**:
- ✅ Box-First: vmap_cur is a "box" passed between passes
- ✅ SSOT: One vmap per block, explicit transfer
- ✅ Fail-Fast: Type error if deferred tuple changes

## Test Commands

### Verify Fix
```bash
# Before fix: Shows different IDs for terminator
NYASH_LLVM_VMAP_TRACE=1 NYASH_LLVM_USE_HARNESS=1 \
  ./target/release/hakorune --backend llvm apps/tests/llvm_stage3_loop_only.hako 2>&1 | \
  grep "\[vmap/id\]"

# After fix: Should show SAME ID throughout block
```

### Full Verification
```bash
# Check full execution
NYASH_LLVM_VMAP_TRACE=1 NYASH_LLVM_USE_HARNESS=1 \
  ./target/release/hakorune --backend llvm apps/tests/llvm_stage3_loop_only.hako

# Expected: Result: 3 (matching VM)
```

## Files Modified

### Trace Implementation (Phase 131-12-P1)
- `/home/tomoaki/git/hakorune-selfhost/src/llvm_py/builders/block_lower.py`
- `/home/tomoaki/git/hakorune-selfhost/src/llvm_py/builders/instruction_lower.py`
- `/home/tomoaki/git/hakorune-selfhost/src/llvm_py/instructions/const.py`
- `/home/tomoaki/git/hakorune-selfhost/src/llvm_py/instructions/copy.py`
- `/home/tomoaki/git/hakorune-selfhost/src/llvm_py/instructions/binop.py`
- `/home/tomoaki/git/hakorune-selfhost/src/llvm_py/utils/values.py`

### Fix Target (Next Phase)
- `/home/tomoaki/git/hakorune-selfhost/src/llvm_py/builders/block_lower.py` (Option 3)

## Related Documents

- Investigation: `/docs/development/current/main/investigations/phase131-12-case-c-llvm-wrong-result.md`
- Detailed Analysis: `/docs/development/current/main/investigations/phase131-12-p1-vmap-identity-analysis.md`
- LLVM Inventory: `/docs/development/current/main/phase131-3-llvm-lowering-inventory.md`
- Environment Variables: `/docs/reference/environment-variables.md`

## Next Steps

1. **Implement Option 3 fix** (store vmap_cur in deferred tuple)
2. **Add Fail-Fast check** in instruction_lower.py (detect missing _current_vmap)
3. **Verify with trace** (consistent IDs across Pass A→C)
4. **Run full test suite** (ensure VM/LLVM parity)
5. **Document pattern** (for future multi-pass architectures)

## Lessons Learned

### Box-First Principle Application
- Mutable builder state (`_current_vmap`) should be **explicitly passed** through phases
- Don't rely on `getattr` fallbacks - they hide bugs
- Per-block context is a "box" - treat it as first-class data

### Fail-Fast Opportunity
```python
# BEFORE (silent fallback)
vmap_ctx = getattr(owner, '_current_vmap', owner.vmap)  # Wrong vmap silently used

# AFTER (fail-fast)
vmap_ctx = getattr(owner, '_current_vmap', None)
if vmap_ctx is None:
    raise RuntimeError("Pass A/C timing bug: _current_vmap not set")
```

### SSOT Enforcement
- `block_end_values` is snapshot SSOT
- `_current_vmap` is working buffer
- Pass C should **restore** working buffer from SSOT or deferred data

---

**Investigation Complete**: Root cause identified with high confidence. Ready for fix implementation.
