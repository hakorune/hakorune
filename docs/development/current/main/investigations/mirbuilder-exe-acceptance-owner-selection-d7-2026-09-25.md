# MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D7

Status: design_stop__2026-09-25
Date: 2026-09-25
Parent: MIRBUILDER-EXE-ACCEPTANCE-ME-RECEIVER-SITE-S0
  (landed — located `Me`/`This` MethodCall receivers consume the
  registered `Receiver` site through `exact_source_receiver_value`;
  json advanced past `incomplete-consumption`)
Owner: workstream row H / unified resume gate 1
Authority: mirbuilder-final-pipeline-ssot.md acceptance table;
  D0–D6 selection lineage.

## Problem

ME-RECEIVER-SITE-S0 landed: `JsonStreamAggregator.ingest`'s loop body
`me.ingestLine(...)` now consumes its `Receiver` site and `ingest`'s
`finish` passes. The app advances to the next named terminal:

```text
lexical scope body failed:
[freeze:contract][static-call/legacy-fallback-retired]
owner=JsonLine method=stringField arity=2
```

The failing call is `JsonLine.stringField(line, "user")`
(`apps/json-stream-aggregator/main.hako:122`), a same-module static-box
method call in a `local` initializer inside `ingestLine`. The retired
legacy fallback is the honest stop; select the owner for issuing this
call.

## Fresh class map (receipt after ME-RECEIVER-SITE-S0)

| class | entries | first named stop |
|---|---|---|
| `callable-loop/route-not-front-selected SourceCallOutsideSelectedFamily` | 3 | boxtorrent_mini (`seedBlocks`), binary_trees (`iterationCheck`), mimalloc_lite (`seedBlocks`) — D5-sealed forks |
| `static-call/legacy-fallback-retired` | 1 | json_stream_aggregator (`JsonStreamAggregator.ingestLine` → `JsonLine.stringField/2`, :122) |
| `CoreMethodSource/NamedArray(TextSourceMissing)` | 1 | allocator_stress |
| backend `opt` type error (toolchain) | 1 | typed_object_newbox_min |
| inference panic `return_type_strategy` | 1 | typed_object_untyped_field_min |

## Questions

1. Which contract owns `JsonLine.stringField(line, "user")` in
   `ingestLine`'s body — the selected same-module static-call route
   (`target_for_source` / `StaticResultPublication`), the qualified
   preflight `first_family` admission, or another existing issuer?
2. Why is `JsonLine.find` inside `ingest`'s loop admissible (singleton
   anchor) while `JsonLine.stringField` in `ingestLine`'s ordinary body
   reaches `legacy-fallback-retired`? Is the difference loop-item vs
   statement position, or `ingestLine`'s own selection/admission state?
3. Does `me.ingestLine(...)` in the loop still count as an uncovered
   call inside the singleton-anchor contract (i.e. would the same body
   stop at `SourceCallOutsideSelectedFamily`/`UnconsumedSelected` later
   even if `stringField` issues)?
4. Bounded candidates (order only — each needs its own census):
   - H1: selected static-call issuance for ordinary body statements
     (`JsonLine.stringField`/`intField`/`boolField` in `ingestLine`).
   - F3b: `UnconsumedSelected` publication consumption on the
     singleton LoopCond route.
   - F3c: `SourceItemsMissing` call-free admission.
   - coverage forks: B3-ArrayPush and ordinary-instance-call coverage
     (both D5-sealed; reopen only through their own cards).

## Boundary

- Includes: owner census for `static-call/legacy-fallback-retired` on a
  same-module static-box method call in a selected instance-method
  body; the relationship between ordinary-body static calls and the
  loop singleton-anchor contract; one Decision with a six-line brief or
  sealed NoSafeSlice.
- Excludes: implementation; reopening the D5-sealed coverage forks;
  backend toolchain; inference panic family; NamedArray family.

## Census answers

1. **Owner:** `VerifiedStaticCallResultPublicationOwnerV1` rows keyed
   `(caller, site)`. Issuance (`whole_source_inventory.rs` `seal_qualified`
   + `issue`) has **no caller-namespace filter** for `Qualified`
   receivers — `JsonLine.stringField` inside `ingestLine`
   (`InstanceBoxMethod` caller, `Variable(JsonLine)` unshadowed
   receiver) already has a row, almost certainly `Selected`
   (`stringField` returns `ExactString`). The sole physical consumer
   `lower_selected_static_result_publication_v1` is already wired
   behind `member_route.rs` `StaticReceiver`
   (`member_route.rs:125-133`). The missing edge is **consumption
   gating only**: `classify_source_context_v1`
   (`static_result_publication_ingress.rs:142-156`) classifies
   `Cataloged` callers as `Cataloged` only when
   `caller.namespace() == StaticBoxMethod`; instance-method callers
   fall to `Unavailable` → `handle_static_method_call_with_descent`
   → `legacy-fallback-retired`.
