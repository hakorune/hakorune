# Phase 33-10: Pattern 4 PHI Loss Analysis & Fix Recommendation

**Created**: 2025-12-06
**Context**: Comparison between Pattern 3 (loop_if_phi.hako) and Pattern 4 (loop_continue_pattern4.hako)

---

## Executive Summary

**Problem**: Pattern 4 loses its loop-body PHI instruction during JoinIR→MIR merge phase.
**Root Cause**: Block overwrite in `instruction_rewriter.rs` line 92
**Impact**: CRITICAL - incorrect sum calculation in continue scenarios

---

## 1. Evidence

### Pattern 3 ✅ (Preserves PHI)
```mir
bb10:
    1: %23 = phi [%20, bb8], [%22, bb9]  ← PHI PRESENT
    1: %24 = const 1
    1: %25 = %11 Add %24
```

### Pattern 4 ❌ (Loses PHI)
```mir
bb10:
    1: %8 = copy %14                     ← NO PHI!
    1: br label bb5
```

### JoinIR Creation Phase (Both Successful)
```
[joinir_block/handle_select] Created merge_block BasicBlockId(5) with 1 instructions
[joinir_block/finalize_block] Preserving 1 PHI instructions in block BasicBlockId(5)
[joinir/meta]   Block BasicBlockId(5): X instructions (1 PHI)
```
- Pattern 3: 5 instructions (1 PHI)
- Pattern 4: 3 instructions (1 PHI)

---

## 2. Root Cause Analysis

### Hypothesis: Block Overwrite in instruction_rewriter.rs

**Location**: `src/mir/builder/control_flow/joinir/merge/instruction_rewriter.rs:92`

**Problem Code**:
```rust
for (old_block_id, old_block) in blocks_merge {
    let new_block_id = remapper.get_block(func_name, *old_block_id)...;
    let mut new_block = BasicBlock::new(new_block_id);  // ← ALWAYS CREATES FRESH BLOCK!
    // ... copy instructions from old_block ...
    current_func.add_block(new_block);  // line 365
}
```

**Why This Fails**:
1. `handle_select()` creates bb10 (BasicBlockId → bb10) with PHI instruction
2. Later, merge phase processes JoinIR's BasicBlockId(5) which also maps to bb10
3. `BasicBlock::new(new_block_id)` creates FRESH block, discarding existing PHI
4. `add_block()` overwrites the block in HashMap

## 3. Recommended Fix

### Step 1: Add Debug Logging (Verification)

Add to `instruction_rewriter.rs` around line 87:

```rust
for (old_block_id, old_block) in blocks_merge {
    let new_block_id = remapper.get_block(func_name, *old_block_id)...;

    // DEBUG: Check if block already exists
    if let Some(ref current_func) = builder.current_function {
        if let Some(existing) = current_func.blocks.get(&new_block_id) {
            let phi_count = existing.instructions.iter()
                .filter(|i| matches!(i, MirInstruction::Phi{..}))
                .count();
            eprintln!("[cf_loop/joinir] 🚨 Block {:?} ALREADY EXISTS: {} inst, {} PHI",
                new_block_id, existing.instructions.len(), phi_count);
        }
    }

    let mut new_block = BasicBlock::new(new_block_id);  // ← This OVERWRITES!
```

### Step 2: Preserve Existing Blocks (RECOMMENDED FIX)

**Modify line 92** in `instruction_rewriter.rs`:

```rust
// OLD: Always creates fresh block (loses existing PHI!)
let mut new_block = BasicBlock::new(new_block_id);

// NEW: Preserve existing block if present
let mut new_block = if let Some(ref mut current_func) = builder.current_function {
    // Remove and reuse existing block (preserves PHI!)
    current_func.blocks.remove(&new_block_id)
        .unwrap_or_else(|| BasicBlock::new(new_block_id))
} else {
    BasicBlock::new(new_block_id)
};
```

**Why This Works**:
- If bb10 was created by `handle_select()` with PHI, we **retrieve and reuse** it
- New instructions from JoinIR merge are **appended** to existing instructions
- PHI instruction at the beginning of bb10 is **preserved**

### Step 3: Alternative - Check add_block() Logic

If Step 2 doesn't work, check `MirFunction::add_block()` implementation:

