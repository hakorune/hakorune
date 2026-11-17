| 群 | 本体行(計測artifact実行可能) | 責務 | A入力 | B file | A行 | Bのみ行 | testのみ行 | 実測0行 | 入/出(HIR下限) | HIR輪 |
|---|---:|---|---:|---:|---:|---:|---:|---:|---:|---|
| `mir/resolved_semantics` | 35225 | | 5235 | 1063 | 28924 | 63 | 3316 | 2922 | 127/31 | 検出 |
| `mir/top-level` | 23993 | | 5253 | 1070 | 9161 | 299 | 9515 | 5018 | 173/61 | 検出 |
| `mir/loop_recipe_contract` | 14210 | | 864 | 433 | 10495 | 0 | 2352 | 1363 | 34/14 | 検出 |
| `normal_callable_semantic_package/ONLC/root_call_entry/expression_*+initializer_*` | 7659 | | 3262 | 155 | 6225 | 39 | 369 | 1026 | 19/32 | 検出 |
| `compiler/03 normal_default_pipeline/published_backend_view` | 7375 | | 1537 | 0 | 5635 | 0 | 1188 | 552 | 5/22 | 検出 |
| `builder/ordinary_new_admission/` | 6850 | | 3121 | 216 | 5955 | 0 | 223 | 672 | 20/48 | 検出 |
| `normal_callable_semantic_package/ONLC (rest: entry, emission, call, read_argument, validation)` | 6315 | | 3635 | 351 | 4911 | 33 | 559 | 812 | 36/40 | 検出 |
| `mir/verification` | 6180 | | 3536 | 446 | 3901 | 100 | 1318 | 861 | 37/12 | 検出 |
| `mir/global_call_route_plan` | 6106 | | 2749 | 222 | 2817 | 12 | 2425 | 852 | 5/5 | 検出 |
| `normal_callable_semantic_package/ONLC/finalized_*` | 5752 | | 1860 | 0 | 4796 | 0 | 396 | 560 | 7/37 | 検出 |
| `builder/control_flow/plan/parts/` | 5725 | | 572 | 415 | 1427 | 0 | 1984 | 2314 | 8/19 | 検出 |
| `builder/calls/` | 5677 | | 2741 | 468 | 3043 | 46 | 1393 | 1195 | 27/45 | 検出 |
| `normal_callable_semantic_package/ordinary_new_coseal* (hub + issue walk)` | 5147 | | 4583 | 695 | 4513 | 7 | 178 | 449 | 62/60 | 検出 |
| `normal_callable_semantic_package/ordinary_new_borrowed (rest: array*, call*, field*, alias, text, incoming)` | 4914 | | 4583 | 695 | 4411 | 0 | 140 | 363 | 29/41 | 検出 |
| `mir/passes` | 4669 | | 2840 | 262 | 2095 | 8 | 1940 | 626 | 9/9 | 検出 |
| `builder/control_flow/plan/normalizer/` | 4418 | | 24 | 5 | 1094 | 16 | 1290 | 2018 | 7/22 | 検出 |
| `normal_callable_semantic_package/ordinary_new_borrowed_static*` | 4418 | | 4574 | 695 | 2994 | 607 | 404 | 413 | 14/27 | 検出 |
| `normal_callable_semantic_package/ordinary_new_*field*` | 4412 | | 4725 | 845 | 3887 | 0 | 223 | 302 | 30/33 | 検出 |
| `normal_callable_semantic_package/ordinary_new_borrowed_formal (rest: actual*, incoming*, candidate, profile...)` | 4379 | | 4576 | 695 | 3843 | 1 | 191 | 344 | 21/32 | 検出 |
| `normal_callable_semantic_package/ordinary_new_borrowed_object*/instance*/guarded*/nullable*` | 4308 | | 4575 | 695 | 3363 | 2 | 453 | 490 | 12/24 | 検出 |
| `normal_callable_semantic_package/ordinary_new_result_class_claim/*` | 4227 | | 4583 | 695 | 3506 | 0 | 511 | 210 | 25/20 | 検出 |
| `normal_callable_semantic_package/ordinary_new_borrowed_formal_source*` | 4203 | | 4576 | 695 | 3836 | 2 | 78 | 287 | 25/29 | 検出 |
| `mir/generic_method_route_plan` | 4161 | | 2749 | 222 | 761 | 11 | 3075 | 314 | 7/6 | 検出 |
| `compiler/05 raw_* (Raw route)` | 4092 | | 0 | 0 | 0 | 0 | 3170 | 922 | 3/16 | 検出 |
| `normal_callable_semantic_package/ONLC/root_home* + root_cleanup* + reclaim` | 4038 | | 3424 | 339 | 3526 | 1 | 179 | 332 | 18/29 | 検出 |
| `compiler/04 normal_source_plan/` | 3650 | | 4784 | 183 | 23 | 0 | 3003 | 624 | 7/13 | 検出 |
| `builder/control_flow/plan/lowerer/` | 3630 | | 232 | 13 | 1941 | 0 | 307 | 1382 | 9/42 | 検出 |
| `normal_callable_semantic_package/ordinary_new_borrowed_formal_result*` | 3394 | | 4576 | 695 | 3078 | 1 | 141 | 174 | 13/30 | 検出 |
| `builder/normal_callable_loop_source_facts/` | 3378 | | 4724 | 845 | 2292 | 2 | 83 | 1001 | 14/36 | 検出 |
| `builder/control_flow/plan/facts/` | 3283 | | 5235 | 1063 | 2205 | 5 | 503 | 570 | 21/14 | 検出 |
| `normal_callable_semantic_package/ONLC/map* + owned_array* + root_map*` | 3032 | | 3608 | 346 | 2342 | 0 | 240 | 450 | 8/18 | 検出 |
| `compiler/08 generic_g0_*` | 2975 | | 87 | 23 | 1983 | 0 | 551 | 441 | 9/13 | 検出 |
| `normal_callable_semantic_package/ordinary_new_return*/normal_return*/root_*/object_packet*` | 2909 | | 3801 | 366 | 2276 | 5 | 321 | 307 | 19/26 | 検出 |
| `normal_callable_semantic_package/instance_construct*/constructor*/entry_home + birth_abi` | 2857 | | 5067 | 923 | 2632 | 0 | 72 | 153 | 30/22 | 検出 |
| `normal_callable_semantic_package/ordinary_new_borrowed_formal_entry*` | 2828 | | 4318 | 600 | 2393 | 19 | 119 | 297 | 11/24 | 検出 |
| `normal_callable_semantic_package/package core (mod, model, issuer*, install*, selected_*)` | 2795 | | 5067 | 930 | 2041 | 6 | 400 | 348 | 50/55 | 検出 |
| `builder/ssa/local/` | 2655 | | 3501 | 448 | 1955 | 0 | 176 | 524 | 26/12 | 検出 |
| `compiler/11 loop_* (per-family projections)` | 2593 | | 917 | 456 | 1152 | 0 | 1185 | 256 | 7/12 | 検出 |
| `compiler/10 nested_predicate_*` | 2522 | | 85 | 23 | 73 | 0 | 2250 | 199 | 5/9 | 検出 |
| `normal_callable_semantic_package/misc top-level (locator, loop_break, app_main, array_i64, container/field children, call_actual, loose tests)` | 2517 | | 4725 | 845 | 2172 | 3 | 86 | 256 | 21/26 | 検出 |
| `normal_callable_semantic_package/ONLC/root_call_entry/lexical_*` | 2490 | | 3198 | 287 | 2091 | 0 | 122 | 277 | 19/22 | 検出 |
| `builder/resolved_lowering/loop_recipe_physicalizer/` | 2476 | | 2 | 0 | 1333 | 0 | 747 | 396 | 5/18 | 検出 |
| `builder/normal_callable_semantic_lowering_state/` | 2436 | | 3615 | 341 | 1837 | 137 | 166 | 296 | 21/26 | 検出 |
| `builder/top: normal_callable_* (other)` | 2419 | | 4843 | 977 | 1506 | 31 | 570 | 312 | 32/30 | 検出 |
| `builder/top: normal_callable_loop_*` | 2394 | | 663 | 176 | 1851 | 96 | 201 | 246 | 10/23 | 検出 |
| `builder/control_flow/plan/loop_cond/` | 2357 | | 552 | 139 | 1443 | 10 | 2 | 902 | 4/11 | 検出 |
| `builder/control_flow/plan/recipe_tree/` | 2346 | | 632 | 420 | 699 | 0 | 308 | 1339 | 8/15 | 検出 |
| `compiler/14 dynamic_full_body_* / selected_dynamic` | 2323 | | 217 | 263 | 1924 | 0 | 201 | 198 | 4/11 | 検出 |
| `normal_callable_semantic_package/ONLC/root_call_entry/bool_condition_*` | 2314 | | 2997 | 308 | 2020 | 0 | 42 | 252 | 10/21 | 検出 |
| `mir/loop_structural_facts` | 2304 | | 908 | 456 | 1459 | 0 | 746 | 99 | 16/8 | 検出 |
| `builder/top: expression leaves (exprs_*, literal*, collection*, new_expression, record_*, enum_*, qmark, property*, map_read...)` | 2296 | | 5253 | 1070 | 1154 | 0 | 566 | 576 | 12/34 | 検出 |
| `mir/resolved_value_profile` | 2284 | | 4652 | 749 | 1560 | 31 | 480 | 213 | 41/6 | 検出 |
| `builder/control_flow/facts/` | 2237 | | 871 | 447 | 1412 | 1 | 390 | 434 | 14/11 | 検出 |
| `normal_callable_semantic_package/ONLC/root_call_entry (rest: packet, direct_scalar_call, validation, borrowed_projection)` | 2190 | | 2927 | 116 | 1672 | 0 | 95 | 423 | 15/29 | 検出 |
| `normal_callable_semantic_package/ordinary_new_lexical*` | 2183 | | 4576 | 695 | 1906 | 8 | 102 | 167 | 29/32 | 検出 |
| `mir/source_call_target` | 2113 | | 5067 | 928 | 1425 | 0 | 473 | 215 | 20/13 | 検出 |
| `normal_callable_semantic_package/ordinary_new_argument*/received*/receiver*` | 2110 | | 4570 | 695 | 1629 | 1 | 114 | 366 | 24/27 | 検出 |
| `normal_callable_semantic_package/ordinary_new core (ledger, candidate, completion index, call_routes, local_commit.rs)` | 2095 | | 4583 | 695 | 1763 | 0 | 130 | 202 | 29/30 | 検出 |
| `mir/typed_object_plan` | 2026 | | 2752 | 222 | 1005 | 0 | 908 | 113 | 1/3 | 検出 |
| `normal_callable_semantic_package/ordinary_new_borrowed_compare* + mul*` | 2023 | | 4367 | 688 | 1733 | 0 | 160 | 130 | 13/17 | 検出 |
