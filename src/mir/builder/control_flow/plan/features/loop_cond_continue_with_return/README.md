# LoopCondContinueWithReturn Box

**Phase**: 29bQ (Self-hosting compiler blocker)

## Route Shape

This box handles loops with the following characteristics:
- **Condition**: `loop(cond)` with a conditional expression
- **Continue**: One or more `continue` statements
- **Heterogeneous return**: `if-else` where then has no exit, else has return
- **Carrier variables**: Loop variables that need PHI nodes and Select merge

## Example

```hako
loop(i < n) {
    local ch = text.substring(i, i + 1)

    // Continue-if
    if in_str == 1 {
        continue
    }

    // Hetero-return-if
    if ch == "\"" {
        in_str = 1
    } else {
        if ch == "}" {
            return i
        }
    }

    i = i + 1
}
```

## Implementation

- **Pipeline**: `src/mir/builder/control_flow/plan/features/loop_cond_continue_with_return_pipeline.rs`
- **Facts**: `src/mir/builder/control_flow/plan/loop_cond_continue_with_return/facts.rs`
- **Normalizer**: `src/mir/builder/control_flow/plan/loop_cond_continue_with_return/normalizer.rs`
- **Planner**: `src/mir/builder/control_flow/planner/single_planner/loop_cond_continue_with_return.rs`

## Key Features

1. **step_bb route shape**: Continue jumps go to step_bb, which merges PHI inputs, then jumps to header_bb
2. **Carrier variable handling**: Variables that persist across iterations with Select merge
3. **Prelude save/restore**: Branch-local bindings in continue-if preludes are isolated

## Out-of-Scope

The following route shapes are explicitly out of scope for this box:

- **Nested return depth 3+**: Deeply nested return statements (depth 3 or more) in else chains
  - Example: `if cond { x = 1 } else { if other { ... } else { if depth == 0 { return } } } }`
  - **Handoff**: `LoopCondContinueWithReturnNested3` (v2 box)
  - **Reason**: Different dominance/merge boundary shape requiring specialized handling

See `LoopCondContinueWithReturnNested3` for these route shapes.

## Test Coverage

- **Simple route shape**: `phase29bq_selfhost_blocker_return_continue_hetero_simple.hako`
  - Shallow return (depth 1) with continue
  - Expected: stdout = 0, rc = 0
