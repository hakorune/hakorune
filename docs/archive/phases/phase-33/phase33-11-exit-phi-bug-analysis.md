# Phase 33-11: Exit Block PHI Node Missing Bug Analysis

## Problem Summary

The MIR generated from JoinIR lowering is missing PHI nodes in the exit block, causing "use of undefined value" errors.

## Test Case

File: `apps/tests/loop_min_while.hako`

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

## Error

```
[ERROR] use of undefined value ValueId(16)
```

## MIR Dump Analysis

```mir
bb8:
    1: ret %16    # %16 is never defined!
```

bb8 is the exit block, but ValueId(16) is never defined anywhere.

## Root Cause

In `src/mir/builder/control_flow.rs::merge_joinir_mir_blocks()`:

### Step 1: Exit block is created empty (line 618-621)
```rust
let exit_block_id = self.block_gen.next();
// ...
let exit_block = BasicBlock::new(exit_block_id);  // Empty!
func.add_block(exit_block);
```

### Step 2: Return instructions are converted to Jump (line 827-841)
```rust
MirInstruction::Return { value } => {
    if let Some(ret_val) = value {
        let remapped_val = value_map.get(ret_val).copied().unwrap_or(*ret_val);
        if debug {
            eprintln!(
                "[cf_loop/joinir]   Return({:?}) → Jump to exit",
                remapped_val
            );
        }
    }
    MirInstruction::Jump {
        target: exit_block_id,
    }
}
```

**Problem**: The return value (`remapped_val`) is logged but NOT STORED anywhere!

### Step 3: Exit block stays empty
The exit block is never populated with:
- PHI nodes to collect return values
- Return instruction to return the PHI'd value

## Expected Fix

The exit block should look like:

```mir
bb8:
    1: %16 = phi [%9 from bb7], [%2 from bb5], ...
    1: ret %16
```

Or simpler, if all functions return the same constant:

```mir
bb8:
    1: %16 = const 0
    1: ret %16
```

## JoinIR Functions Structure

The Pattern 1 lowerer generates 3 functions:

1. **main()**: Calls loop_step, returns 0
2. **loop_step()**: Tail recursion OR Jump to k_exit
3. **k_exit()**: Returns 0

All Return instructions are converted to Jump to bb8 (exit block).

## Solution Strategy

### Option A: Collect Return Values + Generate PHI
1. While converting Return→Jump, collect all return values
2. After merging, generate PHI node in exit block
3. Add Return instruction that returns PHI result

### Option B: Direct Value Propagation
1. Since Pattern 1 always returns 0, directly emit const 0 in exit block
2. Simpler but less general

### Option C: Track Return Values in Hash Map
1. Create `HashMap<BasicBlockId, ValueId>` to track returns
2. Use as PHI incoming values
3. Most robust, handles all patterns

## Recommendation

Start with **Option B** (simplest fix for Pattern 1), then generalize to **Option C** for future patterns.

## Implementation Location

File: `src/mir/builder/control_flow.rs`
Function: `merge_joinir_mir_blocks()`
Lines: ~827-950

## Test Validation

```bash
NYASH_DISABLE_PLUGINS=1 ./target/release/hakorune apps/tests/loop_min_while.hako
```

Expected output:
```
0
1
2
```
Status: Historical
