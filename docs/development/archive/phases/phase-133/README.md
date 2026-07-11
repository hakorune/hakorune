# Phase 133: loop(true) break-once Multiple Post-Loop Assignments

**Status**: ✅ Implemented & Verified
**Date**: 2025-12-18
**Scope**: P0 (Minimal)

## Overview

Phase 133-P0 extends Phase 132-P4's loop(true) break-once pattern to accept **multiple post-loop assignments** before the final return statement. This enables more complex post-loop computation while maintaining the PHI-free Normalized shadow architecture.

## Pattern

```nyash
x = 0              // pre-loop init
loop(true) {       // condition is Bool literal true
    x = 1          // body assignment
    break          // break at end
}
x = x + 2          // first post-loop assignment
x = x + 3          // second post-loop assignment (NEW in P133)
return x           // return updated value (1 + 2 + 3 = 6)
```

## Implementation Changes

### 1. Pattern Detection (loop_true_break_once.rs)

**Extended from Phase 132**:
- Phase 132: `post_nodes == [Assign, Return]` (exactly 1 assign)
- Phase 133: `post_nodes == [Assign+, Return]` (1+ assigns)

```rust
// Phase 133-P0: Detect multi-assign + return pattern (generalize Phase 132-P4)
let has_post_computation = if post_nodes.is_empty() {
    false
} else if post_nodes.len() >= 2 {
    // Check if all nodes except the last are Assign statements
    let all_assigns = post_nodes[..post_nodes.len() - 1]
        .iter()
        .all(|n| matches!(n, StepNode::Stmt { kind: StepStmtKind::Assign { .. }, .. }));

    // Check if the last node is a Return statement
    let ends_with_return = matches!(
        post_nodes.last(),
        Some(StepNode::Stmt { kind: StepStmtKind::Return { .. }, .. })
    );

    all_assigns && ends_with_return
} else {
    false
};
```

### 2. Post-Loop Lowering (loop_true_break_once.rs)

**Iterative Assignment Processing**:
```rust
// Phase 133-P0: Lower multiple post-loop assignments
// Split post_nodes into assigns and return (last element is return)
let assign_nodes = &post_nodes[..post_nodes.len() - 1];
let return_node = post_nodes.last().unwrap();

// Lower all assignment statements
for node in assign_nodes {
    let StepNode::Stmt { kind: StepStmtKind::Assign { target, value_ast }, .. } = node else {
        return Ok(None);
    };
    if LegacyLowerer::lower_assign_stmt(
        target,
        value_ast,
        &mut post_k_func.body,
        &mut next_value_id,
        &mut env_post_k,
    )
    .is_err()
    {
        return Ok(None);
    }
}
```

### 3. SSOT Maintenance

**ExitMeta Contract**:
- **Unchanged**: ExitMeta still uses `env_post_k`'s final values as SSOT
- **Iterative Updates**: Each assignment updates `env_post_k` map
- **Final State**: Last assignment's result becomes the exit value

## JoinIR Structure (Unchanged from Phase 132)

**5-Function Module**:
1. `main(env)` → TailCall(loop_step, env)
2. `loop_step(env)` → TailCall(loop_body, env)
3. `loop_body(env)` → <assign statements> → TailCall(k_exit, env)
4. `k_exit(env)` → TailCall(post_k, env)
5. `post_k(env)` → **<assign>*** → Ret(env[x])  ← **NEW: Multiple assigns**

**Contract**:
- PHI-free: All state passing via env arguments + continuations
- DirectValue mode: ExitMeta uses post_k's final env values

## Scope Limitations (Phase 130 Baseline)

**Allowed Assignments**:
- ✅ `x = <int literal>` (e.g., `x = 0`)
- ✅ `x = y` (variable copy)
- ✅ `x = x + <int literal>` (increment)

**Not Allowed**:
- ❌ `x = call(...)` (function calls)
- ❌ `x = a + b` (general binary ops)
- ❌ `if`, `loop`, `print` in post-loop

## Test Coverage

### Fixture
- `apps/tests/phase133_loop_true_break_once_post_multi_add_min.hako`
- Expected exit code: **6** (1 + 2 + 3)

