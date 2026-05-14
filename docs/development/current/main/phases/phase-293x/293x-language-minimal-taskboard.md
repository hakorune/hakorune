# Phase 293x Language Minimal Surface Taskboard

Status: Active
Lane: `phase-293x language minimal surface lane`
Current blocker token: `LOOP-003 Stage1 LoopRange JoinIR/CorePlan route decision`

## Purpose

Keep the language-minimal implementation rows small enough for the
`1 blocker = 1 semantic slice` rule.

Canonical backlog SSOT:

```text
docs/development/current/main/design/language-minimal-surface-task-breakdown-ssot.md
```

## Completed recent rows

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

- [ ] `LOOP-003` Stage1 LoopRange lowering, blocked on JoinIR/CorePlan route decision

## Blocked / deferred rows

- [ ] `LOOP-004` LoopRange verifier facts

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
- `LOOP-003C`: later verifier facts and read-only index enforcement.

## LOOP-003B update (2026-05-14)

- `LOOP-003B`: landed JSON v0 LoopRange lowering pilot.
- `LOOP-003C`: next verifier facts, read-only index proof surface, and carrier policy.
