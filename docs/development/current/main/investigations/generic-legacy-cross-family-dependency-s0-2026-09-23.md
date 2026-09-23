---
Status: fast__c1_edge_write_landed__c2_c6_pending
Task: GENERIC-LEGACY-CROSS-FAMILY-DEPENDENCY-S0
Date: 2026-09-23
Parent: JOINIR-LOOP-M8-GENERIC-RESIDUAL-S6E
PreviousCard: generic-legacy-disposition-d0-2026-09-23.md (via
  generic-legacy-ifelse-return-nondeterminism-d0-2026-09-23.md)
NextCard: same-row__first_edge_write_cohort
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
- Remaining cohorts C2–C6 stay unwritten (zero edge rows); they are
  the next bounded write slices under this row.
