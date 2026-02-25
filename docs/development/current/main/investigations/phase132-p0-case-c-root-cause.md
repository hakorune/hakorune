# Phase 132-P0 Case C Root Cause Investigation

**Date**: 2025-12-15
**Status**: Root cause identified, fix designed
**Priority**: P0 (blocks LLVM EXE execution)

## Problem Statement

Case C (Pattern 5 + print concat) LLVM EXE fails with domination error:

```
RuntimeError: Instruction does not dominate all uses!
  %phi_1 = phi i64 [ %add_8, %bb6 ]
  %phi_3 = phi i64 [ %phi_1, %bb0 ], [ %add_8, %bb7 ]
```

`%phi_3` in bb4 uses `%phi_1` from bb0 edge, but `%phi_1` is defined in bb3 which doesn't dominate bb0.

## Investigation Process

### Step 0: IR Dump Confirmation

Generated IR shows:
```llvm
bb0:
  br label %bb4

bb3:
  %phi_1 = phi i64 [%add_8, %bb6]    ; Defined in bb3
  ...

bb4:
  %phi_3 = phi i64 [%phi_1, %bb0], [%add_8, %bb7]  ; Uses %phi_1 from bb0!
```

MIR shows correct structure:
- bb0: `ValueId(1) = const 0`
- bb3: `ValueId(1) = PHI [bb6, ValueId(8)]`
- bb4: `ValueId(3) = PHI [(bb0, ValueId(2)), (bb7, ValueId(8))]`

**Key observation**: Same ValueId(1) used in different blocks is normal (SSA allows this), but LLVM builder is confusing them!

### Step 1: VMAP Trace Analysis

VMAP trace showed:
```
[vmap/id] Pass A bb0 snapshot id=139925440649984 keys=[0, 1]
[vmap/id] Pass A bb0 snapshot id=139925440650112 keys=[1, 2, 3]
```

Two different bb0 snapshots! But only one bb0 in `main` function.

### Root Cause Discovery

Checked all functions in MIR JSON:
```json
{
  "name": "condition_fn",
  "blocks": [0]
},
{
  "name": "main",
  "blocks": [0, 3, 4, 5, 6, 7]
}
```

**BINGO**: Two functions have bb0!
- `condition_fn` has bb0 (first snapshot)
- `main` has bb0 (second snapshot, overwrites first)

### Root Cause

**`block_end_values` uses `block_id` as key instead of `(function_name, block_id)` tuple**

**Problem flow**:
1. Process `condition_fn` bb0 → `block_end_values[0] = {0: ..., 1: ...}`
2. Process `main` bb0 → `block_end_values[0] = {1: ..., 2: ..., 3: ...}` (OVERWRITES!)
3. Process `main` bb4's PHI → resolve incoming ValueId(1) from bb0
4. `resolve_incoming(pred_block_id=0, value_id=1)` looks up `block_end_values[0][1]`
5. Gets `main`'s bb0 ValueId(1) (which is copy of PHI) instead of const 0!

**Result**: bb4's PHI gets `%phi_1` (bb3's PHI) instead of `i64 0` (bb0's const), causing domination error.

## Solution Design

### Change 1: Tuple-Key block_end_values

**Old**:
```python
block_end_values: Dict[int, Dict[int, ir.Value]] = {}
block_end_values[bid] = snap
```

**New**:
```python
block_end_values: Dict[Tuple[str, int], Dict[int, ir.Value]] = {}
block_end_values[(func_name, bid)] = snap
```

### Change 2: Thread function name through call chain

**Files to modify**:
1. `llvm_builder.py` - Type annotation
2. `function_lower.py` - Pass `func.name` to `lower_blocks`
3. `block_lower.py` - Accept `func_name` parameter, use tuple keys
4. `resolver.py` - Update `resolve_incoming` to accept `func_name`
5. `phi_wiring/wiring.py` - Update `wire_incomings` to use tuple keys

### Change 3: Verifier (STRICT mode)

Add collision detection:
```python
if key in block_end_values and STRICT:
    existing_func = find_function_for_key(key)
    if existing_func != current_func:
        raise RuntimeError(
            f"Block ID collision: bb{bid} exists in both "
            f"{existing_func} and {current_func}"
        )
```

## Acceptance Criteria

1. **Pattern 1** (Phase 132): Still passes (regression test)
2. **Case C** (Pattern 5): Builds and executes correctly
3. **VM/LLVM parity**: Both produce same result
4. **STRICT mode**: No collisions, no fallback to 0

## Implementation Status

- [x] Root cause identified
- [x] Solution designed
- [ ] Tuple-key implementation
- [ ] STRICT verifier
- [ ] Acceptance testing
- [ ] Documentation update

## Related Documents

- Task: `/home/tomoaki/git/hakorune-selfhost/CURRENT_TASK.md` (Phase 132-P0)
- Inventory: `/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/phase131-3-llvm-lowering-inventory.md`

## Key Insight

**"Same block ID in different functions is a FEATURE, not a bug"**

MIR reuses block IDs across functions (bb0 is common entry block). The LLVM builder MUST namespace block_end_values by function to avoid collisions.

This is a **Box-First principle violation**: `block_end_values` should have been scoped per-function from the start (encapsulation boundary).
