---
Status: design_stop__2026-09-21__SelectedBoundedWarningCohort__ReadyForFastTransition
Task: MIRBUILDER-WARNING-STATIC-METHOD-CROW-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i67-2026-09-21.md
Implementation permission: false until pointer transition commit
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I68
---

# MirBuilder static method C-row facade I0

## Six-line brief

```text
Decision: remove one unused parent facade re-export after the I67 refresh.
Source authority + canonical issuer: the compiler published-backend-view owner.
Non-authority: the historical mir/function.rs facade and warning-count guesses.
Fail-fast boundary: any owner-local warning move, unresolved import, or changed
  C-frame consumer stops the slice.
Smallest next slice: delete only PublishedStaticMethodCallCRowV1 from the
  parent mir/function.rs re-export, then run the two fixed quick gates.
Non-claims: no C-frame change, no ABI change, no dead-code cleanup, no route or
  production switch, and no broad warning census.
```

## Evidence and acceptance

I67 refreshed the fixed baseline at lib **1,754** / lib-test **558**. A complete
`rg` census shows `PublishedStaticMethodCallCRowV1` is defined and consumed by
the compiler published-backend-view owner; `src/mir/function.rs` is the sole
unused facade occurrence. The child historical module imports the compiler
owner directly, so the deletion has no owner-local handoff.

The fast slice must run these commands sequentially:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Success requires both commands to pass, the C-frame and C-row source census to
remain unchanged except for the removed facade item, and the warning count to
drop without a replacement warning. Any warning movement is a same-chain
correction or a stop; it is not silently accepted.
