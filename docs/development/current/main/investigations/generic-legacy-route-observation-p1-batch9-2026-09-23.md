---
Status: landed__2026-09-23
Task: GENERIC-LEGACY-ROUTE-OBSERVATION-P1
Date: 2026-09-23
Parent: JOINIR-LOOP-M8-GENERIC-RESIDUAL-S6E
PreviousCard: generic-legacy-route-observation-p1-batch8-2026-09-23.md
NextCard: same-row__next_serial_batch_selected_by_family_scheduler
Implementation permission: false; record route/bypass/RC/output for the
bounded serial batch only. No source, fixture, corpus-universe,
semantic-receipt, producer, selector, or route changes.
---

# Generic legacy route observation P1 — serial selfhost-subset batch 4 (final subset chunk)

## Six-line brief

```text
Decision: continue the serial P1 route observation; the bounded batch is
  the last 44 unobserved selfhost-subset rows in
  planner_required_selfhost_subset.tsv line order (lines 161-204).
Source authority + canonical issuer: the P0-normalized case universe
  fixes membership; the front is the same two direct invocations per
  case as batches 1-8 (release VM run + callable-lane MIR dump);
  expected outputs are compared per row against the subset manifest's
  own expected column.
Non-authority: fixture/backend names, planner_tag expectations, VM
  output as route proof, already-observed twins as this-row evidence,
  and unobserved rows as Declined.
Fail-fast boundary: a timeout, crash, or a stop before the GenericLoop
  route at a named owner stays unclassified for that axis; no
  manufactured result, no wrapper substitution, no parallel census.
Smallest next slice: observe serially subset lines 161-204; record
  each outcome below and set the manifest observation_state only.
Non-claims: no producer/Recipe/selector arm, no route_loop switch, no
  source/fixture/smoke change, no disposition classification
  (GENERIC-LEGACY-DISPOSITION-D0 owns that), no S6E/S6G/M9/M10 closeout,
  no corpus re-census, no guard vocabulary change.
```

## Fixed boundary

The bounded batch is exactly the last 44 `corpus=selfhost,
mode=selfhost-subset` rows with `observation_state=unobserved`:

```text
sub:161-188 local_expr_*_cleanup continuation (28 rows: compare_chain,
        compare_or, compare_mixed_logic, null_compare_{and,or,mixed_
        logic}, null_not_compare, null_parenthesized_logic,
        arith_compare_{and,or}, compare_rel_mixed_logic, unary_compare_
        mixed, double_not_mixed_logic, parenthesized_compare_mixed,
        not_parenthesized_compare, call_compare_{and,or}, new_compare_
        mixed, string_compare_logic, array_len_compare, map_value_
        compare, literal_bool_mixed, blockexpr_compare, compare_fini,
        call_fini, blockexpr_fini, local_fini_multi_lifo,
        local_expr_null_fini)
sub:189-190 control_loop_{if_break_continue,local_fini}_cleanup
sub:191-204 box_member_* deep cleanup chains (14 rows:
        loop_cleanup, local_fini{,_blockexpr{,_compare_logic{,_unary_
        call{,_literals{,_nested_tail{,_nested_loop_branch{,_method_
        chain_tail{,_side_effect_tail{,_nested_join_tail{,_dual_tail_
        sync{,_guard_sync_tail{,_mirror_sync_tail}}}}}}}}}}})
```

After this batch only the 4 generic-smoke aliases and 1
fixture-inventory row remain `unobserved` in the corpus.

## Front definition and mapping

Identical to batches 1-8: two direct invocations per case (release VM
run; `--dump-mir`), same binary, repo root, timeout 10s, no wrappers;
same `B_outcome` vocabulary and `observation_state` mapping. Expected
outputs come from `planner_required_selfhost_subset.tsv` for these
rows.

## Finite outcomes and acceptance

Identical to batches 2-8 — all 44 recorded closes `landed`; named-owner
stops stay unclassified on the missing axis; a front break stops the
batch. Non-claims identical.

## Observation log

