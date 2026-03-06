# Normalization Entry Point Consolidation (Phase 134 P0)

**Date**: 2025-12-18
**Status**: In Progress
**Scope**: Unified entry point for Normalized shadow detection and execution

---

## Purpose

Consolidate the two separate entry points for Normalized shadow processing into a single, well-defined system:

1. **Before**: Dual entry points with scattered responsibility
   - `try_normalized_shadow()` in routing.rs
   - `suffix_router_box` in patterns/policies/
   - Decision logic ("what to lower") is duplicated and inconsistent

2. **After**: Single decision point using Box-First architecture
   - **NormalizationPlanBox**: Detects pattern and plans consumption (SSOT for "what")
   - **NormalizationExecuteBox**: Executes the plan (SSOT for "how")
   - Both entry points use the same PlanBox for consistent decisions

---

## Architecture

### Entry Points

**Two callers, one SSOT decision**:
- `routing.rs::try_normalized_shadow()`: loop statement normalization
- `suffix_router_box::try_lower_loop_suffix()`: block-suffix entry that delegates to the same loop-only plan
  - Statement-level normalization: only the loop is normalized (consumed=1)
  - Subsequent statements (return, assignments) are handled by normal MIR lowering
- Both call `NormalizationPlanBox::plan_block_suffix()` for detection

### Box Responsibilities

1. **NormalizationPlanBox** (`plan_box.rs`)
   - Responsibility: Shape detection and planning
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
    pub requires_return: bool,    // Reserved for unreachable detection bookkeeping
}

pub enum PlanKind {
    LoopOnly,                     // Current contract: statement-level loop normalization
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

## Shape Detection

### Current Contract
- Shape: `loop(true) { ... break }` (single statement, no return)
- Consumed: 1 statement
- Kind: `PlanKind::LoopOnly`
- Example: `loop(true) { x = 1; break }`
### Statement-Level Normalization
- **Change**: Normalization unit is "statement (loop only)"
- **Shape**: `loop(true)` with **Normalized-supported body shapes** only
  - Body ends with `break` and prior statements are `assignment`/`local` only
  - Body is a single `if` with `break`/`continue` branches (optional else)
- **Consumed**: Always 1 statement (the loop itself)
- **Kind**: `PlanKind::LoopOnly`
- **Subsequent statements**: Handled by normal MIR lowering (not normalized)
- **Example**: `loop(true) { break }; return s.length()` → loop normalized (consumed=1), return handled normally
- **Impact**: Prevents route/shape branching from re-expanding around post-loop statements

### Historical Note
- Older phases grouped post-loop assignments and return into a larger suffix unit.
- Current code no longer models that suffix as a separate plan kind.
- Historical discussion lives in phase docs and investigation notes, not in current code paths.

---

## Contract

### NormalizationPlanBox::plan_block_suffix()

**Inputs**:
- `builder`: Current MirBuilder state (for variable_map access)
- `remaining`: Block suffix to analyze (AST statements)
- `func_name`: Function name (for tracing)
- `debug`: Enable debug logging

**Returns**:
- `Ok(Some(plan))`: Shape detected, plan specifies what to do
- `Ok(None)`: Not a normalized pattern, use legacy fallback
- `Err(msg)`: Internal error (should not happen in well-formed AST)

**Invariants**:
- `consumed <= remaining.len()` (never consume more than available)
- Current statement-level plans always consume exactly one loop statement

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
- Updates variable_map with exit values (DirectValue mode)

---

## Integration Points

### routing.rs
- `try_normalized_shadow()`: Call PlanBox, if LoopOnly → ExecuteBox, return ValueId
- Legacy path: If PlanBox returns None, continue with existing fallback

### suffix_router_box.rs
- `try_lower_loop_suffix()`: Call PlanBox, if LoopOnly → ExecuteBox, return consumed
- Legacy path: If PlanBox returns None, return None (existing behavior)

### build_block() (stmts.rs)
- Existing while loop unchanged
- After suffix_router returns consumed, advance idx by that amount
- No change to default behavior

---

## Testing Strategy

### Unit Tests (plan_box.rs)
- Phase 131 pattern detection (loop-only)
- Return boundary detection (consumed stops at return)
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
- Phase 131 regression: PASS
- Phase 97 regression: PASS
- Default behavior unchanged (dev-only guard)

---

## References

- **Normalized shadow**: `src/mir/control_tree/normalized_shadow/`
- **Boundary contract**: `src/mir/join_ir/lowering/inline_boundary.rs`
- **StepTree**: `src/mir/control_tree/step_tree/`
- **Merge logic**: `src/mir/builder/control_flow/joinir/merge/`
