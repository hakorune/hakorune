# MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D9

Status: design_stop__2026-09-25
Date: 2026-09-25
Parent: MIRBUILDER-EXE-ACCEPTANCE-TARGET-ONLY-EMISSION-S0
  (landed — `TargetOnly` rows at the member_route `StaticReceiver`
  arm now emit through `lower_target_only_static_result_publication_v1`;
  `JsonLine.stringField/2` lowered; json advanced past
  `static-result-ingress/target-only`)
Owner: workstream row H / unified resume gate 1
Authority: mirbuilder-final-pipeline-ssot.md acceptance table;
  D0–D8 selection lineage.

## Problem

TARGET-ONLY-EMISSION-S0 landed. The app advances to the next named
terminal:

```text
lexical scope body failed:
[freeze:contract][ordinary-new/argument-source-unavailable]
```

Emitted at `new_expression.rs:107-109`: a `Raw` `Ordinary` route
`new` took an `OrdinaryNewAdmissionClaimV1` whose
`argument_rows()` is `Err` — the claim was issued but its selected
argument rows were never proven. The check precedes
`prepare_ordinary_new_emission`, so an unavailable claim cannot even
decline selection into the ordinary route.

App `new` inventory (`apps/json-stream-aggregator/main.hako`):
`new MapBox()`/`new ArrayBox()` (`birth` :98-99, `me.field =` —
non-initializer sites → birth-recipe index, no claim),
`new JsonStreamAggregator()` (`main` :171, arity 0 →
`argument_rows = Ok([])`), and
`local stats = new UserStats(user)` (`statsFor` :111, arity 1,
`Local` argument).

## Fresh class map (receipt after TARGET-ONLY-EMISSION-S0)

| class | entries | first named stop |
|---|---|---|
| `callable-loop/route-not-front-selected SourceCallOutsideSelectedFamily` | 3 | boxtorrent_mini, binary_trees, mimalloc_lite — D5-sealed forks |
| `ordinary-new/argument-source-unavailable` | 1 | json_stream_aggregator (`local stats = new UserStats(user)`, `statsFor` :111) |
| `CoreMethodSource/NamedArray(TextSourceMissing)` | 1 | allocator_stress |
| backend `opt`/`mir_call_no_route`/emit (toolchain) | 3 | typed_object_newbox_min, typed_object_untyped_field_min, string_substring_in_range — all baseline-identical |

## Questions

1. Which site owns the stop — is `local stats = new UserStats(user)`
   in `statsFor` the claim that reaches `argument-source-unavailable`,
   or a different `new`?
2. Where does `argument_rows = Err(...)` originate — the coseal
   fallback (`ordinary_new_coseal_issue.rs:318-325` synthesizes
   `Err(SourceMismatch)` whenever `argument_observations` has no row
   for the site) or an observed `ArgumentNotTrivial`/
   `ArgumentOrdinalOverflow`?
3. `argument_observations` is populated only on the gated path
   (`owner_loan` / `is_app_main` / `seed_eligible && (has_map ||
   child_new_ready)`, coseal_issue.rs:201-244) via
   `verify_function_completion_with_new_homes_and_argument_
   observations_v1`; the fallback calls
   `issue_new_home_prefixes_v1` which internally runs the same
   `scan_new_home_flow` observation walk but discards it (returns
   `.0` only, `home_new_prefix.rs:92-108`). Is discarding already-
   computed observations the intended contract, or a dropped fact?
4. `PrefixLocalFlow::observe` (home_prefix_local_flow.rs:121-147)
   requires the argument binding installed in `locals`;
   `install_parameters` is fed `parameter_contracts` on the gated
   path but `std::iter::empty()` on the fallback. `user` (a string
   parameter of `statsFor`) installs as `StoredLocal::Handle` →
   `OrdinaryObservation::Handle` → `into_selected_argument` → `None`
   → `ArgumentNotTrivial`. Is `SelectedNewArgumentKindV1`
   (Integer/Bool/Local-trivial only) the intended ceiling — i.e.
   handle-typed `new` arguments are outside the selected-argument
   vocabulary — or is a Handle argument row a missing Facts kind?
5. When a claim exists but `argument_rows` is unavailable, is the
   designed semantics "claim unusable → hard terminal" (current
   early check) or "claim declines selection → ordinary route with
   `constructor` handoff" (`new_expression.rs:200-213`)? Which owner
   decides — the claim issuer (never issue un-provable claims) or
   the admission consumer (decline → fallback)?

## Bounded candidates (order only)

- H3a: expose the already-computed observations on the fallback
  path — a sibling entry point that returns `scan_new_home_flow`'s
  `.3` (observations) alongside `.0` (prefixes), so claim argument
  rows reflect real observations instead of synthesized
  `SourceMismatch`. Feeding `parameter_contracts` to
  `install_parameters` on that path is a separate question
  (parameter-installation affects `home_prefix` availability too).
- H3b: extend `SelectedNewArgumentKindV1` with a Handle/Box argument
  kind (Facts vocabulary extension; requires Recipe + physical
  emission changes — much larger, separate authority).
- H3c: if the designed semantics is "un-provable claim declines
  selection", move the `argument_rows` check behind
  `prepare_ordinary_new_emission`'s decision — smallest code, but
  weakens the fail-fast boundary (an issued-but-unusable claim
  silently taking the ordinary route may mask a missing issuer).

## Census answers

(to be filled)

## Decision

(to be filled)

## Boundary

- Includes: owner census for `ordinary-new/argument-source-
  unavailable` on `statsFor` `new UserStats(user)`; the relation
  between claim issuance, argument-observation issuance, parameter
  installation, and the admission consumer; one Decision with a
  six-line brief or sealed NoSafeSlice.
- Excludes: implementation; reopening D5-sealed coverage forks;
  F3b/F3c; backend toolchain; inference panic; NamedArray.

## Exit

- [ ] Decision recorded (bounded slice OR sealed NoSafeSlice with
      reopen triggers) with the six-line brief.
- [ ] Next S-card emitted OR the named design card opened;
      pointers synced.