Binary: `target/debug/hakorune` (`--features vm-reference`), HEAD
`b80659301e`, 2026-09-23. All 44 rows observed serially in subset-
manifest line order; no timeouts, no signals. `exp` is the subset
manifest expected value. B-side evidence is uniform: every fixture in
this chunk carries try/finally (`TryCatch`) at a non-loop Body site,
which `callable-semantic-package` refuses via `ResolverDeferred
UnsupportedStatement TryCatch` before Loop evaluation — recorded as
`failed-before-loop`; the two deepest box_member fixtures do not even
parse.

| sub | case | A_rc | A_tail | B_evidence | state |
| --- | --- | --- | --- | --- | --- |
| 161 | local_expr_compare_chain_cleanup | 0 | `1113` (match) | `ResolverDeferred TryCatch` site=Body(1) | failed-before-loop |
| 162 | local_expr_compare_or_cleanup | 0 | `1214` (match) | `TryCatch` Body(1) | failed-before-loop |
| 163 | local_expr_compare_mixed_logic_cleanup | 0 | `1315` (match) | `TryCatch` Body(1) | failed-before-loop |
| 164 | local_expr_null_compare_and_cleanup | 0 | `1416` (match) | `TryCatch` Body(1) | failed-before-loop |
| 165 | local_expr_null_compare_or_cleanup | 0 | `1517` (match) | `TryCatch` Body(1) | failed-before-loop |
| 166 | local_expr_null_compare_mixed_logic_cleanup | 0 | `1659` (match) | `TryCatch` Body(1) | failed-before-loop |
| 167 | local_expr_null_not_compare_cleanup | 0 | `1888` (**exp `1706` — mismatch**) | `TryCatch` Body(1) | failed-before-loop |
| 168 | local_expr_null_parenthesized_logic_cleanup | 0 | `2000` (match) | `TryCatch` Body(1) | failed-before-loop |
| 169 | local_expr_arith_compare_and_cleanup | 0 | `2350` (match) | `TryCatch` Body(1) | failed-before-loop |
| 170 | local_expr_arith_compare_or_cleanup | 0 | `2366` (match) | `TryCatch` Body(1) | failed-before-loop |
| 171 | local_expr_compare_rel_mixed_logic_cleanup | 0 | `2397` (match) | `TryCatch` Body(1) | failed-before-loop |
| 172 | local_expr_unary_compare_mixed_cleanup | 0 | `2409` (**exp `2306` — mismatch**) | `TryCatch` Body(1) | failed-before-loop |
| 173 | local_expr_double_not_mixed_logic_cleanup | 0 | `2523` (**exp `2306` — mismatch**) | `TryCatch` Body(1) | failed-before-loop |
| 174 | local_expr_parenthesized_compare_mixed_cleanup | 0 | `2523` (match) | `TryCatch` Body(1) | failed-before-loop |
| 175 | local_expr_not_parenthesized_compare_cleanup | 0 | `2372` (**exp `2306` — mismatch**) | `TryCatch` Body(1) | failed-before-loop |
| 176 | local_expr_call_compare_and_cleanup | 1 | VM err `canonical-call` | `TryCatch` Body(1) | failed-before-loop |
| 177 | local_expr_call_compare_or_cleanup | 1 | VM err `canonical-call` | `TryCatch` Body(1) | failed-before-loop |
| 178 | local_expr_new_compare_mixed_cleanup | 1 | VM err `canonical-call` | `TryCatch` Body(1) | failed-before-loop |
| 179 | local_expr_string_compare_logic_cleanup | 1 | `ssa/phi_input/non_rematerializable` merge_modified_vars | `TryCatch` Body(1) | failed-before-loop |
| 180 | local_expr_array_len_compare_cleanup | 1 | VM err `NewBox IntrinsicArray unimplemented` | `TryCatch` Body(1) | failed-before-loop |
| 181 | local_expr_map_value_compare_cleanup | 1 | VM err `canonical-call` | `TryCatch` Body(1) | failed-before-loop |
| 182 | local_expr_literal_bool_mixed_cleanup | 0 | `2631` (match) | `TryCatch` Body(1) | failed-before-loop |
| 183 | local_expr_blockexpr_compare_cleanup | 0 | `2735` (match) | `TryCatch` Body(1) | failed-before-loop |
| 184 | local_expr_compare_fini_cleanup | 0 | `1136` (match) | `TryCatch` Body(1) | failed-before-loop |
| 185 | local_expr_call_fini_cleanup | 1 | VM err `canonical-call` | `TryCatch` Body(1) | failed-before-loop |
| 186 | local_expr_blockexpr_fini_cleanup | 0 | `1430` (match) | `TryCatch` Body(1) | failed-before-loop |
| 187 | local_fini_multi_lifo_cleanup | 0 | `cbaY` (match) | `TryCatch` Body(2) | failed-before-loop |
| 188 | local_expr_null_fini_cleanup | 0 | `1548` (match) | `TryCatch` Body(1) | failed-before-loop |
| 189 | control_loop_if_break_continue_cleanup | 0 | `129` (**exp `171` — mismatch**) | `TryCatch` Body(2) | failed-before-loop |
| 190 | control_loop_local_fini_cleanup | 0 | `127` (**exp `156` — mismatch**) | `TryCatch` Body(1) | failed-before-loop |
| 191 | box_member_loop_cleanup | 1 | VM err `canonical-call` | `TryCatch` Body(3) | failed-before-loop |
| 192 | box_member_local_fini_cleanup | 1 | VM err `canonical-call` | `TryCatch` Body(2) | failed-before-loop |
| 193 | box_member_local_fini_blockexpr_cleanup | 1 | `ssa/phi_input/non_rematerializable` finalize | `TryCatch` Body(2) | failed-before-loop |
| 194 | box_member_local_fini_blockexpr_compare_logic_cleanup | 1 | VM err `canonical-call` | `TryCatch` Body(2) | failed-before-loop |
| 195 | box_member_..._unary_call_cleanup | 1 | VM err `canonical-call` | `TryCatch` Body(2) | failed-before-loop |
| 196 | box_member_..._literals_cleanup | 1 | VM err `NewBox IntrinsicArray unimplemented` | `TryCatch` Body(2) | failed-before-loop |
| 197 | box_member_..._literals_nested_tail_cleanup | 1 | VM err `NewBox IntrinsicArray unimplemented` | `TryCatch` Body(2) | failed-before-loop |
| 198-202 | box_member_..._nested_loop_branch{,_method_chain_tail{,_side_effect_tail{,_nested_join_tail{,_dual_tail_sync}}}} (5 rows) | 1 | `generic_loop_v1 pipeline failed: [blockexpr] loop-like stmt in BlockExpr prelude requires route-specific lowering` | `TryCatch` Body(2) | failed-before-loop |
| 203 | box_member_..._dual_tail_sync_guard_sync_tail_cleanup | 1 | Parse error `Unexpected token GUARD, expected identifier` line 134 (`local guard = 0`) | same parse failure on callable lane | failed-before-loop |
| 204 | box_member_..._guard_sync_tail_mirror_sync_tail_cleanup | 1 | same `GUARD` parse error | same parse failure on callable lane | failed-before-loop |

