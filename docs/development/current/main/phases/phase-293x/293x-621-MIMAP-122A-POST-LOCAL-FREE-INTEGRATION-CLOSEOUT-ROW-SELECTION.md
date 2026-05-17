# 293x-621 MIMAP-122A Post-Local-Free-Integration-Closeout Row Selection

Status: landed
Date: 2026-05-18

## Decision

`MIMAP-122A` is the planning row selected by `MIMAP-121A`.

The segment allocation modeled local-free lane is now closed through the
integration route and manifest-backed closeout:

```text
released-span report
  -> local-free candidate ledger
  -> local-free apply-plan ledger
  -> explicit page-model apply
```

This row should select exactly one next allocator behavior, closeout,
substrate, or narrow Hakorune acceptance row using the mimalloc validation
cadence.

Selected row:

```text
PURE-FIRST-GLOBAL-CALL-001
  same-module static helper global-call route support
```

Rationale:

```text
MIMAP-119A exposed a pure-first EXE acceptance gap:
ordinary same-module static helper calls can emit MIR, but route preflight
can reject them as unsupported global_call_routes.

The fix belongs in the compiler route metadata / lowering plan layer, not in
allocator proof-app source workarounds.
```

## Scope

- Review `MIMAP-121A` evidence and the local-free integration closeout.
- Apply
  `docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md`.
- Select exactly one next row.

## Stop Lines

- No new `.hako` behavior.
- No cleanup bundle.
- No raw pointer residence.
- No segment-map pointer membership or lookup.
- No arena backing allocation.
- No atomic bitmap execution.
- No page-source or OSVM execution.
- No real thread scheduling.
- No worker spawning.
- No source-level concurrency feature change.
- No provider activation, hook, host allocator replacement, or
  `#[global_allocator]`.
- No backend `.inc` app/name matcher.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `122A.1` | Review current evidence after MIMAP-121A. | selected row cites validation level. | no behavior |
| `122A.2` | Pick exactly one next row. | new selected card exists. | no bundle |
| `122A.3` | Update current pointers. | current pointer guard passes. | no implementation |

## Landed Result

`MIMAP-122A` selected `PURE-FIRST-GLOBAL-CALL-001`.

The selected row is intentionally narrow:

- same-module static helper global calls only
- accepted only when the target exists, arity matches, body shape is supported,
  and return contract is publishable
- no allocator behavior
- no helper-name or app-name backend matcher
- no broad dynamic dispatch or cross-module global-call widening

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
