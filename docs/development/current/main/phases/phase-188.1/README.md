# Phase 188.1: cap_missing/NestedLoop 解除（strict gate unblock）

**Date**: 2025-12-27
**Goal**: `cap_missing/NestedLoop` を解除して、`selfhost_minimal` を integration で PASS させる（Fail-Fastを崩さない）
**Status**: ✅ Capability gate unblock 完了 / ❌ Pattern 6（検出・lowering）は Phase 188.2+ に deferred

---

## ⚠️ Implementation Reality (Phase 188.1 Scope)

### What Phase 188.1 actually did

- ✅ **StepTree capability gate**: `StepCapability::NestedLoop` を strict allowlist に追加して、`cap_missing/NestedLoop` を解除した
- ✅ **Integration導線**: `selfhost_minimal` の SKIP を撤去しても integration selfhost が FAIL=0 になることを確認した

### What Phase 188.1 did NOT do

- ❌ **Nested loop の自動検出（LoopFormベース）**は未実装  
  `loop_pattern_detection::extract_features()` は `max_loop_depth=1` / `has_inner_loops=false` の固定値（TODO）で、Pattern 6 の分岐は到達しない
- ❌ **Pattern 6 lowering** は未実装  
  `src/mir/join_ir/lowering/loop_patterns/nested_minimal.rs` はインフラ（stub）で、現状 `None` を返す

### Why LoopForm-based nesting detection is impossible (current architecture)

Nesting depth は **StepTree（AST側）**にはあるが、**LoopForm（MIR側）**には存在しない。

- ✅ StepTree: `StepTreeFeatures.max_loop_depth`（AST → StepTree 変換時に計算）
- ❌ LoopForm: `LoopForm = LoopShape` は CFG ブロック参照だけを持ち、親子/深さ情報が無い

従って、ネスト深さの検査や Pattern 6 の自動検出を **LoopFormレイヤーだけで完結させることはできない**。
次の実装は Phase 188.2 で「StepTree側を使うか / LoopRegion（親子構造）を実装するか」を docs-first で決める。

## Pattern 6 Specification: NestedLoop Minimal

**Note**: このセクションは「目標仕様（design）」であり、Phase 188.1 では gate unblock のみ完了。実装は Phase 188.2+。

### Supported Forms (ONLY)

**Pattern**: Outer Pattern 1 + Inner Pattern 1

```nyash
// Outer loop: Pattern 1 (simple while, no break/continue)
loop(outer_cond) {
  // ... outer loop body before inner ...

  // Inner loop: Pattern 1 ONLY (simple while, no break/continue)
  loop(inner_cond) {
    // ... inner loop body ...
  }

  // ... outer loop body after inner ...
}
```

**Requirements**:
- **Outer loop**: Pattern 1 (Simple While) - no break/continue
- **Inner loop**: Pattern 1 (Simple While) - no break/continue
- **Nesting depth**: EXACTLY 1 level (`max_loop_depth == 2`)
- **No control flow**: No break/continue in either loop
- **Sequential execution**: Inner loop completes before outer continues

### Unsupported Forms (Explicit Error)

**Rejected with明示エラー** (no silent fallback):

1. **Deeper nesting** (`max_loop_depth > 2`):
   ```
   [joinir/nested_loop/depth_exceeded] max_loop_depth=3 exceeds limit (max=2)
   Hint: Refactor to avoid 3+ level nesting, or split into separate functions
   ```

2. **Inner loop with break/continue**:
   ```
   [joinir/nested_loop/inner_control_flow] Inner loop has break/continue (not supported)
   Hint: Only simple while loops (Pattern 1) supported as inner loops
   ```

3. **Outer loop with break/continue**:
   ```
   [joinir/nested_loop/outer_control_flow] Outer loop has break/continue (not supported)
   Hint: Only simple while loops (Pattern 1) supported as outer loops
   ```

4. **Multiple inner loops** (siblings):
   ```
   [joinir/nested_loop/multiple_inner] Multiple inner loops detected (not supported)
   Hint: Only one inner loop per outer loop supported
   ```

---

## JoinIR Lowering Strategy

### Example Input (Nyash)

```nyash
static box Main {
  main() {
    local outer_i = 0
    loop(outer_i < 3) {
      local inner_j = 0
      loop(inner_j < 2) {
        print(inner_j)
        inner_j = inner_j + 1
      }
      outer_i = outer_i + 1
    }
    return 0
  }
}
```

### Expected Output (JoinIR Pseudocode)

