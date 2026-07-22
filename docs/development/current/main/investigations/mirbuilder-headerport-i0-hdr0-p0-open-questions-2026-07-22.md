# HDR0-P0 Open Design Questions

Status: **Closed — Q1 through Q4 accepted**
Date: 2026-07-22
Scope: accepted HeaderPort replacement, parity, and cutover policy decisions

Related:

- `docs/development/current/main/CURRENT_STATE.toml`
- `docs/development/current/main/investigations/mirbuilder-headerport-i0-production-cutover-consultation-2026-07-21.md`
- `tools/checks/lib/headerport_header_reader_census.py`
- `docs/development/current/main/investigations/mirbuilder-headerport-i0-hdr0-p0-execution-task-2026-07-22.md`

## Decision

Accepted on 2026-07-22:

```text
Q1 = Pure projection
Q2 = Thread one short HeaderPort loan through resolve, emit, and annotation
Q3 = Keep explicit invocation and legacy compatibility presence separate
Q4 = Atomic all-route CUT0
```

A disconnected sealed adapter is permitted only as CUT0 preparation. It is
not a second production policy. The executable task order and acceptance
criteria now live in the related HDR0-P0 execution task.

## Context

HDR0-M0 inventories 29 production `current_module` occurrences in 20
semantic rows. Two passive HDR0-P0 slices are now landed:

- call-result annotation through `LoweringHeaderPortV1`;
- constructor/birth presence through `RawFunctionHeaderLookupPortV1`.

Both preserve legacy behavior when the explicit header capability is absent.
Production invocation consumers, capture/commit, and CUT0 remain zero.

The questions below are intentionally limited to owner and policy choices.
They do not authorize partial production wiring.

## Question 1 — method-index freshness owner

### Current evidence

The pure candidate projection is mechanically clear:

```text
ModuleDraftCollectorV1
  -> LoweringHeaderPortV1
  -> method_candidates_from_headers
```

The unresolved part is cache lifecycle. The legacy cache lives in
`CompilationContext` and uses only `current_module.functions.len()` as its
freshness key. `prepare_module()` does not establish an invocation generation;
same-size module replacement can therefore retain stale candidates. The
legacy path iterates a `HashMap`, while the header projection sorts symbols.

### Decision question

Which owner should define method-tail cache freshness for an explicit
invocation header?

1. **Pure projection (recommended candidate):** explicit header loans bypass
   `CompilationContext.method_tail_index` and call the deterministic pure
   projection each time; the legacy cache remains only behind the `None`
   compatibility facade until CUT0.
2. **Invocation-owned cache:** add a non-ambient cache keyed by collector
   generation/identity, with explicit invalidation and ordering rules.
3. **Other:** specify the owner, identity key, invalidation event, and parity
   law.

### Required boundary

An explicit header miss must not consult `current_module`, a stale cache, or a
second catalog. No cache may be stored in `MirBuilder`, TLS, or a shared global
map.

### Evidence required to close

- same-size module replacement does not retain old candidates;
- symbol replacement and duplicate candidates have deterministic order;
- legacy and explicit-header projections have a named parity fixture;
- production caller count remains zero until the all-route cutover.

## Question 2 — static tail resolver route

### Current evidence

`try_tail_based_resolver` has a legacy production caller in
`calls/build.rs` and is guarded by `NYASH_BUILDER_TAIL_RESOLVE`. The
`try_tail_based_resolver_with_headers` sibling exists but has no production
caller. The route is therefore not a passive helper-only row.

### Decision question

What is the explicit-header policy for the dev-only suffix resolver?

1. **Thread the short HeaderPort loan:** call the `_with_headers` sibling only
   from an invocation-owned route, preserving the environment gate and making
   an explicit miss terminal.
2. **Quarantine invocation use:** keep the legacy resolver for the
   compatibility route, but reject/park suffix recovery in an explicit
   invocation until a later route card.
3. **Retire the dev resolver:** remove the route and its environment policy
   after parity evidence proves no supported caller needs it.

