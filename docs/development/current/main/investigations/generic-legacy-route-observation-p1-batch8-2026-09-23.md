---
Status: landed__2026-09-23
Task: GENERIC-LEGACY-ROUTE-OBSERVATION-P1
Date: 2026-09-23
Parent: JOINIR-LOOP-M8-GENERIC-RESIDUAL-S6E
PreviousCard: generic-legacy-route-observation-p1-batch7-2026-09-23.md
NextCard: same-row__next_serial_batch_selected_by_family_scheduler
Implementation permission: false; record route/bypass/RC/output for the
bounded serial batch only. No source, fixture, corpus-universe,
semantic-receipt, producer, selector, or route changes.
---

# Generic legacy route observation P1 — serial selfhost-subset batch 3

## Six-line brief

```text
Decision: continue the serial P1 route observation; the bounded batch is
  the next 60 unobserved selfhost-subset rows in
  planner_required_selfhost_subset.tsv line order (lines 101-160).
Source authority + canonical issuer: the P0-normalized case universe
  fixes membership; the front is the same two direct invocations per
  case as batches 1-7 (release VM run + callable-lane MIR dump);
  expected outputs are compared per row against the subset manifest's
  own expected column.
Non-authority: fixture/backend names, planner_tag expectations, VM
  output as route proof, already-observed twins as this-row evidence,
  and unobserved rows as Declined.
Fail-fast boundary: a timeout, crash, or a stop before the GenericLoop
  route at a named owner stays unclassified for that axis; no
  manufactured result, no wrapper substitution, no parallel census.
Smallest next slice: observe serially subset lines 101-160; record
  each outcome below and set the manifest observation_state only.
Non-claims: no producer/Recipe/selector arm, no route_loop switch, no
  source/fixture/smoke change, no disposition classification
  (GENERIC-LEGACY-DISPOSITION-D0 owns that), no S6E/S6G/M9/M10 closeout,
  no corpus re-census, no guard vocabulary change.
```

## Fixed boundary

The bounded batch is exactly the 60 `corpus=selfhost,
mode=selfhost-subset` rows with `observation_state=unobserved` whose
subset-manifest line is in 101-160:

```text
sub:101-137 mostly fast-gate twins (parse_string2_return_prelude,
        parse_local_fini{,_no_init,_nested_scope}, parser_stmt_equals,
        phi_injector_* (5), phi_missing_step_join_cursor,
        return_continue_hetero{,_nested_return_depth3,_simple},
        rewriteknown_{itoa_complex_step,trim_loop_cond_and_methodcall,
        try_apply_loop_true_tail_return}, seek_array_end_return_if,
        stageb_{bundle_mod_if,trace_if_no_else}, scan_with_quote{,
        _loop,_loop_full}, usingcollector_loop_full,
        scan_methods_loop_block, scan_methods_nested_loop_idx{19,28},
        scan_methods_program_block, while_cap, phi_collect_outer_loop,
        scan_methods nested-loop legacy-alias stems (130-134),
        scan_methods_nested_loop_state_machine, scan_ident,
        phi_injector_nested_no_exit_var_step)
sub:138-160 new selfhost cleanup/fini fixtures (23 rows:
        fini_only, local_fini_slot_capture, fini_cleanup_coexist,
        local_{no_init,multibind,multibind_init,multibind_mixed_init,
        triple_noinit}_cleanup, local_expr_{compare,logic,unary_not,
        unary_minus,unary_mix,call_new,array_map_literal,string_
        concat_len,string_subcmp,null_logic,string_trim_chain,
        bool_combo,logic_precedence,logic_parenthesized,double_not_
        compare}_cleanup)
```

Remaining subset rows (lines 161+) plus the 4 generic-smoke aliases
and 1 fixture-inventory row stay `unobserved` for later serial slices.

## Front definition and mapping

Identical to batches 1-7: two direct invocations per case (release VM
run; `--dump-mir`), same binary, repo root, timeout 10s, no wrappers;
same `B_outcome` vocabulary and `observation_state` mapping. Expected
outputs come from `planner_required_selfhost_subset.tsv` for these
rows.

## Finite outcomes and acceptance

Identical to batches 2-7 — all 60 recorded closes `landed`; named-owner
stops stay unclassified on the missing axis; a front break stops the
batch. Non-claims identical.

## Observation log

Binary: `target/debug/hakorune` (`--features vm-reference`), HEAD
`c6434e6ad7`, 2026-09-23. All 60 rows observed serially in subset-
manifest line order; no timeouts, no signals. `exp` is the subset
manifest expected value.

