# Phase 142-loopstmt: Statement-Level Loop Normalization

Status: ✅ P0 Complete
Date: 2025-12-19

---

## 目的（Why）

Phase 131-141 で loop(true) 系の Normalized shadow を段階的に拡張したが、「block suffix (loop + post assigns + return)」の直積パターン増殖を止める必要があった。

**Phase 142-loopstmt** では、正規化単位を **"block suffix" から "statement (loop 1個)" へ寄せる** ことで、パターン爆発を防ぐ。

### Non-Goal

- ⚠️ **Phase 142 (Canonicalizer Pattern Extension) とは別物**
  - Phase 142 = trim leading/trailing, continue pattern (Canonicalizer)
  - Phase 142-loopstmt = Statement-level normalization (Normalized shadow)
  - SSOT 衝突回避のため phase-142-loopstmt として独立管理

---

## P0: Statement-Level Normalization (COMPLETE ✅)

### Summary

Normalization unit changed from "block suffix (loop + post + return)" to "statement (loop only)".

### Implementation

#### 1. PlanBox の挙動変更

**File**: `src/mir/builder/control_flow/normalization/plan_box.rs`

**Before**:
- `loop(true)` の後ろに return が無いと plan が None になりやすい
- LoopWithPost pattern を返す（loop + post assigns + return を一括消費）

**After**:
- `remaining[0]` が `loop(true)` なら 常に `NormalizationPlan::loop_only()` を返す
- **consumed = 1** (loop のみ)
- 後続文（return/assign 等）は通常 MIR lowering へ戻す

**Code changes**:
```rust
// Phase 142-loopstmt P0: Always return loop_only for loop(true)
// Normalization unit is now "statement (loop 1個)" not "block suffix"
// Subsequent statements handled by normal MIR lowering
if debug {
    trace.routing(
        "normalization/plan",
        func_name,
        "Detected loop(true) - Phase 142-loopstmt P0: returning loop_only (consumed=1)",
    );
}
Ok(Some(NormalizationPlan::loop_only()))
```

**Impact**: ~70 lines reduced

#### 2. SuffixRouter の LoopOnly 対応

**File**: `src/mir/builder/control_flow/joinir/patterns/policies/normalized_shadow_suffix_router_box.rs`

**Before**: Lines 64-75 で PlanKind::LoopOnly を reject ("not a suffix")

**After**: LoopOnly も受け入れて execute_box に渡す

**Code changes**:
```rust
// Phase 142-loopstmt P0: Accept both LoopOnly and LoopWithPost
// Normalization unit is now "statement (loop 1個)", not "block suffix"
if debug {
    let description = match &plan.kind {
        PlanKind::LoopOnly => "Loop-only pattern".to_string(),
        PlanKind::LoopWithPost { post_assign_count } => {
            format!("Loop+post pattern: {} post assigns", post_assign_count)
        }
    };
    trace.routing("suffix_router", func_name, &description);
}
```

**Impact**: ~12 lines reduced

#### 3. build_block で consumed 後の break 削除

**File**: `src/mir/builder/stmts.rs`

**Before**:
- suffix_router が Some(consumed) を返したら break
- return statement が消費されたと仮定

**After**:
- consumed 後も `idx += consumed` でスキップし、後続文を通常処理
- Phase 142-loopstmt では loop 正規化後も `return s.length()` 等が残る

**Code changes**:
```rust
Some(consumed) => {
    trace.emit_if(
        "debug",
        "build_block/suffix_router",
        &format!("Phase 142-loopstmt P0: Suffix router consumed {} statement(s), continuing to process subsequent statements", consumed),
        debug,
    );
    // Phase 142-loopstmt P0: Normalization unit is now "statement (loop 1個)"
    // Loop normalization returns consumed=1, and subsequent statements
    // (return, assignments, etc.) are handled by normal MIR lowering
    idx += consumed;
    // No break - continue processing subsequent statements
}
```

### Unit Tests

**File**: `src/mir/builder/control_flow/normalization/plan_box.rs`

