# MIRBUILDER-EXE-ACCEPTANCE-CATALOGED-CALL-EDGE-DOMAIN-D19
## Census: Sealed-Corridor Domain Contract vs Document Publication — JSON Aggregator Boundary (D19)

Date: 2026-09-26
Status: decided — bounded next slice accepted
Family: callable / gate1 / Gate 1 unified lane / cataloged call edge /
  EXE acceptance
Row reference: workstream row H (Gate 1 unified selfhost lane)
Blocking observation: after the return-type arm was deferred to the
CanonicalTyped route, `--emit-mir-json` on
`apps/json-stream-aggregator/main.hako` terminates inside strict
verification at
`[freeze:contract][mir/invoke/call-argument-type-drift]` — nine sites,
all `String`-typed arguments on cataloged same-module call edges.

## Boundary covered by this census

起点: published view scan + strict module verify consumed by the
document/publication pipelines; 終点: physical-lane admission gates
(`hako_physical_validate_ordinary_call`, lifecycle admission).
includes: the cataloged same-module call-edge domain rule arms
(return-type, argument-kind, result-kind vocabulary, map-lease pair).
excludes: per-function structural checks (SSA/dominance/CFG/frame),
object/field/home lifecycle-vocabulary checks, `map::check` lease
lifetimes, instruction-shape validity.

## Classifier-arm audit (same responsibility, third strike)

1. `StaticMethodRequiresIntegerReturn` /
   `FreeFunctionRequiresIntegerReturn`
   (`published_backend_view.rs::validate_static_call` /
   `validate_free_function_call`) — required every published static/
   free definition's signature `return_type == MirType::Integer`.
   Measured: `JsonLine.stringField/2` honestly returns `String`;
   `JsonLine.boolField/2` returns `Bool`. The module's route is always
   `UnsupportedBeforeObject` (`Call{BirthConstructor}` is a lifecycle
   row), so no selected-C consumer ever transports those results.
   Landed (S7): reclassified, not deferred — a non-`Integer` result
   never enters the corridor row vocabulary; it marks
   `has_non_lifecycle_unsupported` so the module classifies
   `UnsupportedBeforeObject` and `map_projection`'s `Domain::I64`
   claim on checked call sites holds by construction. Exactness
   checks (carrier/definition/arity) stay unconditional; the
   `*RequiresIntegerReturn` variants were removed.
2. `call-argument-type-drift` (`verification/invoke.rs::check_call_edge`)
   — a cataloged `Call`/`Invoke{Call}` edge rejects a *proven* non-i64
   argument domain (`Integer | Unknown` only, plus the admitted
   `MapBox` actual/formal pair). Measured drifts, all honest:

   ```text
   JsonLine.stringField/2  bb337 arg=ValueId(43) String -> JsonLine.find/3
   JsonStreamAggregator.ingest/1 bb12 arg=ValueId(26) String -> JsonLine.find/3
   JsonStreamAggregator.ingest/1 bb12 arg=ValueId(35) String -> ingestLine/1
   JsonStreamAggregator.ingestLine/1 bb20 arg=ValueId(28) String -> intField/2
   JsonStreamAggregator.ingestLine/1 bb20 arg=ValueId(32) String -> boolField/2
   JsonStreamAggregator.ingestLine/1 bb17 arg=ValueId(14) String -> stringField/2
   StringHelpers.is_alpha/1 bb93 arg=ValueId(20) String -> index_of/3
   StringHelpers.skip_ws/2  bb208 arg=ValueId(40) String -> is_space/1
   StringHelpers.to_i64/1   bb279 arg=ValueId(194) String -> _digit_value/1
   ```

   The callee formals are all `Unknown`; the arguments are honest
   `String` records — i64 handle transport — but the check reads
   proven semantic domain, not physical width (its own pin
   `cataloged_call_rejects_proven_non_scalar_argument` deliberately
   rejects `Float`, which also rides i64). So the contract is an
   **Integer-domain sealed-ABI** rule, not a transport rule.
3. `call-result-contract-not-connected`
   (`verification/invoke.rs` Invoke arm) — `Invoke{Call}` to a
   cataloged callee requires `ResultKind::I64 | Map`. This arm is
   *vocabulary*, not domain: a `String` result has no `ResultKind`,
   so such an edge cannot be emitted at all. It stays unconditional;
   no drift possible.
4. Map-lease corroboration (`verification/invoke_map.rs:
   borrowed_map_call_edge`) consults `cataloged_call_target`
   (definition presence), not the domain rule — unaffected by any
   arg-domain policy change.
5. `object-field-read-definition-invalid`, `object-definition-missing`,
   `home-destruction-unavailable`, `field-definition-missing` —
   canonical lifecycle vocabulary; stay unconditional.

## The authority question

Is the cataloged call-edge domain rule **canonical validity** (a
module containing `String` args on cataloged call edges is malformed
MIR) or **corridor admission** (the rule binds only modules whose
calls are claimed by a sealed consumer)?

Evidence for corridor admission:

