---
Status: landed__2026-09-23
Task: GENERIC-LEGACY-ROUTE-OBSERVATION-P1
Date: 2026-09-23
Parent: JOINIR-LOOP-M8-GENERIC-RESIDUAL-S6E
PreviousCard: generic-legacy-route-observation-p1-batch9-2026-09-23.md
NextCard: same-row__next_row_selected_by_family_scheduler
Implementation permission: false; record route/bypass/RC/output for the
bounded serial batch only. No source, fixture, corpus-universe,
semantic-receipt, producer, selector, or route changes.
---

# Generic legacy route observation P1 — serial edge-row batch (corpus closeout)

## Six-line brief

```text
Decision: close the serial P1 route observation; the bounded batch is
  the last 5 unobserved corpus rows — the 4 generic-smoke entries
  (2 smoke-script + 2 compat-script-alias) and the 1 fixture-inventory
  row.
Source authority + canonical issuer: the P0-normalized case universe
  fixes membership; for smoke-script/alias rows the front is the
  script itself under the same binary as batches 1-9
  (NYASH_BIN=target/debug/hakorune); for the fixture-inventory row the
  front is the same two direct invocations as batches 1-9.
Non-authority: script names, alias text, VM output as route proof, and
  unobserved rows as Declined.
Fail-fast boundary: a timeout, crash, or a stop at a named owner stays
  unclassified for that axis; no manufactured result, no wrapper
  substitution, no parallel census.
Smallest next slice: observe the 5 rows serially; record each outcome
  below and set the manifest observation_state only.
Non-claims: no producer/Recipe/selector arm, no route_loop switch, no
  source/fixture/smoke change, no disposition classification
  (GENERIC-LEGACY-DISPOSITION-D0 owns that), no S6E/S6G/M9/M10
  closeout, no corpus re-census, no guard vocabulary change; a green
  smoke-script row is not a new production claim.
```

## Fixed boundary

```text
smoke-script:tools/smokes/v2/profiles/integration/joinir/
  generic_loop_continue_release_adopt_vm.sh
smoke-script:tools/smokes/v2/profiles/integration/joinir/
  generic_loop_continue_strict_shadow_vm.sh
compat-script-alias:phase29ca_generic_loop_continue_release_adopt_vm
compat-script-alias:phase29ca_generic_loop_continue_strict_shadow_vm
repo-scan:apps/tests/selfhost_trim_generic_loop_min.hako
  (fixture::selfhost_trim_generic_loop_min)
```

## Front definition and mapping

- Smoke-script/alias rows: `NYASH_BIN=./target/debug/hakorune bash
  <script>` — the script's own PASS/FAIL gate is the front outcome.
- Fixture-inventory row: same two invocations as batches 1-9
  (`--backend vm` run + `--dump-mir`).
- `observation_state` mapping unchanged.

## Observation log

Binary: `target/debug/hakorune` (`--features vm-reference`), HEAD
`9e277c5b4a`, 2026-09-23.

| row | front | outcome | state | notes |
| --- | --- | --- | --- | --- |
| smoke-script generic_loop_continue_release_adopt_vm | script | `[PASS] exit=4` | accepted | canonical release-adopt gate green under the vm-reference build (same result as the D0 slices) |
| smoke-script generic_loop_continue_strict_shadow_vm | script | `[FAIL]` at `[freeze:contract][static-call/legacy-fallback-retired] StringHelpers.to_i64/1` | failed-before-loop | recorded baseline debt; the front stops at the retired static call before the loop route |
| compat-script-alias release_adopt_vm | script (alias) | `[PASS] exit=4` | accepted | alias of the release-adopt script row |
| compat-script-alias strict_shadow_vm | script (alias) | `[FAIL]` at `to_i64/1` retired static call | failed-before-loop | alias of the strict-shadow script row; same baseline debt |
| fixture::selfhost_trim_generic_loop_min | two invocations | A: VM err `canonical-call` RC 1; B: `qualified-preflight actual=Loop` site=Body(3) | rejected | twins `selfhost_trim_generic_loop_min` (fast-gate tsv:5) and `trim-generic-loop` (subset sub:10) reproduce verbatim |

## Result

- 2/5 `accepted` (the release-adopt smoke-script row and its compat
  alias — the script's own gate is green).
- 1/5 `rejected` (the fixture-inventory row's fixture refuses at
  qualified preflight `Loop`, identical to its two twin rows).
- 2/5 `failed-before-loop` (the strict-shadow smoke-script row and its
  compat alias — both stop at the recorded `to_i64/1` retired
  static-call baseline debt).
- 0 `timeout`, **0 rows left `unobserved` anywhere in the corpus** —
  all 398 P0-normalized corpus rows (plus 4 aliases) now carry a
  recorded observation state.
- A-side note: `generic_loop_continue_release_adopt_vm.sh` fails when
  `NYASH_BIN` points at a release build without `vm-reference`
  ("VM keep/reference execution is not available"); the PASS result
  is with the vm-reference debug binary, matching the D0 slices.
- No fixture, source, corpus-universe, receipt, or route change was
  made. `observation_state` is the only manifest column touched.
