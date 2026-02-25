# Phase 197: Pattern 4 Update Semantics

**Status**: In Progress（197-B で ExitMeta/Boundary 経路の修正は完了）
**Date**: 2025-12-06
**Goal**: Fix loop_continue_multi_carrier.hako to output correct values (25, 5)

## Problem Statement

### Current Behavior
- Output: `5, 5`
- Expected: `25, 5`

### Root Cause
The update expressions for carriers are hardcoded in `loop_with_continue_minimal.rs:310-317`:

```rust
let rhs = if carrier_name == "count" {
    const_1  // count = count + 1
} else {
    i_next   // sum = sum + i_next (and other accumulators)
};
```

This works for the simple pattern where:
- `count` carriers always use `+1`
- Other carriers always use `+i_next`

However, this breaks semantic correctness because:
- `sum = sum + i` in the source code should use the **current** `i` value
- The current implementation uses `i_next` (the next iteration's value)

### Test Case Analysis

From `loop_continue_multi_carrier.hako`:

```nyash
local i = 0
local sum = 0
local count = 0
loop(i < 10) {
  i = i + 1          // i goes 1,2,3,4,5,6,7,8,9,10
  if (i % 2 == 0) {
    continue         // Skip even: 2,4,6,8,10
  }
  sum = sum + i      // sum = sum + i (current i)
  count = count + 1  // count = count + 1
}
// Expected: sum=25 (1+3+5+7+9), count=5
```

The update expressions should be:
- `i`: `i = i + 1` (constant increment)
- `sum`: `sum = sum + i` (accumulate current i)
- `count`: `count = count + 1` (constant increment)

### Current Implementation Problem

In `loop_with_continue_minimal.rs`, the Select statement for carriers:

```rust
// carrier_next = carrier_param + rhs
loop_step_func.body.push(JoinInst::Compute(MirLikeInst::BinOp {
    dst: carrier_next,
    op: BinOpKind::Add,
    lhs: carrier_param,
    rhs,  // This is WRONG for sum!
}));
```

For `sum`:
- `rhs = i_next` (next iteration value)
- Should be: `rhs = i_current` (current iteration value)

### 197-B: ExitMeta / Boundary 経路の修正（完了）

さらに、Phase 197-B では「どの ValueId を出口として見るか」という経路も修正した:

- **問題**:
  - もともと k_exit 関数のパラメータ（`carrier_exit_ids` = 例えば ValueId(20), ValueId(21)）を ExitMeta に入れていたが、
  - これらは JoinIR 関数インライン後の MIR では「定義されない」ValueId になってしまうケースがあった。
  - 一方、`loop_step` 内の Jump 引数（`carrier_param_ids`）は MIR マージ時に正しい ValueId に remap される。

- **修正内容**:
  1. `loop_with_continue_minimal.rs` 側で、ExitMeta には k_exit のパラメータではなく、Jump の引数（`carrier_param_ids`）を使うよう変更。
  2. `reconnect_boundary`（mod.rs）に `remapper` パラメータを追加し、Boundary に書かれた JoinIR 側 ValueId を remapper 経由で MIR の ValueId に変換してから variable_map に接続。

この結果:
- carrier の出口 ValueId は「必ず MIR 上で定義がある値」になり、
- ExitBindingBuilder 〜 JoinInlineBoundary 〜 merge_joinir_mir_blocks までの配管が、multi-carrier でも破綻しない状態になった。

## Solution Design

### Phase 197-2: LoopUpdateAnalyzer

Create `src/mir/join_ir/lowering/loop_update_analyzer.rs`:

```rust
pub enum UpdateExpr {
    Const(i64),                                    // count = count + 1
    BinOp { lhs: String, op: BinOpKind, rhs: String }, // sum = sum + i
}

pub struct LoopUpdateAnalyzer;

impl LoopUpdateAnalyzer {
    pub fn analyze_carrier_updates(
        body: &[ASTNode],
        carriers: &[CarrierVar]
    ) -> HashMap<String, UpdateExpr> {
        // Extract update expressions from assignment statements
        // in the loop body
    }
}
```

### Phase 197-3: Update Lowerer

Modify `loop_with_continue_minimal.rs`:

1. Call `LoopUpdateAnalyzer::analyze_carrier_updates()` at the start
2. Remove hardcoded `if carrier_name == "count"` logic
3. Use extracted `UpdateExpr` metadata to generate correct RHS:
   - For `Const(n)`: Use `const_n`
   - For `BinOp { rhs: "i", ... }`: Use `i_current` (not `i_next`)

### Expected Fix

After implementation:
- `sum = sum + i` will use **current** `i` value
- Output will be `25, 5` as expected

## Implementation Steps

1. **Task 197-1**: Create this documentation
2. **Task 197-2**: Implement `LoopUpdateAnalyzer`
3. **Task 197-3**: Update `loop_with_continue_minimal.rs` to use metadata
4. **Task 197-4**: Test and verify output

## References

- Test case: `apps/tests/loop_continue_multi_carrier.hako`
- Lowerer: `src/mir/join_ir/lowering/loop_with_continue_minimal.rs`
- Pattern detection: `src/mir/builder/control_flow/joinir/patterns/pattern4_with_continue.rs`
Status: Historical
