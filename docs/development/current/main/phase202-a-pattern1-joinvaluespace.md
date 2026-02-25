# Phase 202-A: Pattern 1 JoinValueSpace Migration

**Status**: ✅ Complete
**Date**: 2025-12-09
**Commit**: `6e778948`

## Overview

Migrated Pattern 1 (Simple While Loop) from manual `value_counter` to unified `JoinValueSpace` allocation system, following the same pattern established in Phase 201 for Pattern 2.

## Motivation

Pattern 2 (Phase 201) revealed that using separate ValueId allocation mechanisms (manual counters) can cause collisions between:
- **Param region (100-999)**: Used by ConditionEnv, CarrierInfo for loop parameters
- **Local region (1000+)**: Used for temporary values (Const, BinOp, etc.)

Pattern 1 needed the same unification to:
1. **Consistency**: All patterns should use the same allocation mechanism
2. **Future-proofing**: Pattern 1 may need Param region in future enhancements
3. **Safety**: Prevent potential ValueId collision bugs

## Changes

### 1. `simple_while_minimal.rs`

**Before (Phase 188)**:
```rust
pub(crate) fn lower_simple_while_minimal(_scope: LoopScopeShape) -> Option<JoinModule> {
    let mut value_counter = 0u32;
    let mut alloc_value = || {
        let id = ValueId(value_counter);
        value_counter += 1;
        id
    };
    // ...
}
```

**After (Phase 202-A)**:
```rust
pub(crate) fn lower_simple_while_minimal(
    _scope: LoopScopeShape,
    join_value_space: &mut JoinValueSpace,
) -> Option<JoinModule> {
    let mut alloc_value = || join_value_space.alloc_local();
    // ...
}
```

**Key Points**:
- Added `join_value_space: &mut JoinValueSpace` parameter
- Removed manual `value_counter` allocation
- Uses Local region (1000+) exclusively (no Param region needed for Pattern 1)
- Added Phase 202-A documentation comments

### 2. `pattern1_minimal.rs` (Caller)

**Before**:
```rust
let join_module = match lower_simple_while_minimal(ctx.loop_scope) {
    Some(module) => module,
    None => return Ok(None),
};
```

**After**:
```rust
// Phase 202-A: Create JoinValueSpace for unified ValueId allocation
use crate::mir::join_ir::lowering::join_value_space::JoinValueSpace;
let mut join_value_space = JoinValueSpace::new();

let join_module = match lower_simple_while_minimal(ctx.loop_scope, &mut join_value_space) {
    Some(module) => module,
    None => return Ok(None),
};
```

**Key Points**:
- Create `JoinValueSpace` before calling lowerer
- Pass mutable reference to lowerer
- Pattern 1 uses Local region only (no ConditionEnv, no Param allocation)

### 3. `loop_view_builder.rs`

**Before**:
```rust
if let Some(result) = super::simple_while_minimal::lower_simple_while_minimal(scope.clone()) {
    // ...
}
```

**After**:
```rust
// Phase 202-A: Create JoinValueSpace for Pattern 1
use super::join_value_space::JoinValueSpace;
let mut join_value_space = JoinValueSpace::new();

if let Some(result) = super::simple_while_minimal::lower_simple_while_minimal(
    scope.clone(),
    &mut join_value_space,
) {
    // ...
}
```

**Key Points**:
- Create `JoinValueSpace` in `try_pattern1()` helper
- Pass to lowerer for unified allocation

## Technical Details

### ValueId Allocation Strategy (Pattern 1)

Pattern 1 is simpler than Pattern 2:

| Region | Range | Usage | Pattern 1 |
|--------|-------|-------|-----------|
| PHI Reserved | 0-99 | Loop header PHI dst | ❌ Not used |
| Param | 100-999 | ConditionEnv, CarrierInfo | ❌ Not used |
| Local | 1000+ | Const, BinOp, temps | ✅ Used |

**Why Pattern 1 doesn't need Param region**:
- No break conditions → No ConditionEnv needed
- Simple while loop → No complex carrier analysis
- All allocations are for temporary values (Const, Compare, UnaryOp, etc.)

### Example Allocation Sequence

