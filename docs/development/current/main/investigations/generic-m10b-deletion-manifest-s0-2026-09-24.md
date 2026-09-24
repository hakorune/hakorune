---
Status: open__design_stop__manifest_boundary_fixed
Task: GENERIC-M10B-DELETION-MANIFEST-S0
Date: 2026-09-24
Parent: LOOP-PRODUCTION-SELECTION-D0 (Decision recorded)
PreviousCard: loop-production-selection-d0-2026-09-24.md
NextCard: M10b-I0-R0 (JOINIR-LOOP-PORTABLE-RECIPE-CUTOVER0-I0-R0)
Contract source: `joinir-loop-selfhost-recipe-pipeline-ssot.md` M10b
  entry row — "GENERIC-M10B-DELETION-MANIFEST-S0 freezes the exact
  current symbols and caller counts immediately before cutover. The
  atomic commit uses that checked manifest; names copied from
  historical docs are never deletion authority."
---

# GENERIC-M10B-DELETION-MANIFEST-S0 — deletion manifest freeze design

## Six-line brief

```text
Decision: freeze the M10b delete set as a checked manifest — the 38
  `retire_row = M10b-I0-R0` edge rows from the legacy disposition TSV,
  re-derived against live source with per-symbol caller counts,
  classified into the SSOT's seven delete categories.
Source authority + canonical issuer: live `src/` symbol definitions and
  caller sites (rg/cargo-verified); the disposition TSV is inventory
  evidence, not deletion authority.
Non-authority: historical symbol names, the 19-route scheduler's
  internal ordering, test-only references as production evidence, and
  any caller count not re-derived in this slice.
Fail-fast boundary: a manifest row whose symbol or file no longer
  exists, a caller count that disagrees with the frozen value, or a
  symbol outside the seven categories stops the cutover — the manifest
  is the atomic commit's sole deletion authority.
Smallest next slice: one bounded freeze slice producing
  `design/fixtures/generic-m10b-deletion-manifest-v1.tsv` plus the
  manifest card's landed record.
Non-claims: no deletion, no `route_loop` edit, no switch, no
  neutralization, no new Verified*/Prepared* receipt, no production
  change; the freeze is read-only evidence.
```

## Delete-set categories (SSOT fixed)

The M10b atomic commit deletes exactly these symbol families in one
commit:

| # | SSOT category | Maps to disposition `current_role` |
| --- | --- | --- |
| 1 | ordered retry scheduler | `C2:shared-route-authority` (`route_entry/registry/{selection,handlers,types,loop_preflight,live_ordered_terminality/*,legacy_receipt,execution_witness,direct_loop_break_terminality,*tests}`) + the `route_loop` caller edge itself |
| 2 | Generic post-effect retry debt | `C2:retry-fallback` |
| 3 | private continuation/error-to-None edges | symbol-level rows inside the C2/C3 files |
| 4 | Generic V0/V1 registry handler/predicate edges | `C1:plan-generic_loop`, `C1:policies`, `C1:facts-canon-generic_loop` |
| 5 | nested Generic `.ok()`/retry edges | `C4:nested-mutation-bypass` (`nested_loop_depth1*`) |
| 6 | selected old JoinIR caller/physical edges | `C3:mutating-composer-lowerer` (`plan/features/generic_loop_*`, `recipe_tree/generic_loop_composer`, `skeletons/generic_loop`) |
| 7 | new-subtree AST reconstruction facades | symbol-level facades inside the C1/C3 files |
| 8 | env flag host | `C6:env-flag-host` (`NYASH_JOINIR_LOWER_GENERIC` inside `config/env/joinir_dev.rs` — flag edge only; the env registry file is retained) |

## Manifest schema

`docs/development/current/main/design/fixtures/generic-m10b-deletion-manifest-v1.tsv`,
one row per frozen symbol/file edge:

```text
manifest_row_id  symbol  file  delete_category(1-8)  edge_kind
  production_callers  test_callers  caller_evidence  disposition_row_ref
```

- `symbol`: exact Rust path or file-stem identity, re-derived from live
  source in this slice.
- `production_callers`/`test_callers`: live counts at freeze time.
- `caller_evidence`: the rg/cargo command class used (recorded as a
  stable token, not a pasted log).
- `disposition_row_ref`: the `edge:` id from
  `generic-loop-legacy-disposition-v1.tsv` when one exists; `-` for
  symbol-level rows added by this freeze.
- A `totals` trailer row pins the per-category counts so M10b's commit
  can verify "old-symbol census counts are zero" against the same
  queries.

## Boundary and verification

- The freeze re-derives every row against HEAD at freeze time. Any
  drift between the disposition TSV's file set and live source (a
  renamed/removed file, a new caller, a symbol that vanished) is a
  fail-fast stop, not a silent manifest edit.
- 38 disposition rows are the starting file set; symbol-level rows are
  added only where the seven categories require finer granularity
  (categories 3 and 7). No row is dropped without recording why in the
  manifest card.
- Test-only files inside the 38-row set (registry `*tests`,
  `generic_loop_p0c_tests`, `generic_loop_whole_parity_tests`,
  `effect_order_matrix_tests`, `scoped_nongeneric_cutover_tests`) are
  frozen as delete-set members too — they die with their subjects in
  the atomic commit.
- The manifest is verified by re-running the recorded query class and
  diffing counts; `git diff --check` and the pointer guard gate the
  landing commit.

## Fail-fast boundary

- A live caller of a frozen symbol that is not itself in the delete
  set (or a retained shared owner) is a cutover blocker, recorded in
  the card — it does not get deleted to make the manifest green.
- `route_loop` (`route_entry/router.rs:255`, invoked at
  `routing.rs:552`) is the switch point: its own deletion/replacement
  is M10b's atomic commit, not this manifest.
- Symbols whose `retire_row` is `M11`/`M12`/`retained` stay out of the
  delete set even if they share files or names with delete-set
  members.

## Non-claims

- No source edit, no deletion, no neutralization, no `route_loop` or
  registry change, no production switch.
- No new `Verified*`/`Prepared*` receipt and no semantic issuance —
  the manifest is caller/symbol census evidence only.
- No re-classification of corpus rows, routes, or the disposition
  TSV's non-M10b buckets.
- No M10b activation: this row ends at a checked manifest plus its
  landed record; the atomic commit is `M10b-I0-R0`'s own card.

## Exit

- Bounded freeze slice under `fast`: enumerate the 38 file rows + the
  category 3/7 symbol rows against live source, write the manifest
  TSV, record totals, update this card to `landed`, sync pointers.
- Gates for the freeze slice: `git diff --check`, pointer guard, and a
  repeat of the recorded caller-count queries on the committed tree
  (manifest self-check).
