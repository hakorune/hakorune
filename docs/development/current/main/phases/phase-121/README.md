# Phase 121: StepTree→Normalized Shadow Lowering (if-only, dev-only)

**Status**: ✅ COMPLETE
**Date**: 2025-12-18
**Scope**: if-only patterns (no loops)

## Overview

Phase 121 establishes a minimal shadow lowering route from StepTree (structure SSOT) to Normalized JoinIR form, with parity verification against the existing router for if-only patterns.

## Objectives

- Create dev-only shadow lowering path: StepTree → JoinModule (Normalized)
- Verify parity with existing router (exit contracts + writes)
- Maintain default behavior (dev-only, no production impact)
- Establish foundation for future Normalized migration

## Implementation

### P0: Design Documentation ✅

**Commit**: `8d930d2dc` - "docs: Phase 121 StepTree→Normalized shadow plan"

Added comprehensive design section to `docs/development/current/main/design/control-tree.md`:
- Input SSOT: StepTree + StepTreeContract
- Output: JoinModule (Normalized dialect)
- Execution conditions: dev-only, strict fail-fast
- Prohibitions: fallback, env direct reads, hardcoding

### P1: Module Structure ✅

**Commit**: `1e5432f61` - "feat(control_tree): add StepTree→Normalized shadow lowerer (if-only, dev-only)"

Created modular structure in `src/mir/control_tree/normalized_shadow/`:

**`mod.rs`**: Module interface and exports

**`contracts.rs`**: Capability checking
- `UnsupportedCapability` enum (Loop/Break/Continue/Other)
- `CapabilityCheckResult` type
- `check_if_only()` function

**`builder.rs`**: Shadow lowering
- `StepTreeNormalizedShadowLowererBox` (Box-First principle)
- `try_lower_if_only()` - Returns `Result<Option<(JoinModule, JoinFragmentMeta)>, String>`
  - `Ok(None)`: Out of scope
  - `Ok(Some(...))`: Success
  - `Err(...)`: Internal error
- `get_status_string()` - Dev logging

**`parity.rs`**: Comparison & verification
- `MismatchKind` enum (ExitMismatch/WritesMismatch/UnsupportedKind)
- `ShadowParityResult` type
- `compare_exit_contracts()`, `compare_writes_contracts()`, `check_full_parity()`

**Tests**: 12 unit tests (all passing)

### P2: Dev-Only Wiring ✅

**Commit**: `89b868703` - "feat(joinir/dev): wire Phase 121 StepTree shadow lowering (strict fail-fast)"

Integrated into `src/mir/builder/calls/lowering.rs`:
- Wired into existing `lower_function_body()` after capability guard
- Only runs when `joinir_dev_enabled()` returns true
- Strict mode fail-fast with `freeze_with_hint` (hint required)
- Dev logging: `[trace:dev] phase121/shadow: ...`

**Behavior**:
- Default: No impact (dev-only gate)
- Dev mode: Shadow lowering attempted, status logged
- Strict mode: Fail-fast on if-only mismatch

### P3: Parity SSOT ✅

Included in P1 (`parity.rs`):
- Minimal comparison: exits + writes contracts
- No value comparison (too fragile)
- BTreeSet deterministic ordering
- Clear mismatch classification

### P4: Smoke Tests ✅

**Commit**: `0892df6df` - "test(joinir): Phase 121 shadow parity smokes (VM + LLVM EXE)"

Created smoke tests in `tools/smokes/v2/profiles/integration/apps/`:
- `phase121_shadow_if_only_vm.sh` ✅ (3/3 tests PASS)
- `phase121_shadow_if_only_llvm_exe.sh` (created, LLVM harness config needed)

**Test fixtures** (existing):
1. `phase103_if_only_merge_min.hako` - Basic if merge (output: 2)
2. `phase114_if_only_return_then_post_min.hako` - Return + post (output: 7\n2)
3. `phase117_if_only_nested_if_call_merge_min.hako` - Nested if (output: 2\n3\n4)

