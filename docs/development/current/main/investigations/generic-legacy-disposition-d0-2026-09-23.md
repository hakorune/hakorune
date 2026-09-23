---
Status: design_stop__disposition_classification_design_open
Task: GENERIC-LEGACY-DISPOSITION-D0
Date: 2026-09-23
Parent: JOINIR-LOOP-M8-GENERIC-RESIDUAL-S6E
PreviousCard: generic-legacy-route-observation-p1-batch10-2026-09-23.md
NextCard: same-row__first_classification_write_cohort
Implementation permission: false; fix the classification contract,
vocabularies, and guard delta only. No manifest column write, no
guard/checker edit, no fixture/source change, no disposition or
retirement claim from this card.
---

# Generic legacy disposition D0 — classification contract design

## Six-line brief

```text
Decision: open GENERIC-LEGACY-DISPOSITION-D0, the ordered successor
  of ROUTE-OBSERVATION-P1; the first bounded write cohort is the 40
  `accepted` corpus rows — the only class that can leave the
  "unclassified accepted" closeout counter nonzero — classified
  strictly from the landed P1 batch-card evidence with no
  re-observation.
Source authority + canonical issuer: the P0-normalized case universe
  fixes membership; the ten landed P1 batch cards
  (849daed591..f5514cf608) are the only observation evidence; the row
  contract is `generic-loop-source-to-portable-recipe-ssot.md` D0 row
  (observed normalized universe -> checked disposition; unclassified
  accepted count zero).
Non-authority: compat-lane green output as disposition proof, raw
  stdout as route proof, the corpus guard's current P0-only
  vocabulary as the disposition vocabulary, or this card as a write
  permission.
Fail-fast boundary: a row whose P1 evidence does not name a stable
  authority stays unclassified behind a named owner/repair row
  (NoSafeSlice), never converted to Declined/Rejected/accepted by
  vote; the two nondeterministic if_else_return{,_var} rows are held
  exactly this way.
Smallest next slice: under work_mode=fast, one commit that writes the
  40 accepted rows' D0-owned columns (decision, disposition,
  current_acceptance, observed_route, target_owner, parity_gate where
  applicable) together with the corpus-guard vocabulary update in the
  same commit; rejected (196) and failed-before-loop (162) cohorts
  follow in later slices.
Non-claims: no producer/Recipe/selector arm, no route switch, no
  fixture/source change, no corpus re-census, no edge-record writes
  (S0 owns those), no deletion, no S6E/S6G/M9/M10/Row-F closeout, no
  claim that a `portable-owner` row is production-adopted — the
  disposition records which existing authority the front already
  selects, not a new switch.
```

## Input inventory (from landed P1 cards)

```text
398 case rows, observation_state:
  accepted            40  (12 generic-fixture inventory + 6 phase29bq
                           + 9 fast-gate + 9 subset + 2 smoke + 2 alias)
  rejected           196  (named route/semantic/handoff refusals)
  failed-before-loop 162  (import-view/preflight/parser/integrity/
                           TryCatch/retired-static stops)
  unobserved           0
```

## Disposition contract per observation class

Disposition vocabulary (fixed by manifest contract):
`portable-owner | accepted-typed-reject | nonproduction-future-evidence`.

1. `accepted` (40 — first write cohort)
   - Rows whose B-axis emitted canonical `define i64 @main()` through an
     existing authority → `portable-owner`, `target_owner` = the named
     authority (e.g. `callable-loop recipe` for
     `loop_simple_while_inline_explicit_step_min`; `canonical-main`
     callable route for the `if_return` family), `current_acceptance =
     accepted`, `observed_route` transcribed from the batch card's
     B_evidence.
   - Smoke-script/alias rows whose own gate is green →
     `portable-owner`, `target_owner` = the script's recorded
     production-route gate.
   - `parse_program2_if_else_return{,_var}` (2 rows): intermittent
     legacy `void @main` emission → hold unclassified behind a named
     nondeterminism repair row; do not manufacture a stable
     disposition. These two rows keep the "unclassified accepted"
     counter nonzero and are the card's recorded hold-back until the
     repair row lands.
   - A-side expected-output mismatches (batches 7-9) do not alter the
     B-axis disposition; they are recorded evidence for the repair
     row, not a disposition input.
2. `rejected` (196 — follow-on slice)
   - Rows refused by a named canonical owner → `accepted-typed-reject`
     with `target_owner` = the refusing owner token (e.g.
     `callable-loop-handoff/nested-loop-profile-not-admitted`,
     `callable-loop/recipe source-unsupported-body-statement`).
   - Rows refused where the named owner is a compatibility/legacy
     surface → `nonproduction-future-evidence`; the refusal is
     evidence about the retired path, not the canonical front.
3. `failed-before-loop` (162 — follow-on slice)
   - The cross-field rule blocks a *closed* disposition: these rows
     carry `nonproduction-future-evidence` with `target_owner` = the
     named blocking owner (e.g. `ResolverDeferred TryCatch`,
     `mir/main-import-view/selected-header-missing`, parser `GUARD`
     token) and stay open for the named repair row that unblocks
     them. They do not count as checked dispositions for S6E
     closeout; the card records this explicitly rather than
     laundering them through `accepted-typed-reject`.

## Column vocabulary (proposed, frozen at write time)

- `decision`: `D0-DISPOSITION-CHECKED` (replaces `P0-INVENTORY-ONLY`
  per classified row).
- `target_owner`: named authority token as above; `-` only where the
  class carries no owner yet.
- `observed_route`: transcribed B_evidence route/refusal token, or
  `-` where the card recorded none.
- `parity_gate`: stays `not-run`; no parity gate exists for this
  corpus yet — do not invent one.
- `nested_bypass`: stays `unknown`/`-`; P1 did not record bypass
  states per row.
- Edge-record columns (`symbol, current_role, production_callers,
  test_callers, first_effect, cutover_action, retire_row,
  replacement_owner`): stay `-`; edge records remain zero and belong
  to GENERIC-LEGACY-CROSS-FAMILY-DEPENDENCY-S0.

## Guard contract delta (required same-commit)

`tools/checks/lib/generic_legacy_corpus_universe_guard.py` currently
hard-fails any non-P0 case value (`decision==P0-INVENTORY-ONLY`,
`disposition==nonproduction-future-evidence`,
`current_acceptance!=accepted`, `parity_gate==not-run`,
`observed_route`/`target_owner`/`nested_bypass` sentinel-only). The
first classification write is impossible without a checker update;
repo rule: contract change ships with the write in one commit under
`work_mode=fast`.

Also recorded, same repair scope: the guard test
`generic_legacy_corpus_universe_guard_test.py:37` asserts 389 case
records while the manifest holds 398 (the nine variant fixtures added
by the three App-Main0 D0 slices) — part of the recorded
inplace-replacement guard reds baseline debt; verify before trusting
the guard as green evidence.

## Closeout rule carried forward

Row contract: accepted cases use only `portable-owner` or
`accepted-typed-reject`; `unclassified accepted count = 0`. The two
held nondeterminism rows are the named exception — S6E cannot close
while they are unclassified, and M10b stays blocked on unclassified
Generic fixtures per the Loop selfhost pipeline SSOT. Future evidence
cannot retire currently accepted input; failed/unobserved cases block
closeout.
