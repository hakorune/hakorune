---
Status: landed__2026-09-23__edge_inventory_complete
Task: GENERIC-LEGACY-CROSS-FAMILY-DEPENDENCY-S0
Date: 2026-09-23
Parent: JOINIR-LOOP-M8-GENERIC-RESIDUAL-S6E
PreviousCard: generic-legacy-disposition-d0-2026-09-23.md (via
  generic-legacy-ifelse-return-nondeterminism-d0-2026-09-23.md)
NextCard: family_scheduler__next_ordered_row
Implementation permission: false; fix the edge-record inventory
boundary, classification buckets, and bounded write slice only. No
manifest write, no caller-graph edit, no source/fixture/route change,
no deletion from this card.
---

# Generic legacy cross-family dependency S0 — edge classification design

## Six-line brief

```text
Decision: open the ordered S0 row. D0's classification contract is
  met (398/398 case rows carry a checked decision; unclassified
  accepted = 0), so the family scheduler advances to the next row in
  the GENERIC-LEGACY ladder: classify every legacy Generic
  edge/symbol by its repository caller graph into Generic-only /
  neutralize-first / M11 / M12 / named JoinModule ownership, by
  writing the manifest's sealed `record_kind = edge` records.
Source authority + canonical issuer: the ordered row map and the
  retirement-ledger cohort table in
  generic-loop-source-to-portable-recipe-ssot.md (candidate paths are
  the authority, not name-matching), the sealed 25-column union
  schema + `record_kind = edge` shape in the same SSOT, and the
  repository caller graph (rg-level reference counts) as the only
  caller evidence.
Non-authority: filename stem similarity as ownership proof, `mod.rs`
  re-export presence as "used", doc mentions as callers, or any
  route/admission re-decision — S0 classifies existing edges, it
  does not re-decide which route is canonical.
Fail-fast boundary: if a candidate file's production callers cannot
  be counted to a named caller set or the count contradicts the
  retirement-ledger cohort it was grouped under, stop that file as
  an unclassified edge with the named reason — do not guess a
  bucket. Zero-caller files still carry the named evidence
  (`production_callers=0`, callers enumerated, not assumed).
Smallest next slice: under work_mode=fast, write the first bounded
  edge cohort — the "old AST/facts authority" group
  (`plan/generic_loop/**`, `facts/canon/generic_loop/**`,
  `generic_loop_canon/**`, `policies/generic_loop_*`; ~85 files) —
  with per-file caller counts, then continue cohort by cohort.
Non-claims: no deletion, no wholesale removal of
  `generic_loop_canon/**` or name-matched subtrees, no environment
  reference removal, no fixture/source change, no case-row rewrite,
  no S6E/S6G/M10/Row-F closeout claim. Deletion belongs to
  GENERIC-LEGACY-DEAD-CODE-R1 / M10b-I0-R0 only.
```

## Fixed evidence boundary

```text
manifest: docs/development/current/main/design/fixtures/
  generic-loop-legacy-disposition-v1.tsv (union schema sealed at P0)
edge record shape (guard _check_edge):
  record_kind=edge; id + canonical_fixture(path) required;
  all case columns = sentinel "-";
  symbol, current_role, production_callers, test_callers,
  first_effect, cutover_action, retire_row, replacement_owner
  required (non-sentinel)
granularity: one edge record per candidate file
candidate universe (retirement-ledger cohorts, SSOT):
  C1 old AST/facts authority:
    plan/generic_loop/** (58 files), facts/canon/generic_loop/** (8),
    generic_loop_canon/** (17), policies/generic_loop_* (2)
  C2 ordered route authority:
    joinir/route_entry/registry/** Generic V0/V1 route rows +
    execution_witness.rs, legacy_receipt.rs
  C3 mutating composer/lowerer:
    recipe_tree/generic_loop_composer.rs, skeletons/generic_loop.rs,
    features/generic_loop_*, features/generic_loop_body/**
  C4 retry/fallback + direct nested mutation bypasses:
    nested_loop_depth1*, nested adoption, located composer
  C5 recent test-only source evidence:
    resolved_semantics/generic_resolved_carrier_*,
    loop_structural_facts/generic_resolved_carrier_*
  C6 shared infrastructure (retain through R1):
    RecipeBody/RecipeBlock, located handoff, separate
    join_ir/lowering, NYASH_JOINIR_LOWER_GENERIC, non-Generic route
    policy and physicalizers
```

## Classification buckets (cutover_action / retire_row / replacement_owner)

```text
generic-only-r1:
  cutover_action=generic-only; retire_row=GENERIC-LEGACY-DEAD-CODE-R1;
  replacement_owner=<named G0/policy/resolver product or "-" with
  named parity product>
neutralize-first:
  cutover_action=neutralize-first; retire_row=M10b-I0-R0;
  replacement_owner=<named selected product>
shared-retain-m11 / shared-retain-m12:
  cutover_action=shared-retain; retire_row=M11-R1 or M12 named row;
  replacement_owner=<named JoinModule or reconsideration row>
named-joinmodule:
  cutover_action=assign-joinmodule; retire_row=<named row>;
  replacement_owner=<named JoinModule owner>
test-evidence-retire:
  cutover_action=migrate-fixtures-then-retire; retire_row=<named row>;
  replacement_owner=<G0 product receiving the fixture evidence>
```

## Design boundary

- Caller counts are production vs test split:
  `production_callers` counts non-test callers, `test_callers`
  counts `#[cfg(test)]`/`*_tests.rs`/tools callers. A file with
  production_callers=0 and only test callers is still evidence —
  the count is recorded, not rounded to a bucket.
- `first_effect` names the first observable effect of the edge
  (e.g. `builder-mutation`, `route-selection`, `receipt-issue`,
  `test-only`); it is not a route claim.
- Case rows are P0/D0-frozen: S0 touches only `record_kind=edge`
  records. Case-row columns on edge rows stay sentinel per the
  sealed schema (one parser, no second shape).
- The row's Done condition per SSOT: shared `UpdateCanon`,
  `RecipeBody/RecipeBlock`, located handoff, separate
  `join_ir/lowering`, and `NYASH_JOINIR_LOWER_GENERIC` are assigned
  outside R1 — shared infrastructure must land in a named
  M11/M12/JoinModule bucket, never `generic-only-r1`.
- Guard change rule mirrors D0: any new vocabulary (bucket tokens)
  is added to `generic_legacy_corpus_universe_guard.py` in the same
  commit as the manifest writes that use it.
- S0 does not close S6E (which still owes one portable
  producer+observation cohort) and does not unblock Row F —
  `LOOP-PRODUCTION-SELECTION-D0` remains gated on the M10 seal
  series and M8 all-route coverage.

## Landed writes (2026-09-23) — C1 old AST/facts authority

- Counting method (documented, deterministic): anchored module-path
  tokens only — `generic_loop_canon::<rel>`,
  `canon::generic_loop::<rel>`, `generic_loop::<rel>` attributed to
  `plan::generic_loop` (prefix `plan::` or bare token inside
  `plan/**`; `canon::`/`skeletons::` prefixes excluded), the two
  `policies/generic_loop_*` stems, plus `mod <stem>` declarations
  restricted to the true parent module file. Bare-stem fallback was
  rejected (non-discriminating for `utils`/`error`/`tests` stems).
  Callers inside the 147-file legacy-Generic candidate set are
  excluded — edge records measure the boundary, not internal edges.
- C1 census result (83 files): **64 generic-only** (zero outside
  production callers; test callers recorded where present) /
  **8 neutralize-first** (outside production callers must be
  disconnected at M10b before R1 deletion) / **11 shared-retain**
  (`plan/generic_loop/located_representation/**` held as the
  located-handoff candidate subtree for M11-R1).
- Outside production callers recorded: `plan/generic_loop/mod` 15,
  `generic_loop_canon/mod` 5 (shared `facts/extractors/*` +
  `plan/facts/loop_continue_only_facts` call
  `generic_loop_canon::canon_update_for_loop_var`),
  `plan/generic_loop/facts{,/extract}` 4+2, `facts_types` 3,
  `facts/canon/generic_loop.rs` 1 (parent `facts/canon.rs` decl),
  both `policies/generic_loop_*` 1 each.
- Guard: `generic_legacy_corpus_universe_guard_test.py` 6/6 OK;
  no guard vocabulary change needed (`_check_edge` requires
  non-sentinel fields only).
- Remaining cohorts C3–C6 stay unwritten; they are the next bounded
  write slices under this row.

## Landed writes (2026-09-23) — C2 ordered route authority

- Cohort: all 58 files under `joinir/route_entry/registry/**` —
  the Generic V0/V1 route rows plus the shared ordered-route
  machinery they are embedded in (the split is exactly what S0
  exists to classify).
- Result: **25 generic-only** (24 `generic_*_tests.rs` +
  `handlers/generic.rs` V0/V1 handler), **2 neutralize-first**
  (`execution_witness.rs`, `legacy_receipt.rs` — ledger C4
  retry/fallback debt deleted at `M10b-I0-R0`), **30 shared-retain**
  (13 files carry Generic rows/mentions → `M10b-I0-R0`; 17 files
  carry no Generic content → `retained`; `legacy_observer.rs` →
  `M12` for the non-Generic route retirements).
- Shared-retain split: 13 files carry Generic rows/mentions →
  `retire_row=M10b-I0-R0` (Generic edges disconnected at cutover,
  file retained); 17 files carry no Generic content → `retained`
  (no retirement row; assigned outside all retire rows — new
  `retire_row` value, vocabulary-only, guard unchanged).
- Sole outside-boundary production edge into C2:
  `loop_recipe_contract/route_id.rs` references `GENERIC_LOOP_V0/V1`
  constants in `types.rs` — recorded on the shared `types.rs`
  record (shared-retain/M10b). Facade `registry/mod.rs` absorbs
  174 prod + 27 test symbol-level references; per-file edges funnel
  through it, so symbol-level probes (`route_generic_loop_v{0,1}`,
  `pred_generic_loop_v{0,1}`, `RouteExecutionWitnessV1`,
  `LegacyGeneric*V1`) were counted separately and show zero
  outside-boundary callers.
- `handlers/generic.rs` records `prod=0`: its only callers are
  inside the registry boundary (`handlers.rs` wrappers), which the
  shared `handlers.rs`/`selection.rs`/`mod.rs` records neutralize
  at M10b — the file then dies caller-zero at R1.
- Guard: 6/6 OK. Cumulative edge rows: 141 (83 C1 + 58 C2).
- `retire_row` vocabulary in use:
  `GENERIC-LEGACY-DEAD-CODE-R1` | `M10b-I0-R0` | `M11-R1` | `M12` |
  `retained`.

## Landed writes (2026-09-23) — C3/C4/C5/C6

- C3 mutating composer/lowerer + C4 nested mutation bypasses
  (35 unique files: `recipe_tree/generic_loop_composer.rs`,
  `skeletons/generic_loop.rs`, `features/generic_loop_*`,
  `features/generic_loop_body/**`, `features/nested_loop_depth1*`,
  `plan/nested_loop_depth1/**`):
  20 generic-only / 15 neutralize-first. Outside production callers
  recorded on `features/generic_loop_context` (3, incl.
  `raw_loop_child_entry` + `normal_callable_loop_physical_adapter`),
  `generic_loop_body/mod` (2), `features/nested_loop_depth1` (8,
  loop_cond_* + loop_true features), `plan/nested_loop_depth1/mod`
  (10), `nested_loop_depth1_preheader` (3), plus parent `mod.rs`
  declaration edges.
- C5 recent test-only source evidence (11 files:
  `resolved_semantics/generic_resolved_carrier_*`,
  `loop_structural_facts/generic_resolved_carrier_*`): all are
  `#[cfg(test)]`-gated witnesses (incl. the `#[path]`-wired
  `generic_resolved_carrier_source_lease` test module); classified
  `migrate-fixtures-then-retire` → `GENERIC-LEGACY-DEAD-CODE-R1`,
  owner `generic_g0 parity evidence`. Test callers recorded
  (cfg-gated `mod` decls + `explicit_parameter_type_map.rs` test
  import).
- C6 shared infrastructure (83 new rows + 5 reclassifications):
  - `UpdateCanon` is genuinely shared: `generic_loop_canon/types.rs`
    (definition), `update/{mod,literal_step,literal_match}.rs`, and
    `generic_loop_canon/mod.rs` reclassified from C1 buckets to
    `shared-retain`/`M12` — non-Generic extractors
    (`facts/extractors/{mod,if_phi_join,common_helpers/increment}`,
    `plan/facts/loop_continue_only_facts`) call
    `canon_update_for_loop_var`, so the UpdateCanon portion cannot
    die at Generic R1.
  - `plan/parts/associated_source{,/**}` (20 files): the located
    handoff → `shared-retain`/`M11-R1`.
  - `join_ir/lowering/**` (56 files): separate lowering →
    `shared-retain`/`retained`.
  - `recipes/body.rs` + `recipe_tree/block.rs`
    (RecipeBody/RecipeBlock), `loop_route_policy/mod.rs`,
    `loop_accum_physicalizer*` → `shared-retain`/`retained`.
  - `config/env/joinir_dev.rs` hosts `NYASH_JOINIR_LOWER_GENERIC`
    (120 prod callers) → `shared-retain`/`M10b-I0-R0` (the Generic
    flag is removed at cutover; the env registry file is retained).
- Final edge inventory: **270 rows**, every legacy-Generic candidate
  file (180) carries an edge record; buckets: 105 generic-only-R1,
  24 neutralize-first-M10b, 11 migrate-fixtures-then-retire-R1,
  31 shared-retain-M11, 14 shared-retain-M10b, 6 shared-retain-M12,
  79 shared-retain-retained.
- The row's Done condition is met in the manifest: every shared item
  named by the contract (`UpdateCanon`, `RecipeBody/RecipeBlock`,
  located handoff, `join_ir/lowering`,
  `NYASH_JOINIR_LOWER_GENERIC`) is assigned outside R1. No file was
  deleted; case rows untouched; guard 6/6 OK.
