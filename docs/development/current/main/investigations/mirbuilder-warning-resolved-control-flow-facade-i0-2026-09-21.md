---
Status: closed__2026-09-21__WarningResolvedControlFlowFacade
Task: MIRBUILDER-WARNING-RESOLVED-CONTROL-FLOW-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i48-2026-09-21.md
Implementation permission: true for the two cfg(test) resolved-control-flow facade re-exports and their dependent child re-export only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I49
---

# Warning cleanup: resolved-control-flow facade exports

## Six-line brief

```text
Decision: gate the two resolved-control-flow parent re-exports and the now-unused
  child re-export because their observed consumers are test-only plan-contract
  readers.
Source authority + canonical issuer: function_control child modules; the
  mod.rs names are forwarding facades, not completion issuers.
Non-authority: cargo-fix, wildcard imports, visibility changes, completion
  redesign, exit-contract semantics, or warning guesses.
Fail-fast boundary: any production facade consumer, compile error, changed
  test warning, or warning-count mismatch rejects the slice.
Smallest next slice: add cfg(test) to the two selected mod.rs names and the
  dependent function_control.rs child export, then run fixed gates.
Non-claims: no control-flow behavior change, suppression, or production cutover.
```

## Preconditions and acceptance

I48 records one lib-only unused-import warning group covering two names in
`src/mir/resolved_control_flow/mod.rs:23,25`. Gating the parent facade exposes
one dependent child-facade warning at `function_control.rs:423`; it is part of the
same forwarding chain. Direct source census found the
selected facade names are read only by the test-side Main plan contract helper;
production control-flow code uses the child module's concrete owner and the
argument-observation completion entry.

Acceptance requires lib warnings to drop from **1,776 to 1,775**, lib-test
warnings to remain **561**, and both fixed commands to exit 0:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Only these three `#[cfg(test)]` facade annotations may change: the two parent
exports and the dependent child export.

## Closeout evidence

The permitted source edits gated the two resolved-control-flow facade names and
the dependent `function_control.rs:423` child re-export under `#[cfg(test)]`.
The fixed gates completed sequentially with exit 0:

* lib: **1,775 warnings** — `/tmp/hakorune-warning-i0-resolved-control-flow-facade-lib-20260921.log`
* lib test: **561 warnings** — `/tmp/hakorune-warning-i0-resolved-control-flow-facade-lib-test-20260921.log`

No completion issuer, exit-contract owner, or production control-flow path
changed.
