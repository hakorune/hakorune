---
Status: closed__2026-09-21__WarningBoundedBodySnapshotTestFacade
Task: MIRBUILDER-WARNING-BOUNDED-BODY-SNAPSHOT-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i23-2026-09-21.md
Implementation permission: true for one cfg(test) bounded-body snapshot import only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I24
---

# Warning cleanup: bounded-body snapshot test facade

## Six-line brief

```text
Decision: gate the bounded-body strict-JSON import consumed only by local tests;
  keep strict_json_tree_v0.rs as the owner.
Source authority + canonical issuer: strict_json_tree_v0.rs; cfg(test) name
  resolution is the only moved boundary.
Non-authority: cargo-fix, wildcard imports, parser/schema semantics, visibility
  redesign, dead-code ownership, or warning guesses.
Fail-fast boundary: any production consumer, compile error, changed test
  warning, or warning-count mismatch rejects the slice.
Smallest next slice: add cfg(test) to the grouped strict-JSON import, then run
  the fixed gates once each.
Non-claims: no bounded-body parser or snapshot-schema redesign, suppression, or
  production route change.
```

## Precondition and acceptance

I23 records one lib-only grouped unused-import diagnostic at
`src/analysis/bounded_body_snapshot_v0/mod.rs:32`. The local strict-JSON tree
tests consume all three re-exported types; production code does not.
Acceptance requires lib warnings to drop from 1,808 to 1,807, lib-test warnings
to remain 561, both fixed quick-profile commands to exit 0, and fmt/diff/
pointer/lifecycle guards to remain green.

Only the import declaration may change. If the consumer classification or count
differs, stop and return to design stop.

## Execution evidence

The grouped strict-JSON import is now gated with `#[cfg(test)]`; the parser,
snapshot schema, and production analysis surface are unchanged. `cargo check
--profile quick --lib -j4` exited 0 with lib warnings at **1,807**. The
sequential `cargo test --profile quick --lib --no-run -j4` exited 0 with
lib-test warnings at **561** and produced the test executable. Fmt, diff,
pointer, and lifecycle guards remain required at closeout.
