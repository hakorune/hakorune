# Normalization Entry Point Consolidation (Phase 134 P0)

**Date**: 2025-12-18
**Status**: In Progress
**Scope**: Unified entry point for Normalized shadow detection and execution

---

## Purpose

Consolidate the two separate entry points for Normalized shadow processing into a single, well-defined system:

1. **Before**: Dual entry points with scattered responsibility
   - `try_normalized_shadow()` in routing.rs (loop-only)
   - `suffix_router_box` in patterns/policies/ (loop + post statements)
   - Decision logic ("what to lower") is duplicated and inconsistent

2. **After**: Single decision point using Box-First architecture
   - **NormalizationPlanBox**: Detects pattern and plans consumption (SSOT for "what")
   - **NormalizationExecuteBox**: Executes the plan (SSOT for "how")
   - Both entry points use the same PlanBox for consistent decisions

---

## Architecture

### Entry Points

**Two callers, one SSOT decision**:
- `routing.rs::try_normalized_shadow()`: Loop-only patterns (Phase 131)
- `suffix_router_box::try_lower_loop_suffix()`: Loop patterns at block suffix (Phase 131+)
  - **Phase 142 P0**: Now accepts both LoopOnly and LoopWithPost (deprecated)
  - Statement-level normalization: only the loop is normalized (consumed=1)
  - Subsequent statements (return, assignments) handled by normal MIR lowering
- Both call `NormalizationPlanBox::plan_block_suffix()` for detection

### Box Responsibilities

1. **NormalizationPlanBox** (`plan_box.rs`)
   - Responsibility: Pattern detection and planning
   - API: `plan_block_suffix(builder, remaining, func_name, debug) -> Result<Option<NormalizationPlan>>`
   - Returns: Plan with consumed count and kind, or None if not applicable

2. **NormalizationExecuteBox** (`execute_box.rs`)
   - Responsibility: Execute the plan (StepTree build, lowering, merge)
   - API: `execute(builder, plan, remaining, func_name, debug) -> Result<()>`
   - Uses: StepTreeBuilderBox, NormalizedShadowLowererBox, merge logic

### Data Structures

**NormalizationPlan** (`plan.rs`):
```rust
pub struct NormalizationPlan {
    pub consumed: usize,          // Number of statements to consume from remaining
    pub kind: PlanKind,           // What to lower
    pub requires_return: bool,    // Whether pattern includes return (unreachable detection)
}

pub enum PlanKind {
    LoopOnly,                     // Phase 131+142 P0: loop(true) { ... break } alone
    #[deprecated]                 // Phase 142 P0: DEPRECATED - statement-level normalization
    LoopWithPost {                // Phase 132-133 (legacy): loop + post assigns + return
        post_assign_count: usize,
    },
}
```

---

## Design Principles

### Box-First
- **Plan Box**: Single responsibility for detection (no execution)
- **Execute Box**: Single responsibility for execution (no detection)
- Clear separation enables independent testing and evolution

### SSOT (Single Source of Truth)
- **Entry**: NormalizationPlanBox is the only place that decides normalization applicability
- **Contract**: Documented in this README (normative SSOT)
- **No duplication**: Suffix router and routing.rs both delegate to the same PlanBox

### Fail-Fast
- Invalid patterns return `Err` (not silent fallback)
- STRICT mode (JOINIR_DEV_STRICT) treats contract violations as panics
- Clear error messages with hints for debugging

### Legacy Preservation
- Existing behavior unchanged (dev-only guard)
- Non-normalized patterns return `Ok(None)` → legacy fallback
- No breaking changes to existing smokes

#### Removal conditions (SSOT)

この legacy fallback は “残す理由/撤去条件” をキャンペーンSSOTで固定する（設計の迷走防止）。
- SSOT: `docs/development/current/main/design/compiler-cleanliness-campaign-ssot.md`（Cleanliness Wave 3 / Normalization fallback）

---

## Pattern Detection (Phase 131-135, updated Phase 142 P0)

### Phase 131: Loop-Only
- Pattern: `loop(true) { ... break }` (single statement, no return)
- Consumed: 1 statement
- Kind: `PlanKind::LoopOnly`
- Example: `loop(true) { x = 1; break }`

### Phase 132: Loop + Single Post
- Pattern: `loop(true) { ... break }; <assign>; return <expr>`
- Consumed: 3 statements (loop, assign, return)
- Kind: `PlanKind::LoopWithPost { post_assign_count: 1 }`
- Example: `loop(true) { x = 1; break }; x = x + 2; return x`

### Phase 133: Loop + Multiple Post
- Pattern: `loop(true) { ... break }; <assign>+; return <expr>`
- Consumed: 2 + N statements (loop, N assigns, return)
- Kind: `PlanKind::LoopWithPost { post_assign_count: N }`
- Example: `loop(true) { x = 1; break }; x = x + 2; x = x + 3; return x`