2. **Loop vs ordinary:** the loop route drains identical rows
   caller-agnostically for any `Cataloged` root
   (`raw_loop_child_port.rs:152-227`,
   `target_for_source`/`selected_static_result_handoff_for_source`) —
   that is why `JsonLine.find` inside `ingest`'s loop works.
   `ingestLine` IS a source-backed callable (callable ledger
   installed via `with_cataloged_callable_source_scope`), but its
   ordinary body statements are driven by
   `drive_located_invocation_body_v1` → the raw recursive dispatcher
   → `member_route` `StaticReceiver`, where the ingress gate returns
   `Unavailable`. Difference = **route's consumption gate**, not
   selection state.
3. **`me.ingestLine` coverage:** resolved — `ingest`'s `finish`
   already passed (ME-RECEIVER-SITE-S0 evidence); the singleton
   anchor covered the loop items and `JsonLine.find`'s publication
   was consumed. `UnconsumedSelected` is not pending for `ingest`.
   After H1, `ingestLine`'s `stringField` row will be consumed by
   the same physical bridge (`TargetOnly`/`Selected` consumption is
   drain-safe against `StaticResultPublicationResidual`).
4. **Why the gate exists:** the same ingress is probed
   unconditionally at the head of every `me.method` resolution
   (`static_current_owner_policy.rs:39-49`), BEFORE the
   DeclaredInstance receiver ingress (`:78-87`), and
   `NoExactStaticTarget` is a hard error there. Instance callers
   never hold `CurrentOwner` static rows
   (`whole_source_inventory.rs:343-354`), so admitting them at that
   probe would freeze every working `me.method` in instance bodies.
   The gate is **caller-scoped where it should be probe-scoped**:
   the `StaticReceiver` route plan probe is already reached only for
   `Variable`-receiver static sites, which never collide with the
   me-call probe. The "receiver-bearing sibling" in the test
   (`static_result_publication_ingress.rs:324-337`) is the
   `DeclaredInstanceReceiverIngressV1`/`CanonicalInstance` lane —
   `me`/`this` only, `JsonLine.stringField` never reaches it.

## Decision

```text
Decision: H1 — admit non-StaticBoxMethod Cataloged callers into the
  static-result-publication ingress only when the probed
  (owner, method, arity) resolves through the declaration catalog to a
  same-module StaticBoxMethod declaration.
Source authority + canonical issuer:
  VerifiedWholeSourceStaticCallTargetInventoryV1
  -> VerifiedStaticCallResultPublicationOwnerV1 rows keyed
  (caller, site); physical emission via existing
  lower_selected_static_result_publication_v1 bridge.
Non-authority: classify_source_context_v1's blanket caller-namespace
  gate; handle_static_method_call_with_descent legacy lane stays
  retired for non-Math.
Fail-fast boundary: NoExactStaticTarget (declared but row-absent) and
  TargetOnly remain named terminals; Selected rows consumed by the
  existing bridge and drain-audited by StaticResultPublicationResidual.
Smallest next slice: in classify_source_context_v1, admit a Cataloged
  non-StaticBoxMethod caller when
  declarations.declaration_for(StaticBoxMethod, owner, method, arity)
  resolves — the ingress's currently-ignored _owner/_method/
  _argument_count params become the discriminator. The me-call probe
  passes owner="<source-owned>", which never resolves, so the
  DeclaredInstance sibling stays protected without a probe flag;
  Math/builtin owners never resolve either, so Math keeps
  Unavailable -> compatibility on every caller.
Non-claims: no instance-call (DeclaredInstance) admission change; no
  loop-route change; no coverage-fork reopen (B3/ordinary-instance
  stay D5-sealed); RawStructuredChildScopePortV1 forwarder inherits
  the same rule with no leak; local test green is not a production
  claim.
```

Design note: the refined mechanism replaces a probe-origin flag with
declaration-gating, which is the honest ownership boundary — the
claim-ingress owns exactly the same-module-declared static targets.
Two protections hold by construction:

- `me.method` probe (`static_current_owner_policy.rs:43-49`) passes
  `owner="<source-owned>"`; `declaration_for` never resolves that
  owner, so instance callers keep `Unavailable` -> DeclaredInstance,
  while StaticBoxMethod callers keep the namespace clause and their
  `CurrentOwner` rows (StaticCurrentOwner lane untouched).
- `Math.x`/builtins never resolve `declaration_for`, so they keep
  `Unavailable` -> `handle_static_method_call_with_descent` ->
  `qualified_math_compatibility_owner` on every caller — no
  `NoExactStaticTarget` regression for Math in instance bodies.

## Exit

- [x] Decision recorded (bounded slice OR sealed NoSafeSlice with
      reopen triggers) with the six-line brief.
- [x] Next S-card emitted OR the named design card opened;
      pointers synced.
