# MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D8

Status: design_stop__2026-09-25
Date: 2026-09-25
Parent: MIRBUILDER-EXE-ACCEPTANCE-INSTANCE-STATIC-INGRESS-S0
  (landed — declaration-gated claim-ingress admits `Cataloged`
  instance callers; `JsonLine.stringField/2` row exists and is
  consumed; json advanced past `legacy-fallback-retired`)
Owner: workstream row H / unified resume gate 1
Authority: mirbuilder-final-pipeline-ssot.md acceptance table;
  D0–D7 selection lineage.

## Problem

INSTANCE-STATIC-INGRESS-S0 landed: the `StaticReceiver` claim-ingress
now admits `Cataloged` non-`StaticBoxMethod` callers whose probed
(owner, method, arity) resolves via `declaration_for` to a same-module
static-box declaration. The app advances to the next named terminal:

```text
lexical scope body failed:
[freeze:contract][static-result-ingress/target-only/
StaticCallTargetAuthorityUnavailable] JsonLine.stringField/2
```

`take_for_source` found the exact `(caller, site)` row and consumed
it (`static_call_result_publication_owner.rs:381-384`), but the row
is `TargetOnly`: the exact target `JsonLine.stringField/2` is proven
while its result representation is `Unavailable`.

## Fresh class map (receipt after INSTANCE-STATIC-INGRESS-S0)

| class | entries | first named stop |
|---|---|---|
| `callable-loop/route-not-front-selected SourceCallOutsideSelectedFamily` | 3 | boxtorrent_mini (`seedBlocks`), binary_trees (`iterationCheck`), mimalloc_lite (`seedBlocks`) — D5-sealed forks |
| `static-result-ingress/target-only` | 1 | json_stream_aggregator (`JsonLine.stringField/2`, `ingestLine` :122) |
| `CoreMethodSource/NamedArray(TextSourceMissing)` | 1 | allocator_stress |
| backend `opt`/`mir_call_no_route`/emit (toolchain) | 3 | typed_object_newbox_min, typed_object_untyped_field_min, string_substring_in_range — all baseline-identical |

## Questions

1. Which contract owns the `TargetOnly` disposition — is
   `lower_target_only_static_result_publication_v1`
   (`calls/static_result_publication_physical_bridge.rs:135-163`,
   currently test-only) the designed physical consumer for it, or is
   `TargetOnly` a deliberate dead-end pending a result-representation
   proof?
2. Why is `JsonLine.stringField` `Unavailable` — its body returns
   `line.substring(...)`, whose `line` parameter carries no provable
   receiver fact (`prove_core_string_method` → `unavailable_target`).
   Is parameter-receiver-fact unprovability the intended contract
   (i.e. only provable receivers publish results), or is a
   receiver-fact issuer missing?
3. If `TargetOnly` may emit: does `GlobalCall` with `Unknown` result
   type preserve the one-meaning contract (target exact, result
   unpublished) and satisfy the same `static-target-only` named
   terminals (`source-arity`/`target-projection`/`physical-arity`)
   already pinned in the physical bridge?
4. What are `JsonLine.intField`/`boolField`/`find` dispositions in
   `ingestLine` — `Selected` or also `TargetOnly`? If `TargetOnly`,
   the TargetOnly→physical edge is the only way `ingestLine`
   finishes; if `Selected`, the next boundary is elsewhere.
5. `UnconsumedTargetOnly` is a `FinishError` variant: does the drain
   audit intend TargetOnly rows to reach a real physical consumer
   (not just take-consumption on the error path)?

## Bounded candidates (order only)

- H2a: wire `StaticResultPublicationIngressV1::TargetOnly` at
  `member_route.rs:134-140` (and the symmetric me-call-probe arm at
  `static_current_owner_policy.rs:61-67`) to
  `lower_target_only_static_result_publication_v1` — existing
  physical lowerer, keeps `no-exact-static-target` a named terminal.
- H2b: prove `JsonLine.stringField`'s result (receiver-fact issuer
  for parameter receivers — separate authority, larger).
- Others stay queued: F3b `UnconsumedSelected` consumption, F3c
  `SourceItemsMissing`, D5-sealed coverage forks.

## Boundary

- Includes: owner census for `static-result-ingress/target-only` on
  an instance-caller same-module static call; the relationship
  between `TargetOnly` rows, the physical bridge, and the result-
  representation catalog; one Decision with a six-line brief or
  sealed NoSafeSlice.
- Excludes: implementation; reopening D5-sealed coverage forks;
  widening result-representation proving (H2b needs its own card if
  selected); backend toolchain; inference panic; NamedArray.

## Exit

- [ ] Decision recorded (bounded slice OR sealed NoSafeSlice with
      reopen triggers) with the six-line brief.
- [ ] Next S-card emitted OR the named design card opened;
      pointers synced.
