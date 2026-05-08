---
Status: Landed
Date: 2026-05-08
Lane: phase-293x real-app bringup
Card: 293x-039-M10A-EXPORT-ATTRS-CONSISTENCY-GATE
Scope: M10a LLVM/runtime-decl export attrs consistency guard
---

# 293x-039 M10a Export Attrs Consistency Gate

## Decision

M10 strong LLVM export attrs are not implemented yet.

This card lands the structural gate that must precede them: active export
points are locked to the current weak attr vocabulary until verifier-owned
proof exists.

Current accepted attrs:

```text
llvm_py:
  readonly
  nocapture

.hako runtime-decl manifest:
  nounwind
  readonly
  willreturn
```

## Responsibility

- `src/llvm_py/instructions/llvm_attrs.py` owns the compat/probe keep
  builder-finalization attr policy.
- `docs/development/current/main/design/runtime-decl-manifest-v0.toml` owns
  backend-private `.hako ll emitter` runtime declare attrs.
- `lang/src/shared/backend/ll_emit/generated/runtime_decl_defaults.hako` is
  generated from the manifest and must not drift.
- `tools/checks/k2_wide_export_attrs_consistency_guard.sh` owns the M10a
  fail-fast lock.

## Non-Goals

- No `noalias` emission.
- No `nonnull` emission.
- No `dereferenceable` emission.
- No backend alignment export.
- No stronger `nocapture` inference.
- No `readnone` promotion.
- No MIR contract fact export to backend optimization.

## Acceptance

- `llvm_py` attr policy only wires `readonly` and `nocapture`.
- runtime-decl manifest attrs only use `nounwind`, `readonly`, and
  `willreturn`.
- generated runtime-decl defaults do not contain stronger attrs.
- taskboard and optimization SSOTs identify the guard as live while keeping
  strong attrs blocked.

## Gates

```bash
bash tools/checks/k2_wide_export_attrs_consistency_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