### Smoke Tests
1. **VM**: `tools/smokes/v2/profiles/integration/apps/archive/phase133_loop_true_break_once_post_multi_add_vm.sh`
2. **LLVM EXE**: `tools/smokes/v2/profiles/integration/apps/archive/phase133_loop_true_break_once_post_multi_add_llvm_exe.sh`

### Regression Tests
- ✅ Phase 132 (exit code 3)
- ✅ Phase 131 (exit code 1)
- ✅ Phase 97 (numeric output 2, -1, 3)

## Verification Results

```bash
# Unit tests
cargo test --lib
# Result: 1176 passed; 0 failed

# Phase 133 smokes
bash tools/smokes/v2/profiles/integration/apps/archive/phase133_loop_true_break_once_post_multi_add_vm.sh
# Result: PASS (exit code 6)

bash tools/smokes/v2/profiles/integration/apps/archive/phase133_loop_true_break_once_post_multi_add_llvm_exe.sh
# Result: PASS (exit code 6)

# Regression smokes
bash tools/smokes/v2/profiles/integration/apps/archive/phase132_loop_true_break_once_post_add_llvm_exe.sh
# Result: PASS (exit code 3)

bash tools/smokes/v2/profiles/integration/apps/archive/phase131_loop_true_break_once_llvm_exe.sh
# Result: PASS (exit code 1)

bash tools/smokes/v2/profiles/integration/apps/archive/phase97_next_non_ws_llvm_exe.sh
# Result: PASS (numeric output)
```

## Design Principles

### 1. Minimal Change
- Extends Phase 132's detection logic from `len() == 2` to `len() >= 2`
- Replaces single assignment lowering with iterative loop
- **Zero changes** to JoinIR structure, ExitMeta, or merge logic

### 2. SSOT Preservation
- `env_post_k` remains the single source of truth for exit values
- Each assignment updates `env_post_k` map in order
- ExitMeta collection happens **after** all assignments

### 3. Reuse
- Uses `LegacyLowerer::lower_assign_stmt` for each assignment
- Leverages Phase 130's proven assignment handling
- No new lowering code required

### 4. Fail-Fast
- Contract violations trigger `freeze_with_hint` in strict mode
- Missing env fields → explicit error with hint
- Invalid assignment → fallback to legacy (Ok(None))

## Dev Toggle

**Required**: `NYASH_JOINIR_DEV=1` and `HAKO_JOINIR_STRICT=1`

This is a dev-only Normalized shadow feature. With toggle OFF, the pattern falls back to legacy lowering with zero impact on production code.

## Files Modified

1. `src/mir/control_tree/normalized_shadow/loop_true_break_once.rs` (3 edits)
   - Extended pattern detection
   - Iterative assignment lowering
   - Updated debug logging

2. `apps/tests/phase133_loop_true_break_once_post_multi_add_min.hako` (new)
3. `tools/smokes/v2/profiles/integration/apps/archive/phase133_loop_true_break_once_post_multi_add_vm.sh` (new)
4. `tools/smokes/v2/profiles/integration/apps/archive/phase133_loop_true_break_once_post_multi_add_llvm_exe.sh` (new)

## Acceptance Criteria

- ✅ cargo test --lib PASS (1176 tests)
- ✅ Phase 133 VM smoke: PASS (exit code 6)
- ✅ Phase 133 LLVM EXE smoke: PASS (exit code 6)
- ✅ Phase 132 regression: PASS (exit code 3)
- ✅ Phase 131 regression: PASS (exit code 1)
- ✅ Phase 97 regression: PASS (numeric output)
- ✅ Dev toggle OFF: Zero impact on production

## Summary

Phase 133-P0 successfully generalizes Phase 132-P4's single post-loop assignment to support **multiple post-loop assignments**, enabling more complex post-loop computation patterns. The implementation maintains all architectural invariants (PHI-free, DirectValue mode, SSOT) while adding only ~30 lines of code.

**Key Achievement**: Transformed fixed-length pattern detection (`len() == 2`) to flexible pattern detection (`len() >= 2`) with zero impact on existing contracts.
