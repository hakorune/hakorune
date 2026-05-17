# 293x-613 MIMAP-114A Post-Local-Free-Scalar-Closeout Row Selection

Status: landed
Date: 2026-05-18

## Decision

`MIMAP-114A` is the planning row selected by `MIMAP-113A`.

The segment allocation modeled lane now has a closeout guard for:

```text
released-span ledger
  -> local-free candidate ledger
  -> local-free apply-plan ledger
```

This row should select exactly one next allocator behavior, closeout,
substrate, or narrow Hakorune acceptance row using the mimalloc validation
cadence.

## Result

`MIMAP-114A` selects:

```text
MIMAP-115A segment allocation modeled local-free page-model apply route
```

Validation cadence:

```text
L2 proof row:
  dedicated proof app via run_proof_app.sh --only MIMAP-115A
  dedicated public guard

L3 compatibility:
  run MIMAP-008 page/free-list pilot guard if the implementation changes
  HakoAllocPageModel.releaseLocal behavior
```

The selected row should consume successful `MIMAP-111A` apply-plan reports and
apply the scalar block span to an explicitly supplied `HakoAllocPageModel` by
calling `HakoAllocPageModel.releaseLocal(block_id)` for each block in the span.
It may open page-local free-list mutation only through that existing page model
contract. It must not execute real segment free, use raw pointers, perform
segment-map lookup, execute atomics, call OSVM/page-source seams, or activate
providers.

## Scope

- Review `MIMAP-113A` evidence and the scalar local-free closeout.
- Apply
  `docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md`.
- Select exactly one next row.

## Stop Lines

- No new `.hako` behavior.
- No cleanup bundle.
- No real segment allocation/free execution.
- No free-list mutation.
- No page state mutation.
- No arena backing allocation.
- No raw pointer residence.
- No segment-map pointer membership or lookup.
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
| `114A.1` | Review current evidence after MIMAP-113A. | selected row cites validation level. | no behavior |
| `114A.2` | Pick exactly one next row. | new selected card exists. | no bundle |
| `114A.3` | Update current pointers. | current pointer guard passes. | no implementation |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
