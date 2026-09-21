---
Status: closed__2026-09-21__WarningAPrimeFacade
Task: MIRBUILDER-WARNING-A-PRIME-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i50-2026-09-21.md
Implementation permission: true for the two cfg(test) A-prime facade exports only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I51
---

# Warning cleanup: A-prime physical-demand facade

## Six-line brief

```text
Decision: gate the selected A-prime physical-demand function and reject-type
  re-exports because their observed consumers are test-only package fixtures.
Source authority + canonical issuer: a_prime_i64_physical_capability issuer and
  model child modules; mod.rs names are forwarding facades.
Non-authority: cargo-fix, wildcard imports, visibility changes, A-prime issuer
  redesign, physical demand semantics, or warning guesses.
Fail-fast boundary: any production facade consumer, compile error, changed test
  warning, or warning-count mismatch rejects the slice.
Smallest next slice: add cfg(test) to mod.rs lines 10 and 13 and run fixed gates.
Non-claims: no physical-demand behavior change, suppression, or cutover.
```

## Preconditions and acceptance

I50 records two lib-only unused-import warning groups in
`src/mir/compiler/a_prime_i64_physical_capability/mod.rs:10,13`. Direct source
census found the selected function and reject type are consumed only by
normal-callable package tests through this facade; production uses the
`_from_parts` issuer and child model paths.

Acceptance requires lib warnings to drop from **1,775 to 1,773**, lib-test
warnings to remain **561**, and both fixed commands to exit 0:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Only the two selected `#[cfg(test)]` facade annotations may change.

## Closeout evidence

The permitted source edits gated the A-prime physical-demand function and
reject-type facades under `#[cfg(test)]`. The fixed gates completed sequentially
with exit 0:

* lib: **1,773 warnings** — `/tmp/hakorune-warning-i0-a-prime-facade-lib-20260921.log`
* lib test: **561 warnings** — `/tmp/hakorune-warning-i0-a-prime-facade-lib-test-20260921.log`

No A-prime issuer, model, physical-demand, or production emitter behavior
changed.
