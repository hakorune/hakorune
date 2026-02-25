# Phase 279 P0: Unify type propagation pipeline (SSOT)

Status: planned / implementation

Problem:
- Type propagation and PHI type resolution are executed in multiple routes with different orderings.
- This creates “double bugs”: the same fixture can pass in one route and fail in another, even when the frontend and MIR are identical.

Goal:
- Define a single **TypePropagationPipeline** (SSOT) with a fixed order, and make all routes call it.

Constraints:
- No new environment variables.
- No by-name hardcode dispatch.
- Fail-fast on invariants (no silent fallback).

---

## 1) Define the SSOT pipeline (single entry)

Create one entry function (name is flexible, keep it unambiguous):
- `run_type_propagation_pipeline(module: &mut MirModule, mode: ...) -> Result<(), String>`

Fixed order (SSOT):
1. Copy type propagation
2. BinOp type re-propagation
3. PHI type resolution
4. Minimal follow-ups required for downstream typing (Compare / TypeOp / etc.), only if already needed by current backends

Hard rule:
- No route may run PHI type resolution before BinOp re-propagation.

Rationale:
- PHI type inference depends on stabilized incoming value types.

---

## 2) Route integration (remove local ordering)

Make all relevant routes call the SSOT entry and remove/disable any local ordering logic.

Known routes to check (examples; confirm in code):
- Builder lifecycle path (emits MIR directly)
- JoinIR → MIR bridge path
- Any “analysis/json emit” path that downstream LLVM harness relies on for `value_types`

Acceptance:
- Each route calls the same SSOT entry.
- There is no remaining “partial pipeline” that can reorder steps.

---

## 3) Fail-fast order guard (prevent regressions)

Add an invariant checker that makes order drift obvious:
- If PHI type resolution is invoked while BinOp re-propagation has not run, return `Err(...)`.

This is not a feature toggle. It is a structural guard.

---

## 4) Backends: define the contract for `value_types`

Document (in code/doc) what the downstream expects:
- A `ValueId` that is `f64` must be consistently typed as `f64` across:
  - MIR instruction dst_type
  - propagated/inferred `value_types`
  - PHI dst_type
- “i64 handle” vs “unboxed f64” must be consistent for the LLVM harness.

Avoid “best-effort” inference at the harness layer. If the type is unknown, fail-fast where the SSOT contract is violated.

---

## 5) Minimal acceptance tests

Minimum:
- A representative fixture that exercises:
  - Copy chain
  - BinOp promotion (e.g. Int+Float)
  - PHI over that promoted value
  and is executed via all relevant routes.
- The MIR JSON `value_types` is consistent across routes.

Suggested validation commands (keep local, do not add CI jobs):
- `cargo build --release`
- relevant smoke(s): `tools/smokes/v2/run.sh --profile quick`

---

## 6) Completion criteria

- No route-specific ordering logic remains.
- Order guard prevents PHI-before-BinOp execution.
- A reproduction fixture cannot diverge across routes due to type propagation ordering.
- Documentation points to this phase as the SSOT for “pipeline unification”.
