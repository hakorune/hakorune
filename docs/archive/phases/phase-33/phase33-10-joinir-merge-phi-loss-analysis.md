# JoinIR→MIR Merge Processing PHI Loss Analysis

## Executive Summary

**Problem**: Pattern 4 (loop_continue_pattern4.hako) loses PHI instructions during JoinIR→MIR merge, while Pattern 3 (loop_if_phi.hako) preserves them correctly.

**Critical Finding**: Both patterns successfully create PHI instructions during JoinIR→MIR conversion, but Pattern 4's PHI disappears during the merge phase.

## Test Cases

### Pattern 3: loop_if_phi.hako ✅ PHI Preserved
```nyash
loop(i <= 5) {
  if (i % 2 == 1) { sum = sum + i } else { sum = sum + 0 }
  i = i + 1
}
```

**Lowerer**: `loop_with_if_phi_minimal.rs`
**JoinIR**: Uses `Select` instruction for if-else PHI in loop body

### Pattern 4: loop_continue_pattern4.hako ❌ PHI Lost
```nyash
loop(i < 10) {
  i = i + 1
  if (i % 2 == 0) { continue }
  sum = sum + i
}
```

**Lowerer**: `loop_with_continue_minimal.rs`
**JoinIR**: Uses `Select` instruction for continue vs. normal path (line 304)

## PHI Creation Phase (JoinIR→MIR Conversion)

### Pattern 3 - Select→PHI Conversion ✅
```
[joinir_block/handle_select] Created merge_block BasicBlockId(5) with 1 instructions
[joinir_block/handle_select] After insert: merge_block BasicBlockId(5) has 1 instructions
[joinir_block/finalize_block] Preserving 1 PHI instructions in block BasicBlockId(5)
[joinir/meta]   Block BasicBlockId(5): 5 instructions (1 PHI)
```

**Result**: `%23 = phi [%20, bb8], [%22, bb9]` appears in final MIR bb10

### Pattern 4 - Select→PHI Conversion ✅
```
[joinir_block/handle_select] Created merge_block BasicBlockId(5) with 1 instructions
[joinir_block/handle_select] After insert: merge_block BasicBlockId(5) has 1 instructions
[joinir_block/finalize_block] Preserving 1 PHI instructions in block BasicBlockId(5)
[joinir/meta]   Block BasicBlockId(5): 3 instructions (1 PHI)
```

**Result**: PHI is created but does **NOT** appear in final MIR bb10!

## Key Difference: Instruction Count

- **Pattern 3**: BasicBlockId(5) has **5 instructions** (1 PHI + 4 others)
- **Pattern 4**: BasicBlockId(5) has **3 instructions** (1 PHI + 2 others)

## Final MIR Comparison

### Pattern 3 - bb10 (from BasicBlockId(5))
```mir
bb10:
    1: %23 = phi [%20, bb8], [%22, bb9]  ← PHI PRESENT ✅
    1: %24 = const 1
    1: %25 = %11 Add %24
    1: %11 = copy %25
    1: %12 = copy %23
    1: br label bb5
```

### Pattern 4 - bb10 (from BasicBlockId(5))
```mir
bb10:
    1: %8 = copy %14                     ← NO PHI! ❌
    1: br label bb5
```

## Hypothesis: Merge Processing Difference

The merge processing in `instruction_rewriter.rs` (lines 117-213) should handle PHI instructions correctly:

```rust
for inst in &old_block.instructions {
    // ... skip conditions ...

    let remapped = remapper.remap_instruction(inst);

    let remapped_with_blocks = match remapped {
        MirInstruction::Phi { dst, inputs, type_hint: None } => {
            // PHI block remapping (lines 183-196)
            MirInstruction::Phi {
                dst,
                inputs: inputs.iter()
                    .map(|(bb, val)| (local_block_map.get(bb).copied().unwrap_or(*bb), *val))
                    .collect(),
                type_hint: None,
            }
        }
        other => other,
    };

    new_block.instructions.push(remapped_with_blocks);  // Line 212
}
```

## Possible Causes

### Theory 1: Block Replacement
- Pattern 4's bb10 might be getting **completely replaced** instead of merged
- Check if `finalize_block()` in `joinir_block_converter.rs` (lines 691-727) is being called differently

### Theory 2: Tail Call Handling
- Pattern 4 has a tail call that might trigger different merge logic
- The tail call detection (lines 146-167) might affect how blocks are merged
- Pattern 3 may not have a tail call in the same position

### Theory 3: Return Conversion
- BasicBlockId(5) has terminator `Return { value: Some(ValueId(99991)) }`
- This gets converted to `Jump` to exit block (lines 302-320)
- Something in this conversion might be dropping instructions

### Theory 4: Block Overwrite
- Pattern 4's shorter instruction count (3 vs 5) suggests instructions are being lost
- Check if the merge process overwrites the block instead of preserving PHI

## Investigation Path

1. **Enable debug output** for instruction_rewriter.rs merge processing
2. **Check finalize_block calls** - are they different between patterns?
3. **Trace BasicBlockId(5)** - what happens to it during merge?
4. **Compare Return terminator handling** - does the conversion lose instructions?
5. **Check local_block_map** - is BasicBlockId(5) being mapped correctly?

## Next Steps

1. Add detailed logging to `instruction_rewriter.rs` around line 122-213
2. Trace the exact path Pattern 4's BasicBlockId(5) takes during merge
3. Compare with Pattern 3's path to find the divergence point
4. Check if the problem is in the initial block creation or later overwriting

## Code Locations

- JoinIR Block Converter: `src/mir/join_ir_vm_bridge/joinir_block_converter.rs`
  - `handle_select()`: lines 407-484 (creates PHI)
  - `finalize_block()`: lines 691-727 (preserves existing PHI)
- Merge Processing: `src/mir/builder/control_flow/joinir/merge/instruction_rewriter.rs`
  - `merge_and_rewrite()`: lines 18-405
  - PHI remapping: lines 183-196
- Pattern Lowerers:
  - Pattern 3: `src/mir/join_ir/lowering/loop_with_if_phi_minimal.rs`
  - Pattern 4: `src/mir/join_ir/lowering/loop_with_continue_minimal.rs` (Select at line 304)

## Expected Outcome

Pattern 4's bb10 should contain:
```mir
bb10:
    1: %XX = phi [%YY, bb8], [%ZZ, bb9]  ← Missing PHI!
    1: %8 = copy %14
    1: br label bb5
```

The PHI instruction for `sum_merged = Select(continue_cond, sum_param, sum_next)` (line 304 in loop_with_continue_minimal.rs) is being lost during merge.
Status: Historical