| sub | case | A_rc | A_tail | B_evidence | state |
| --- | --- | --- | --- | --- | --- |
| 101 | parse_string2_return_prelude_call | 1 | `[static-call/legacy-fallback-retired] Main.parse_string2_min/1` | `mir/main-import-view/selected-header-missing` | failed-before-loop |
| 102 | parse_local_fini | 0 | `SEBAF0` | `semantic-package ResolverDeferred UnsupportedStatement TryCatch` site=Body(1) | failed-before-loop |
| 103 | parse_local_fini_no_init | 0 | `BA` | `TryCatch` site=Body(1) | failed-before-loop |
| 104 | parse_local_fini_nested_scope | 0 | `IAMO` | `TryCatch` site=Body(1) | failed-before-loop |
| 105 | parser_stmt_equals | 1 | `[static-call/legacy-fallback-retired] ParserStmtBox.equals/1` | `mir/main-qualified-recipe/relation-missing` | failed-before-loop |
| 106-109 | phi_injector_{k_loop_no_exit,len_loop,nested_loop_count,var_step_len_loop} | 1 | VM err `canonical-call` | `qualified-preflight actual=New` site=Body(0) Init | failed-before-loop |
| 110 | phi_missing_step_join_cursor | 1 | VM err `canonical-call` | `route-not-front-selected CarrierRelation(MissingIncrement)` site=Body(2) | rejected |
| 111-113 | return_continue_hetero{,_nested_return_depth3,_simple} | 1 | VM err `canonical-call` | `qualified-preflight actual=New` site=Body(0) Init | failed-before-loop |
| 114 | rewriteknown_itoa_complex_step | 1 | `[static-call/legacy-fallback-retired] ItoaMini.run/0` | `ResolverDeferred UnsupportedExpression` in LoopBody(1) | rejected |
| 115 | rewriteknown_trim_loop_cond_and_methodcall | 1 | `[static-call/legacy-fallback-retired] TrimMini.run/0` | `mir/main-import-view/selected-header-missing` | failed-before-loop |
| 116 | rewriteknown_try_apply_loop_true_tail_return | 1 | `[static-call/legacy-fallback-retired] RewriteKnownMini.run/0` | `mir/main-import-view/selected-header-missing` | failed-before-loop |
| 117 | seek_array_end_return_if | 1 | VM err `canonical-call` | `qualified-preflight actual=New` site=Body(0) Init | failed-before-loop |
| 118 | stageb_bundle_mod_if | 1 | `DuplicateMainBox` | `mir/normal-root/consume IntegrityInvalid` | failed-before-loop |
| 119 | stageb_trace_if_no_else | 1 | VM err `canonical-call` | `qualified-preflight actual=Loop` site=Body(2) | rejected |
| 120-122 | scan_with_quote{,_loop,_loop_full} | 1 | `[static-call/legacy-fallback-retired] Utils.i2s/1` (122: `ParserCommonUtilsBox.i2s/1`) | `route-not-front-selected GenericLoopV1NotSelected` fn=Main.scan_with_quote*/3 site=Body(9) | rejected |
| 123 | usingcollector_loop_full | 1 | `[static-call/legacy-fallback-retired] UsingCollectorBox.collect/1` | `mir/main-import-view/selected-header-missing` | failed-before-loop |
| 124 | scan_methods_loop_block | 1 | VM err `canonical-call` | `ResolverDeferred UnresolvedName "j"` in LoopBody(4) | rejected |
| 125,126 | scan_methods_nested_loop_idx{19,28} | 1 | VM err `canonical-call` | `semantic-package LoopBreakSource Composite(ForestLookup)` | rejected |
| 127 | scan_methods_program_block | 1 | `ssa/phi_input/without_def` finalize error | `LoopBreakSource Composite(ForestLookup)` | rejected |
| 128 | while_cap | 1 | Parse error `parser/while_legacy_replaced_by_loop_condition` | same parser grammar-contract reject | failed-before-loop |
| 129 | phi_collect_outer_loop | 1 | VM err `canonical-call` | `qualified-preflight actual=New` site=Body(0) Init | failed-before-loop |
| 130 | nested-loop-cluster3 (legacy alias) | 0 | `0` | `LoopBreakSource Composite(ForestLookup)` | rejected |
| 131 | nested-loop-depth1-methodcall (legacy alias) | 1 | VM err `canonical-call` | `qualified-preflight actual=UnaryOp` site=Body(5) Init | failed-before-loop |
| 132-134 | nested-loop idx35-rbrace / Program-stmt nested-{methodcall,}loop (legacy aliases) | 1/1/0 | VM err `canonical-call` / `canonical-call` / `0` | `LoopBreakSource Composite(ForestLookup)` | rejected |
| 135 | scan_methods_nested_loop_state_machine | 0 | `2` | `LoopBreakSource Composite(ForestLookup)` | rejected |
| 136 | scan_ident | 1 | `[static-call/legacy-fallback-retired] Utils.i2s/1` | `qualified-preflight actual=Print` site=Body(1) | failed-before-loop |
| 137 | phi_injector_nested_no_exit_var_step | 1 | VM err `canonical-call` | `qualified-preflight actual=New` site=Body(0) Init | failed-before-loop |
| 138-142 | fini_only, local_fini_slot_capture, fini_cleanup_coexist, local_no_init_cleanup, local_multibind_cleanup | 0 | `SBA`/`BSC`/`111`/`11`/`13` (all match exp) | `semantic-package ResolverDeferred UnsupportedStatement TryCatch` site=Body(1) | failed-before-loop |
| 143,144 | local_multibind_{init,mixed_init}_cleanup | 1 | Parse error `Invalid expression at line 6` (`local a = 3, b = 4` multibind form) | same parse failure on callable lane | failed-before-loop |
| 145-147 | local_{triple_noinit,expr_compare,expr_logic}_cleanup | 0 | `46`/`55`/`100` (match exp) | `TryCatch` site=Body(1) | failed-before-loop |
| 148 | local_expr_unary_not_cleanup | 0 | `99` (**exp `91` — mismatch**) | `TryCatch` site=Body(1) | failed-before-loop |
| 149 | local_expr_unary_minus_cleanup | 0 | `78` (match exp) | `TryCatch` site=Body(1) | failed-before-loop |
| 150 | local_expr_unary_mix_cleanup | 0 | `66` (**exp `61` — mismatch**) | `TryCatch` site=Body(1) | failed-before-loop |
| 151 | local_expr_call_new_cleanup | 1 | `[static-call/legacy-fallback-retired] Main.id/1` | `TryCatch` site=Body(1) | failed-before-loop |
| 152 | local_expr_array_map_literal_cleanup | 1 | VM err `NewBox IntrinsicArray unimplemented` | `TryCatch` site=Body(1) | failed-before-loop |
| 153,154 | local_expr_string_{concat_len,subcmp}_cleanup | 1 | VM err `canonical-call` | `TryCatch` site=Body(1) | failed-before-loop |
| 155 | local_expr_null_logic_cleanup | 0 | `455` (match exp) | `TryCatch` site=Body(1) | failed-before-loop |
| 156 | local_expr_string_trim_chain_cleanup | 1 | `[static-call/legacy-fallback-retired] StrUtil.trim/1` | `TryCatch` site=Body(1) | failed-before-loop |
| 157 | local_expr_bool_combo_cleanup | 0 | `777` (**exp `703` — mismatch**) | `TryCatch` site=Body(1) | failed-before-loop |
| 158 | local_expr_logic_precedence_cleanup | 0 | `888` (**exp `803` — mismatch**) | `TryCatch` site=Body(1) | failed-before-loop |
| 159 | local_expr_logic_parenthesized_cleanup | 0 | `999` (**exp `911` — mismatch**) | `TryCatch` site=Body(1) | failed-before-loop |
| 160 | local_expr_double_not_compare_cleanup | 0 | `1110` (**exp `1003` — mismatch**) | `TryCatch` site=Body(1) | failed-before-loop |