**Test conditions**:
- `HAKO_JOINIR_STRICT=1` (strict mode)
- `NYASH_JOINIR_DEV=1` (dev mode)
- `NYASH_DISABLE_PLUGINS=1` (VM required)

### P5: Documentation ✅

**This document** + updates to:
- `docs/development/current/main/design/control-tree.md` (Phase 121 design)
- `docs/development/current/main/10-Now.md` (updated)
- `docs/development/current/main/01-JoinIR-Selfhost-INDEX.md` (updated)
- `docs/development/current/main/30-Backlog.md` (updated)

## Verification

### Build ✅
```bash
cargo build --lib
cargo test --lib normalized_shadow
# 12 tests passed
```

### Smoke Tests ✅
```bash
bash tools/smokes/v2/profiles/integration/apps/phase121_shadow_if_only_vm.sh
# [PASS] phase121_shadow_if_only_vm: All tests passed
```

### Manual Verification ✅
```bash
NYASH_JOINIR_DEV=1 ./target/release/hakorune apps/tests/phase103_if_only_merge_min.hako 2>&1 \
  | grep "phase121/shadow"
# [trace:dev] phase121/shadow: shadow_lowered=true ...
```

## Design Highlights

### Box-First Principle
- Single responsibility modules (contracts/builder/parity)
- Clear boundaries (capability check → lowering → parity)
- Testable units (12 unit tests)

### Fail-Fast Enforcement
- No fallback on error (dev log or strict freeze)
- Explicit unsupported reasons (Loop/Break/Continue)
- Mandatory hint on freeze (`freeze_with_hint` requires non-empty hint)

### SSOT Discipline
- No AST re-analysis (contract-only decisions)
- No env direct reads (all through `config::env/*`)
- No hardcoding (no fixture name branching)

## Next Steps (Future Phases)

Phase 121 is complete. Future work:
1. **Actual lowering** - Currently returns empty JoinModule, implement real conversion
2. **Loop support** - Extend beyond if-only scope
3. **Value parity** - Compare generated values (post-stabilization)
4. **LLVM harness** - Complete LLVM smoke test configuration
5. **Production migration** - Gradually enable for production paths

## Known Limitations

- **Phase 121 scope**: contract parity（exits/writes）までを固定（値の一致や JoinModule の実行は対象外）
- **If-only scope**: Loops/breaks/continues rejected
- **No value comparison**: Only contracts compared (exits/writes)
- **JoinModule emission**: Phase 122+ で段階的に追加（Phase 121 自体は parity の足場）

## Files Modified

**New files** (6):
- `src/mir/control_tree/normalized_shadow/mod.rs`
- `src/mir/control_tree/normalized_shadow/contracts.rs`
- `src/mir/control_tree/normalized_shadow/builder.rs`
- `src/mir/control_tree/normalized_shadow/parity_contract.rs`
- `tools/smokes/v2/profiles/integration/apps/phase121_shadow_if_only_vm.sh`
- `tools/smokes/v2/profiles/integration/apps/phase121_shadow_if_only_llvm_exe.sh`

**Modified files** (2):
- `src/mir/control_tree/mod.rs` (normalized_shadow module export)
- `src/mir/builder/calls/lowering.rs` (dev-only wiring)
- `docs/development/current/main/design/control-tree.md` (Phase 121 design)

## Commits

1. `8d930d2dc` - docs: Phase 121 StepTree→Normalized shadow plan
2. `1e5432f61` - feat(control_tree): add StepTree→Normalized shadow lowerer (if-only, dev-only)
3. `89b868703` - feat(joinir/dev): wire Phase 121 StepTree shadow lowering (strict fail-fast)
4. `0892df6df` - test(joinir): Phase 121 shadow parity smokes (VM + LLVM EXE)

**Total**: +793 lines, 4 commits, 8 files
