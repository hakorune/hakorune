---
Status: SSOT
Decision: accepted
Date: 2026-03-28
Scope: backend owner cutover 中の legacy / compare / archive / delete 候補を 1 か所で追跡する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/backend-owner-cutover-ssot.md
  - docs/development/current/main/design/backend-legacy-preservation-and-archive-ssot.md
  - docs/development/current/main/phases/phase-29x/README.md
---

# 29x-96 Backend Owner Legacy Ledger

## Rule

- backend owner state が変わる commit では、この ledger も同コミットで更新する。
- `daily から外す` と `delete する` は別判定だよ。
- preservation-first SSOT を満たさない surface は、demote はしても即 delete しない。

## Current Ledger

| surface | current_state | target_state | retire_trigger | archive_or_delete | notes |
| --- | --- | --- | --- | --- | --- |
| `lang/src/shared/backend/ll_emit/**` | compare bridge + narrow daily owner candidate | future mainline backend owner | boundary-only wave が stable | preserve | mainline owner 候補なので legacy 扱いしない |
| `apps/tests/phase29x_backend_owner_hako_ll_compare_min.hako` | compare-proof asset | archive/delete candidate | compare bridge retirement | archive then delete | explicit compare lane only; 3 shape daily flip is already landed |
| `tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_hako_ll_compare_min.sh` | compare-proof asset | archive/delete candidate | compare bridge retirement | archive then delete | keep until bridge-only compare lane is retired |
| `apps/tests/phase29x_backend_owner_daily_min.hako` | boundary-only owner-flip asset | keep until broader owner cutover settles | owner flip wave complete | decide later | daily narrow evidence app |
| `tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_print_min.sh` | active daily owner proof | keep until broader owner cutover settles | owner flip wave complete | decide later | `hello_simple_llvm_native_probe_v1` now proves `.hako ll emitter` daily owner |
| `tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_string_length_min.sh` | active daily owner proof | keep until broader owner cutover settles | owner flip wave complete | decide later | `string_length_ascii_min_v1` now proves `.hako ll emitter` daily owner |
| `tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_string_indexof_min.sh` | active daily owner proof | keep until broader owner cutover settles | owner flip wave complete | decide later | `string_indexof_ascii_min_v1` now proves `.hako ll emitter` daily owner |
| `tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_indexof_line_min.sh` | active daily owner proof | keep until broader owner cutover settles | owner flip wave complete | decide later | `indexof_line_pure_min_v1` now proves `.hako ll emitter` daily owner |
| `tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_substring_concat_loop_min.sh` | active daily owner proof | keep until broader owner cutover settles | owner flip wave complete | decide later | `substring_concat_loop_pure_min_v1` now proves `.hako ll emitter` daily owner |
| `tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_length_min.sh` | active daily owner proof | keep until broader owner cutover settles | owner flip wave complete | decide later | `runtime_data_string_length_ascii_min_v1` now proves `.hako ll emitter` daily owner |
| `tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_array_length_min.sh` | active daily owner proof | keep until broader owner cutover settles | owner flip wave complete | decide later | `runtime_data_array_length_min_v1` now proves `.hako ll emitter` daily owner |
| `tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_map_size_min.sh` | active daily owner proof | keep until broader owner cutover settles | owner flip wave complete | decide later | `runtime_data_map_size_min_v1` now proves `.hako ll emitter` daily owner |
| `tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_array_has_min.sh` | active daily owner proof | keep until broader owner cutover settles | owner flip wave complete | decide later | `runtime_data_array_has_missing_min_v1` now proves `.hako ll emitter` daily owner |
| `tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_array_get_min.sh` | active daily owner proof | keep until broader owner cutover settles | owner flip wave complete | decide later | `runtime_data_array_get_missing_min_v1` now proves `.hako ll emitter` daily owner |
| `tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_array_push_min.sh` | active daily owner proof | keep until broader owner cutover settles | owner flip wave complete | decide later | `runtime_data_array_push_min_v1` now proves `.hako ll emitter` daily owner |
| `tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_map_has_min.sh` | active daily owner proof | keep until broader owner cutover settles | owner flip wave complete | decide later | `runtime_data_map_has_missing_min_v1` now proves `.hako ll emitter` daily owner |
| `tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_map_get_min.sh` | active daily owner proof | keep until broader owner cutover settles | owner flip wave complete | decide later | `runtime_data_map_get_missing_min_v1` now proves `.hako ll emitter` daily owner |
| `tools/smokes/v2/profiles/integration/phase29ck_boundary/entry/phase29ck_boundary_pure_first_min.sh` | legacy boundary lock in `phase29ck-boundary-legacy` | archive/delete candidate | compare bridge retirement + preservation bundle ready | archive then delete | daily owner proof already moved to `phase29x_backend_owner_daily_ret_const_min.sh` |
| `tools/smokes/v2/profiles/integration/phase29ck_boundary/entry/phase29ck_boundary_pure_bool_phi_branch_min.sh` | legacy boundary lock in `phase29ck-boundary-legacy` | archive/delete candidate | compare bridge retirement + preservation bundle ready | archive then delete | daily owner proof already moved to `phase29x_backend_owner_daily_bool_phi_branch_min.sh` |
| `tools/smokes/v2/profiles/integration/phase29ck_boundary/entry/phase29ck_boundary_pure_print_min.sh` | legacy boundary lock in `phase29ck-boundary-legacy` | archive/delete candidate | compare bridge retirement + preservation bundle ready | archive then delete | daily owner proof already moved to `phase29x_backend_owner_daily_print_min.sh` |
| `tools/smokes/v2/profiles/integration/phase29ck_boundary/string/phase29ck_boundary_pure_string_length_min.sh` | legacy boundary lock in `phase29ck-boundary-legacy` | archive/delete candidate | compare bridge retirement + preservation bundle ready | archive then delete | daily owner proof already moved to `phase29x_backend_owner_daily_string_length_min.sh` |
| `tools/smokes/v2/profiles/integration/phase29ck_boundary/string/phase29ck_boundary_pure_string_indexof_min.sh` | legacy boundary lock in `phase29ck-boundary-legacy` | archive/delete candidate | compare bridge retirement + preservation bundle ready | archive then delete | daily owner proof already moved to `phase29x_backend_owner_daily_string_indexof_min.sh` |
| `tools/smokes/v2/profiles/integration/phase29ck_boundary/string/phase29ck_boundary_pure_indexof_line_min.sh` | legacy boundary lock in `phase29ck-boundary-legacy` | archive/delete candidate | compare bridge retirement + preservation bundle ready | archive then delete | daily owner proof already moved to `phase29x_backend_owner_daily_indexof_line_min.sh` |
| `tools/smokes/v2/profiles/integration/phase29ck_boundary/string/phase29ck_boundary_pure_substring_concat_loop_min.sh` | legacy boundary lock in `phase29ck-boundary-legacy` | archive/delete candidate | compare bridge retirement + preservation bundle ready | archive then delete | daily owner proof already moved to `phase29x_backend_owner_daily_substring_concat_loop_min.sh` |
| `tools/smokes/v2/profiles/integration/phase29ck_boundary/string/phase29ck_boundary_pure_string_concat3_extern_min.sh` | legacy boundary lock in `phase29ck-boundary-legacy` | archive/delete candidate | compare bridge retirement + preservation bundle ready | archive then delete | daily owner proof already moved to `phase29x_backend_owner_daily_concat3_extern_min.sh` |
| `tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_length_min.sh` | legacy boundary lock in `phase29ck-boundary-legacy` | archive/delete candidate | compare bridge retirement + preservation bundle ready | archive then delete | daily owner proof already moved to `phase29x_backend_owner_daily_runtime_data_length_min.sh` |
| `tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_length_min.sh` | legacy boundary lock in `phase29ck-boundary-legacy` | archive/delete candidate | compare bridge retirement + preservation bundle ready | archive then delete | daily owner proof already moved to `phase29x_backend_owner_daily_runtime_data_array_length_min.sh` |
| `tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_map_size_min.sh` | legacy boundary lock in `phase29ck-boundary-legacy` | archive/delete candidate | compare bridge retirement + preservation bundle ready | archive then delete | daily owner proof already moved to `phase29x_backend_owner_daily_runtime_data_map_size_min.sh` |
| `tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_push_min.sh` | legacy boundary lock in `phase29ck-boundary-legacy` | archive/delete candidate | compare bridge retirement + preservation bundle ready | archive then delete | daily owner proof already moved to `phase29x_backend_owner_daily_runtime_data_array_push_min.sh` |
| `tools/smokes/v2/suites/integration/phase29ck-boundary.txt` | active boundary suite with flipped owner locks removed | active boundary suite only | none | preserve | keep focused on unflipped boundary coverage and active pure-first canaries |
| `tools/smokes/v2/suites/integration/phase29ck-boundary-legacy.txt` | temporary legacy suite for flipped owner locks | archive/delete candidate | compare bridge retirement + preservation bundle ready | archive then delete | default suite から外した retired locks だけを置く |
| `lang/c-abi/shims/hako_llvmc_ffi*` | legacy daily owner for unflipped shapes | compare/compat only | shape-by-shape daily flip | demote first, delete later | delete は preservation-first 条件を満たした後だけ |
| `src/host_providers/llvm_codegen/transport.rs` 内 ll-emit branch | mixed bridge residue | split out then delete old branch | dedicated `ll_emit_bridge.rs` landed | delete | route/policy を持たせない |
| `src/host_providers/llvm_codegen/ll_emit_bridge.rs` | explicit bridge | keep while compare bridge is needed | compare bridge retirement | decide later | temporary bridge, not permanent route |
| `lang/src/shared/backend/ll_emit/mir_json_loader_box.hako` | dead compare residue | removed | no live import remained | delete early | removed in this wave; keep ledger row as history |
| `lang/src/llvm_ir/**` | compat keep | compat keep / archive candidate | separate preserve-first decision | preserve | do not reopen as daily owner |
| `lang/src/llvm_ir/archive/**` | archive-preserved | archive-preserved | none | preserve | no revive without separate SSOT |
| `src/llvm_py/**` | compat/probe keep | future archive candidate | preservation bundle ready | archive, not now | not active delete target in this wave |
| `tools/llvmlite_harness.py` | compat/probe keep | future archive candidate | preservation bundle ready | archive, not now | monitor/probe only |
| `crates/nyash-llvm-compiler/src/native_driver.rs` | bootstrap / monitor seam | preserve until separate backend-zero decision | separate owner-axis closeout | preserve | not an immediate delete target |
| `crates/nyash-llvm-compiler/src/native_ir.rs` | bootstrap / monitor seam | preserve until separate backend-zero decision | separate owner-axis closeout | preserve | not an immediate delete target |
