# Phase 192: Loop Pattern Structure-Based Detection

**Status**: Completed – 2025-12-06  
**Scope**: JoinIR loop patterns 1–4 (Simple While / Break / If-Else PHI / Continue)

---

## 1. Goal

Replace ad-hoc, name-based loop pattern detection with a single, structure-based pipeline so that:

- Pattern routing depends only on loop structure (if/break/continue, assignments), not function names or variable names.
- New patterns can be added by extending a classifier table instead of scattering `if func_name == "main"` checks.

---

## 2. Architecture

The router now follows a three-step pipeline:

```text
AST (condition, body)
    ↓ extract_features_from_ast
LoopFeatures { has_break, has_continue, has_if_else_phi, carrier_count, ... }
    ↓ classify(&LoopFeatures)
LoopPatternKind::{Pattern1SimpleWhile, Pattern2Break, Pattern3IfPhi, Pattern4Continue}
    ↓ LOOP_PATTERNS table
LoopPatternEntry::lower(builder, &LoopPatternContext)
```

Key types:

- `LoopFeatures` – structural features of the loop body
- `LoopPatternKind` – classifier output enum
- `LoopPatternContext` – carries AST + features + pattern_kind to the lowerers

---

## 3. Implementation Notes

Files:

- `src/mir/loop_pattern_detection.rs`
  - Extended `LoopFeatures` with:
    - `has_break`, `has_continue`
    - `has_if`, `has_if_else_phi`
    - `carrier_count`, `break_count`, `continue_count`
  - `pub fn classify(features: &LoopFeatures) -> LoopPatternKind`
    - `Pattern1SimpleWhile`
    - `Pattern2Break`
    - `Pattern3IfPhi`
    - `Pattern4Continue`

- `src/mir/builder/control_flow/joinir/patterns/router.rs`
  - `LoopPatternContext::new(condition, body, func_name, debug)`:
    - Scans AST for `Continue` / `Break` (`detect_continue_in_ast`, `detect_break_in_ast`)
    - Calls `extract_features_from_ast` to build `LoopFeatures`
    - Calls `classify(&features)` to compute `pattern_kind`
  - `LOOP_PATTERNS` table:
    - Entries now rely on `ctx.pattern_kind`; `func_name` is used only for debug logging.

Detection rules (conceptual):

- Pattern 4 (Continue): `has_continue && !has_break`
- Pattern 3 (If-Else PHI): `has_if_else_phi && !has_break && !has_continue`
- Pattern 1 (Simple While): `!has_break && !has_continue && !has_if_else_phi`
- Pattern 2 (Break): `has_break && !has_continue`

Each pattern exposes:

```rust
pub fn can_lower(builder: &MirBuilder, ctx: &LoopPatternContext) -> bool;
pub fn lower(builder: &mut MirBuilder, ctx: &LoopPatternContext) -> Result<Option<ValueId>, String>;
```

---

## 4. Behavioural Results

With structure-based detection in place, all four representative tests route and lower via JoinIR-only paths:

- Pattern 1 – Simple While
  - `apps/tests/loop_min_while.hako`
  - Output: `0, 1, 2`

- Pattern 2 – Loop with Break
  - `apps/tests/joinir_min_loop.hako`

- Pattern 3 – Loop with If-Else PHI
  - `apps/tests/loop_if_phi.hako`
  - Output: `sum = 9`

- Pattern 4 – Loop with Continue
  - `apps/tests/loop_continue_pattern4.hako`
  - Output: `25`

No pattern depends on function names (e.g. `"main"`) or specific variable names (e.g. `"sum"`) any more.

---

## 5. Future Work

- Extend `LoopFeatures` / `LoopPatternKind` for:
  - Nested loops
  - Multiple carrier variables
  - More complex continue/break combinations
- Align LoopForm/LoopScopeShape-based detection with this AST-based pipeline so both views are consistent.
Status: Historical

