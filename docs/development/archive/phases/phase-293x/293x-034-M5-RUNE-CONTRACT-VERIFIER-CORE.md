---
Status: Landed
Date: 2026-05-08
Lane: phase-293x real-app bringup
Card: 293x-034-M5-RUNE-CONTRACT-VERIFIER-CORE
Scope: M5 rune contract verifier core
---

# 293x-034 M5 Rune Contract Verifier Core

## Decision

`@rune Contract(no_alloc)` and `@rune Contract(no_safepoint)` are no longer
pure comments once MIR verification runs.

The accepted first row is a MIR verifier core only:

```text
FunctionMetadata.runes
-> verification/rune_contracts.rs
-> VerificationError::RuneContractViolation
```

Backend optimization/export use remains disabled until a later row proves and
publishes a backend-facing fact.

## Responsibility

- Parser fronts own syntax and declaration-local `attrs.runes`.
- MirBuilder owns copying declaration-local runes to `FunctionMetadata.runes`.
- MIR verifier owns contract proof for this row.
- Backends must not infer or trust contract facts from names, routes, or helper
  choices.

## Live Checks

- `Contract(no_alloc)` rejects any instruction whose effect mask contains
  `Effect::Alloc`.
- `Contract(no_safepoint)` rejects explicit `MirInstruction::Safepoint`.

Stable fail-fast tags:

```text
[freeze:contract][rune/no_alloc]
[freeze:contract][rune/no_safepoint]
```

## Non-Goals

- No backend optimization/export use.
- No `Contract(pure)` / `Contract(readonly)` verifier yet.
- No new syntax.
- No parser change.
- No runtime helper change.
- No inferred contracts from method names or app-specific boxes.

## Acceptance

- `MirVerifier::verify_function` runs the rune contract verifier.
- `no_alloc` rejects `NewBox` and allocation-effect calls.
- `no_safepoint` rejects explicit `Safepoint`.
- Non-live contract metadata stays no-op.
- Reference docs state that backend use is still disabled.

## Gates

```bash
cargo test -q rune_contracts --lib
cargo fmt --check
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