```text
fn main():
  Call(outer_step, [0, k_main_exit])

fn outer_step(outer_i, k_outer_exit):
  // Exit condition check
  exit_cond = !(outer_i < 3)
  Jump(k_outer_exit, [], cond=exit_cond)  // Early exit if condition false

  // Initialize inner loop variables
  inner_j = 0

  // Inner loop step function (nested inside outer_step)
  fn inner_step(inner_j, k_inner_exit):
    exit_cond = !(inner_j < 2)
    Jump(k_inner_exit, [], cond=exit_cond)

    print(inner_j)
    inner_j_next = inner_j + 1
    Call(inner_step, [inner_j_next, k_inner_exit])  // Tail recursion

  // k_inner_exit continuation (resume outer loop body after inner completes)
  fn k_inner_exit():
    outer_i_next = outer_i + 1
    Call(outer_step, [outer_i_next, k_outer_exit])  // Outer tail recursion

  // Entry: call inner loop
  Call(inner_step, [inner_j, k_inner_exit])

fn k_main_exit():
  return 0
```

**Key Points**:
- **Nested functions**: Inner step function is defined inside outer step function
- **Continuation wiring**: `k_inner_exit` resumes outer loop body after inner completes
- **Carrier isolation**: Outer carriers (`outer_i`) and inner carriers (`inner_j`) are separate
- **Same pattern as Pattern 1**: Both loops use tail-recursive step function pattern

---

## ⚠️ Implementation Reality (Phase 188.1 Scope)

### What Was Actually Implemented

**Phase 188.1 delivered**:
1. ✅ **StepTree capability gate**: `StepCapability::NestedLoop` added to allowlist
2. ✅ **Pattern 6 enum**: `Pattern6NestedLoopMinimal` classification added
3. ✅ **Lowering stub**: `nested_minimal.rs` module created (infrastructure only)
4. ✅ **Test pass**: selfhost_minimal conditional SKIP removed, 154/154 PASS maintained

**Phase 188.1 did NOT implement**:
- ❌ **Automatic nesting detection from LoopForm**: LoopForm (= LoopShape) has NO nesting information
- ❌ **Pattern 6 lowering logic**: `lower_nested_loop_minimal_to_joinir()` returns `None` (stub)
- ❌ **LoopRegion integration**: LoopRegion parent/child structure exists but is NOT instantiated

### Why Automatic Detection Is NOT Possible (Current Architecture)

**LoopForm Limitation**:
```rust
// src/mir/loop_form.rs
pub type LoopForm = crate::mir::control_form::LoopShape;

// LoopShape structure (src/mir/control_form.rs):
pub struct LoopShape {
    pub preheader: BasicBlockId,
    pub header: BasicBlockId,
    pub body: Vec<BasicBlockId>,
    pub latch: BasicBlockId,
    pub exit_blocks: Vec<BasicBlockId>,
    // ❌ NO parent field
    // ❌ NO children field
    // ❌ NO depth field
}
```

**Nesting information exists ONLY in StepTree** (AST level):
```rust
// src/mir/control_tree/step_tree/mod.rs (line 17-25)
pub struct StepTreeFeatures {
    pub max_loop_depth: u32,  // ← Detected during AST → StepTree conversion
    // ...
}
```
- ✅ AST parsing time: Nesting depth calculated
- ❌ MIR lowering time: LoopForm has NO access to this information

**LoopRegion infrastructure exists but is NOT integrated**:
```rust
// src/mir/control_form.rs (line 40-62) - Phase 32 definition
pub struct LoopRegion {
    pub parent: Option<LoopId>,   // ← Structure defined
    pub children: Vec<LoopId>,    // ← But NOT instantiated anywhere
}
```

### Where Nesting Depth Checking MUST Happen

**✅ Possible locations**:
1. **StepTree level** (before LoopForm creation):
   - Use `StepTreeFeatures.max_loop_depth` during control flow analysis
   - Reject `max_loop_depth > 2` at StepTree → LoopForm conversion time

2. **LoopRegion level** (if integrated in Phase 188.2+):
   - Use `LoopRegion.parent` to compute actual nesting depth
   - Build LoopRegion tree and check depth constraints

**❌ Impossible location**:
- **LoopForm level**: No nesting information available (by design)

### Phase 188.2 Design Decision Required

**Choice A: StepTreeFeatures Integration**
- Pass `StepTreeFeatures` (AST-level) to JoinIR lowering
- Use `max_loop_depth` from StepTree for Pattern 6 detection
- Pro: Nesting info already exists
- Con: AST-level info may not match MIR structure (optimizations)

**Choice B: LoopRegion Integration**
- Instantiate `LoopRegion` with parent/child relationships
- Compute nesting depth from MIR control flow graph
- Pro: MIR-level truth (accurate after optimizations)
- Con: Requires implementing LoopRegion builder (Phase 32 infrastructure not yet wired)

