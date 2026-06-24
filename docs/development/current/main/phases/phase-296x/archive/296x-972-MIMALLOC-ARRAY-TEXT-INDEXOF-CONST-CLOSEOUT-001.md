# 296x-972 MIMALLOC-ARRAY-TEXT-INDEXOF-CONST-CLOSEOUT-001

Status: Landed
Date: 2026-06-16

## Purpose

Close the `kilo_leaf_array_string_indexof_const` exact-AOT row after the
region payload, C ABI reader, runtime helper, and backend lowering all reached
the active C ABI `ny-llvmc` route.

## Decision

This row is a keeper for the selected exact front.

```text
keeper_scope=array_text_indexof_const_region_lowering
winner_claim=front_local
product_default_changed=0
source_hako_changed=0
```

The keeper is not a general StringBox / ArrayBox storage rewrite. It is a
guarded region lowering for one metadata-proven shape:

```text
Array.get(row)
String.indexOf(const_utf8)
found predicate
accumulator += 1
post-loop Array.length preserved
```

## Evidence

```text
bench_key=kilo_leaf_array_string_indexof_const
repeat=3
aot_status=ok
c_instr=37326772
c_cycles=5730118
ny_aot_instr=4136429
ny_aot_cycles=1197019
ratio_instr=9.02
ratio_cycles=4.79
c_ipc=6.51
ny_aot_ipc=3.46
```

The C pair now returns `hits + rows`, matching the Hako source return shape
`hits + lines.length()`.

## Guarded Boundaries

```text
mir_json_field=array_text_indexof_const_region_plans
backend_reads_region_plan=1
raw_mir_window_rescan=0
benchmark_name_branch=0
helper_name_inference=0
product_arraybox_storage_changed=0
product_stringbox_storage_changed=0
mirbuilder_changed_for_fastpath=0
```

## Closed Owner

```text
closed_owner=array_text_indexof_const_region_helper_boundary
closed_helper=hako.array_text.indexof_const_found_count_region
closed_backend_seam=loop_header_helper_call_then_exit
closed_body_policy=loop_body_unreachable
exit_phi_preserved=1
post_loop_length_preserved=1
```

## Next

Return to fresh front selection. Do not keep extending this front unless a new
high-confidence owner appears from perf evidence.

```text
selected_next=MIMALLOC-FRESH-FRONT-SELECTION-AFTER-ARRAY-TEXT-INDEXOF-CLOSEOUT-001
summary=ok
```

## Proof Bundle

```bash
cargo test -p nyash_kernel array_text_indexof_const_found_count_region_counts_hits -- --nocapture
cargo test --lib build_mir_json_root_emits_array_text_indexof_const_region_plans -- --nocapture
cargo check --bin hakorune
bash tools/perf/build_perf_release.sh
bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_leaf_array_string_indexof_const 1 3
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
