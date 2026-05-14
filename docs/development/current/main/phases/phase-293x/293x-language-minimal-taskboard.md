# Phase 293x Language Minimal Surface Taskboard

Status: Active
Lane: `phase-293x language minimal surface lane`
Current blocker token: `ARRAY-002D ArrayBox JSON v0/backend guard`

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

## Current split rows

- [ ] `ARRAY-002D` ArrayBox JSON v0/backend guard
- [ ] `RESULT-002A` prelude enum missing-arm diagnostics
- [ ] `RESULT-002B` prelude enum payload diagnostics
- [ ] `RESULT-002C` known-enum exhaustiveness underscore rules
- [ ] `RESULT-002D` generic enum expected-type diagnostics

## Blocked / deferred rows

- [ ] `LOOP-003` Stage1 LoopRange lowering, blocked on JoinIR/CorePlan route decision
- [ ] `LOOP-004` LoopRange verifier facts
- [ ] `PACKED-002` PackedArray non-escaping auto-use pilot
- [ ] `GUARDLET-001` guard-let pattern sugar after Result/Option diagnostics

## Stop lines

```text
no source-level range-loop desugar
no broad Array<T> inference outside explicit inference rows
no PackedArray<T> fallback to ArrayBox
no try / throw / ? family
no guard-let before RESULT-002 diagnostics
```
