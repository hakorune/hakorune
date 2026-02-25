# Phase 283: JoinIR If-Condition ValueId Remap Fix

Status: **P0 ✅ complete (2025-12-23)**

Goal:
- Fix a Pattern3 (if-sum) JoinIR lowering bug where complex if-conditions like `i % 2 == 1` could produce MIR that uses an undefined `ValueId`.

SSOT References:
- JoinIR lowering (if-sum): `src/mir/join_ir/lowering/loop_with_if_phi_if_sum.rs`
- Condition value lowering: `src/mir/join_ir/lowering/condition_lowerer.rs`
- MIR verification (undefined values): `src/mir/verification/ssa.rs`
- Test fixture: `apps/tests/loop_if_phi.hako`

## Problem

Observed error (VM):
- `use of undefined value ValueId(...)` inside the loop body, originating from the modulo expression in the if-condition.

The symptom appeared in `apps/tests/loop_if_phi.hako`:
```hako
if (i % 2 == 1) { ... } else { ... }
```

## Root Cause

In `lower_if_sum_pattern`, the if-condition was extracted (and its JoinIR value-expression lowered) **before** `i_param` / `sum_param` existed, so `Variable("i")` inside complex expressions resolved through the caller-provided `ConditionEnv`.

That early `ConditionEnv` mapping could not be remapped by the boundary/merger, resulting in JoinIR ValueIds that became undefined host ValueIds in MIR.

## Fix (P0)

- Move `extract_if_condition(...)` so it runs **after** `local_cond_env` is constructed and shadowed:
  - `local_cond_env.insert(loop_var, i_param)`
  - `local_cond_env.insert(update_var, sum_param)`
  - (and any condition bindings remapped to `main_params`)

This ensures complex expressions like `i % 2` resolve `i` to `i_param` (a loop_step param that the merger remaps), preventing undefined ValueIds.

## Fixture Note (Coercion SSOT)

After Phase 275 (C2), implicit `"String" + Integer` is a TypeError. The fixture uses:
- `sum.toString()` instead of `"sum=" + sum` or `str(sum)`.

## Acceptance

- `apps/tests/loop_if_phi.hako` runs on VM without undefined ValueId errors and prints `sum=9`.

Recommended smoke checks:
- VM: `tools/smokes/v2/profiles/integration/apps/phase283_p0_loop_if_phi_vm.sh`
  - Checks stdout output: `sum=9`
  - Exit code: 0
- LLVM harness: `tools/smokes/v2/profiles/integration/apps/phase283_p0_loop_if_phi_llvm.sh`
  - **Note**: LLVM harness (`NYASH_LLVM_USE_HARNESS=1`) suppresses program stdout
  - Only check: `Result: 0` line in stderr (exit code = 0)
  - Direct execution: `NYASH_LLVM_USE_HARNESS=1 ./target/release/hakorune --backend llvm apps/tests/loop_if_phi.hako`