- The introducing commit (`4da7d21969`, "enforce i64-only
  ordinary-call parameter conformance") frames the rule as sealing a
  physical-lane ABI hole: "the emitter's `i64 %v` spelling is now the
  only reachable shape" — the contract is defined by what a sealed
  consumer can spell/transport.
- `MirType::String` is `Storage::Handle` (map_projection.rs:64): the
  argument physically rides the `i64` spelling faithfully; the check
  rejects it on *semantic domain* grounds — a lane contract about
  which domains a sealed edge may carry, not a validity defect.
- The module is never corridor-admissible anyway
  (`SameModuleInstance`, `BirthConstructor`, `Callee::Value` ->
  `UnsupportedBeforeObject`); the app crossed String-domain args
  before the source lane cataloged the calls — cataloging made the
  edges visible to a check written for the numeric corridor.

Evidence for canonical validity (weighed, rejected):

- The edge is called "the scalar edge" and the pin comments say
  "spells every argument as a scalar". Read as transport this is
  compatible with handle args (`i64` spelling); read as domain it is
  the corridor contract restated. Either way it does not make the
  instruction malformed — `Call`/`Invoke{Call}` carry `Vec<ValueId>`
  and the callee/arities resolve exactly; nothing in the canonical
  vocabulary forbids the edge.
- Rejecting the module as malformed would make document publication
  impossible for any multi-domain module and would require a new
  call-edge family before any JSON emits — a corridor expansion, not
  a verification fix.

## Decision

```text
Decision:
  The cataloged call-edge domain rule (proven non-Integer argument
  kinds, including the MapBox actual/formal pair branch) is a
  sealed-corridor admission contract. Document publication verifies
  the canonical structure of the same edges (callee resolution and
  arity) but does not enforce the sealed domain.
Source authority + canonical issuer:
  `PublishedMirBackendView` route classification decides which
  corridors claim the module; `MirVerifier` verifies canonical
  structure and, under the sealed policy, the corridor domain.
Non-authority:
  `metadata.value_types` semantic records are evidence for the
  domain check, not the caller's intent; the document emitter never
  re-routes a call.
Fail-fast boundary:
  Every `verify_module` caller keeps the sealed policy by default —
  `compile_normal_with_published` (CanonicalTyped and
  Unsupported+lifeccycle routes), `module_postprocess`, raw lanes and
  tests are unchanged. The document entry alone passes the document
  policy; arity/definition drift still freezes there.
Smallest next slice:
  Thread an explicit call-edge policy through `invoke::check_module`
  -> `MirVerifier`, add the document verify entry, and call it from
  `compile_normal_for_mir_json`.
Non-claims:
  No String/handle corridor admission for backend lanes; no change
  to `Callee` emission; no weakening of `verify_module` for any
  existing caller; the document still freezes on arity drift and on
  every structural/lifecycle check.
```

## Options weighed

- (A) Explicit call-edge policy in the verifier (accepted) — one
  authority, callers name the question they ask; all existing
  callers keep `Sealed`.
- (B) Treat the domain rule as canonical validity — rejected: it
  would hard-block document publication for honest multi-domain
  modules and force an unadmitted-family corridor expansion ahead of
  Gate 1 evidence.
- (C) Widen the admitted argument domains (`String`, `Bool`,
  `Float`) — rejected: contradicts the deliberate `Float` pin and
  reopens residence ambiguity (pinned-text values also record
  `String`), the exact hazard class `4da7d21969` sealed.
- (D) Move `check_call_edge` wholesale into corridor admission —
  rejected: `module_postprocess` verifies before the canonical
  definition table is published (the check no-ops there), and the
  map-lease arm protects lifecycle-corridor modules that never claim
  `CanonicalTyped`; the check must stay in verification.

## Bounded next slice (S7)

1. `invoke::check_module` gains a call-edge policy parameter; the
   arg-domain block (map-formal/map-actual pair + `scalar_drift`) is
   skipped under the document policy while the arity rule and every
   other `check_module` arm stay unconditional.
2. `MirVerifier::verify_module` keeps the sealed policy (all current
   callers unchanged); a `verify_document_module` entry delegates the
   same body with the document policy.
3. `compile_normal_for_mir_json` calls the document entry.
4. Pins: document policy admits a `String`-recorded argument on a
   cataloged edge and still rejects arity drift; sealed policy keeps
   rejecting `Float`/`String`/`Box` proven args (existing tests).
5. Positive view pin for arm 1: a non-Integer-return cataloged call
   yields `UnsupportedBeforeObject` with the call kept off the
   corridor row vocabulary, while an Integer-return sibling call in
   the same module stays a checked corridor row.

## Non-claims

- No claim the JSON app's calls are admitted to any sealed backend
  corridor — `UnsupportedBeforeObject` remains its route.
- No claim `Callee::Value`/instance/string-arg call families gain a
  selected-C consumer.
- VM `ingest/1` ledger-less spine; Gates 2–4; overall MirBuilder
  completion.