### Phase 135: Loop + Return (Zero Post Assigns) **LEGACY**
- Pattern: `loop(true) { ... break }; return <expr>` (0 post-loop assignments)
- Consumed: 2 statements (loop, return)
- Kind: `PlanKind::LoopWithPost { post_assign_count: 0 }`
- Example: `loop(true) { x = 1; break }; return x`
- **Improvement**: Unifies Phase 131 and Phase 132-133 patterns under `LoopWithPost` enum
- **Compatibility**: Phase 131 (loop-only, no return) remains as `PlanKind::LoopOnly`
- **Status**: Deprecated in Phase 142 P0 (see below)

### Phase 142 P0: Statement-Level Normalization **CURRENT**
- **Change**: Normalization unit changed from "block suffix" to "statement (loop only)"
- **Pattern**: `loop(true)` with **Normalized-supported body shapes** only
  - Body ends with `break` and prior statements are `assignment`/`local` only
  - Body is a single `if` with `break`/`continue` branches (optional else)
- **Consumed**: Always 1 statement (the loop itself)
- **Kind**: `PlanKind::LoopOnly`
- **Subsequent statements**: Handled by normal MIR lowering (not normalized)
- **Example**: `loop(true) { break }; return s.length()` → loop normalized (consumed=1), return handled normally
- **Impact**: Prevents pattern explosion by separating loop normalization from post-loop statements
- **Deprecated**: `LoopWithPost` variant no longer created, kept for backward compatibility only

---

## Contract

### NormalizationPlanBox::plan_block_suffix()

**Inputs**:
- `builder`: Current MirBuilder state (for variable_map access)
- `remaining`: Block suffix to analyze (AST statements)
- `func_name`: Function name (for tracing)
- `debug`: Enable debug logging

**Returns**:
- `Ok(Some(plan))`: Pattern detected, plan specifies what to do
- `Ok(None)`: Not a normalized pattern, use legacy fallback
- `Err(msg)`: Internal error (should not happen in well-formed AST)

**Invariants**:
- `consumed <= remaining.len()` (never consume more than available)
- If `requires_return` is true, `remaining[consumed-1]` must be Return
- Post-assign patterns require `consumed >= 2` (loop + return minimum, Phase 135+)

### NormalizationExecuteBox::execute()

**Inputs**:
- `builder`: Current MirBuilder state (mutable, will be modified)
- `plan`: Normalization plan from PlanBox
- `remaining`: Same AST slice used for planning
- `func_name`: Function name (for tracing)
- `debug`: Enable debug logging

**Returns**:
- `Ok(())`: Successfully executed and merged
- `Err(msg)`: Lowering or merge failed

**Side Effects**:
- Modifies `builder` state (adds blocks, instructions, PHI)
- May emit return statement (for suffix patterns)
- Updates variable_map with exit values (DirectValue mode)

---

## Integration Points

### routing.rs
- `try_normalized_shadow()`: Call PlanBox, if LoopOnly → ExecuteBox, return ValueId
- Legacy path: If PlanBox returns None, continue with existing fallback

### suffix_router_box.rs
- `try_lower_loop_suffix()`: Call PlanBox, if LoopWithPost → ExecuteBox, return consumed
- Legacy path: If PlanBox returns None, return None (existing behavior)

### build_block() (stmts.rs)
- Existing while loop unchanged
- After suffix_router returns consumed, advance idx by that amount
- No change to default behavior

---

## Testing Strategy

### Unit Tests (plan_box.rs)
- Phase 131 pattern detection (loop-only)
- Phase 132 pattern detection (loop + single post)
- Phase 133 pattern detection (loop + multiple post)
- **Phase 135 pattern detection (loop + zero post)** **NEW**
- Return boundary detection (consumed stops at return)
- **Return boundary with trailing statements** **NEW**
- Non-matching patterns (returns None)

### Regression Smokes
- **Phase 135**: `phase135_loop_true_break_once_post_empty_return_{vm,llvm_exe}.sh` **NEW**
- Phase 133: `phase133_loop_true_break_once_post_multi_add_{vm,llvm_exe}.sh`
- Phase 132: `phase132_loop_true_break_once_post_add_llvm_exe.sh`
- Phase 131: `phase131_loop_true_break_once_{vm,llvm_exe}.sh`
- Phase 97: `phase97_next_non_ws_llvm_exe.sh`

---

## Acceptance Criteria

- Entry SSOT: This README documents the normative contract
- By-name avoidance: Uses boundary contract SSOT (no variable name guessing)
- cargo test --lib: All unit tests PASS
- Phase 133 smokes: 2/2 PASS (VM + LLVM EXE)
- Phase 131/132 regression: PASS
- Phase 97 regression: PASS
- Default behavior unchanged (dev-only guard)

---

## References

- **Normalized shadow**: `src/mir/control_tree/normalized_shadow/`
- **Boundary contract**: `src/mir/join_ir/lowering/inline_boundary.rs`
- **StepTree**: `src/mir/control_tree/step_tree/`
- **Merge logic**: `src/mir/builder/control_flow/joinir/merge/`
