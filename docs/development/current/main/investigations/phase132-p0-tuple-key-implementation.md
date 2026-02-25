# Phase 132-P0: block_end_values Tuple-Key Implementation

**Date**: 2025-12-15
**Status**: ✅ Implementation Complete
**Related**: phase132-p0-case-c-root-cause.md

## Summary

Implemented tuple-key `(func_name, block_id)` for `block_end_values` to prevent cross-function block ID collisions in LLVM backend.

## Root Cause (from investigation)

```python
# ❌ Before: Block ID collision across functions
block_end_values: Dict[int, Dict[int, ir.Value]]
# main:bb0 and condition_fn:bb0 collide!

# ✅ After: Function-scoped keys
block_end_values: Dict[Tuple[str, int], Dict[int, ir.Value]]
# ("main", 0) and ("condition_fn", 0) are distinct
```

## Implementation (5 files modified)

### 1. `src/llvm_py/llvm_builder.py` (Type annotation)
```python
# Line 116: Updated type annotation
self.block_end_values: Dict[Tuple[str, int], Dict[int, ir.Value]] = {}
```

### 2. `src/llvm_py/builders/function_lower.py` (Pass func_name)
```python
# Line 303: Pass func_name to lower_blocks
_lower_blocks(builder, func, block_by_id, order, loop_plan, func_name=name)

# Line 308: Pass func_name to resolve_jump_only_snapshots
_resolve_jump_only_snapshots(builder, block_by_id, func_name=name)

# Line 333: Pass func_name to finalize_phis
_finalize_phis(builder, func_name=name)
```

### 3. `src/llvm_py/builders/block_lower.py` (Tuple-key usage)
```python
# Line 184: Accept func_name parameter
def lower_blocks(..., func_name: str = "unknown"):

# Line 61: Accept func_name parameter
def resolve_jump_only_snapshots(..., func_name: str = "unknown"):

# Line 118-119: Use tuple-key for read
if (func_name, bid) in builder.block_end_values:
    snapshot = builder.block_end_values[(func_name, bid)]

# Line 177: Use tuple-key for write (Pass B)
builder.block_end_values[(func_name, bid)] = snapshot

# Line 529: Use tuple-key for write (Pass A)
builder.block_end_values[(func_name, bid)] = snap
```

### 4. `src/llvm_py/resolver.py` (resolve_incoming)
```python
# Line 127: Accept func_name parameter
def resolve_incoming(self, pred_block_id: int, value_id: int, func_name: str = "unknown"):

# Line 143: Use tuple-key for snapshot lookup
snapshot = self.block_end_values.get((func_name, pred_block_id), {})
```

### 5. `src/llvm_py/phi_wiring/wiring.py` (PHI wiring)
```python
# Line 242: Accept func_name parameter in finalize_phis
def finalize_phis(builder, func_name: str = "unknown"):

# Line 140: Accept func_name parameter in wire_incomings
def wire_incomings(builder, ..., func_name: str = "unknown"):

# Line 155: Use tuple-key for PHI lookup
cur = (snap.get((func_name, int(block_id)), {}) or {}).get(int(dst_vid))

# Line 219: Pass func_name to resolve_incoming
val = builder.resolver.resolve_incoming(pred_match, vs, func_name=func_name)

# Line 256: Pass func_name to wire_incomings
wired = wire_incomings(builder, ..., func_name=func_name)
```

## Testing Results

### ✅ Pattern 1 (Phase 132 regression check)
```bash
# Test file: /tmp/p1_return_i.hako
static box Main {
    main() {
        local i = 0
        loop(i < 3) { i = i + 1 }
        return i
    }
}

# VM Result: RC: 3 ✅
# LLVM Result: Result: 3 ✅ (without STRICT mode)
# LLVM STRICT: ValueId collision error (separate issue)
```

**Status**: ✅ No regression - Pattern 1 still works correctly

### ⚠️ Case C (Pattern 5) - Dominance Error Persists
```bash
# Test file: apps/tests/llvm_stage3_loop_only.hako
# VM Result: Result: 3 ✅
# LLVM Result: PHI dominance error ❌

Error: Instruction does not dominate all uses!
  %phi_1 = phi i64 [ %add_8, %bb6 ]
  %phi_3 = phi i64 [ %phi_1, %bb0 ], [ %add_8, %bb7 ]
```

**Analysis**: The dominance error is NOT caused by block_end_values collision.
It's a different issue related to PHI node placement and control flow structure.

### Verification Logs
```bash
# Pass B resolution working correctly:
[vmap/resolve/passB] Resolving 2 jump-only blocks: [6, 7]
[vmap/resolve/passB] bb6 is jump-only, resolving from pred bb5
[vmap/resolve/passB] bb5 is normal block with snapshot (5 values)
[vmap/resolve/passB] bb6 resolved from bb5: 5 values
[vmap/resolve/passB] ✅ bb6 final snapshot: 5 values, keys=[3, 7, 8, 9, 10]
```

## Design Principles Applied

### Box-First (SSOT)
- Each function is an independent Box
- `block_end_values` keys are scoped to function
- `(func_name, block_id)` is the SSOT identifier

### Fail-Fast
- STRICT mode detects collisions immediately
- Updated error messages include `func_name` context

## Conclusion

### ✅ Implementation Complete
- All 5 files updated with tuple-key logic
- Type annotations consistent
- Function signatures updated
- All call sites pass `func_name`

### ✅ Regression Prevention
- Pattern 1 still works correctly
- VM/LLVM parity maintained for simple cases

### ⚠️ Case C Needs Further Investigation
The dominance error in Case C is **not fixed** by this change.
**Root cause**: Different issue - likely related to:
- PHI node placement in complex control flow (break/continue)
- Block ordering or dominator tree structure
- Need separate investigation (Phase 132-P1?)

## Next Steps

1. **Accept tuple-key fix**: Merge this implementation (prevents future collisions)
2. **Investigate Case C separately**: Create Phase 132-P1 for dominance error
3. **Add tuple-key validation**: Optional STRICT check that all lookups use tuple-key

## Files Modified

1. `/home/tomoaki/git/hakorune-selfhost/src/llvm_py/llvm_builder.py`
2. `/home/tomoaki/git/hakorune-selfhost/src/llvm_py/builders/function_lower.py`
3. `/home/tomoaki/git/hakorune-selfhost/src/llvm_py/builders/block_lower.py`
4. `/home/tomoaki/git/hakorune-selfhost/src/llvm_py/resolver.py`
5. `/home/tomoaki/git/hakorune-selfhost/src/llvm_py/phi_wiring/wiring.py`

## References

- Phase 132 Inventory: `docs/development/current/main/phase131-3-llvm-lowering-inventory.md`
- Root Cause Analysis: `docs/development/current/main/investigations/phase132-p0-case-c-root-cause.md`
