---
Status: Ready
Scope: return-in-loop minimal (stdlib `StringUtils.is_integer`)
Related:
- docs/development/current/main/phases/phase-29ar/README.md
- docs/development/current/main/design/return-in-loop-minimal-ssot.md
- docs/development/current/main/design/coreplan-flowbox-interface-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29ar P1: ExitIfReturn minimal

Goal: accept a single return-in-loop shape (`StringUtils.is_integer`) by adding
the minimal CorePlan vocabulary `ExitIfReturn { cond, value }`, without widening
general loop semantics.

## Non-goals

- No Break/Continue/Unwind generalization (Return-only).
- No nested return-in-loop.
- No new env vars; keep default behavior/logs unchanged.

## Implementation steps (critical order)

### Step 1: Add minimal vocabulary (CorePlan)

- Add `ExitIfReturn { cond, value }` as an effect-only op (Return-only).
- It must not allow arbitrary branching (no goto/label semantics).

### Step 2: Verifier (strict/dev)

Fail-fast rules (strict/dev only):
- `ExitIfReturn` is only allowed inside loop bodies (effect-only contexts).
- `value` is mandatory and must match “exactly one return value”.

### Step 3: Lowering / Emit

- Lower to `if cond { return value }` using existing CorePlan/Frag emission.
- Do not re-parse CFG/AST during emit; do not bypass `block_params` / join rules.

### Step 4: Facts + Composer adopt (strict/dev only)

- Add a conservative subset extractor for `StringUtils.is_integer` only.
- In strict/dev, adopt the CorePlan path when the subset matches.
- If it does not match, keep `Ok(None)` (or strict/dev fail-fast only for gated contradictions).

### Step 5: Fixture + smoke (SSOT)

- Fixture: `apps/tests/phase29ar_string_is_integer_min.hako`
- Smoke: `tools/smokes/v2/profiles/integration/joinir/phase29ar_string_is_integer_min_vm.sh`
- Wire the smoke into `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`.

The smoke must:
- Require a strict/dev-only adopt tag (stable tag name).
- Assert the freeze tag is absent.

## Verification (acceptance)

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

