# Phase 146: Loop/If Condition ANF Implementation

**Status**: P0/P1/P147 complete
**Date**: 2025-12-19
**Context**: Phase 145 P0/P1/P2 complete (ANF infrastructure). Phase 146/147 adds condition expression support.

## Overview

Phase 146 enables ANF (A-Normal Form) transformation for loop and if conditions, extending Phase 145's compound expression support to control flow conditions.

## Phase 146 P0: ANF Routing SSOT 統一

**Goal**: Add scope check to ANF routing, unify SSOT, remove legacy inline lowering.

### Implementation

1. **expr_lowerer_box.rs**: Added scope check to ANF routing (L54-79)
   - `PureOnly` scope: Skip ANF (P1 で dev-only 有効化)
   - `WithImpure` scope: Try ANF (Phase 145 behavior)

2. **post_if_post_k.rs**: Replaced legacy inline lowering with SSOT (L271-285)
   - Use `NormalizedExprLowererBox::lower_expr_with_scope()` first
   - Fallback to `lower_condition_legacy()` helper if needed
   - Added helper function `lower_condition_legacy()` (L379-413)

3. **contract.rs**: Already has `CondLoweringFailed` variant (L84)

### Files Modified (3 files)

- `src/mir/control_tree/normalized_shadow/common/expr_lowerer_box.rs` (+10 lines)
- `src/mir/control_tree/normalized_shadow/post_if_post_k.rs` (+44 lines, -25 lines legacy)
- `src/mir/control_tree/normalized_shadow/anf/contract.rs` (no change, already has variant)

### Files Created (4 files)

- `apps/tests/phase146_p0_if_cond_unified_min.hako` (exit code 7)
- `tools/smokes/.../phase146_p0_if_cond_unified_vm.sh`
- `tools/smokes/.../phase146_p0_if_cond_unified_llvm_exe.sh`
- `docs/development/current/main/phases/phase-146/README.md` (this file)

### Acceptance Criteria (P0)

- [x] Scope check added to ANF routing
- [x] Legacy inline lowering removed from post_if_post_k.rs
- [x] SSOT unified (lower_expr_with_scope is only entry point)
- [x] Build passes (cargo build --release)
- [x] Tests pass (cargo test --release --lib)
- [x] Phase 145 regression: 0 failures
- [x] Fixture exit code: 7 (VM + LLVM EXE)

## Phase 146 P1: 条件式 ANF 有効化（done）

**Goal**: Enable ANF in conditions for `PureOnly` scope behind a dev flag, starting with whitelisted intrinsic (`String.length()`).

### Implementation

1. **expr_lowerer_box.rs**: Allow ANF for PureOnly with `HAKO_ANF_ALLOW_PURE=1`
2. **anf/execute_box.rs**: Add Compare operator support (`== != < <= > >=`)
3. **config/env/joinir_dev.rs**: Add `anf_allow_pure_enabled()` function
4. **Whitelist**: KnownIntrinsic registry を利用し、`String.length()` のみ許可

### Files Created (P1)

- `apps/tests/phase146_p1_if_cond_intrinsic_min.hako` (exit code 7)
- `tools/smokes/.../phase146_p1_if_cond_intrinsic_vm.sh`
- `tools/smokes/.../phase146_p1_if_cond_intrinsic_llvm_exe.sh`

### Acceptance Criteria (P1)

- [x] `HAKO_ANF_ALLOW_PURE=1` で PureOnly scope の ANF が有効化される
- [x] `String.length()` のみ許可され、他の MethodCall は out-of-scope
- [x] Fixture exit code: 7 (VM + LLVM EXE)

## Phase 147 P0: 複合条件の順序固定（done）

**Goal**: Extend recursive ANF to Compare operators for compound conditions.

### Implementation

1. **anf/contract.rs**: `AnfParentKind::Compare` を追加
2. **anf/plan_box.rs**: Compare vs BinaryOp を判別して ParentKind を決定
3. **anf/execute_box.rs**: Compare でも再帰的 ANF を適用（left-to-right）

### Acceptance Criteria (P147)

- [x] Compare を含む複合条件でも評価順序が固定される
- [x] ANF の再帰が Compare にも適用される

## Testing

### P0 Smoke Tests

```bash
# VM
./tools/smokes/v2/profiles/integration/apps/archive/phase146_p0_if_cond_unified_vm.sh
# Expected: exit 7

# LLVM EXE
./tools/smokes/v2/profiles/integration/apps/archive/phase146_p0_if_cond_unified_llvm_exe.sh
# Expected: exit 7
```

### P1 Smoke Tests

```bash
# VM (dev-only)
HAKO_ANF_DEV=1 HAKO_ANF_ALLOW_PURE=1 \
  ./tools/smokes/v2/profiles/integration/apps/archive/phase146_p1_if_cond_intrinsic_vm.sh
# Expected: exit 7

# LLVM EXE (dev-only)
HAKO_ANF_DEV=1 HAKO_ANF_ALLOW_PURE=1 \
  ./tools/smokes/v2/profiles/integration/apps/archive/phase146_p1_if_cond_intrinsic_llvm_exe.sh
# Expected: exit 7
```

### Regression Tests

Phase 145 smokes must still pass:
- `phase145_p1_anf_length_min` → exit 12
- `phase145_p2_compound_expr_binop_min` → exit 18
- `phase145_p2_compound_expr_double_intrinsic_min` → exit 5

## References

- **Plan File**: `/home/tomoaki/.claude/plans/buzzing-strolling-volcano.md`
- **Phase 145**: `docs/development/current/main/phases/phase-145-anf/README.md`
- **ANF Contract**: `docs/development/current/main/phases/phase-144-anf/INSTRUCTIONS.md`