```rust
// If this uses .insert(), it will OVERWRITE existing blocks!
pub fn add_block(&mut self, block: BasicBlock) {
    self.blocks.insert(block.id, block);  // ← HashMap::insert overwrites!
}

// Should be:
pub fn add_block(&mut self, block: BasicBlock) {
    if let Some(existing) = self.blocks.get_mut(&block.id) {
        // Merge: prepend existing PHI, append new instructions
        let mut merged = existing.instructions.clone();
        merged.extend(block.instructions);
        existing.instructions = merged;
        existing.terminator = block.terminator;
    } else {
        self.blocks.insert(block.id, block);
    }
}
```

---

## 4. Testing Strategy

### 4.1 Before Fix - Reproduce PHI Loss

```bash
./target/release/hakorune --dump-mir apps/tests/loop_continue_pattern4.hako 2>&1 | \
  grep -A3 "^bb10:"
```

**Current Output (BUG)**:
```mir
bb10:
    1: %8 = copy %14    ← NO PHI!
    1: br label bb5
```

### 4.2 After Fix - Verify PHI Preservation

```bash
# Same command after applying Step 2 fix
./target/release/hakorune --dump-mir apps/tests/loop_continue_pattern4.hako 2>&1 | \
  grep -A3 "^bb10:"
```

**Expected Output (FIXED)**:
```mir
bb10:
    1: %16 = phi [%4, bb8], [%14, bb9]  ← PHI RESTORED!
    1: %8 = copy %14
    1: br label bb5
```

### 4.3 Compare with Pattern 3 (Control)

Pattern 3 should continue to work correctly (no regression):

```bash
./target/release/hakorune --dump-mir apps/tests/loop_if_phi.hako 2>&1 | \
  grep -A3 "^bb10:"
```

**Expected Output (Control)**:
```mir
bb10:
    1: %23 = phi [%20, bb8], [%22, bb9]  ← PHI PRESENT (as before)
    1: %24 = const 1
    1: %25 = %11 Add %24
```

---

## 5. Code Locations

### Files to Modify

| File | Lines | Purpose |
|------|-------|---------|
| `src/mir/builder/control_flow/joinir/merge/instruction_rewriter.rs` | 92 | Fix block overwrite |
| `src/mir/builder/control_flow/joinir/merge/instruction_rewriter.rs` | 87-90 | Add debug logging |

### Reference Files (For Understanding)

| File | Purpose |
|------|---------|
| `src/mir/join_ir_vm_bridge/joinir_block_converter.rs` | Select→PHI conversion (handle_select) |
| `src/mir/join_ir/lowering/loop_with_continue_minimal.rs` | Pattern 4 lowerer (Select at line 304) |
| `src/mir/join_ir/lowering/loop_with_if_phi_minimal.rs` | Pattern 3 lowerer (comparison) |
| `apps/tests/loop_continue_pattern4.hako` | Test case showing PHI loss |
| `apps/tests/loop_if_phi.hako` | Control test case (PHI preserved) |

---

## 6. Summary & Next Steps

### 6.1 Key Findings

1. **PHI Loss Mechanism**: Block overwrite in merge phase, not JoinIR conversion
2. **Root Cause**: `BasicBlock::new()` always creates fresh block at line 92
3. **Impact**: Pattern 4 produces incorrect results due to missing PHI
4. **Solution**: Preserve existing blocks by removing before recreating

### 6.2 Implementation Checklist

- [ ] Add debug logging (Step 1) - 5 minutes
- [ ] Apply block preservation fix (Step 2) - 10 minutes
- [ ] Test Pattern 4 PHI restoration - 5 minutes
- [ ] Verify Pattern 3 no regression - 2 minutes
- [ ] Run full test suite - 10 minutes
- [ ] Commit with detailed message - 5 minutes

**Total Time**: ~40 minutes

### 6.3 Expected Impact

**Correctness**:
- ✅ Pattern 4 will produce correct sum values
- ✅ PHI instructions preserved in all loop patterns
- ✅ No regression for existing patterns

**Code Quality**:
- ✅ Proper block lifecycle management
- ✅ No silent instruction loss
- ✅ Better debugging with added logging

---

## 7. Related Documentation

- **Analysis Document**: [phase33-10-joinir-merge-phi-loss-analysis.md](phase33-10-joinir-merge-phi-loss-analysis.md)
- **Local Pattern Analysis**: [phase33-10-local-pattern-mir-analysis.md](phase33-10-local-pattern-mir-analysis.md)
- **JoinIR Design**: [if_joinir_design.md](../../../private/roadmap2/phases/phase-33-joinir-if-phi-cleanup/if_joinir_design.md)

---

**Created**: 2025-12-06
**Status**: Analysis Complete, Ready for Implementation
**Priority**: HIGH (Correctness Issue)
Status: Historical
