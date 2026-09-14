---
Status: closed__bounded_implementation__2026-09-14
Task: MIR-CALL-RAW-LOOP-COMPATIBILITY-SCOPE-SELECTION-I0
Date: 2026-09-14
Priority: recover the compatibility root terminal required by the parent StringBox I0
Parent: mir-call-raw-loop-compatibility-scope-selection-d0-2026-09-14.md
NextCard: MIR-CALL-R7-STRINGBOX-LOWER-STRUCTURAL-MEMBERSHIP-I0
Implementation permission: true for the existing root port choice and focused route tests
---

# Raw Loop compatibility scope selection I0

## Six-line brief

Decision: construct the raw invocation child port without callable-loop scope
for the compatibility Program(JSON v0) root; retain scope and ledger for the
source-backed selected root.

Source authority + canonical issuer: `normal_default_root_catalog_lifecycle.rs`
owns the source-backed/compatibility disposition, while the existing module
invocation session issues the root scope and the installed package issues the
callable ledger.

Non-authority: Hako lowerer membership, environment toggles, planner strictness,
fallback/retry, JSON/MIR scans, and fabricated compatibility ledgers.

Fail-fast boundary: `raw_loop_child_port` continues to reject an explicitly
armed scope whose callable ledger is absent with the named
`callable-ledger-missing` terminal.

Smallest next slice: branch only the existing port constructor in
`program_root_lowering.rs`, then prove compatibility, source-backed, and
missing-ledger behavior with route-level tests.

Non-claims: no Generic G0 migration, loop recipe change, Hako owner change,
fallback deletion, backend parity, source-artifact transport, or whole-R7
completion.

Census boundary: `module_invocation_session` -> `normal_default_program_root`
-> `program_root_lowering` -> `raw_loop_child_port`; includes only root port
capability selection and its existing terminal tests; excludes loop semantic
lowering after the port is selected.

## Ordered tasks

1. Confirm the existing `Installed` versus `Compatibility` branch and preserve
   the source-backed scope+ledger route.
2. Use the no-scope constructor only for compatibility lowering; do not change
   the scope issuer or the raw Loop owner.
3. Add compatibility Loop success, source-backed Loop route, and explicit
   missing-ledger fail-fast tests. Assert no partial MIR on the fail-fast row.
4. Update the owning Builder README/reference and the parent recovery D0 with
   the implementation evidence, then run focused tests and classify red.

## Acceptance

- A compatibility Program(JSON v0) Loop no longer fails solely because the
  invocation session supplied a scope without a ledger.
- A source-backed selected Loop still enters the invocation-aware route with
  both scope and ledger.
- An explicitly missing ledger retains the named fail-fast terminal and does
  not publish partial MIR.
- The parent StringBox phase14/16/17 probes can reach their selected owner;
  their membership evidence remains owned by the parent I0 card.

## Implementation evidence (2026-09-14)

The existing constructor choice in `program_root_lowering.rs` now branches on
the already-issued root disposition: `Installed` keeps the callable-loop scope,
direct-call loan, and ledger route; compatibility uses the legacy child
constructor without a scope. No scope issuer or Loop owner was changed.

Focused exact tests, run from the single rebuilt `target/quick` test binary,
all passed:

- `compatibility_loop_uses_legacy_child_terminal_without_callable_scope`
  reached the existing compatibility terminal.
- `source_backed_loop_keeps_invocation_scope_and_ledger_route` reached the
  existing `[callable-loop/recipe]` boundary and did not report
  `callable-ledger-missing`.
- `armed_scope_without_ledger_fails_before_legacy_loop_effects` retained the
  named fail-fast and observed no new MIR block.

The source-backed fixture's recipe-boundary rejection is a pre-existing
downstream baseline; this recovery slice claims capability selection only.
The Hako StringBox owner and its positive/negative membership evidence remain
owned by the parent I0.