### Required boundary

An explicit collector miss must not retry `current_module` suffixes. No
route-name string match or caller-authored symbol inventory may become
authority. Partial raw-only or canonical-only production wiring is forbidden.

### Evidence required to close

Unique, ambiguous, and missing suffix fixtures under the environment gate,
including stale-module negative cases and instruction/ValueId no-delta on
miss.

## Question 3 — materializer `legacy_presence` policy

### Current evidence

`try_global_additional_resolvers_with_lookup` is fallback-free when `lookup`
is `Some`. Its `None` compatibility path receives `legacy_presence`, which is
computed from `current_module.functions.contains_key`. This deliberately keeps
legacy behavior and is not equivalent to an invocation header miss. The
`condition_fn` compatibility branch is a separate policy and must not be
folded into header presence.

### Decision question

How should the two materializer modes coexist during migration?

1. **Keep an explicit split:** `Some(header)` is the future route and
   fallback-free; `None` remains a named legacy compatibility facade until the
   atomic CUT0.
2. **Remove the compatibility presence input:** require an explicit header for
   direct module presence and fail-fast all legacy callers before wiring.
3. **Retire direct materialization:** move direct global presence to the
   canonical callable catalog and remove this resolver family.

### Required boundary

Do not silently change `legacy_presence`, `condition_fn`, or explicit-miss
semantics. A header miss may continue to ordinary semantic resolution only; it
may not switch header source or retry the module map.

### Evidence required to close

Separate legacy/explicit-header parity fixtures, explicit-miss no-retry proof,
condition_fn isolation, and a caller inventory showing which route owns each
mode.

## Question 4 — lifecycle/publication activation boundary

### Current evidence

The future owner family is mechanically identified:

```text
ModuleLoweringInvocationStateV1
  -> ModuleLoweringPortV1
  -> PreparedInvocationDrainV1
  -> one external module commit
```

The live path still installs/takes `current_module` and publishes function
drafts directly. Existing disconnected fixtures cover capture, restore,
collector admission, shell drain, and failure retention, but they do not prove
the production all-route cutover.

### Decision question

What exact gate authorizes the first production lifecycle connection?

1. **Atomic all-route CUT0:** connect raw, A+/trivial, acyclic/recursive,
   Main/condition_fn, shell drain, finalizer input, and external commit in one
   cutover after HDR0-G0.
2. **Staged wiring with a sealed adapter:** define a single outer adapter that
   keeps all legacy routes disconnected until the final activation flag, with
   no route-specific production consumer.
3. **Other:** specify how one collector, one drain, one finalizer, and one
   external commit remain invariant during the transition.

### Required boundary

Before this decision is accepted, production capture/commit must remain zero.
No Builder-owned collector, header cache, second shell, retry, or post-drain
`current_module` read may be introduced.

### Evidence required to close

Success, primary-error, cleanup-error, admission-error, drain-error,
finalizer-error, and panic parity for every route family; one consumer count
for capture, drain, finalizer, and external commit; and a fast gate proving no
partial route cutover.

## Confirmed classifications (not open questions)

- Located legacy observation: diagnostic/disconnected only; no production
  caller exists, so no collector activation is selected.
- Shell metadata: `ModuleLoweringShellPortV1` is the future owner family; its
  production connection remains part of the lifecycle question above.
- Constructor/birth presence: `LoweringHeaderPortV1::contains_symbol` is the
  explicit-header owner; compatibility branch policy remains unchanged.

## Non-claims

This document does not claim:

- production HeaderPort capture/commit;
- retirement of `current_module` readers;
- FACTSESSION activation or finalization repair removal;
- JoinIR, FastMem, LLVM, or selfhost parser migration progress;
- completion of the execution tasks selected by Questions 1–4.

The first selected code-facing task is HDR0-P0-AUTHORITY-ERASURE0. Production
capture/commit remains forbidden until the atomic CUT0 gate is complete.