## Result

- 0/44 `accepted`, 0/44 `rejected`: no row reaches Loop evaluation.
- 44/44 `failed-before-loop`: `ResolverDeferred UnsupportedStatement
  TryCatch` at non-loop sites covers 42 rows — the entire
  `local_expr_*`/`fini`/`box_member_*` cleanup family carries
  try/finally before any loop work — plus 2 parse rejects
  (`Unexpected token GUARD, expected identifier`: `guard` is a
  reserved token and cannot be a local name).
- 0 `timeout`, 0 left `unobserved` — the selfhost-subset cohort (198
  rows, subset tsv:6-204) is now fully observed.
- Expected-output mismatches (A-side RC 0, manifest expected differs):
  sub:167 `1706`→`1888`, 172 `2306`→`2409`, 173 `2306`→`2523`,
  175 `2306`→`2372`, 189 `171`→`129`, 190 `156`→`127` — six more
  `local_expr_*`/`control_loop_*` rows, same digit-difference shape as
  batch 8. Recorded verbatim; no repair.
- 18/44 ran green to the exact subset expected output on the compat
  lane; A-side also surfaced `ssa/phi_input/non_rematerializable`
  (179,193), `NewBox IntrinsicArray` interp limits (180,196,197), and
  the `generic_loop_v1` `[blockexpr] loop-like stmt in BlockExpr
  prelude` pipeline refuse (198-202) — all compat-lane context, not
  route evidence.
- No fixture, source, corpus-universe, receipt, or route change was
  made. `observation_state` is the only manifest column touched.
