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

## Census answers

1. **Physical consumer exists and is already production-wired in a
   sibling lane.** `lower_target_only_static_result_publication_v1`
   (plain variant, `physical_bridge.rs:135-163`) takes exactly the
   member_route arm's shape (`builder`, `descent`, `target_key`,
   `source_argument_count`) and emits
   `emit_static_global_target_value_terminal_v1` with no result
   claim. Its `with_expected_sites` sibling is ALREADY production
   for the qualified-main lane (`member_route.rs:108-113`), so
   TargetOnly→physical emission is the designed semantics; the plain
   variant is currently exercised only by
   `me_method_canonical_cutover_tests.rs:184`.
2. **Why `stringField` is `Unavailable`:** its returns are
   `line.substring(start, end)` (param receiver — no provable
   `SourceCoreReceiverFactV1` → `prove_core_string_method` →
   `unavailable_target`) and `""` literal — mixed `ExactString`/
   `Unknown` → disposition `Unavailable` → `TargetOnly`. Param
   receiver-fact unprovability is the catalog's honest contract —
   hako params carry no declared types, so only provable receivers
   publish results. `intField` (`StringHelpers.to_i64` — cross-
   module `using` call, outside the same-module target inventory)
   is `TargetOnly` for the same reason; `find` (i64 arithmetic
   returns) is likely `Selected`; `boolField` (bool literals) is
   disposition-dependent. Conclusion: `ingestLine` cannot finish
   while `TargetOnly` is a dead-end — H2a is the only viable edge.
3. **One-meaning preserved:** the row's exact `(caller, site)` →
   target identity is source-proven; emitting `GlobalCall` with an
   unpublished (Unknown) result is the same physical meaning the
   retired lane produced — minus the name-based target guess. The
   `static-target-only` named terminals
   (`source-arity`/`target-projection`/`physical-arity`) already pin
   the fault surface.
4. **Drain semantics:** `UnconsumedTargetOnly` exists as a finish
   error, but `take_for_source` marks the row consumed on take
   (`:381-384`) — drain pressure is on take-consumption, not
   physical emission. Wiring emission does not weaken the audit.
5. **me-call probe arm:** `static_current_owner_policy.rs:61-67`
   holds the same `TargetOnly`→error shape for `CurrentOwner` rows
   in `StaticBoxMethod` callers. It is deliberately EXCLUDED —
   `me.method` has a downstream sibling the `StaticReceiver` probe
   lacks, no production site needs it yet, and one edge stays one
   edge. Reopen if a `me.method` `TargetOnly` terminal surfaces.

## Decision

```text
Decision: H2a — at the member_route StaticReceiver arm, wire
  StaticResultPublicationIngressV1::TargetOnly to the existing
  lower_target_only_static_result_publication_v1 physical lowerer.
Source authority + canonical issuer:
  StaticCallResultTargetOnlyV1 row (source-proven exact target) ->
  emit_static_global_target_value_terminal_v1; result stays
  unpublished (no PreparedStaticCallResultPublicationV1 commit).
Non-authority: the qualified-main expected-sites variant is a
  different handoff shape and stays where it is; the me-call probe's
  TargetOnly arm stays a named error (deliberately excluded sibling).
Fail-fast boundary: static-target-only/source-arity,
  target-projection, physical-arity propagate; NoExactStaticTarget
  stays a hard named terminal; row already consumed by take.
Smallest next slice: replace the member_route.rs:134-140 TargetOnly
  error arm with lower_target_only_static_result_publication_v1
  (builder, descent, *target.target(), arguments.len()). No new
  issuer/terminal; the existing error string is retired in place.
Non-claims: does not prove stringField's result (H2b untouched);
  downstream JsonLine callees (substring/StringHelpers) may hit
  their own known terminals; me-call probe TargetOnly unchanged;
  test green is not a production claim.
```

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

- [x] Decision recorded (bounded slice OR sealed NoSafeSlice with
      reopen triggers) with the six-line brief.
- [x] Next S-card emitted OR the named design card opened;
      pointers synced.
