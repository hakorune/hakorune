---
Status: closed__2026-09-21__WarningDirectStaticPhysicalInputRowTestFacade
Task: MIRBUILDER-WARNING-DIRECT-STATIC-PHYSICAL-INPUT-ROW-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i26-2026-09-21.md
Implementation permission: true for one cfg(test) direct-static physical-input row re-export only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I27
---

# Warning cleanup: direct-static physical-input row test facade

## Six-line brief

```text
Decision: gate the direct-static physical-input row re-export consumed only by
  the script_physical_exit test fixture; keep physical_input.rs as owner.
Source authority + canonical issuer: physical_input.rs; cfg(test) name
  resolution is the only moved boundary.
Non-authority: cargo-fix, wildcard imports, physical-input semantics, visibility
  redesign, production route changes, or warning guesses.
Fail-fast boundary: any production consumer, compile error, changed test
  warning, or warning-count mismatch rejects the slice.
Smallest next slice: split the grouped re-export, gate only the row type, then
  run the fixed quick-profile gates once each.
Non-claims: no physical-input redesign, suppression, or direct-static cutover.
```

## Preconditions and acceptance

I26 records one lib-only unused-import diagnostic at
`src/mir/builder/normal_script_direct_static_join_handoff.rs:294`.
`VerifiedScriptDirectStaticPhysicalInputRowV1` is used by the
`script_physical_exit` test fixture; production code uses only
`VerifiedScriptDirectStaticPhysicalInputV1`.

Acceptance requires lib warnings to drop from **1,805 to 1,804**, lib-test
warnings to remain **561**, and both fixed commands to exit 0:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Only the re-export declaration may change. If the consumer classification or
warning count differs, stop and return to design stop.

## Execution evidence

The row re-export is now gated with `#[cfg(test)]`; the aggregate physical
input export remains unchanged. The fixed sequential gates exited 0:

* `cargo check --profile quick --lib -j4`: lib warnings **1,804**.
* `cargo test --profile quick --lib --no-run -j4`: lib-test warnings **561**;
  the test executable was produced.

The physical-input owner, direct-static lowering, and semantic route were not
changed.