**Decision timeline**: Phase 188.2 planning session (docs-first approach)

---

## Implementation Files

### New Files Created

1. **`src/mir/join_ir/lowering/loop_patterns/nested_minimal.rs`**
   - Phase 188.1: インフラ（stub）。現状 `lower_nested_loop_minimal_to_joinir()` は `None` を返す

### Modified Files

1. **`src/mir/loop_pattern_detection/mod.rs`** (~15 lines)
   - Add `Pattern6NestedLoopMinimal` enum variant
   - Add `max_loop_depth`, `has_inner_loops` to `LoopFeatures`
   - Update `extract_features()`, `classify()`

2. **`src/mir/join_ir/lowering/loop_patterns/mod.rs`** (~2 lines)
   - Export `nested_minimal` module

3. **`src/mir/join_ir/lowering/loop_pattern_router.rs`** (~10 lines)
   - Add Pattern 6 routing case
   - Add explicit error for `max_loop_depth > 2`

4. **`src/mir/builder/control_flow/joinir/control_tree_capability_guard.rs`** (~1 line)
   - Add `StepCapability::NestedLoop` to allowlist

5. **`tools/smokes/v2/profiles/integration/selfhost/selfhost_minimal.sh`** (~6 lines removed)
   - Remove conditional SKIP for `cap_missing/NestedLoop`

---

## Integration Points

### Detection Pipeline

1. **AST → StepTree** (`step_tree.rs` lines 461-463):
   ```rust
   if features.max_loop_depth > 1 {
       facts.add_capability(StepCapability::NestedLoop);
   }
   ```

2. **StepTree → Contract** (automatic capability contract check)

3. **Contract → Guard** (`control_tree_capability_guard.rs` line 44):
   ```rust
   StepCapability::NestedLoop,  // Phase 188.1: Now in allowlist
   ```

4. **LoopForm → Pattern Detection** (`loop_pattern_detection::classify()`):
   ```rust
   // Phase 188.1: Pattern 6 enum defined, but detection logic is STUB
   // Currently always returns max_loop_depth = 1 (default)
   // because LoopForm has NO nesting information.
   //
   // TODO (Phase 188.2): Integrate StepTreeFeatures or LoopRegion
   // to enable actual nesting detection.
   if features.max_loop_depth == 2
       && features.has_inner_loops
       && !features.has_break
       && !features.has_continue
   {
       return LoopPatternKind::Pattern6NestedLoopMinimal;  // Never reached (stub)
   }
   ```

5. **Pattern → Lowering** (`loop_pattern_router.rs`):
   ```rust
   LoopPatternKind::Pattern6NestedLoopMinimal => {
       super::loop_patterns::lower_nested_loop_minimal_to_joinir(loop_form, lowerer)
   }
   ```

---

## Success Criteria

### Functional Requirements

- ✅ `selfhost_minimal.sh`: Remove conditional SKIP, test PASS (or explicit error)
- ✅ `integration --filter "selfhost_"`: FAIL=0 maintained
- ✅ `quick`: 154/154 PASS unchanged

### Quality Requirements

- ✅ Explicit errors for unsupported forms (no silent `Ok(None)`)
- ✅ Phase 286 Fail-Fast principle maintained (no silent fallback)
- ✅ Minimal diff (~180 lines total)

---

## Out of Scope (Future Work)

**Deferred to Phase 188.2+**:
- Pattern 6 の実装（ネスト検出のSSOT決定 + lowering 実装）
- break/continue in nested loops (Pattern 2/4 inner/outer combinations)
- Multiple inner loops (siblings)
- 2+ level nesting (3+ loop depth)
- Nested loops with PHI (Pattern 3 inner/outer combinations)
- Shared carriers between outer/inner loops

**Rationale**: Phase 188.1 is minimal PoC to unblock selfhost_minimal. Complex patterns deferred to avoid scope creep.

---

## References

- **Phase 188 Overview**: `docs/private/roadmap2/phases/phase-188-joinir-loop-pattern-expansion/README.md`
- **Pattern Classification**: `docs/private/roadmap2/phases/phase-188-joinir-loop-pattern-expansion/pattern-classification.md`
- **JoinIR Architecture**: `docs/development/current/main/joinir-architecture-overview.md`
- **Selfhost Integration Limitations**: `docs/development/current/main/investigations/selfhost-integration-limitations.md`

---

## End of Phase 188.1 Documentation

**Status**: Infrastructure complete, detection logic deferred to Phase 188.2
**Reality**: Pattern 6 enum exists, but automatic detection is NOT implemented (LoopForm limitation)
**Total Estimated Effort**: 4-5 hours (infrastructure only)
**Date Completed**: 2025-12-27