For `loop_min_while.hako`:
```nyash
loop(i < 3) {
    print(i)
    i = i + 1
}
```

**Allocated ValueIds**:
```rust
i_init       = alloc_local()  // ValueId(1000)
loop_result  = alloc_local()  // ValueId(1001)
const_0_main = alloc_local()  // ValueId(1002)
i_param      = alloc_local()  // ValueId(1003)
const_3      = alloc_local()  // ValueId(1004)
cmp_lt       = alloc_local()  // ValueId(1005)
exit_cond    = alloc_local()  // ValueId(1006)
const_1      = alloc_local()  // ValueId(1007)
i_next       = alloc_local()  // ValueId(1008)
const_0_exit = alloc_local()  // ValueId(1009)
```

All in Local region (1000+), no collision possible.

## Testing

### Build Status
```bash
$ cargo build --release --lib
✅ Success (0 errors, 4 warnings)
```

### Unit Tests
```bash
$ cargo test --release --lib pattern
✅ 119 passed; 0 failed; 18 ignored
```

### Full Test Suite
```bash
$ cargo test --release --lib
✅ 821 passed; 0 failed; 64 ignored
```

### E2E Tests
```bash
$ ./target/release/hakorune apps/tests/loop_min_while.hako
✅ Output: "0 1 2" (correct)

$ ./target/release/hakorune apps/tests/minimal_ssa_bug_loop.hako
✅ RC: 0 (success)
```

## Benefits

1. **Consistency**: All patterns (1, 2, 3, 4) use JoinValueSpace
2. **Safety**: Guaranteed no ValueId collisions between regions
3. **Maintainability**: Single allocation mechanism to understand
4. **Future-proof**: Easy to add Param region if Pattern 1 needs ConditionEnv later
5. **Debuggability**: Clear region boundaries make debugging easier

## Comparison with Pattern 2

| Aspect | Pattern 1 | Pattern 2 |
|--------|-----------|-----------|
| ConditionEnv | ❌ No | ✅ Yes |
| Param region | ❌ Not used | ✅ Used (100+) |
| Local region | ✅ Used (1000+) | ✅ Used (1000+) |
| CarrierInfo | ❌ No | ✅ Yes |
| Break conditions | ❌ No | ✅ Yes |

**Key Difference**: Pattern 1 is simpler - it only needs Local region because it has no complex condition analysis.

## Next Steps

### Phase 202-B: Pattern 3 Migration (Planned)
- Migrate Pattern 3 (If-Else PHI) to JoinValueSpace
- Similar to Pattern 2 (needs both Param and Local regions)
- Will use ConditionEnv for PHI value resolution

### Phase 202-C: Pattern 4 Migration (Planned)
- Migrate Pattern 4 (Continue) to JoinValueSpace
- Similar complexity to Pattern 3
- Needs Param region for continue condition analysis

## References

- **Phase 201**: Pattern 2 JoinValueSpace migration (reference implementation)
- **JoinValueSpace Design**: `src/mir/join_ir/lowering/join_value_space.rs`
- **Pattern 1 Implementation**: `src/mir/join_ir/lowering/simple_while_minimal.rs`
- **Pattern 2 Reference**: `src/mir/join_ir/lowering/loop_with_break_minimal.rs`

## Commit Message

```
feat(joinir): Phase 202-A Pattern 1 uses JoinValueSpace

Migrated Pattern 1 (Simple While) to use JoinValueSpace for unified
ValueId allocation, following the same pattern as Pattern 2 (Phase 201).

Changes:
- simple_while_minimal.rs: Added join_value_space parameter, replaced
  value_counter with join_value_space.alloc_local()
- pattern1_minimal.rs: Create JoinValueSpace before calling lowerer
- loop_view_builder.rs: Create JoinValueSpace in try_pattern1()

Pattern 1 uses Local region (1000+) only, since it doesn't need
ConditionEnv (no Param region allocation required).

Tested:
- cargo build --release --lib: Success (0 errors, 4 warnings)
- cargo test --release --lib pattern: 119 passed
- E2E test apps/tests/loop_min_while.hako: Outputs "0 1 2" correctly

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>
```
Status: Active  
Scope: Pattern1 の Join Value Space 適用（JoinIR v2）
