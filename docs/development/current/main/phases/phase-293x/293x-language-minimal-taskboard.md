# Phase 293x Language Minimal Surface Taskboard

Status: Active
Lane: `phase-293x language minimal surface lane`
Current blocker token: `MIMAP-001 upstream source pin`

## Purpose

Keep the language-minimal implementation rows small enough for the
`1 blocker = 1 semantic slice` rule.

Canonical backlog SSOT:

```text
docs/development/current/main/design/language-minimal-surface-task-breakdown-ssot.md
```

## Completed recent rows

- [x] `LOOP-003A` Stage1 LoopRange route decision
- [x] `LOOP-003B` JSON v0 LoopRange lowering pilot
- [x] `LOOP-003C` LoopRange function metadata facts
- [x] `LOOP-003D` LoopRange carrier policy
- [x] `PACKED-003` source PackedArray direct-read consumption
- [x] `PACKED-001` PackedArray declaration eligibility gate
- [x] `ENUMVAR-001` enum variant canonical `Type::Variant` surface
- [x] `LOCALTYPE-001` local type annotation metadata capsule
- [x] `ARRAY-001` typed-context `Array<T>` literal lowering
- [x] `RESULT-001` Result/Option prelude diagnostics
- [x] `ARRAY-002A` typed `Array<T>` method contract
- [x] `ARRAY-002B` typed local Array element checks
- [x] `ARRAY-002C` unsupported Array inference fail-fast
- [x] `ARRAY-002D` ArrayBox JSON v0/backend guard
- [x] `RESULT-002A` prelude enum missing-arm diagnostics
- [x] `RESULT-002B` prelude enum payload diagnostics
- [x] `RESULT-002C` known-enum exhaustiveness underscore rules
- [x] `RESULT-002D` generic enum expected-type diagnostics
- [x] `GUARDLET-001` guard-let pattern sugar after Result/Option diagnostics
- [x] `PACKED-002` PackedArray non-escaping auto-use pilot

## Current split rows

- [x] `PACKED-004` source PackedArray backend fail-fast hardening
- [x] `USES-002A` declared uses capability plan mapping

## Blocked / deferred rows

- [ ] `LOOP-004` canonical loop formatter/docs

## Stop lines

```text
no source-level range-loop desugar
no broad Array<T> inference outside explicit inference rows
no PackedArray<T> fallback to ArrayBox
no try / throw / ? family
no guard-let before RESULT-002 diagnostics
```

## LOOP-003 split update (2026-05-14)

- `LOOP-003A`: landed Stage1 route decision and explicit JSON bridge fail-fast receiver.
- `LOOP-003B`: next Stage1 LoopRange lowering pilot.
- `LOOP-003C`: landed function-level loop_range_facts metadata.

## LOOP-003B update (2026-05-14)

- `LOOP-003B`: landed JSON v0 LoopRange lowering pilot.
- `LOOP-003C`: landed function-level loop_range_facts metadata.
- `LOOP-003D`: landed carrier policy; fresh body-local writes are allowed while loop-carried writes remain fail-fast.
- `PACKED-003`: landed source PackedArray direct-read consumption metadata.
- `PACKED-004`: landed backend fail-fast hardening.
- `MIMAP-001`: active next upstream source pin.

## Remaining implementation rows estimate (2026-05-14)

### Must finish before returning to mimalloc migration

| Row | Purpose | Expected size |
| --- | --- | --- |
| `PACKED-004` | Complete as `293x-330`; source direct-read consumption plans are included in the backend fail-fast gate. | complete |

After this set, switch to the mimalloc blueprint board:

```text
docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md
```

Blueprint-only rows `MIMAP-001` through `MIMAP-004` may start earlier if they do
not change source/runtime behavior.

### Should finish before selfhost migration

| Row | Purpose | Expected size |
| --- | --- | --- |
| `TYPE-002` | Stage1 alias diagnostics without making aliases nominal. | 1 commit |
| `CONTRACT-003A` | Runtime `assert` lowering decision and minimal insertion. | 1-2 commits |
| `CONTRACT-003B` | `requires` entry checks and stable diagnostics. | 1-2 commits |
| `TRANS-002A` | Transition legality checker for enum-state methods. | 1-2 commits |
| `USES-002A` | Capability checker for declared `uses` metadata. | 1-2 commits |
| `DEL-004` | Legacy `from`/`extends` quarantine and migration naming cleanup. | 1 commit |

### Later / optional language surface

| Row | Purpose | Expected size |
| --- | --- | --- |
| `DEL-005+` | Interface MVP and delegate-implements integration. | 3-5 commits |
| `SPAN-001` / `VIEW-001` | Span API first; view syntax only if API is insufficient. | design + 3-6 commits |
| `MOD-001+` | using/module migration and visibility semantics. | design + 3-6 commits |
| `CHECK-001` | check report object. | design + 2-4 commits |
| `CONST-001+` | const evaluator / const assert. | design + 4-8 commits |
