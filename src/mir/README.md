到達と実行可能行は `05ebd7735a` のソースで再計測した。A は現行名簿の5,230入力、B は selfhost の1,192ファイル。HIR 2列は従来の計測値を載せている。

| 群 | 本体行(計測artifact実行可能) | 責務 | A入力 | B file | A行 | Bのみ行 | testのみ行 | 実測0行 | 入/出(HIR下限) | HIR輪 |
|---|---:|---|---:|---:|---:|---:|---:|---:|---:|---|
| `mir/resolved_semantics` | 35248 | | 5079 | 1063 | 28962 | 63 | 3237 | 2986 | 127/31 | 検出 |
| `mir/top-level` | 23759 | | 5097 | 1070 | 9159 | 299 | 9322 | 4979 | 173/61 | 検出 |
| `mir/loop_recipe_contract` | 14036 | | 855 | 433 | 9720 | 0 | 2692 | 1624 | 34/14 | 検出 |
| `normal_callable_semantic_package/ONLC/root_call_entry/expression_*+initializer_*` | 7659 | | 3124 | 155 | 6227 | 39 | 370 | 1023 | 19/32 | 検出 |
| `compiler/03 normal_default_pipeline/published_backend_view` | 7375 | | 1439 | 0 | 5635 | 0 | 1188 | 552 | 5/22 | 検出 |
| `builder/ordinary_new_admission/` | 6868 | | 3008 | 216 | 5973 | 0 | 223 | 672 | 20/48 | 検出 |
| `normal_callable_semantic_package/ONLC (rest: entry, emission, call, read_argument, validation)` | 6315 | | 3493 | 351 | 4910 | 33 | 560 | 812 | 36/40 | 検出 |
| `mir/verification` | 6180 | | 3414 | 446 | 3903 | 100 | 1316 | 861 | 37/12 | 検出 |
| `mir/global_call_route_plan` | 6106 | | 2645 | 222 | 2817 | 12 | 2423 | 854 | 5/5 | 検出 |
| `normal_callable_semantic_package/ONLC/finalized_*` | 5752 | | 1767 | 0 | 4796 | 0 | 396 | 560 | 7/37 | 検出 |
| `builder/calls/` | 5677 | | 2643 | 468 | 3007 | 46 | 1413 | 1211 | 27/45 | 検出 |
| `normal_callable_semantic_package/ordinary_new_coseal* (hub + issue walk)` | 5083 | | 4422 | 695 | 4452 | 7 | 176 | 448 | 62/60 | 検出 |
| `normal_callable_semantic_package/ordinary_new_borrowed (rest: array*, call*, field*, alias, text, incoming)` | 4914 | | 4422 | 695 | 4411 | 0 | 140 | 363 | 29/41 | 検出 |
| `mir/passes` | 4669 | | 2735 | 262 | 2090 | 8 | 1943 | 628 | 9/9 | 検出 |
| `builder/control_flow/plan/parts/` | 4629 | | 537 | 415 | 1370 | 0 | 1446 | 1813 | 8/19 | 検出 |
| `builder/control_flow/plan/normalizer/` | 4426 | | 9 | 4 | 1305 | 16 | 1087 | 2018 | 7/22 | 検出 |
| `normal_callable_semantic_package/ordinary_new_borrowed_static*` | 4418 | | 4413 | 695 | 2994 | 607 | 404 | 413 | 14/27 | 検出 |
| `normal_callable_semantic_package/ordinary_new_*field*` | 4412 | | 4564 | 845 | 3887 | 0 | 223 | 302 | 30/33 | 検出 |
| `normal_callable_semantic_package/ordinary_new_borrowed_formal (rest: actual*, incoming*, candidate, profile...)` | 4379 | | 4415 | 695 | 3843 | 1 | 191 | 344 | 21/32 | 検出 |
| `normal_callable_semantic_package/ordinary_new_borrowed_object*/instance*/guarded*/nullable*` | 4308 | | 4414 | 695 | 3363 | 2 | 453 | 490 | 12/24 | 検出 |
| `normal_callable_semantic_package/ordinary_new_result_class_claim/*` | 4227 | | 4422 | 695 | 3506 | 0 | 511 | 210 | 25/20 | 検出 |
| `normal_callable_semantic_package/ordinary_new_borrowed_formal_source*` | 4203 | | 4415 | 695 | 3836 | 2 | 78 | 287 | 25/29 | 検出 |
| `mir/generic_method_route_plan` | 4161 | | 2645 | 222 | 761 | 11 | 3075 | 314 | 7/6 | 検出 |
| `normal_callable_semantic_package/ONLC/root_home* + root_cleanup* + reclaim` | 4038 | | 3305 | 339 | 3526 | 1 | 179 | 332 | 18/29 | 検出 |
| `builder/control_flow/plan/lowerer/` | 3532 | | 259 | 13 | 1937 | 0 | 267 | 1328 | 9/42 | 検出 |
| `normal_callable_semantic_package/ordinary_new_borrowed_formal_result*` | 3394 | | 4415 | 695 | 3078 | 1 | 141 | 174 | 13/30 | 検出 |
| `normal_callable_semantic_package/ONLC/map* + owned_array* + root_map*` | 2949 | | 3464 | 346 | 2342 | 0 | 240 | 367 | 8/18 | 検出 |
| `builder/control_flow/plan/facts/` | 2945 | | 5079 | 1063 | 1748 | 2 | 571 | 624 | 21/14 | 検出 |
| `normal_callable_semantic_package/ordinary_new_return*/normal_return*/root_*/object_packet*` | 2909 | | 3650 | 366 | 2276 | 5 | 321 | 307 | 19/26 | 検出 |
| `compiler/08 generic_g0_*` | 2872 | | 66 | 23 | 1983 | 0 | 506 | 383 | 9/13 | 検出 |
| `normal_callable_semantic_package/instance_construct*/constructor*/entry_home + birth_abi` | 2857 | | 4912 | 923 | 2632 | 0 | 72 | 153 | 30/22 | 検出 |
| `normal_callable_semantic_package/ordinary_new_borrowed_formal_entry*` | 2828 | | 4161 | 600 | 2393 | 19 | 119 | 297 | 11/24 | 検出 |
| `normal_callable_semantic_package/package core (mod, model, issuer*, install*, selected_*)` | 2795 | | 4912 | 930 | 2041 | 6 | 400 | 348 | 50/55 | 検出 |
| `builder/ssa/local/` | 2655 | | 3382 | 448 | 1963 | 0 | 168 | 524 | 26/12 | 検出 |
| `builder/normal_callable_loop_source_facts/` | 2653 | | 4563 | 845 | 2170 | 31 | 113 | 339 | 14/36 | 検出 |
| `compiler/11 loop_* (per-family projections)` | 2577 | | 908 | 456 | 1061 | 0 | 1261 | 255 | 7/12 | 検出 |
| `normal_callable_semantic_package/misc top-level (locator, loop_break, app_main, array_i64, container/field children, call_actual, loose tests)` | 2517 | | 4564 | 845 | 2172 | 3 | 86 | 256 | 21/26 | 検出 |
| `normal_callable_semantic_package/ONLC/root_call_entry/lexical_*` | 2490 | | 3082 | 287 | 2091 | 0 | 122 | 277 | 19/22 | 検出 |
| `builder/resolved_lowering/loop_recipe_physicalizer/` | 2476 | | 1 | 0 | 1259 | 0 | 747 | 470 | 5/18 | 検出 |
| `builder/normal_callable_semantic_lowering_state/` | 2436 | | 3471 | 341 | 1770 | 137 | 233 | 296 | 21/26 | 検出 |
| `builder/top: normal_callable_* (other)` | 2419 | | 4683 | 977 | 1506 | 31 | 570 | 312 | 32/30 | 検出 |
| `builder/top: normal_callable_loop_*` | 2372 | | 661 | 178 | 1783 | 96 | 235 | 258 | 10/23 | 検出 |
| `normal_callable_semantic_package/ONLC/root_call_entry/bool_condition_*` | 2314 | | 2892 | 308 | 2020 | 0 | 42 | 252 | 10/21 | 検出 |
| `mir/resolved_value_profile` | 2284 | | 4491 | 749 | 1560 | 31 | 446 | 247 | 41/6 | 検出 |
| `compiler/14 dynamic_full_body_* / selected_dynamic` | 2227 | | 205 | 263 | 1924 | 0 | 201 | 102 | 4/11 | 検出 |
| `mir/loop_structural_facts` | 2227 | | 899 | 456 | 1371 | 0 | 703 | 153 | 16/8 | 検出 |
| `normal_callable_semantic_package/ONLC/root_call_entry (rest: packet, direct_scalar_call, validation, borrowed_projection)` | 2190 | | 2812 | 116 | 1672 | 0 | 95 | 423 | 15/29 | 検出 |
| `normal_callable_semantic_package/ordinary_new_lexical*` | 2183 | | 4415 | 695 | 1906 | 8 | 102 | 167 | 29/32 | 検出 |
| `builder/control_flow/plan/loop_cond/` | 2115 | | 57 | 2 | 1037 | 0 | 48 | 1030 | 4/11 | 検出 |
| `mir/source_call_target` | 2113 | | 4912 | 928 | 1422 | 0 | 476 | 215 | 20/13 | 検出 |
| `normal_callable_semantic_package/ordinary_new_argument*/received*/receiver*` | 2110 | | 4409 | 695 | 1629 | 1 | 114 | 366 | 24/27 | 検出 |
| `normal_callable_semantic_package/ordinary_new core (ledger, candidate, completion index, call_routes, local_commit.rs)` | 2095 | | 4422 | 695 | 1763 | 0 | 130 | 202 | 29/30 | 検出 |
| `builder/top: expression leaves (exprs_*, literal*, collection*, new_expression, record_*, enum_*, qmark, property*, map_read...)` | 2091 | | 5097 | 1070 | 1154 | 0 | 397 | 540 | 12/34 | 検出 |
| `builder/control_flow/plan/recipe_tree/` | 2046 | | 543 | 415 | 269 | 0 | 379 | 1398 | 8/15 | 検出 |
| `mir/typed_object_plan` | 2026 | | 2647 | 222 | 1005 | 0 | 908 | 113 | 1/3 | 検出 |
| `normal_callable_semantic_package/ordinary_new_borrowed_compare* + mul*` | 2023 | | 4207 | 688 | 1733 | 0 | 160 | 130 | 13/17 | 検出 |
| `builder/raw_invocation_source_transport/` | 2001 | | 3638 | 479 | 1748 | 31 | 51 | 171 | 21/43 | 検出 |
| `builder/control_flow/plan/loop_break/` | 1973 | | 57 | 2 | 462 | 0 | 1025 | 486 | 4/4 | 検出 |
| `mir/callable_result_representation` | 1958 | | 4912 | 928 | 1635 | 9 | 129 | 185 | 30/6 | 検出 |
| `normal_callable_semantic_package/ONLC/root_call_entry/field_write+append_*` | 1937 | | 3108 | 288 | 1660 | 1 | 30 | 246 | 12/22 | 検出 |

## 制御フローの定義数

| 対象 | 受け口 | 独立の定義 | 共通部品 |
|---|---:|---:|---:|
| if | 5 | 6 | 0 |
| loop header PHI | 3 | 6 | 3 |
| break | 2 | 4 | 0 |
| continue | 2 | 6 | 0 |

発行・転送の根拠は `tools/mir_map/control_flow_sites.json`、入力ごとの到達から集計した A/B 件数は生成結果の `control_flow_watchlist`。
