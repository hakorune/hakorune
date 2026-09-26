# MIRBUILDER-EXE-ACCEPTANCE-DECLARED-INSTANCE-LOCATOR-D15

Status: design_stop__2026-09-26
Date: 2026-09-26
Parent: workstream row H (unified resume, gate 1);
  COND-SUBSTRING-COVERAGE-S2 landed
Mode: design_stop — census and Decision only.

## Blocking observation

```text
EXE (pure-first, debug binary target/debug/hakorune):
  [freeze:contract][mir/callable-semantic-package/port]
  DeclaredInstanceLocatorNotConsumed
  (package complete(): selected coverage + ordinary-new
  coverage already consumed; declared-instance call
  locator rows remain)

VM (debug binary): unchanged known terminal —
  loop winner selection declined: zero selected family
  candidates fn=JsonStreamAggregator.ingest/1 [Body(2)]
  (ledger-less instance method → route_loop spine;
  separate authority, recorded non-claim).
```

`trim/1`'s condition-position `s.substring` coverage is
now armed (S2); every selected callable lowers through
the package port and `complete()` reaches the
declared-instance locator count check. Some locator rows
were never consumed during lowering.

## Census questions

- Which owner issues `issue_declared_instance_call_package_locator_v1`
  rows (issuer.rs:710) — which call sites they cover
  (the `JsonStreamAggregator` instance-method call sites
  in `main`, `ingest/1` callee side, or both).
- Which owner consumes them: `lowering_port.rs:420-428`
  marks `declared_instance_consumed` inside the cataloged
  child lowering callback — determine which rows that
  path sees and which rows remain.
- Whether the unconsumed rows are a bounded gap on the
  recorded instance-method lane (ledger-less VM terminal
  family, already a non-claim) or a new missing consumer
  on the cataloged path.

## Boundary of this census

- Covers: `DeclaredInstanceLocatorNotConsumed` at package
  `complete()` on the json_stream_aggregator EXE lane,
  from locator issue to consumption marking.
- Excludes: VM ledger-less lane (`ingest` — recorded
  non-claim); `ConditionalUpdateIf` F2 parts boundary;
  static publication gaps (binary_trees / mimalloc).

## Exit

- [ ] Unconsumed locator rows identified by call site and
  intended consumer.
- [ ] Existing owner vs unclaimed gap classified.
- [ ] One bounded S-card emitted, or NoSafeSlice with the
  missing authority named.