## Result

- 0/60 `accepted`: no canonical admits in this chunk.
- 15/60 `rejected`: `route-not-front-selected`
  (`GenericLoopV1NotSelected` 3, `CarrierRelation` 1), qualified
  preflight `Loop` statement (1), and semantic-package loop issuers
  (`LoopBreakSource` 8, `ResolverDeferred` inside LoopBody 2).
- 45/60 `failed-before-loop`: `ResolverDeferred UnsupportedStatement
  TryCatch` at non-loop sites dominates (23 — every `fini`/`local_
  cleanup` fixture carries try/finally outside any loop), qualified
  preflight non-`Loop` items (9 — `New` initializers, `UnaryOp`,
  `Print`), `mir/main-import-view/selected-header-missing` (6),
  `mir/main-qualified-recipe/relation-missing` (1),
  `mir/normal-root/consume IntegrityInvalid` (1), and parser rejects
  (3 — retired `while` grammar 1, `local a = 3, b = 4` multibind
  form 2).
- 0 `timeout`, 0 left `unobserved`.
- **Expected-output mismatches first recorded here** (all A-side RC 0
  on the compat lane, manifest expected differs): sub:148 `91`→`99`,
  150 `61`→`66`, 157 `703`→`777`, 158 `803`→`888`, 159 `911`→`999`,
  160 `1003`→`1110` — six `local_expr_*_cleanup` fixtures whose digits
  differ per position. Recorded verbatim; no fixture/manifest repair
  (P1 does not judge which side is stale).
- 20/60 ran green to the exact subset expected output on the compat
  lane; sub:127 again hit the `ssa/phi_input/without_def` finalize
  failure (same as its fast-gate twin); sub:152 hit the
  `NewBox IntrinsicArray` interp limit.
- No nondeterminism appeared (no canonical admits).
- No fixture, source, corpus-universe, receipt, or route change was
  made. `observation_state` is the only manifest column touched.