**Updated tests** (7 tests total):
- `test_plan_block_suffix_phase131_loop_only()` - unchanged
- `test_plan_block_suffix_phase142_loop_with_subsequent_stmts()` - was phase132, now expects LoopOnly
- `test_plan_block_suffix_phase142_loop_only_always()` - was phase133, now expects LoopOnly
- `test_plan_block_suffix_phase142_loop_with_trailing_stmt()` - new, no return but still LoopOnly
- `test_plan_block_suffix_no_match_empty()` - unchanged
- `test_plan_block_suffix_no_match_not_loop()` - unchanged
- `test_plan_block_suffix_no_match_loop_not_true()` - unchanged

**Results**: ✅ 7/7 passed

### E2E Tests

#### Fixture

**File**: `apps/tests/phase142_loop_stmt_only_then_return_length_min.hako`

```hako
static box Main {
    main() {
        local s
        s = "abc"
        loop(true) {
            break
        }
        return s.length()
    }
}
```

**Expected**: exit code 3 (s="abc" → s.length() → 3)

#### VM Smoke Test

**File**: `tools/smokes/v2/profiles/integration/apps/archive/phase142_loop_stmt_only_then_return_length_min_vm.sh`

**Command**:
```bash
bash tools/smokes/v2/profiles/integration/apps/archive/phase142_loop_stmt_only_then_return_length_min_vm.sh
```

**Result**: ✅ PASS (exit code 3)

#### LLVM EXE Smoke Test

**File**: `tools/smokes/v2/profiles/integration/apps/archive/phase142_loop_stmt_only_then_return_length_min_llvm_exe.sh`

**Status**: ✅ P1 COMPLETE (LLVM EXE parity achieved)

**Command**:
```bash
bash tools/smokes/v2/profiles/integration/apps/archive/phase142_loop_stmt_only_then_return_length_min_llvm_exe.sh
```

**Result**: ✅ PASS (exit code 3, parity with VM)

---

## Refactoring (COMPLETE ✅)

### Task 1: suffix_router コメント更新

**Commit**: `21a3c6b5d` - "docs(normalization): Update suffix_router comments for Phase 142-loopstmt P0"

**Changes**:
- Updated header comments to reflect post statements no longer required
- Documented LoopOnly pattern acceptance

### Task 2: LoopWithPost Deprecation

**Commit**: `aaba27d31` - "refactor(normalization): Deprecate LoopWithPost variant"

**Changes**:
- Added `#[deprecated]` to `PlanKind::LoopWithPost` enum variant
- Added deprecation to `loop_with_post()` constructor function
- Documented migration path to LoopOnly
- Kept for backward compatibility

**Deprecation warnings** (4 locations):
- `normalized_shadow_suffix_router_box.rs:69`
- `routing.rs:457`
- `plan.rs:74`
- `execute_box.rs:66`

### Task 3: README 更新

**Commit**: `3ef929df5` - "docs(normalization): Update README for Phase 142-loopstmt P0"

**Changes**:
- Added Phase 142-loopstmt P0 section
- Marked Phase 132-135 as LEGACY
- Updated suffix_router description

---

## Acceptance Criteria (All Met ✅)

- [x] PlanBox が loop(true) に対して常に loop_only を返す（consumed=1）
- [x] SuffixRouter が LoopOnly を受け入れて実行する
- [x] build_block が consumed 後も後続文を処理する
- [x] Fixture が exit code 3 を返す（VM）
- [x] 既存 smokes が緑（Phase 97/131/136/137/139/141）
  - Phase 131 VM: ✅ PASS
  - Normalization unit tests: ✅ 10 passed
- [x] Out-of-scope は常に Ok(None) でフォールバック（既定挙動不変）
- [x] Documentation 作成
- [x] Refactoring tasks 完了（3 commits）
- [x] Main implementation commit

---

## Design Principles

### Pattern Explosion Prevention

- **制御フローの骨格（loop/if/post_k/continuation）**: 正規化（段階投入）で固める
- **式（return value を含む）**: 一般化（AST walker）で受ける（Phase 140+）

### Normalization Unit Evolution

**Phase 141 以前**:
```
NormalizationPlan::loop_with_post(n) → consumed = 1 + n + 1 (loop + assigns + return)
```

