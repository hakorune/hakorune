# Phase 188.2: StepTree nesting depth SSOT (Option A) — strict Fail-Fast

**Date**: 2025-12-27
**Status**: ✅ Option A implemented / ❌ Pattern 6 lowering deferred (Phase 188.3+)
**Goal**: `StepTreeFeatures.max_loop_depth` を SSOT として、strict mode で depth > 2 を明示エラーにする

---

## ✅ Decision: Option A Adopted (2025-12-27)

**Chosen Approach**: StepTreeFeatures Integration

**What Phase 188.2 Implements**:
- Use `StepTree.features.max_loop_depth` as SSOT for nesting depth
- Add explicit error for `max_loop_depth > 2` in strict mode (capability_guard.rs)
- Update NestedLoop capability hint to reflect current support

**Implementation Location**:
- `src/mir/builder/control_flow/joinir/control_tree_capability_guard.rs` (lines 32-76)

**Status**: Option A depth checking implemented, Option B deferred

---

## What Phase 188.2 delivered

- ✅ strict mode の depth 制約を SSOT 化（`max_loop_depth > 2` は明示エラー）
- ✅ NestedLoop capability hint を現実に合わせて更新（1-levelは許可、2+ level は depth_exceeded）
- ✅ unit tests 更新（depth=2 PASS / depth=3 FAIL）
- ✅ quick/integration は不変（回帰なし）

### Verification (reference)

- quick: `./tools/smokes/v2/run.sh --profile quick`
- integration selfhost: `./tools/smokes/v2/run.sh --profile integration --filter "selfhost_"`

---

## Background

Phase 188.1 delivered infrastructure (capability allowlist, enum, stub module), but **detection and lowering are NOT implemented** due to architectural limitations.

**See**: `docs/development/current/main/phases/phase-188.1/README.md` § Implementation Reality

---

## Design Decision Required (Choose A or B)

### Option A: StepTreeFeatures Integration

**Approach**:
- Pass `StepTreeFeatures` from StepTree to LoopForm creation
- Store `max_loop_depth` in LoopForm (add field) or pass via context
- Use AST-level nesting depth for Pattern 6 detection

**Pros**:
- Nesting info already exists in StepTree
- No new detection logic required
- Fast implementation

**Cons**:
- AST-level info may diverge from MIR structure (after optimizations)
- Adding fields to LoopForm breaks separation of concerns

**Estimated Effort**: 1-2 days

---

### Option B: LoopRegion Integration

**Approach**:
- Implement LoopRegion builder (instantiate parent/child structure)
- Build LoopRegion tree from MIR control flow graph
- Compute nesting depth from LoopRegion.parent traversal
- Replace LoopForm with LoopRegion in lowering pipeline

**Pros**:
- MIR-level truth (accurate after optimizations)
- Leverages Phase 32 infrastructure
- Clean separation (LoopRegion is designed for nesting)

**Cons**:
- Requires implementing LoopRegion builder (non-trivial)
- Larger architectural change
- May affect other loop analysis code

**Estimated Effort**: 1-2 weeks

---

## Planning Session Agenda

**Step 1: Docs-First Design Review**
- Review Option A vs B trade-offs
- Consider impact on selfhost_minimal requirements
- Decide on approach based on scope/timeline

**Step 2: Detailed Implementation Plan**
- Write step-by-step implementation guide
- Identify critical files
- Estimate effort per task

**Step 3: Implementation**
- Execute chosen approach
- Test with selfhost_minimal
- Verify 154/154 PASS maintained

---

## Out of Scope (Phase 188.2)

Still deferred to Phase 188.3+:
- Break/continue in nested loops (Pattern 2/4 combinations)
- Multiple inner loops (siblings)
- 2+ level nesting (depth > 2) の lowering（Phase 188.2 は strict Fail-Fast のみ）

**Rationale**: Phase 188.2 focuses on 1-level nesting detection only (minimal scope).

---

## References

- **Phase 188.1 Reality**: `docs/development/current/main/phases/phase-188.1/README.md` § Implementation Reality
- **LoopRegion Definition**: `src/mir/control_form.rs` (lines 40-62)
- **StepTreeFeatures Definition**: `src/mir/control_tree/step_tree/mod.rs` (lines 17-25)
- **LoopForm/LoopShape Definition**: `src/mir/loop_form.rs`, `src/mir/control_form.rs`

---

## End of Phase 188.2 Planning Stub

**Status**: Option A is complete; remaining work is Phase 188.3+
**Dependencies**: Phase 188.1 “Implementation Reality” remains SSOT
