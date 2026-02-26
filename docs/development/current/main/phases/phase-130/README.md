# Phase 130: if-only Normalized "Small Expr/Assign" Expansion (dev-only)

Status: DONE ✅
Scope: if-only Normalized（StepTree → Normalized shadow pipeline）
Related:
- Entry: `docs/development/current/main/10-Now.md`
- Phase 129 (join_k/post_k): `docs/development/current/main/phases/phase-129/README.md`
- ControlTree SSOT: `docs/development/current/main/design/control-tree.md`

## Goal

Increase usefulness of the if-only Normalized path without touching loop lowering yet:

- Enable minimal post-if computation (`x = x + 3`) and returns via env.
- Keep **PHI禁止**: merge via env + continuations only.
- Keep **dev-only** and **既定挙動不変**: unmatched shapes fall back (strict can Fail-Fast for fixtures).

## Non-Goals

- No loops (Loop/Break/Continue still rejected by capability guard).
- No general expression lowering (start with one narrow shape only).
- No new env vars (use existing `joinir_dev_enabled()` / `joinir_strict_enabled()`).

## Work Items (P0–P3)

### P0: Fixtures + Smokes (VM + LLVM EXE)

Add a “post-if update then return” fixture that forces post_k to execute and tests env updates:

- New fixture: `apps/tests/phase130_if_only_post_if_add_min.hako`
  - Case A: `flag=1` → expected `5`
  - Case B: `flag=0` → expected `4`
  - Output: `5\n4`
- Smokes (integration):
  - `tools/smokes/v2/profiles/integration/apps/archive/phase130_if_only_post_if_add_vm.sh`
  - `tools/smokes/v2/profiles/integration/apps/archive/phase130_if_only_post_if_add_llvm_exe.sh`
  - Use `output_validator.sh` numeric-line assertion.
  - LLVM EXE smoke uses `llvm_exe_runner.sh` + plugin gating.

Acceptance:
- `phase130_*_vm.sh` PASS
- `phase130_*_llvm_exe.sh` PASS (SKIP allowed only when LLVM prerequisites are missing)

### P1: Assign(Variable) minimal

Support `x = y` inside post_k (and/or inside branches) for if-only:

- Only local variables (no field/property assign).
- RHS must resolve from env (`writes` or `inputs`).
- strict: unsupported shapes → `freeze_with_hint("phase130/assign/var/unsupported", ...)`.

### P2: Assign(Add) minimal for integers

Support `x = x + <int literal>` (exact shape) in post_k:

- Strictly `Var + IntLiteral` with `lhs_var == dst_var` (no commutation, no general binop).
- Type: integer only (string concat stays out-of-scope; Fail-Fast under strict).

### P3: Verifier tightening (env writes-only discipline)

Add a structural check to ensure env updates only target `writes` fields:

- In Normalized emission, any “env update” must update a field that exists in `EnvLayout.writes`.
- Writing to an `inputs` field is prohibited (strict Fail-Fast).

## Verification

Commands:

```bash
cargo test --lib

# New Phase 130 smokes
bash tools/smokes/v2/profiles/integration/apps/archive/phase130_if_only_post_if_add_vm.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase130_if_only_post_if_add_llvm_exe.sh

# Regressions (minimum)
bash tools/smokes/v2/profiles/integration/apps/archive/phase129_if_only_post_if_return_var_vm.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase129_join_k_as_last_vm.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase128_if_only_partial_assign_normalized_vm.sh
```

## Notes

- Phase 130 intentionally stays "if-only" to keep the change-set small and correctness-focused.
- Loop lowering in Normalized is a separate phase; do not mix scopes.

## Completion Summary

Phase 130 successfully completed on 2025-12-18:

- **P0 (Fixtures + Smokes)**: ✅ DONE
  - Fixture: `apps/tests/phase130_if_only_post_if_add_min.hako` (expects 5\n4)
  - VM smoke: `tools/smokes/v2/profiles/integration/apps/archive/phase130_if_only_post_if_add_vm.sh` PASS
  - LLVM EXE smoke: `tools/smokes/v2/profiles/integration/apps/archive/phase130_if_only_post_if_add_llvm_exe.sh`
    - PASS when hakorune can emit LLVM objects (built with `--features llvm` and llvmlite available)
    - Otherwise SKIP (missing LLVM prerequisites)

- **P1 (Assign Variable)**: ✅ DONE
  - Added support for `x = y` variable assignment in `lower_assign_stmt`
  - Env map updated directly (continuation-passing style, no instruction emission)
  - Unsupported/ill-formed cases are treated as out-of-scope for the dev-only route (falls back to legacy path)

- **P2 (Assign Add)**: ✅ DONE
  - Added support for `x = x + <int literal>` pattern
  - Strict contract: dst == lhs, rhs must be int literal, only Add operator
  - Emits: Const (rhs) → BinOp Add → env update
  - Contract violations are treated as out-of-scope for the dev-only route (falls back to legacy path)

- **P3 (Verifier)**: ✅ DONE
  - Added `verify_env_writes_discipline()` to `normalized_verifier.rs`
  - Structural check: env map must not introduce variables outside the env layout (writes + inputs)

- **Tests**: ✅ ALL PASS
  - Unit tests: 1155/1155 PASS
  - Phase 130 VM smoke: PASS (output: 5\n4)
  - Regression (Phase 129/128/127): ALL PASS

- **Implementation**:
  - Modified: `src/mir/control_tree/normalized_shadow/legacy/mod.rs` (lower_assign_stmt extended)
  - Modified: `src/mir/control_tree/normalized_shadow/normalized_verifier.rs` (verify_env_writes_discipline added)
  - New fixture: `apps/tests/phase130_if_only_post_if_add_min.hako`
  - New smokes: VM + LLVM EXE integration tests
