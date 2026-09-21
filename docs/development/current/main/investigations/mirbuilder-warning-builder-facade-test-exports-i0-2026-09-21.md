---
Status: closed__2026-09-21__WarningBuilderFacadeTestExports
Task: MIRBUILDER-WARNING-BUILDER-FACADE-TEST-EXPORTS-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i47-2026-09-21.md
Implementation permission: true for the two cfg(test) builder facade re-exports and their dependent child re-export only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I48
---

# Warning cleanup: builder facade test exports

## Six-line brief

```text
Decision: gate the two resolved-lowering re-exports at src/mir/builder.rs and
  their now-unreferenced child facade at resolved_lowering/mod.rs because the
  observed consumers are test-only facade users.
Source authority + canonical issuer: resolved_lowering child modules; the
  builder.rs names are forwarding facades, not semantic issuers.
Non-authority: cargo-fix, wildcard imports, visibility changes, issuer/session
  redesign, physical emitter behavior, or warning guesses.
Fail-fast boundary: any production facade consumer, compile error, changed
  test warning, or warning-count mismatch rejects the slice.
Smallest next slice: add cfg(test) to builder.rs lines 663 and 666 plus the
  dependent resolved_lowering/mod.rs child export, then run fixed gates.
Non-claims: no lowering behavior change, suppression, or production cutover.
```

## Preconditions and acceptance

I47 records two lib-only unused-import diagnostics in
`src/mir/builder.rs:663,666`. Gating the second parent facade exposes one
dependent child-facade warning at `resolved_lowering/mod.rs:69`; it is part of the
same forwarding chain. Direct source census found no production consumer
through either facade: the emitter and session owners use their
`resolved_lowering` child paths directly; only test modules use the builder
facades.

Acceptance requires lib warnings to drop from **1,778 to 1,776**, lib-test
warnings to remain **561**, and both fixed commands to exit 0:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Only these three `#[cfg(test)]` facade annotations may change: the two parent
exports and the dependent child export.

## Closeout evidence

The permitted source edits gated the two builder facade re-exports and the
dependent `resolved_lowering/mod.rs:69` child facade under `#[cfg(test)]`.
The fixed gates completed sequentially with exit 0:

* lib: **1,776 warnings** — `/tmp/hakorune-warning-i0-builder-facade-test-exports-lib-20260921.log`
* lib test: **561 warnings** — `/tmp/hakorune-warning-i0-builder-facade-test-exports-lib-test-20260921.log`

No resolved-lowering issuer, canonical session owner, physical emitter, or
production lowering path changed.
