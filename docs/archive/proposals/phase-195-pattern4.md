# Phase 195: Pattern 4 (Loop with Continue) Implementation Plan

Status: Historical

**Status**: Deferred (not yet implemented)

## Overview

Pattern 4 handles loops with `continue` statements that skip to the next iteration. This is the most complex loop pattern due to additional control flow requirements.

## Why Deferred?

1. **Continue semantics require additional PHI and control flow analysis**
   - Continue creates an additional edge to the loop header
   - Requires phi nodes for both continue and normal paths
   - More complex than break (which exits the loop)

2. **Pattern 3 covers most practical cases**
   - Pattern 1: Simple while loops
   - Pattern 2: Loops with break
   - Pattern 3: Loops with if + PHI (most common complex pattern)
   - Pattern 4: Loops with continue (less common in practice)

3. **Lower priority than break/if patterns**
   - Break patterns (Pattern 2) are more common
   - If + PHI patterns (Pattern 3) handle complex control flow
   - Continue can often be refactored using if statements

## Example Use Case

```nyash
local i = 0
local sum = 0
loop(i < 10) {
  i = i + 1
  if (i % 2 == 0) {
    continue  // Skip even numbers
  }
  sum = sum + i
}
// sum = 25 (1+3+5+7+9)
```

## Implementation Requirements

When implemented, Pattern 4 lowering will need to:

1. **Detect continue statements** in the loop body
2. **Generate PHI nodes** for continue target (loop header)
3. **Handle carrier variables** (i, sum) across continue boundaries
4. **Generate exit PHI nodes** for final values after loop

## Control Flow Diagram

```
       header
         |
         v
       body
         |
    /----+----\
   /           \
  v             v
continue      normal
  |             |
  \-----+-------/
        |
        v
      latch
        |
    /---+---\
   /         \
  v           v
loop        exit
```

## Workaround

Until Pattern 4 is implemented, use Pattern 3 (if + PHI) instead:

```nyash
local i = 0
local sum = 0
loop(i < 10) {
  i = i + 1
  if (not (i % 2 == 0)) {  // Invert condition
    sum = sum + i
  }
}
```

## Migration Path

1. **Pattern 1**: Simple while loops (no break/continue)
2. **Pattern 2**: Loops with break
3. **Pattern 3**: Loops with if + PHI
4. **Pattern 4**: (FUTURE) Loops with continue statements

## Related Files

- `src/mir/builder/control_flow/joinir/patterns/pattern4_with_continue.rs` - Stub implementation
- `src/mir/join_ir/lowering/loop_with_continue_minimal.rs` - Lowering logic (TODO)

## Timeline

- Phase 195+: Implementation planned but deferred
- Priority: Lower than Pattern 1-3
- Complexity: High (additional control flow edges)