**Phase 142-loopstmt P0**:
```
NormalizationPlan::loop_only() → consumed = 1 (loop のみ)
後続文は通常 MIR lowering で処理
```

### Fail-Fast Policy

- **Out-of-scope**: 常に `Ok(None)` でフォールバック（既定挙動不変）
- **Fail-Fast**: "in-scope のはずなのに壊れた" ケースのみ（internal error）

---

## Files Modified

### Core Implementation

1. `src/mir/builder/control_flow/normalization/plan_box.rs`
   - Always return loop_only for loop(true)
   - 7 unit tests updated

2. `src/mir/builder/control_flow/joinir/patterns/policies/normalized_shadow_suffix_router_box.rs`
   - Accept LoopOnly patterns
   - Remove rejection logic

3. `src/mir/builder/stmts.rs`
   - Remove break after consumed
   - Continue processing subsequent statements

### Tests

4. `apps/tests/phase142_loop_stmt_only_then_return_length_min.hako` (NEW)
5. `tools/smokes/v2/profiles/integration/apps/archive/phase142_loop_stmt_only_then_return_length_min_vm.sh` (NEW)

### Documentation

6. `src/mir/builder/control_flow/normalization/plan.rs` (deprecation)
7. `src/mir/builder/control_flow/normalization/README.md` (updated)

### Statistics

- **Total commits**: 4
- **Files changed**: 7
- **Net change**: -38 lines (削減成功！)
- **Code reduction**: ~82 lines deleted (pattern detection logic)

---

## Verification Commands

### Unit Tests
```bash
cargo test -p nyash-rust --lib mir::builder::control_flow::normalization
```

### Regression Tests
```bash
# Phase 131 regression
bash tools/smokes/v2/profiles/integration/apps/archive/phase131_loop_true_break_once_vm.sh

# Phase 141 regression
bash tools/smokes/v2/profiles/integration/apps/archive/phase141_p1_if_only_post_k_return_length_vm.sh

# Phase 142-loopstmt P0
bash tools/smokes/v2/profiles/integration/apps/archive/phase142_loop_stmt_only_then_return_length_min_vm.sh
```

---

## Next Steps

### P1: LLVM EXE Smoke Test (DONE ✅)

- File: `tools/smokes/v2/profiles/integration/apps/archive/phase142_loop_stmt_only_then_return_length_min_llvm_exe.sh`
- Gate: `llvm_exe_preflight_or_skip` により、Phase 130 の LLVM EXE 前提が満たされない環境では SKIP を維持
- Contract: exit code 3 parity with VM

### P2: Code Contract Enforcement (Planned)

**Goal**: Prevent future regression to pattern explosion

**Options**:
- **Option A (Recommended)**: Remove LoopWithPost creation path entirely
- **Option B**: Introduce Outcome { consumed, stop_block } SSOT

### Phase 143-loopvocab (Planned)

**Goal**: 「語彙追加」で `loop(true){ if(cond) break/continue }` を吸収

**Approach**:
- Extend StepTree/ControlTree vocabulary (not new patterns)
- Use NormalizedExprLowererBox for pure conditions
- JoinIR Jump { cond: Some(vid) } for conditional exits

---

## Related Documentation

- [10-Now.md](../../10-Now.md) - Current progress
- [30-Backlog.md](../../30-Backlog.md) - Future phases
- [phase-143-loopvocab/README.md](../phase-143-loopvocab/README.md) - Next planned vocabulary expansion
- [normalized-expr-lowering.md](../../design/normalized-expr-lowering.md) - Normalized design SSOT
- [control-tree.md](../../design/control-tree.md) - ControlTree design
- [normalization/README.md](../../../src/mir/builder/control_flow/normalization/README.md) - Architecture

---

## Commits

1. `21a3c6b5d` - docs(normalization): Update suffix_router comments for Phase 142-loopstmt P0
2. `aaba27d31` - refactor(normalization): Deprecate LoopWithPost variant
3. `3ef929df5` - docs(normalization): Update README for Phase 142-loopstmt P0
4. `275fe45ba` - feat(normalization): Phase 142-loopstmt P0 - Statement-level normalization
