---
Status: Active
Date: 2026-06-02
Scope: active mimalloc migration, optimization, and provider-benchmark workstream.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
  - docs/development/current/main/design/current-docs-archive-policy-ssot.md
  - docs/development/current/main/design/mimalloc-benchmark-dll-roadmap-ssot.md
  - docs/development/current/main/design/mimalloc-benchmark-route-taxonomy-ssot.md
  - docs/development/current/main/design/type-abi-route-descriptor-plane-ssot.md
  - docs/development/current/main/design/hako-alloc-policy-state-contract-ssot.md
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/investigations/mimalloc-current-history-2026-06-02.md
---

# Mimalloc Current Workstream

This is the active restart card. It intentionally stays compact.

Full historical MIM-001..MIM-146 prose was archived to:

```text
docs/development/current/main/investigations/mimalloc-current-history-2026-06-02.md
```

Use that archive for exact old evidence. Use this file for the current decision
surface, next task order, and parking lot.

## Goal

Keep proving that `.hako` mimalloc can be built, packaged, and compared against
C mimalloc without opening product allocator replacement prematurely.

Current focus:

```text
provider/DLL benchmark bridge
same-machine C mimalloc comparison
next owner selection from local evidence
algorithm-port coverage separation
```

## Stop Line

- no new numbered row for inventory-only work
- no row-specific `.sh` guard
- no full external benchmark corpus import
- no copied benchmark executables in git
- no provider activation as product default
- no allocator replacement claim
- no hook installation claim
- no production `#[global_allocator]` claim
- no winner claim
- no source syntax expansion unless a tracked reference decision accepts it

## Current Decisions

```text
parity/front:
  direct exact remains the .hako mimalloc optimization front

public/default front:
  compatibility reference only; not the parity owner

provider package:
  handoff artifact and benchmark smoke path only

LD_PRELOAD shim:
  experiment/measurement bridge only

Hakozuna mixed-ws:
  repo-local CRT fixture is connected
  compare same-machine system malloc / C mimalloc / optional Hakorune provider
  do not compare Ubuntu numbers horizontally while CPU differs

record/state direction:
  PageModel remains box owner
  primitive mutable state may become record-shaped residence only through
  RecordStateResidencePlanV0-style metadata/plans

Inline(required):
  small receiver-local leaf helper only
  multi-block hot paths use HotCore/direct-exact plans instead

algorithm-port coverage:
  `.hako` hako_alloc policy/model coverage and benchmark-only replacement-front
  execution coverage are different surfaces. Do not read the fixed-slot
  replacement front as proof that the full `.hako` mimalloc algorithm is wired
  into LD_PRELOAD/product replacement.
```

## Algorithm Port Coverage

Use this section before selecting the next implementation owner.

Current reading:

```text
.hako model/policy coverage:
  size_class_policy=modeled
  page_local_free_stack=modeled
  same_thread_local_free=modeled
  object_lifecycle_hot_core=modeled
  page_map/realloc/huge/osvm/remote_free=policy_or_seam_modeled

benchmark-only replacement front execution:
  fixed_slot_native_free_stack=executed
  matched_fixed_slot_size=executed for selected fixtures
  hako_size_class_good_size_slot=executed only when
    --replacement-front-match-hako-size-class is passed
  product_bins=benchmark_native_bins_v0 when
    --replacement-front-native-bins-mode is passed
  product_pages=report_only_plan
  in_place_realloc_within_fixed_slot=executed
  thread_local_arena_remote_free_bridge=executed

not yet bridged:
  size_class_policy_to_product_replacement_pages
  measured DirectArrayI64 source route in product-like execution
  selected next structural owner after HotCore/PageModel measurement
  general page queue / segment / OSVM product allocator front
```

Stable report entry:

```bash
python3 tools/allocator/hako_mimalloc_algorithm_coverage.py
```

Overlay a generated mixed-ws compare report when selecting the next bridge
owner from executed benchmark evidence:

```bash
python3 tools/allocator/hako_mimalloc_algorithm_coverage.py \
  --benchmark-report target/hakozuna-mixed-ws-page-bins-current/report.out
```

The overlay may show `replacement_front_product_bins_consumer_enabled=1` for a
benchmark-only route, while `replacement_front_is_full_hako_algorithm=0` and
`replacement_front_product_pages_consumer_enabled=0` keep product allocator
claims closed.

The first benchmark-only HotCore/PageModel bridge is available as:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --replacement-front-page-bins-mode \
  --replacement-front-hotcore-page-model-mode \
  --threads 1 \
  --out target/hakozuna-mixed-ws-hotcore-page-model/report.out \
  --out-dir target/hakozuna-mixed-ws-hotcore-page-model/artifacts \
  --sample-count 3
```

Expected route fields:

```text
replacement_front_algorithm_shape=page_bin_hotcore_page_model_benchmark_front
replacement_front_product_bins_route=benchmark_page_bins_hotcore_page_model
replacement_front_page_bins_route=benchmark_page_bins_hotcore_page_model
replacement_front_hotcore_consumer_enabled=1
replacement_front_hotcore_route=benchmark_page_bins_hotcore_page_model
replacement_front_product_pages_consumer_enabled=0
replacement_front_is_full_hako_algorithm=0
```

This bridge consumes the HotCore/PageModel-shaped alloc/free helper route in the
benchmark-only replacement front. It is still not product pages, activation,
hook installation, or a full `.hako` allocator algorithm claim.

Local 3-sample refresh:

```text
page_bins_refresh:
  report=target/hakozuna-mixed-ws-page-bins-refresh/report.out
  median_ops_per_sec=8,136,100.692
  hotcore_consumer_enabled=0

hotcore_page_model:
  report=target/hakozuna-mixed-ws-hotcore-page-model/report.out
  median_ops_per_sec=8,945,904.118
  hotcore_consumer_enabled=1
  route=benchmark_page_bins_hotcore_page_model

hotcore_size_class_table:
  attempted_change=lower benchmark-only HotCore/PageModel size_to_bin through
    an 8-byte bucket table instead of the generated ordered range scan
  report=target/hakozuna-mixed-ws-hotcore-size-table-7/report.out
  baseline_report=target/hakozuna-mixed-ws-hotcore-baseline-7/report.out
  baseline_median_ops_per_sec=8,241,237.504
  median_ops_per_sec=8,691,873.099
  size_class_lookup_route=table_8byte_bucket
  decision=keeper

hotcore_size_class_table_eager_init:
  attempted_change=initialize benchmark-only bins in the replacement-front
    constructor while keeping the size-class table route
  report=target/hakozuna-mixed-ws-hotcore-size-table-eager-init-7/report.out
  previous_keeper_report=target/hakozuna-mixed-ws-hotcore-size-table-7/report.out
  previous_keeper_median_ops_per_sec=8,691,873.099
  median_ops_per_sec=9,424,804.200
  size_class_lookup_route=table_8byte_bucket
  eager_init_mode=1
  decision=keeper

hotcore_size_class_table_eager_init_refresh:
  report=target/hakozuna-mixed-ws-eager-init-refresh-7/report.out
  median_ops_per_sec=7,523,888.345
  product_pages_nonlinear_mode=0
  page_bins_lookup_route=range_scan
  decision=current_same-run_baseline_for_product_pages_nonlinear_probe
```

Interpretation: HotCore/PageModel wrapper mode is a structural bridge keeper
because it consumes the next `.hako` semantic boundary and improves over the
same-run page-bins refresh. It is not a winner/performance claim against the
older page-bins best sample or C mimalloc.

The size-class table lookup is the current malloc-owner keeper: `perf` on the
current bridge selected `malloc` plus the ownership lookup as the dominant
replacement-front symbols, and the table path improves the same-run 7-sample
HotCore/PageModel median. The eager-init follow-up is the current top keeper:
it keeps the same benchmark-only route and table lookup, but moves bin
initialization into the replacement-front constructor. Product pages,
activation, hooks, globals, full `.hako` algorithm claims, and winner claims
remain closed.

Rejected local probe:

```text
free_release_owned_direct_probe:
  attempted_change=replace free() out-param find_owned call with generated release_owned(ptr)
  asm_effect=free stack canary/out-param call removed
  median_ops_per_sec=8,522,097.800
  previous_hotcore_page_model_median_ops_per_sec=8,945,904.118
  decision=nonkeeper

hotcore_cold_init_probe:
  attempted_change=mark generated init_bins as cold,noinline
  median_ops_per_sec=8,735,302.853
  previous_hotcore_page_model_median_ops_per_sec=8,945,904.118
  decision=nonkeeper

page_map_backed_bridge_probe:
  attempted_change=add benchmark-only page-map register/lookup/unregister
    ownership table to page-bins replacement front
  report=target/hakozuna-mixed-ws-page-map-current/report.out
  product_pages_consumer_enabled=1
  page_bins_lookup_route=page_map_lookup
  median_ops_per_sec=4,021,684.925
  previous_hotcore_page_model_median_ops_per_sec=8,945,904.118
  decision=nonkeeper

hotcore_skip_counters_probe:
  attempted_change=allow --replacement-front-skip-hot-counters for page-bins
    HotCore/PageModel front and compile counters to no-op macros
  report=target/hakozuna-mixed-ws-hotcore-skip-counters-current/report.out
  direct_core_call_count_total=0
  median_ops_per_sec=8,713,306.090
  previous_hotcore_page_model_median_ops_per_sec=8,945,904.118
  decision=nonkeeper

hotcore_free_lookup_probe:
  attempted_change=add free-only ownership decode that returns only bin/index,
    removing free() stack-canary/out-param-heavy find_owned call shape
  asm_effect=free symbol no longer needs stack canary for the ownership lookup
  report=target/hakozuna-mixed-ws-hotcore-free-lookup-current/report.out
  median_ops_per_sec=8,823,010.411
  previous_hotcore_page_model_median_ops_per_sec=8,945,904.118
  decision=nonkeeper

hotcore_find_owned_large_first_probe:
  attempted_change=scan generated find_owned ownership ranges from larger size
    classes first, on top of the size-class table plus eager-init keeper
  report=target/hakozuna-mixed-ws-hotcore-large-first-7/report.out
  median_ops_per_sec=8,171,670.453
  previous_hotcore_size_class_table_eager_init_median_ops_per_sec=9,424,804.200
  direct_core_call_count_total=8,336
  host_passthrough_count_total=8
  decision=nonkeeper

hotcore_inline_free_owned_probe:
  attempted_change=inline generated owned-pointer release directly in free(),
    avoiding find_owned out-params on the free path while leaving realloc on
    find_owned
  report=target/hakozuna-mixed-ws-hotcore-inline-free-owned-7/report.out
  median_ops_per_sec=9,113,527.208
  previous_hotcore_size_class_table_eager_init_median_ops_per_sec=9,424,804.200
  direct_core_call_count_total=8,336
  host_passthrough_count_total=8
  decision=nonkeeper

hotcore_find_owned_btree_probe:
  attempted_change=lower generated find_owned range checks through an address
    decision tree instead of the linear bin range scan
  report=target/hakozuna-mixed-ws-hotcore-find-btree-7/report.out
  median_ops_per_sec=9,425,870.243
  previous_hotcore_size_class_table_eager_init_median_ops_per_sec=9,424,804.200
  long_run_perf_ops_per_sec=70,795,149.026
  current_keeper_long_run_perf_ops_per_sec=74,332,371.909
  direct_core_call_count_total=8,336
  host_passthrough_count_total=8
  decision=nonkeeper_too_small_and_long_run_regressed

hotcore_unreachable_bin_default_probe:
  attempted_change=mark generated alloc_from_bin default as unreachable under
    the size-class table route, aiming to remove the post-table switch fallback
    range check
  report=target/hakozuna-mixed-ws-hotcore-unreachable-bin-default-7/report.out
  median_ops_per_sec=7,519,758.165
  previous_hotcore_size_class_table_eager_init_median_ops_per_sec=9,424,804.200
  asm_effect=removed the switch range fallback check, but changed generated
    switch/code layout enough to regress throughput
  direct_core_call_count_total=8,336
  host_passthrough_count_total=8
  decision=nonkeeper

provider_gap_refresh_2026_06_04:
  exact_front_report=target/mimalloc-opt-refresh/direct-exact-repeat8192.out
  direct_exact_hako_body_elapsed_ns=3,000,000
  direct_exact_c_mimalloc_body_elapsed_ns=3,271,981
  direct_exact_body_elapsed_ratio=0.917
  provider_host_report=target/mimalloc-opt-refresh/provider-host-refresh-gap-s5.out
  provider_host_median_ops_per_sec=5,721,593.006
  provider_host_vs_mimalloc_ratio=0.446
  provider_host_init_real_fallback_per_provider_operation=1.157
  direct_libc_probe_report=target/mimalloc-opt-refresh/provider-host-direct-libc-gap-s5.out
  direct_libc_probe_median_ops_per_sec=5,144,681.300
  direct_libc_probe_vs_mimalloc_ratio=0.414
  direct_libc_probe_init_real_fallback_per_provider_operation=0.765
  direct_libc_probe_decision=nonkeeper_reverted
  native_slot_thread_probe=target/mimalloc-opt-refresh/provider-native-slot-lockonly-assume-gap-s5.out
  native_slot_thread_probe_decision=nonkeeper_reverted_invalid_pointer_under_ldpreload_threads
  selected_next_owner=provider_alloc_free_internal_real_malloc_boundary
  selected_next_action=split provider ABI/shim boundary before another host-wrapper C-shape probe

provider_abi_claim_boundary:
  provider_kind_split=accepted
  provider_api_layout_ssot=docs/development/current/main/design/provider-abi-v1-ssot.md
  provider_bound_enabled_terms=documented
  provider_free_claim=implemented
  provider_usable_size_claim=implemented_narrow
  provider_usable_size_host_ladder=target/prov-abi/usable-size-host-ladder.out
  provider_usable_size_host_gap=target/prov-abi/usable-size-host-gap-s3.out
  provider_usable_size_native_ladder=target/prov-abi/usable-size-native-ladder.out
  provider_usable_size_native_gap=target/prov-abi/usable-size-native-gap-t1-s3.out
  provider_usable_size_claim_host_backed=enabled_with_HostAllocatorV0
  provider_usable_size_claim_native_slot=enabled
  provider_realloc_claim=implemented_narrow
  provider_realloc_host_ladder=target/prov-abi/realloc-claim-host-ladder.out
  provider_realloc_host_gap=target/prov-abi/realloc-claim-host-gap-s3.out
  provider_realloc_native_ladder=target/prov-abi/realloc-claim-native-ladder.out
  provider_realloc_native_gap=target/prov-abi/realloc-claim-native-gap-t1-s3.out
  provider_realloc_claim_host_backed=enabled_with_HostAllocatorV0
  provider_realloc_claim_native_slot=enabled
  host_allocator_vtable=implemented_for_host_backed
  host_allocator_host_ladder=target/prov-abi/host-vtable-host-ladder.out
  host_allocator_native_ladder=target/prov-abi/host-vtable-native-ladder.out
  host_allocator_host_gap=target/prov-abi/host-vtable-host-gap-s3.out
  host_allocator_host_usable_gap=target/prov-abi/host-vtable-host-usable-gap-s3.out
  host_allocator_vtable_init_host_backed=enabled
  provider_direct_libc_symbol_dependency=0
  ld_preload_reentry_for_host_alloc=0
  product_activation=0
  global_allocator_claim=0
  hook_installed=0

provider_host_vtable_refresh_2026_06_04:
  algorithm_coverage_report=target/mimalloc-opt-refresh/algorithm-coverage-after-host-vtable.out
  algorithm_coverage_host_report=target/mimalloc-opt-refresh/algorithm-coverage-provider-host-vtable-gap-s5.out
  algorithm_coverage_host_usable_report=target/mimalloc-opt-refresh/algorithm-coverage-provider-host-vtable-usable-gap-s5.out
  replacement_front_is_full_hako_algorithm=0
  replacement_front_multithread_claim=0
  host_vtable_normal_report=target/mimalloc-opt-refresh/provider-host-vtable-gap-s5.out
  host_vtable_normal_provider_median_ops_per_sec=4,741,831.011
  host_vtable_normal_provider_vs_mimalloc_ratio=0.957
  host_vtable_normal_provider_slower_than_mimalloc_percent=4.5
  host_vtable_normal_tracking_insert_probe_total=3,150
  host_vtable_normal_tracking_lookup_probe_total=3,138
  host_vtable_usable_report=target/mimalloc-opt-refresh/provider-host-vtable-usable-gap-s5.out
  host_vtable_usable_provider_median_ops_per_sec=6,493,801.666
  host_vtable_usable_provider_vs_mimalloc_ratio=1.304
  host_vtable_usable_provider_slower_than_mimalloc_percent=-23.3
  host_vtable_usable_tracking_insert_probe_total=0
  host_vtable_usable_tracking_lookup_probe_total=0
  provider_host_allocator_init_result_total=6
  provider_host_allocator_vtable_init_count_total=6
  provider_host_passthrough_count_total=0
  provider_runtime_real_fallback_count_total=0
  provider_init_real_fallback_per_provider_operation=0.007
  provider_activation=0
  production_replacement_active=0
  hook_installed=0
  global_allocator_product_claim=0
  winner_claim=0
  decision=HostAllocatorV0_is_keeper
  selected_next_owner=shim_pointer_tracking_removal_for_provider_host_mainline
  selected_next_action=turn_provider_usable_size_claim_into_the_normal_owned-pointer_query_path_before_any_new_provider_C_shape_probe

provider_host_claim_mainline_refresh_2026_06_04:
  report=target/mimalloc-opt-refresh/provider-host-claim-mainline-gap-s5.out
  algorithm_coverage_report=target/mimalloc-opt-refresh/algorithm-coverage-provider-host-claim-mainline-gap-s5.out
  provider_median_ops_per_sec=8,950,548.221
  provider_vs_mimalloc_ratio=1.160
  provider_slower_than_mimalloc_percent=-13.8
  provider_claim_mainline_mode_enabled_total=6
  shim_tracking_insert_probe_total=0
  shim_tracking_lookup_probe_total=0
  provider_host_allocator_init_result_total=6
  provider_host_allocator_vtable_init_count_total=6
  provider_host_passthrough_count_total=0
  provider_runtime_real_fallback_count_total=0
  provider_init_real_fallback_per_provider_operation=0.007
  replacement_front_is_full_hako_algorithm=0
  replacement_front_multithread_claim=0
  provider_activation=0
  production_replacement_active=0
  hook_installed=0
  global_allocator_product_claim=0
  winner_claim=0
  decision=claim_mainline_is_keeper
  selected_next_owner=threaded_provider_claim_mainline_evidence
  selected_next_action=measure_provider_claim_mainline_under_threaded_workload_before_any_product_activation_or_winner_claim

provider_host_claim_mainline_thread_refresh_2026_06_04:
  tls_guard_change=provider_ldpreload_in_provider_call_guard_is_thread_local
  provider_benchmark_front_class=provider_host_adapter
  provider_ldpreload_measurement_route=provider_host_adapter_ldpreload
  provider_ldpreload_hako_hot_path_claim=0
  provider_ldpreload_hako_object_lifecycle_hot_path=0
  provider_ldpreload_hako_object_lifecycle_metadata_only=1
  provider_manifest_hako_provider_alloc_free_route=host_malloc_free_wrapper
  provider_manifest_hako_provider_alloc_free_uses_host_malloc=1
  provider_manifest_hako_provider_alloc_free_uses_hako_object_lifecycle=0
  provider_manifest_hako_provider_object_lifecycle_entrypoint_usage=metadata_verification_only
  thread2_report=target/mimalloc-opt-refresh/provider-host-claim-mainline-tls-guard-thread2-gap-s5.out
  thread2_provider_median_ops_per_sec=11,056,504.265
  thread2_provider_vs_mimalloc_ratio=1.077
  thread2_provider_slower_than_mimalloc_percent=-7.2
  thread2_provider_init_fallback_in_provider_call_count_total=0
  thread4_report=target/mimalloc-opt-refresh/provider-host-claim-mainline-tls-guard-thread4-gap-s5-i100k.out
  thread4_provider_median_ops_per_sec=61,968,455.887
  thread4_provider_vs_mimalloc_ratio=0.266
  thread4_provider_slower_than_mimalloc_percent=276.5
  thread4_provider_init_fallback_in_provider_call_count_total=0
  provider_claim_mainline_mode_enabled_total=6
  shim_tracking_insert_probe_total=0
  shim_tracking_lookup_probe_total=0
  provider_host_passthrough_count_total=0
  provider_runtime_real_fallback_count_total=0
  replacement_front_is_full_hako_algorithm=0
  replacement_front_multithread_claim=0
  hako_mimalloc_thread_hot_path_claim=0
  provider_host_adapter_thread_evidence=1
  provider_activation=0
  production_replacement_active=0
  hook_installed=0
  global_allocator_product_claim=0
  winner_claim=0
  decision=tls_guard_is_keeper_but_host_backed_provider_is_not_threaded_mimalloc_keeper
  selected_next_owner=thread_local_or_pure_provider_allocator_thread_shape
  selected_next_action=do_not_chase_host_backed_provider_C_shape_for_thread_perf; measure thread-local replacement front or pure-provider allocator boundary next

type_abi_route_descriptor_plane_task_2026_06_04:
  status=landed
  task_id=TYPEROUTE-001
  ssot=docs/development/current/main/design/type-abi-route-descriptor-plane-ssot.md
  descriptor_plane=Type_ABI
  execution_plane=Provider_ABI
  hot_replacement_plane=Replacement_front
  host_escape_plane=HostAllocator_vtable
  behavior_change=0
  provider_abi_execution_change=0
  replacement_front_hot_path_change=0
  type_abi_hot_path_lookup_allowed=0
  required_field_next_0=type_abi_route_descriptor_present=1
  required_field_next_1=type_abi_hot_path_lookup_count=0
  required_split=declared_route_vs_execution_route
  required_guard_next=host_backed_adapter_must_not_claim_hako_hot_path
  selected_next_action=add_type_abi_route_descriptor_report_boundary_without_changing_allocator_execution

type_abi_route_descriptor_report_boundary_2026_06_05:
  status=landed
  task_id=TYPEROUTE-002
  ssot=docs/development/current/main/design/type-abi-route-descriptor-plane-ssot.md
  touched_tools=tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py,tools/allocator/hakozuna_mixed_ws_gap_ladder.py
  behavior_change=0
  provider_abi_execution_change=0
  replacement_front_hot_path_change=0
  type_abi_route_descriptor_present=1
  type_abi_descriptor_plane=route_descriptor_control_plane
  type_abi_hot_path_lookup_count=0
  selected_next_task=TYPEROUTE-003
  selected_next_action=add_declared_route_vs_execution_route_and_guard_host_backed_adapter_hot_path_claim

type_abi_declared_execution_route_split_2026_06_05:
  status=landed
  task_id=TYPEROUTE-003
  ssot=docs/development/current/main/design/type-abi-route-descriptor-plane-ssot.md
  touched_tools=tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py,tools/allocator/hakozuna_mixed_ws_gap_ladder.py
  behavior_change=0
  provider_abi_execution_change=0
  replacement_front_hot_path_change=0
  declared_route_field=subject_N_declared_route
  execution_route_field=subject_N_execution_route
  provider_declared_route_field=provider_declared_route
  provider_execution_route_field=provider_execution_route
  host_backed_adapter_hako_hot_path_claim_guard=1
  selected_next_task=TYPEROUTE-004
  selected_next_action=wire_hako_check_or_python_readonly_descriptor_consumption

type_abi_route_descriptor_readonly_consumption_2026_06_05:
  status=landed
  task_id=TYPEROUTE-004
  ssot=docs/development/current/main/design/type-abi-route-descriptor-plane-ssot.md
  touched_tools=tools/allocator/type_abi_route_descriptor_readonly.py
  output_contract=type-abi-route-descriptor-readonly-v0
  readonly_descriptor_consumption=1
  python_introspection_adapter=1
  hako_check_core_change=0
  behavior_change=0
  provider_abi_execution_change=0
  replacement_front_hot_path_change=0
  type_abi_hot_path_lookup_count=0
  selected_next_task=TYPEROUTE-005
  selected_next_action=add_provider_registration_report_pairing_descriptor_and_ops

provider_registration_report_pairing_2026_06_05:
  status=landed
  task_id=TYPEROUTE-005
  ssot=docs/development/current/main/design/type-abi-route-descriptor-plane-ssot.md
  touched_tools=tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py,tools/allocator/hakozuna_mixed_ws_gap_ladder.py
  provider_registration_v1_present=1
  provider_registration_descriptor_plane=type_abi_route_descriptor
  provider_registration_ops_plane=provider_abi_execution_ops
  provider_registration_descriptor_ops_pairing=1
  provider_registration_hot_path_uses=provider_ops_only
  provider_registration_type_abi_hot_path_lookup_count=0
  behavior_change=0
  provider_abi_execution_change=0
  replacement_front_hot_path_change=0
  product_activation=0
  hook_installed=0
  global_allocator_claim=0
  winner_claim=0
  selected_next_action=refresh_provider_host_benchmark_with_descriptor_and_registration_fields

provider_registration_benchmark_refresh_2026_06_05:
  status=landed
  report_t1=target/type-abi-provider-registration-refresh-t1.out
  report_t4=target/type-abi-provider-registration-refresh-t4.out
  descriptor_fields_present=1
  provider_registration_v1_present=1
  provider_registration_hot_path_uses=provider_ops_only
  provider_registration_type_abi_hot_path_lookup_count=0
  provider_kind=host_backed_adapter
  provider_declared_route=provider_hako_object_lifecycle_ldpreload
  provider_execution_route=provider_host_adapter_ldpreload
  provider_hako_hot_path_claim=0
  t1_provider_median_ops_per_sec=25,337,519.138
  t1_provider_vs_mimalloc_ratio=0.582
  t1_provider_slower_than_mimalloc_percent=71.7
  t4_provider_median_ops_per_sec=9,970,115.213
  t4_provider_vs_mimalloc_ratio=0.144
  t4_provider_slower_than_mimalloc_percent=596.7
  shim_tracking_insert_probe_total=0
  shim_tracking_lookup_probe_total=0
  provider_activation=0
  hook_installed=0
  global_allocator_claim=0
  winner_claim=0
  selected_next_owner=thread_local_or_pure_provider_allocator_thread_shape
  selected_next_action=measure_thread_local_replacement_front_or_pure_provider_boundary_before_more_host_backed_C_shape_work

bench_route_equivalence_2026_06_05:
  status=landed
  task_id=BENCH-ROUTE-EQUIV-001
  purpose=compare_C_mimalloc_LD_PRELOAD_with_hako_replacement_front_under_same_benchmark_conditions
  frontdoor_tool=tools/allocator/hakozuna_mixed_ws_gap_ladder.py
  required_same_condition_fields=same_benchmark_binary,same_workload,same_threads,same_iters_per_thread,same_working_set,same_sample_count
  required_measurement_quality_fields=min_sample_seconds_required,min_observed_sample_seconds,measurement_quality
  subject_reference=c_mimalloc_ldpreload
  subject_candidate=hakorune_replacement_front_ldpreload
  candidate_execution_route=replacement_front_benchmark
  candidate_benchmark_front_class=replacement_front_c_shim
  provider_abi_execution_in_comparison=0
  replacement_front_bypasses_type_abi=1
  replacement_front_bypasses_provider_dispatch=1
  type_abi_hot_path_lookup_count=0
  product_activation=0
  hook_installed=0
  global_allocator_claim=0
  winner_claim=0
  report_t1=target/bench-route-equiv-t1.out
  report_t4_tls=target/bench-route-equiv-t4-tls.out
  report_t4_tls_quality_status=too_short_after_measurement_quality_guard
  t1_replacement_front_median_ops_per_sec=24,898,409.505
  t1_replacement_front_vs_mimalloc_ratio=0.899
  t1_replacement_front_slower_than_mimalloc_percent=11.3
  t4_tls_replacement_front_median_ops_per_sec=11,545,260.218
  t4_tls_replacement_front_vs_mimalloc_ratio=0.385
  t4_tls_replacement_front_slower_than_mimalloc_percent=159.7
  t4_tls_interpretation=superseded_by_quality_guard_too_short_sample
  selected_next_owner=benchmark_measurement_quality_gate
  selected_next_action=require_min_sample_seconds_before_threaded_keeper_or_nonkeeper_claim

bench_route_equivalence_quality_refresh_2026_06_05:
  status=landed
  task_id=BENCH-ROUTE-EQUIV-002
  purpose=prevent_sub_50ms_Hakozuna_mixed_ws_samples_from_selecting_threaded_keeper_or_nonkeeper
  changed_tools=tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py,tools/allocator/hakozuna_mixed_ws_gap_ladder.py
  new_report_fields=min_sample_seconds_required,min_observed_sample_seconds,median_observed_sample_seconds,measurement_quality
  short_sample_guard_report=target/bench-route-equiv-short-quality-fail.out
  short_sample_guard_summary=measurement_too_short
  short_sample_guard_exit_status=1
  long_sample_report=target/bench-route-equiv-long-quality-s3.out
  long_sample_measurement_quality=ok
  long_sample_min_observed_sample_seconds=0.070000
  long_sample_median_observed_sample_seconds=0.100000
  long_sample_threads=4
  long_sample_iters_per_thread=20,000,000
  long_sample_sample_count=3
  long_sample_warmup_count=1
  long_sample_replacement_front_vs_mimalloc_ratio=1.392
  long_sample_replacement_front_slower_than_mimalloc_percent=-28.1
  corrected_interpretation=previous_8192_iter_threaded_gap_was_measurement_quality_too_short_not_thread_shape_evidence
  selected_next_owner=route_equivalent_long_sample_thread_profile
  selected_next_action=use_long_quality_ok_reports_before_more_replacement_front_thread_shape_work

replacement_front_ordinary_app_route_descriptor_2026_06_05:
  status=landed
  task_id=BENCH-ROUTE-EQUIV-003
  purpose=separate_current_benchmark_replacement_front_from_future_ordinary_app_product_replacement_route
  descriptor_plane=type_abi_route_descriptor_control_plane
  execution_route_now=replacement_front_benchmark
  ordinary_app_route_candidate=replacement_front_product_ldpreload
  product_gate=closed
  product_activation_ready=0
  benchmark_only=1
  product_claim=0
  provider_dispatch_hot_path=0
  type_abi_hot_path_lookup_count=0
  changed_tools=tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py,tools/allocator/hakozuna_mixed_ws_gap_ladder.py,tools/allocator/type_abi_route_descriptor_readonly.py,tools/allocator/replacement_front_report.py
  selected_next_owner=product_replacement_front_activation_contract
  selected_next_action=keep_activation_closed_until_dedicated_product_replacement_row

replacement_front_product_activation_contract_2026_06_05:
  status=landed
  task_id=BENCH-ROUTE-EQUIV-004
  purpose=define_required_report_contract_before_replacement_front_product_activation_can_open
  product_gate=closed
  product_activation_ready=0
  production_replacement_active=0
  hook_installed=0
  global_allocator_product_claim=0
  replacement_front_product_activation_contract_v0=1
  requires_quality_ok=1
  requires_provider_dispatch_bypass=1
  requires_type_abi_hot_lookup_zero=1
  requires_cross_thread_policy=1
  requires_remote_abandoned_counters=1
  requires_rollback_optout_plan=1
  activation_blockers_initial=benchmark_only,product_gate_closed,no_activation_row,no_rollback_optout_plan
  changed_tools=tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py,tools/allocator/hakozuna_mixed_ws_gap_ladder.py,tools/allocator/type_abi_route_descriptor_readonly.py
  selected_next_owner=product_replacement_front_smoke_pack
  selected_next_action=add_non_activating_malloc_family_and_thread_safety_smokes

replacement_front_product_smoke_pack_2026_06_05:
  status=landed
  task_id=BENCH-ROUTE-EQUIV-005
  purpose=prove_product_candidate_replacement_front_malloc_family_and_thread_safety_without_activation
  product_smoke_pack_v0=1
  product_smoke_pack_non_activating=1
  product_gate=closed
  product_activation_ready=0
  production_replacement_active=0
  hook_installed=0
  global_allocator_product_claim=0
  malloc_family_smoke=malloc,calloc,realloc,free,null_free
  malloc_family_host_passthrough_count=0
  cross_thread_free_policy=remote_queue
  cross_thread_realloc_policy=unsupported_counted
  abandoned_owner_policy=mark_abandoned_no_host_free
  smoke_pack_report=target/replacement-front-product-smoke-pack-gap.out
  smoke_pack_malloc_family_smoke_ok=1
  smoke_pack_malloc_family_host_passthrough_count=0
  smoke_pack_cross_thread_free_remote_free_push_count=1
  smoke_pack_cross_thread_free_remote_free_drain_count=1
  smoke_pack_abandoned_owner_abandoned_arena_count=1
  smoke_pack_abandoned_owner_abandoned_remote_free_count=1
  smoke_pack_cross_thread_realloc_unsupported_count=1
  smoke_pack_cross_thread_realloc_host_passthrough_count=0
  changed_tools=tools/allocator/replacement_front_templates.py,tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py,tools/allocator/hakozuna_mixed_ws_gap_ladder.py
  selected_next_owner=product_replacement_front_long_quality_smoke_route
  selected_next_action=run_quality_ok_product_candidate_smoke_pack_with_long_route_equivalent_benchmark

replacement_front_product_smoke_pack_long_quality_2026_06_05:
  status=landed
  task_id=BENCH-ROUTE-EQUIV-006
  purpose=prove_product_candidate_smoke_pack_survives_quality_ok_route_equivalent_thread_benchmark
  report=target/replacement-front-product-smoke-pack-long-quality.out
  measurement_quality=ok
  min_sample_seconds_required=0.050000
  min_observed_sample_seconds=0.067000
  median_observed_sample_seconds=0.096000
  threads=4
  iters_per_thread=20,000,000
  sample_count=3
  warmup_count=1
  replacement_front_product_smoke_pack_v0=1
  replacement_front_product_smoke_pack_non_activating=1
  replacement_front_vs_mimalloc_ratio=1.402
  replacement_front_slower_than_mimalloc_percent=-28.7
  product_gate=closed
  product_activation_ready=0
  production_replacement_active=0
  hook_installed=0
  global_allocator_product_claim=0
  selected_next_owner=product_replacement_rollback_optout_plan
  selected_next_action=document_rollback_optout_before_any_activation_row

replacement_front_product_rollback_optout_plan_2026_06_05:
  status=landed
  task_id=BENCH-ROUTE-EQUIV-007
  purpose=document_default_off_rollback_and_per_process_optout_before_any_product_activation_row
  rollback_optout_plan_v0=1
  rollback_optout_env=HAKORUNE_REPLACEMENT_FRONT_DISABLE
  rollback_optout_env_value=1
  per_process_disable=1
  activation_mode=explicit_only
  activation_default=off
  activation_report_required=1
  rollback_report_path_required=1
  product_gate=closed
  product_activation_ready=0
  production_replacement_active=0
  hook_installed=0
  global_allocator_product_claim=0
  activation_blockers=benchmark_only,product_gate_closed,no_activation_row
  removed_activation_blocker=no_rollback_optout_plan
  changed_tools=tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py,tools/allocator/hakozuna_mixed_ws_gap_ladder.py,tools/allocator/type_abi_route_descriptor_readonly.py
  selected_next_owner=product_replacement_activation_preflight
  selected_next_action=add_non_activating_preflight_report

replacement_front_product_activation_preflight_2026_06_05:
  status=landed
  task_id=BENCH-ROUTE-EQUIV-008
  purpose=add_non_activating_product_candidate_preflight_report_before_any_activation_row
  product_preflight_report_v0=1
  product_preflight_non_activating=1
  product_gate=closed
  product_activation_ready=0
  production_replacement_active=0
  hook_installed=0
  global_allocator_product_claim=0
  preflight_evidence_ready_reported=1
  preflight_activation_ready=0
  preflight_quality_ok_reported=1
  preflight_provider_dispatch_bypass_ok_reported=1
  preflight_type_abi_hot_lookup_zero_ok_reported=1
  preflight_cross_thread_policy_ok_reported=1
  preflight_remote_abandoned_counters_ok_reported=1
  preflight_rollback_optout_ok_reported=1
  preflight_missing_always_includes=product_gate_open,activation_row
  activation_blockers=benchmark_only,product_gate_closed,no_activation_row
  changed_tools=tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py,tools/allocator/hakozuna_mixed_ws_gap_ladder.py,tools/allocator/type_abi_route_descriptor_readonly.py
  selected_next_owner=product_replacement_preflight_evidence_refresh
  selected_next_action=run_non_activating_preflight_smoke_and_readonly_descriptor_report

allocator_tool_boxshape_cleanup_2026_06_05:
  status=landed
  task_id=BENCH-ROUTE-EQUIV-CLEAN-001
  purpose=split_hakozuna_ldpreload_compare_report_control_plane_before_touching_execution_path_again
  cleanup_kind=BoxShape
  behavior_change=0
  product_gate=closed
  product_activation_ready=0
  production_replacement_active=0
  hook_installed=0
  global_allocator_product_claim=0
  target_primary=tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py
  target_primary_lines_before=1862
  target_primary_lines_after=1749
  first_split=product_activation_and_preflight_report_fields
  keep_in_compare=argparse,subject_orchestration,ldpreload_runner_invocation
  move_out=report_control_plane_field_emission
  new_module=tools/allocator/replacement_front_report.py
  changed_tools=CURRENT_TASK.md,tools/allocator/README.md,tools/allocator/replacement_front_report.py,tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py,tools/checks/k2_wide_phase296x_mimalloc_source_level_owner_refresh_guard.sh
  selected_next_owner=allocator_tool_smoke_runner_split
  selected_next_action=extract_replacement_front_focused_smokes_without_behavior_change

allocator_tool_smoke_runner_split_2026_06_05:
  status=landed
  task_id=BENCH-ROUTE-EQUIV-CLEAN-002
  purpose=split_replacement_front_focused_smoke_build_run_assert_logic_from_hakozuna_compare
  cleanup_kind=BoxShape
  behavior_change=0
  product_gate=closed
  product_activation_ready=0
  production_replacement_active=0
  hook_installed=0
  global_allocator_product_claim=0
  target_primary=tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py
  target_primary_lines_before=1749
  target_primary_lines_after=1610
  new_module=tools/allocator/replacement_front_smokes.py
  keep_in_compare=argparse,subject_orchestration,ldpreload_runner_invocation,smoke_report_field_emission
  move_out=focused_smoke_c_compile_run_assert
  changed_tools=tools/allocator/replacement_front_smokes.py,tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py,tools/allocator/README.md,tools/checks/k2_wide_phase296x_mimalloc_source_level_owner_refresh_guard.sh
  selected_next_owner=allocator_tool_smoke_template_split
  selected_next_action=move_smoke_c_templates_behind_reexport_facade_without_behavior_change

allocator_tool_smoke_template_split_2026_06_05:
  status=landed
  task_id=BENCH-ROUTE-EQUIV-CLEAN-003
  purpose=move_focused_smoke_c_source_text_to_dedicated_module_without_behavior_change
  cleanup_kind=BoxShape
  behavior_change=0
  product_gate=closed
  product_activation_ready=0
  production_replacement_active=0
  hook_installed=0
  global_allocator_product_claim=0
  target_primary=tools/allocator/replacement_front_templates.py
  target_primary_lines_before=1663
  target_primary_lines_after=1512
  new_module=tools/allocator/replacement_front_smoke_templates.py
  keep_in_templates=benchmark-only fixed-slot shim template,deterministic workload helpers,SizeClassBox mirror helpers
  move_out=focused_smoke_c_source_text
  changed_tools=tools/allocator/replacement_front_smoke_templates.py,tools/allocator/replacement_front_templates.py,tools/allocator/replacement_front_smokes.py,tools/allocator/README.md,tools/checks/k2_wide_phase296x_mimalloc_source_level_owner_refresh_guard.sh
  selected_next_owner=allocator_tool_sizeclass_split
  selected_next_action=move_size_class_helpers_into_dedicated_module_without_behavior_change

allocator_tool_support_helper_split_2026_06_05:
  status=landed
  task_id=BENCH-ROUTE-EQUIV-CLEAN-004
  purpose=move_shared_helper_math_and_size_class_workload_classification_to_dedicated_module_without_behavior_change
  cleanup_kind=BoxShape
  behavior_change=0
  product_gate=closed
  product_activation_ready=0
  production_replacement_active=0
  hook_installed=0
  global_allocator_product_claim=0
  target_primary=tools/allocator/replacement_front_templates.py
  target_primary_lines_before=1512
  target_primary_lines_after=1336
  new_module=tools/allocator/replacement_front_support.py
  keep_in_templates=benchmark-only fixed-slot shim template,smoke re-export facade
  move_out=shared helper math,size-class mirror helpers,workload histogram
  changed_tools=tools/allocator/replacement_front_support.py,tools/allocator/replacement_front_templates.py,tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py,tools/allocator/README.md,tools/checks/k2_wide_phase296x_mimalloc_source_level_owner_refresh_guard.sh
  selected_next_owner=allocator_tool_compare_report_split
  selected_next_action=move_report_assembly_and_route_metadata_helpers_out_of_compare_without_behavior_change

allocator_tool_compare_report_split_2026_06_05:
  status=landed
  task_id=BENCH-ROUTE-EQUIV-CLEAN-005
  purpose=move_manifest_route_classification_and_report_math_to_dedicated_module_without_behavior_change
  cleanup_kind=BoxShape
  behavior_change=0
  product_gate=closed
  product_activation_ready=0
  production_replacement_active=0
  hook_installed=0
  global_allocator_product_claim=0
  target_primary=tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py
  target_primary_lines_before=1612
  target_primary_lines_after=1424
  new_module=tools/allocator/hakozuna_mixed_ws_report_support.py
  keep_in_compare=argparse,runner,subject orchestration,LD_PRELOAD subject setup,report assembly
  move_out=manifest decoding,route classification,report-only math helpers
  changed_tools=tools/allocator/hakozuna_mixed_ws_report_support.py,tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py,tools/allocator/README.md,tools/checks/k2_wide_phase296x_mimalloc_source_level_owner_refresh_guard.sh
  selected_next_owner=allocator_tool_compare_report_assembly_split
  selected_next_action=move_report_assembly_line_building_out_of_compare_without_behavior_change

allocator_tool_compare_report_render_split_2026_06_05:
  status=landed
  task_id=BENCH-ROUTE-EQUIV-CLEAN-006
  purpose=move_hakozuna_mixed_ws_report_line_assembly_to_dedicated_module_without_behavior_change
  cleanup_kind=BoxShape
  behavior_change=0
  product_gate=closed
  product_activation_ready=0
  production_replacement_active=0
  hook_installed=0
  global_allocator_product_claim=0
  target_primary=tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py
  target_primary_lines_before=1424
  target_primary_lines_after=754
  new_module=tools/allocator/hakozuna_mixed_ws_report_render.py
  keep_in_compare=argparse,runner,subject orchestration,LD_PRELOAD subject setup
  move_out=report assembly,manifest decoding,route classification,report-only math,subject report line generation
  changed_tools=tools/allocator/hakozuna_mixed_ws_report_render.py,tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py,tools/allocator/README.md,tools/checks/k2_wide_phase296x_mimalloc_source_level_owner_refresh_guard.sh
  selected_next_owner=allocator_tool_shim_build_split
  selected_next_action=move_replacement_front_shim_build_helpers_out_of_compare_without_behavior_change

thread_local_replacement_front_profile_2026_06_05:
  status=landed
  input_report=target/bench-route-equiv-t4-tls.out
  perf_data=target/bench-route-equiv-thread-owner-perf/perf-long.data
  perf_report=target/bench-route-equiv-thread-owner-perf/perf-long-report.txt
  perf_primary_symbol=malloc
  perf_primary_symbol_pct=52.56
  perf_secondary_symbol=free
  perf_secondary_symbol_pct=11.49
  perf_memset_pct_total=9.29
  replacement_front_direct_core_call_count=8,003,592
  replacement_front_remote_free_push_count=0
  replacement_front_remote_free_drain_count=0
  replacement_front_arena_registry_overflow_count=0
  selected_owner=thread_local_replacement_front_malloc_free_hot_metadata_path
  selected_reason=perf_points_at_same_thread_malloc_free_not_remote_free
  slot2048_probe_report=target/thread-local-front-slot2048-t4.out
  slot2048_probe_decision=nonkeeper_worse_than_slot1040
  slot1024_probe_report=target/thread-local-front-slot1024-t4.out
  slot1024_probe_decision=nonkeeper_worse_than_slot1040
  hako_sizeclass_probe_report=target/thread-local-front-hako-sizeclass-t4.out
  hako_sizeclass_probe_decision=nonkeeper_worse_than_slot1040
  no_requested_size_probe_report=target/thread-local-front-no-requested-size-t4-s5.out
  no_requested_size_probe_decision=nonkeeper_reverted
  current_baseline_report=target/thread-local-front-slot1040-t4-s5.out
  current_baseline_replacement_front_vs_mimalloc_ratio=0.407
  current_baseline_replacement_front_slower_than_mimalloc_percent=145.8
  selected_next_action=inspect_malloc_free_generated_asm_for_TLS_metadata_store_shape_before_new_probe

thread_local_replacement_front_tls_asm_probe_2026_06_05:
  status=landed
  task_kind=report_only
  tool=tools/allocator/replacement_front_tls_asm_probe.py
  report=target/thread-local-front-tls-asm-probe.out
  malloc_instruction_count=86
  free_instruction_count=186
  malloc_fs_ref_count=14
  free_fs_ref_count=19
  malloc_requested_size_store=1
  free_requested_size_clear=1
  free_slot_index_magic_division=1
  free_remote_registry_path=1
  production_replacement_active=0
  hook_installed=0
  global_allocator_product_claim=0
  winner_claim=0
  selected_owner=thread_local_replacement_front_free_slot_index_decode
  selected_next_action=probe_free_slot_index_decode_shape_before_retrying_metadata_store_changes

thread_substrate_boundary_docs_2026_06_05:
  status=landed
  task_kind=docs_report_only
  owner_doc=docs/development/current/main/design/hako-thread-substrate-boundary-ssot.md
  related_docs=docs/reference/concurrency/semantics.md,docs/development/current/main/design/concurrency-async-pre-selfhost-ssot.md,docs/development/current/main/design/mimalloc-concurrency-substrate-boundary-ssot.md
  nowait_os_thread_spawn=0
  c_pthread_benchmark_hako_thread_support_claim=0
  benchmark_thread_origin=c_pthread
  hako_source_thread_support_claim=0
  allocator_threading_evidence=c_side
  worker_local_is_allocator_substrate=1
  scoped_context_is_task_local=1
  behavior_change=0
  source_syntax_expansion=0
  product_activation_ready=0
  production_replacement_active=0
  hook_installed=0
  global_allocator_product_claim=0
  next_thread_task=THREAD-API-001
  next_thread_task_scope=add_ThreadApi_yield_now_current_thread_id_and_route_runtime_policy_yields_through_ThreadApi
  selected_next_action=probe_free_slot_index_decode_shape_before_retrying_metadata_store_changes

thread_api_yield_current_id_2026_06_05:
  status=landed
  task_kind=runtime_substrate_cleanup
  owner_doc=docs/development/current/main/design/hako-thread-substrate-boundary-ssot.md
  code_owner=src/runtime/ring0/traits.rs
  added_thread_api=yield_now,current_thread_id
  host_thread_id_shape=u64
  runtime_policy_direct_yield_now_count=0
  worker_pool_enabled=0
  thread_spawn_join_added=0
  hako_source_thread_support_claim=0
  nowait_os_thread_spawn=0
  behavior_change=intended_none
  next_thread_task=THREAD-API-002
  next_thread_task_scope=inventory_and_classify_direct_std_thread_spawn_before_spawn_join_substrate
  selected_next_action=probe_free_slot_index_decode_shape_before_retrying_metadata_store_changes

thread_api_spawn_inventory_2026_06_05:
  status=landed
  task_kind=docs_report_only
  owner_doc=docs/development/current/main/design/hako-thread-substrate-boundary-ssot.md
  direct_std_thread_spawn_total=6
  runtime_substrate_spawn_candidate_count=2
  box_specific_spawn_workaround_count=2
  kernel_native_stress_spawn_count=2
  thread_spawn_join_added=0
  hako_source_thread_support_claim=0
  nowait_os_thread_spawn=0
  behavior_change=0
  next_thread_task=THREAD-API-003
  next_thread_task_scope=add_opaque_ThreadHandle_and_ThreadExit_without_exposing_to_hako_source
  selected_next_action=probe_free_slot_index_decode_shape_before_retrying_metadata_store_changes

thread_api_spawn_join_substrate_2026_06_05:
  status=landed
  task_kind=runtime_substrate
  owner_doc=docs/development/current/main/design/hako-thread-substrate-boundary-ssot.md
  code_owner=src/runtime/ring0/traits.rs
  added_thread_api=spawn,join
  thread_handle_shape=u64_opaque
  thread_exit_shape=Ok|Panic(String)
  thread_spawn_spec_shape=optional_name
  source_syntax_exposure=0
  nowait_os_thread_spawn=0
  worker_pool_enabled=0
  direct_std_thread_spawn_total=6
  direct_spawn_callsite_rewrite_count=0
  hako_source_thread_support_claim=0
  next_thread_task=THREAD-REG-001
  next_thread_task_scope=route_runtime_delayed_thread_candidates_or_add_thread_registry_cleanup_without_source_exposure
  selected_next_action=probe_free_slot_index_decode_shape_before_retrying_metadata_store_changes

thread_api_detach_spawn_task_after_2026_06_05:
  status=landed
  task_kind=runtime_substrate_cleanup
  owner_doc=docs/development/current/main/design/hako-thread-substrate-boundary-ssot.md
  code_owner=src/runtime/global_hooks.rs
  added_thread_api=detach
  routed_runtime_callsite=spawn_task_after_fallback
  direct_spawn_callsite_rewrite_count=1
  direct_std_thread_spawn_total_after=5
  runtime_substrate_spawn_candidate_count_after=1
  source_syntax_exposure=0
  nowait_os_thread_spawn=0
  worker_pool_enabled=0
  hako_source_thread_support_claim=0
  spawn_task_after_fallback_success_returns_true=1
  thread_spawn_failed_tag=[freeze:contract][thread/spawn_failed]
  thread_detach_failed_tag=[freeze:contract][thread/detach_failed]
  next_thread_task=THREAD-REG-002
  next_thread_task_scope=route_or_park_nyash_future_delay_i64_before_worker_pool
  selected_next_action=probe_free_slot_index_decode_shape_before_retrying_metadata_store_changes

thread_api_detach_future_delay_2026_06_05:
  status=landed
  task_kind=runtime_plugin_substrate_cleanup
  owner_doc=docs/development/current/main/design/hako-thread-substrate-boundary-ssot.md
  code_owner=crates/nyash_kernel/src/plugin/future.rs
  routed_runtime_callsite=nyash_future_delay_i64
  routed_thread_api=spawn,detach,sleep
  direct_spawn_callsite_rewrite_count=1
  direct_std_thread_spawn_total_after=5
  runtime_substrate_spawn_candidate_count_after=0
  box_specific_spawn_workaround_count_after=3
  kernel_native_stress_spawn_count_after=2
  future_delay_spawn_failed_sets_failed_future=1
  future_delay_detach_failed_sets_failed_future=1
  source_syntax_exposure=0
  nowait_os_thread_spawn=0
  worker_pool_enabled=0
  hako_source_thread_support_claim=0
  next_thread_task=THREAD-REG-003
  next_thread_task_scope=route_http_server_client_handler_spawn_through_ThreadApi
  selected_next_action=probe_free_slot_index_decode_shape_before_retrying_metadata_store_changes

thread_api_detach_http_server_client_2026_06_05:
  status=landed
  task_kind=box_specific_substrate_cleanup
  owner_doc=docs/development/current/main/design/hako-thread-substrate-boundary-ssot.md
  code_owner=src/boxes/http_server_box.rs
  routed_box_callsite=HTTPServerBox_client_handler
  routed_thread_api=spawn,detach
  active_connection_registry=id_only
  active_connection_unregister_on_handler_completion=1
  active_connection_unregister_on_spawn_failure=1
  http_server_active_connections_unbounded_growth=0
  direct_spawn_callsite_rewrite_count=1
  direct_std_thread_spawn_total_after=4
  runtime_substrate_spawn_candidate_count_after=0
  box_specific_spawn_workaround_count_after=2
  kernel_native_stress_spawn_count_after=2
  source_syntax_exposure=0
  nowait_os_thread_spawn=0
  worker_pool_enabled=0
  hako_source_thread_support_claim=0
  next_thread_task=THREAD-SCHED-001
  next_thread_task_scope=design_worker_pool_route_after_thread_registry_and_capture_safety_boundary_check
  selected_next_action=probe_free_slot_index_decode_shape_before_retrying_metadata_store_changes

worker_pool_scheduler_substrate_2026_06_05:
  status=landed
  task_kind=runtime_scheduler_substrate
  owner_doc=docs/development/current/main/design/hako-thread-substrate-boundary-ssot.md
  code_owner=src/runtime/scheduler.rs
  added_scheduler=WorkerPoolScheduler
  worker_pool_default_enabled=0
  worker_pool_source_route_enabled=0
  worker_pool_thread_api_spawn=1
  worker_pool_join_on_drop=1
  worker_pool_spawn_after_route=delayed_queue_poll
  worker_pool_delayed_tasks_occupy_worker_while_waiting=0
  source_syntax_exposure=0
  nowait_os_thread_spawn=0
  hako_source_thread_support_claim=0
  next_thread_task=THREAD-SAFETY-001
  next_thread_task_scope=send_share_thread_root_boundary_before_source_worker_routes
  selected_next_action=probe_free_slot_index_decode_shape_before_retrying_metadata_store_changes

worker_pool_delayed_timer_cleanup_2026_06_05:
  status=landed
  task_kind=runtime_scheduler_cleanup
  owner_doc=docs/development/current/main/design/hako-thread-substrate-boundary-ssot.md
  code_owner=src/runtime/scheduler.rs
  worker_pool_spawn_after_route=threadapi_single_timer_enqueue
  worker_pool_delayed_tasks_require_external_poll=0
  worker_pool_delayed_tasks_occupy_worker_while_waiting=0
  worker_pool_delayed_timer_threads_per_scheduler=1
  worker_pool_delayed_timer_threads_per_delayed_task=0
  worker_pool_thread_registry_unregister_guard=raii_drop
  worker_pool_threads_unregistered_on_panic_unwind=1
  test_reset_worker_id_reuse=0
  source_syntax_exposure=0
  nowait_os_thread_spawn=0
  worker_pool_source_route_enabled=0

thread_safety_registry_task_boundary_2026_06_05:
  status=accepted
  task_kind=docs_task_boundary
  owner_doc=docs/development/current/main/design/hako-thread-substrate-boundary-ssot.md
  next_thread_task=THREAD-SAFETY-001B
  next_thread_task_scope=implement_ThreadRegistry_v0_without_send_share_or_source_worker_route
  worker_id_distinct_from_host_thread_id=1
  thread_registry_gc_roots_enabled=0
  hako_send_share_enforced=0
  source_syntax_exposure=0
  nowait_os_thread_spawn=0
  worker_pool_source_route_enabled=0
  selected_next_action=probe_free_slot_index_decode_shape_before_retrying_metadata_store_changes

thread_registry_v0_2026_06_05:
  status=landed
  task_kind=runtime_thread_registry_substrate
  owner_doc=docs/development/current/main/design/hako-thread-substrate-boundary-ssot.md
  code_owner=src/runtime/thread_registry.rs
  scheduler_owner=src/runtime/scheduler.rs
  thread_registry_v0=1
  worker_id_shape=u64_opaque
  worker_id_distinct_from_host_thread_id=1
  thread_registry_snapshot_available=1
  worker_pool_threads_registered=1
  worker_pool_threads_unregistered_on_exit=1
  thread_registry_gc_roots_enabled=0
  hako_send_share_enforced=0
  source_syntax_exposure=0
  nowait_os_thread_spawn=0
  worker_pool_source_route_enabled=0
  next_thread_task=THREAD-SAFETY-001D
  next_thread_task_scope=descriptor_only_send_share_thread_root_capability_before_worker_route
  selected_next_action=probe_free_slot_index_decode_shape_before_retrying_metadata_store_changes

thread_capability_descriptor_v0_2026_06_05:
  status=landed
  task_kind=runtime_descriptor_vocabulary
  owner_doc=docs/development/current/main/design/hako-thread-substrate-boundary-ssot.md
  code_owner=src/runtime/thread_capability.rs
  hako_send_capability_descriptor_present=1
  hako_share_capability_descriptor_present=1
  hako_thread_root_descriptor_present=1
  hako_thread_capability_keys=hako.thread.send,hako.thread.share,hako.thread.root
  hako_send_share_enforced=0
  thread_registry_gc_roots_enabled=0
  worker_pool_source_route_enabled=0
  source_syntax_exposure=0
  nowait_os_thread_spawn=0
  type_abi_hot_path_thread_lookup=0
  next_thread_task=P2P-THREAD-001
  next_thread_task_scope=inventory_or_route_p2p_async_reply_helpers_through_ThreadApi
  selected_next_action=probe_free_slot_index_decode_shape_before_retrying_metadata_store_changes

p2p_threadapi_async_reply_cleanup_2026_06_05:
  status=landed
  task_kind=box_specific_substrate_cleanup
  owner_doc=docs/development/current/main/design/hako-thread-substrate-boundary-ssot.md
  code_owner=src/boxes/p2p_box.rs
  routed_box_callsite=P2PBox_sys_ping_reply,P2PBox_debug_async_reply
  routed_thread_api=spawn,detach,sleep
  direct_spawn_callsite_rewrite_count=2
  direct_std_thread_spawn_total_after=2
  runtime_substrate_spawn_candidate_count_after=0
  box_specific_spawn_workaround_count_after=0
  kernel_native_stress_spawn_count_after=2
  p2p_async_reply_threadapi_route=1
  source_syntax_exposure=0
  nowait_os_thread_spawn=0
  hako_source_thread_support_claim=0
  next_thread_task=MIMALLOC-THREAD-EVIDENCE-REFRESH
  next_thread_task_scope=refresh_replacement_front_thread_evidence_before_product_impl
  selected_next_action=run_replacement_front_thread_evidence_refresh

mimalloc_thread_evidence_refresh_2026_06_05:
  status=landed
  task_kind=benchmark_evidence_refresh
  smoke_report=target/hakozuna-mixed-ws-replacement-smoke/report.out
  perf_report=target/hakozuna-mixed-ws-replacement-perf/report.out
  smoke_sample_count=3
  perf_sample_count=7
  benchmark_threads=2
  replacement_front_smoke_route=thread_local_cross_thread_smoke
  replacement_front_perf_route=locked_global_multithread_front
  replacement_front_cross_thread_free_smoke_ok=1
  replacement_front_cross_thread_free_remote_free_push_count=1
  replacement_front_cross_thread_free_remote_free_drain_count=1
  replacement_front_cross_thread_free_arena_registry_overflow_count=0
  replacement_front_abandoned_owner_smoke_ok=1
  replacement_front_abandoned_owner_abandoned_arena_count=1
  replacement_front_abandoned_owner_abandoned_remote_free_count=1
  replacement_front_abandoned_owner_host_passthrough_count=0
  replacement_front_cross_thread_realloc_smoke_ok=1
  replacement_front_cross_thread_realloc_unsupported_count=1
  replacement_front_cross_thread_realloc_host_passthrough_count=0
  replacement_front_evidence_owner=locked_global_multithread_front
  replacement_front_multithread_perf_candidate=1
  replacement_front_thread_local_perf_candidate=0
  c_mimalloc_median_ops_per_sec=11362345.188
  replacement_front_median_ops_per_sec=11490686.798
  replacement_front_throughput_vs_c_mimalloc=1.011295
  replacement_front_is_full_hako_algorithm=0
  replacement_front_product_activation_ready=0
  replacement_front_product_activation_blockers=benchmark_only,product_gate_closed,no_activation_row
  provider_activation=0
  production_replacement_active=0
  hook_installed=0
  global_allocator_product_claim=0
  winner_claim=0
  hako_source_thread_support_claim=0
  next_thread_task=MIM-LOCKED-GLOBAL-001
  next_thread_task_scope=turn_locked_global_multithread_front_evidence_into_next_benchmark_only_impl_slice_without_product_activation
  selected_next_action=select_locked_global_multithread_front_impl_slice

mimalloc_locked_global_bins_impl_slice_2026_06_05:
  status=landed
  task_kind=benchmark_only_impl_slice
  scope=allow_locked_global_multithread_route_for_native_bins_and_page_bins_replacement_fronts
  report=target/hakozuna-mixed-ws-locked-bins/report.out
  sample_count=3
  benchmark_threads=2
  measurement_quality=ok
  activation=0
  provider_activation=0
  production_replacement_active=0
  hook_installed=0
  global_allocator_product_claim=0
  winner_claim=0
  hako_source_thread_support_claim=0
  accepted_route=locked_global_multithread_front
  rejected_route=thread_local_or_remote_free_for_bins
  replacement_front_native_bins_mode=1
  replacement_front_lock_mode=1
  replacement_front_evidence_owner=locked_global_multithread_front
  replacement_front_multithread_perf_candidate=1
  subject_2_thread_safety_claim=measured
  subject_2_replacement_front_lock_mode_enabled_total=4
  subject_2_replacement_front_lock_enter_count_total=8332
  subject_2_replacement_front_realloc_inplace_count_total=48
  subject_2_throughput_median_ops_per_sec=15393140.816
  subject_2_throughput_vs_c_mimalloc=1.444462
  next_thread_task=MIM-PAGE-BINS-LOCKED-001
  next_thread_task_scope=measure_locked_page_bins_and_hotcore_page_model_under_same_multithread_contract_before_product_pages

locked_page_bins_hotcore_refresh_2026_06_05:
  status=landed
  task_kind=benchmark_evidence_refresh
  scope=measure_page_bins_and_hotcore_under_locked_global_multithread_contract
  page_bins_report=target/hakozuna-mixed-ws-locked-page-bins/report.out
  hotcore_report=target/hakozuna-mixed-ws-locked-hotcore-page-model/report.out
  hotcore_size_table_eager_report=target/hakozuna-mixed-ws-locked-hotcore-size-table-eager-init/report.out
  coverage_report=target/hakozuna-mixed-ws-locked-hotcore-size-table-eager-init/coverage.out
  sample_count=3
  benchmark_threads=2
  measurement_quality=ok
  provider_activation=0
  production_replacement_active=0
  hook_installed=0
  global_allocator_product_claim=0
  winner_claim=0
  hako_source_thread_support_claim=0
  replacement_front_lock_mode=1
  replacement_front_evidence_owner=locked_global_multithread_front
  replacement_front_multithread_perf_candidate=1
  replacement_front_is_full_hako_algorithm=0
  replacement_front_product_activation_ready=0
  replacement_front_product_activation_blockers=benchmark_only,product_gate_closed,no_activation_row
  page_bins_route=benchmark_page_bins
  page_bins_size_class_lookup_route=range_scan
  page_bins_median_ops_per_sec=10607097.209
  hotcore_route=benchmark_page_bins_hotcore_page_model
  hotcore_size_class_lookup_route=range_scan
  hotcore_median_ops_per_sec=13895643.716
  hotcore_size_table_eager_route=benchmark_page_bins_hotcore_page_model
  hotcore_size_table_eager_size_class_lookup_route=table_8byte_bucket
  hotcore_size_table_eager_median_ops_per_sec=9035505.017
  selected_locked_page_route=benchmark_page_bins_hotcore_page_model
  selected_locked_page_reason=hotcore_locked_beats_plain_page_bins_in_same_thread_contract;size_table_eager_regresses_under_lock
  product_pages_consumer_enabled=0
  coverage_summary=ok
  coverage_hotcore_replacement_consumer_enabled=1
  coverage_structural_owner_selected=page_model_hot_array_source_route_measurement
  next_thread_task=MIM-HOTCORE-LOCKED-STABILITY-001
  next_thread_task_scope=repeat_selected_locked_hotcore_page_model_route_with_7_samples_before_product_pages

locked_hotcore_stability_refresh_2026_06_05:
  status=landed
  task_kind=measurement_quality_refresh
  scope=repeat_selected_locked_hotcore_page_model_route_with_7_samples_and_min_sample_seconds_gate
  short_report=target/hakozuna-mixed-ws-locked-hotcore-page-model-7/report.out
  quality_report=target/hakozuna-mixed-ws-locked-hotcore-page-model-quality-7/report.out
  coverage_report=target/hakozuna-mixed-ws-locked-hotcore-page-model-quality-7/coverage.out
  sample_count=7
  benchmark_threads=2
  benchmark_iters_per_thread=15000000
  min_sample_seconds_required=0.050000
  min_observed_sample_seconds=0.059000
  median_observed_sample_seconds=0.064000
  measurement_quality=ok
  provider_activation=0
  production_replacement_active=0
  hook_installed=0
  global_allocator_product_claim=0
  winner_claim=0
  hako_source_thread_support_claim=0
  replacement_front_lock_mode=1
  replacement_front_evidence_owner=locked_global_multithread_front
  replacement_front_multithread_perf_candidate=1
  replacement_front_is_full_hako_algorithm=0
  replacement_front_product_activation_ready=0
  replacement_front_product_activation_blockers=benchmark_only,product_gate_closed,no_activation_row
  selected_locked_page_route=benchmark_page_bins_hotcore_page_model
  selected_locked_page_lookup_route=range_scan
  selected_locked_page_median_ops_per_sec=16022119.206
  selected_locked_page_vs_c_mimalloc=0.031812
  c_mimalloc_median_ops_per_sec=503655734.785
  system_malloc_median_ops_per_sec=471889829.580
  subject_2_replacement_front_lock_enter_count_total=243012424
  subject_2_replacement_front_realloc_inplace_count_total=1306856
  product_pages_consumer_enabled=0
  coverage_summary=ok
  coverage_hotcore_replacement_consumer_enabled=1
  coverage_hotcore_replacement_measurement_reported=1
  coverage_structural_owner_selected=page_model_hot_array_source_route_measurement
  decision=do_not_open_product_pages_from_locked_global_route_yet
  decision_reason=quality_measurement_shows_locked_global_hotcore_correct_but_far_below_c_mimalloc
  next_thread_task=MIM-LOCKED-OWNER-ATTRIBUTION-001
  next_thread_task_scope=attribute_locked_global_hotcore_cost_before_product_pages_or_thread_local_reopen

mimalloc_locked_owner_attribution_plan_2026_06_05:
  status=accepted
  task_kind=benchmark_only_owner_attribution
  scope=split_locked_global_hotcore_cost_before_product_pages_or_thread_local_reopen
  baseline_report=target/hakozuna-mixed-ws-locked-hotcore-page-model-quality-7/report.out
  baseline_route=benchmark_page_bins_hotcore_page_model
  baseline_measurement_quality=ok
  baseline_median_ops_per_sec=16022119.206
  c_mimalloc_median_ops_per_sec=503655734.785
  product_pages_consumer_enabled=0
  provider_activation=0
  production_replacement_active=0
  hook_installed=0
  global_allocator_product_claim=0
  winner_claim=0
  first_slice=MIM-LOCKED-OWNER-ATTRIBUTION-001A
  first_slice_scope=enable_bins_skip_hot_counters_and_measure_locked_hotcore_counter_cost
  first_slice_safety=keep_global_lock_enabled;do_not_add_no_lock_multithread_claim
  parked_probe=unsafe_no_lock_bins_multithread_attribution_until_counter_probe_result

mimalloc_locked_hot_counter_attribution_2026_06_05:
  status=landed
  task_kind=benchmark_only_owner_attribution
  scope=enable_bins_skip_hot_counters_and_measure_locked_hotcore_counter_cost
  baseline_report=target/hakozuna-mixed-ws-locked-hotcore-page-model-quality-7/report.out
  skip_counter_report=target/hakozuna-mixed-ws-locked-hotcore-skip-counters-quality-7/report.out
  t1_nolock_skip_counter_report=target/hakozuna-mixed-ws-t1-hotcore-skip-counters-nolock-7/report.out
  t1_locked_skip_counter_report=target/hakozuna-mixed-ws-t1-hotcore-skip-counters-locked-7/report.out
  sample_count=7
  benchmark_iters_per_thread=15000000
  min_sample_seconds_required=0.050000
  measurement_quality=ok
  product_pages_consumer_enabled=0
  provider_activation=0
  production_replacement_active=0
  hook_installed=0
  global_allocator_product_claim=0
  winner_claim=0
  bins_skip_hot_counters_supported=1
  bins_skip_hot_counters_scope=benchmark_only_attribution
  skip_counter_safety=global_lock_still_enabled
  baseline_locked_hotcore_median_ops_per_sec=16022119.206
  locked_hotcore_skip_counter_median_ops_per_sec=16954382.797
  locked_hotcore_counter_tax_decision=non_dominant
  locked_hotcore_counter_tax_ratio=1.058182
  t1_hotcore_skip_counter_nolock_median_ops_per_sec=77975049.512
  t1_hotcore_skip_counter_locked_median_ops_per_sec=66563145.001
  t1_uncontended_lock_tax_ratio=0.853650
  decision=lock_contention_or_critical_section_shape_remains_primary_owner_candidate
  next_thread_task=MIM-LOCKED-OWNER-ATTRIBUTION-001B
  next_thread_task_scope=attribute_locked_global_critical_section_or_reopen_thread_local_bins_without_product_activation

mimalloc_replacement_front_fidelity_guard_2026_06_05:
  status=accepted
  task_kind=design_guard
  owner_doc=docs/development/current/main/design/mimalloc-replacement-front-fidelity-ssot.md
  scope=prevent_fast_non_mimalloc_allocator_routes_from_becoming_keepers
  current_reading=.hako_model_yes;generated_c_replacement_front_not_yet_full_mimalloc_execution
  language_cost_primary=0
  replacement_front_execution_shape_primary=1
  mimalloc_fidelity_guard=1
  required_keeper_shape=thread_local_page_arena;page_local_free;local_free;remote_free;owner_drain
  forbidden_keeper_shape=global_lock_hot_path;global_per_bin_stack_final;range_scan_hot_ownership;product_claim_before_remote_abandoned_counters
  next_thread_task=MIM-TLS-PAGE-ARENA-001
  next_thread_task_scope=docs_then_benchmark_only_ReplacementFrontTlsPageArenaPlanV0_without_product_activation

mimalloc_tls_page_arena_task_2026_06_05:
  status=accepted
  task_kind=benchmark_only_impl_plan
  owner_doc=docs/development/current/main/design/mimalloc-replacement-front-fidelity-ssot.md
  route_name=BenchmarkPageBinsHotCoreTlsRouteV0
  plan_name=ReplacementFrontTlsPageArenaPlanV0
  product_activation=0
  production_replacement_active=0
  hook_installed=0
  global_allocator_product_claim=0
  winner_claim=0
  first_slice=report_and_generator_flags
  first_slice_fields=replacement_front_thread_local_page_bins_mode,replacement_front_thread_local_hotcore_route,replacement_front_global_lock_hot_path_count
  second_slice=same_thread_tls_active_page_alloc_free
  third_slice=BenchmarkPageFromPtrBridgeV0_to_remove_range_scan_from_hot_free
  fourth_slice=remote_free_queue_and_abandoned_counters
  keeper_rule=fast_and_mimalloc_fidelity_guard_passed

product_pages_indexed_lookup_probe:
  attempted_change=replace the generated linear page-bins ownership scan with
    a benchmark-only page-key indexed ownership table
  report=target/hakozuna-mixed-ws-product-pages-nonlinear-7/report.out
  baseline_report=target/hakozuna-mixed-ws-eager-init-refresh-7/report.out
  median_ops_per_sec=7,168,818.507
  baseline_median_ops_per_sec=7,523,888.345
  product_pages_consumer_enabled=1
  product_pages_product_connected=0
  page_bins_lookup_route=indexed_page_table
  page_index_insert_count_total=125,632
  page_index_probe_count_total=4,208
  page_index_collision_count_total=0
  page_index_overflow_count_total=0
  decision=nonkeeper

coverage_after_product_pages_closure:
  replacement_front_product_pages_bridge_blocker=non_linear_probe_closed_nonkeeper
  replacement_front_product_pages_next_bridge=select_next_perf_owner
  replacement_front_product_pages_non_linear_lookup_probe_closed=1
  replacement_front_product_pages_non_linear_lookup_decision=nonkeeper
  structural_owner_selected=page_model_hot_array_source_route_measurement
  structural_owner_next_action=split_or_sink_public_init_stores_around_primitive_hot_state_body
  structural_owner_candidate_1_ready=0
  next_perf_owner_selection_plan_v0=1
  next_perf_owner_selected=primitive_dominant_mixed_store_shape
  next_perf_owner_selected_reason=backend_store_shape_classifier_ready
  next_perf_owner_next_bridge=split_or_sink_public_init_stores_around_primitive_hot_state_body
  perf_backend_store_shape_classifier_v0=1
  perf_backend_store_shape_selected=primitive_dominant_mixed_store_shape
  perf_backend_store_shape_hot_store_field_buckets=free_top:primitive_hot_state,block_size:public_semantics
  perf_backend_store_shape_weighted_dominant_bucket=primitive_hot_state
  perf_backend_store_shape_primitive_hot_state_store_percent=42.70
  perf_backend_store_shape_public_or_proof_store_percent=5.79
```

Interpretation: local C-shape cleanups can improve isolated assembly while
regressing end-to-end mixed-ws throughput. The naive page-map-backed ownership
bridge proves the report shape can consume product pages, but its linear lookup
is too expensive for the current optimization owner. The non-linear page-key
ownership table also regresses the same-run eager-init baseline, so product
pages stay parked until a different structural owner or workload evidence
selects them. After product-pages indexed lookup and record-state lowering are
both closed as nonkeepers for this slice, the perf attribution report now
classifies the backend store shape as primitive-dominant mixed traffic:
primitive hot-state stores dominate, while public/init/proof stores are still
visible enough to avoid a clean owner claim. The next concrete owner is to
split or sink public/init stores around the primitive hot-state body before
another generated-C local probe. The bins counter-skip
probe removes hot count writes but does not improve median throughput. The
free-only ownership decode makes the generated `free` assembly cleaner, but it
also fails the median-throughput keeper bar. The large-first ownership scan
regresses despite unchanged direct-core and host-passthrough counters, so the
current small-to-large find_owned ordering stays in place. Inlining the free
ownership release into `free()` also regresses with unchanged counters, so the
current find_owned/free routing stays in place. The generated address decision
tree produces only a noise-level 7-sample median delta and regresses the 10M
perf run, so it is also not a keeper. Marking the table-selected bin switch
default as unreachable removes one fallback check but badly regresses generated
code layout. Keep the existing HotCore/PageModel bridge and do not re-open
these probes without new perf owner evidence.

Stop-the-line: the current owner has enough negative local C-shape evidence.
Do not continue by adding more generated `find_owned` ordering, free-only
decode, counter, or switch-layout probes. The next positive-net candidate must
be structural: either consume more real `.hako` allocator state through a
planned bridge, or add a new route/report vocabulary that changes the owner
family before editing generated C again.

The current `ny_main` asm classifier reinforces this stop-line. The hottest
annotated field candidate is `HakoAllocPageModel.free_top`, with nearby
`requested_bytes` and `peak_used` stores:

```text
perf_top_instruction_category=store_like
perf_top_instruction_field_hints=0xa0:free_top
hot_instruction_0_context=...requested_bytes...free_top...peak_used...
backend_store_shape_classifier_v0=1
backend_store_shape_selected=primitive_dominant_mixed_store_shape
backend_store_shape_next_bridge=split_or_sink_public_init_stores_around_primitive_hot_state_body
backend_store_shape_weighted_dominant_bucket=primitive_hot_state
```

A repeat-amplified perf/asm check keeps the same owner shape:

```text
report=target/mimalloc-store-shape-repeat65536.asm.txt
runs=40
in_process_operation_repeat=65536
top_symbol=ny_main
symbol_collapse_detected=1
top_instruction_percent=37.86
top_instruction_category=store_like
top_instruction_field_hints=0xa0:free_top
backend_store_shape_selected=primitive_dominant_mixed_store_shape
backend_store_shape_hot_store_field_buckets=free_top:primitive_hot_state,local_free:direct_array_owner,reserved:public_semantics,used:primitive_hot_state,local_free_count:observer_counter
backend_store_shape_weighted_dominant_bucket=primitive_hot_state
backend_store_shape_primitive_hot_state_store_percent=42.70
backend_store_shape_public_or_proof_store_percent=5.79
inlined_hot_body_classifier_v0=1
inlined_hot_body_selected=acquire_fresh_small_like
inlined_hot_body_next_bridge=split_public_proof_stores_from_acquire_fresh_small_like_body
inlined_hot_body_acquire_fresh_small_percent=37.86
inlined_hot_body_release_local_known_live_percent=13.48
inlined_hot_body_init_public_store_percent=14.19
inlined_hot_body_split_ready=0
inlined_hot_body_split_blocker=checked_public_proof_accumulator_requires_overflow_policy
inlined_hot_body_split_next_bridge=add_public_proof_accumulator_overflow_policy_before_source_reorder
public_proof_accumulator_plan_v0=1
public_proof_accumulator_fields=requested_bytes
public_proof_accumulator_policy=checked_add_sign_guard
public_proof_accumulator_source_reorder_allowed=0
public_proof_accumulator_observed_no_overflow=1
public_proof_accumulator_general_no_overflow_proof=0
```

Attempted symbol-specific annotate on the same `perf.data` for
`HakoAllocPageModel.acquireFreshSmall/1`,
`HakoAllocPageModel.releaseLocalKnownLive/1`, and `Main.runOne/2` produced no
samples even though the global report has samples under `ny_main`. Treat this
as inlined/fused hot-body evidence for this measurement; do not wait on
symbol-specific annotate before classifying the `ny_main` instruction shape.

Do not reopen counter deletion/gating from this evidence. `requested_bytes`
is public/proof-visible, and the counter-skip probe already regressed. Treat
the next structural path as backend/store-shape separation evidence: split
or sink initialization/public stores around the primitive hot-state body, then
remeasure. The inlined hot-body classifier narrows the first target further:
the dominant context is `acquireFreshSmall`-like
(`requested_bytes/free_top/peak_used`). The next source/backend slice should
therefore focus on this acquire-like body before reopening
release/init/store-layout probes. However, the same context also shows the
`requested_bytes` public/proof accumulator. That accumulator now has an
explicit reject-before-accumulate source cap before the primitive `free_top`
store, so the representative 8192-repeat workload can provide source-side
overflow-policy proof for that cap. The weighted store classifier still says
the store owner is primitive hot-state dominant, so do not misread the current
evidence as primarily a public/proof counter deletion opportunity.

The workload arithmetic is now available as a separate requested-bytes
accumulator contract:

```text
output_contract=hako-mimalloc-requested-bytes-accumulator-contract-v0
accumulator_field=requested_bytes
accumulator_update=reject_before_accumulate_source_limit
source_overflow_policy_ready=1
source_overflow_limit=536870911
per_run_requested_bytes=33254
expected_no_overflow=1
observed_no_overflow=1
expected_within_source_overflow_limit=1
observed_within_source_overflow_limit=1
general_no_overflow_proof=1
source_reorder_allowed=1
```

The source now routes allocation paths through
`recordRequestedBytes(requested_size)`, which rejects accumulation before
mutating allocation state when the update would exceed the source policy cap.
The VM page-model proof was recovered first (`shape=14`, current
DirectArrayI64/i64 field contract, and selected
`HakoAllocPageModel.acquire_usize/1` shim allowance). Then an exact-numeric
helper-field mutation regression was fixed across VM and pure-first EXE and
pinned with
`tools/checks/k2_wide_vm_exact_numeric_helper_field_mutation_guard.sh`, so the
policy can stay in a helper instead of being pulled back into callers for VM
reasons. The LLVM main-lane page-model proof is pinned separately by
`tools/checks/k2_wide_mimalloc_page_model_guard.sh`, which now runs the
pure-first EXE through `tools/allocator/mimalloc_direct_exact_env.sh` so
`DirectArrayI64` free-stack semantics are tested on the direct-exact front, not
the public ArrayBox compatibility path. Direct-exact mimalloc guards should
source that preset and call `mimalloc_direct_exact_env_check`, rather than
hand-typing `HAKO_ARRAY_SLOT_STORE` / `HAKO_TYPED_OBJECT_STORE`. The
representative 8192-repeat workload is within the cap and may proceed to a
source-reorder probe; the repeat-amplified 65536 workload is outside this cap
and still reports `source_reorder_allowed=0` until a broader cap/contract is
explicitly accepted.

A `.hako` acquire-family store-order probe was measured and closed as a
nonkeeper:

```text
report=target/mimalloc-acquire-store-order-probe.asm.txt
changed=acquire_usize/acquireFreshSmall stored free_top before requested_bytes
body_elapsed_ns_before=18000000
body_elapsed_ns_after=19000000
store_like_percent_before=68.59
store_like_percent_after=22.41
top_instruction_after=memory_load_used
decision=nonkeeper_reverted
reason=instruction_shape_changed_without_positive_body_time_evidence
source_reorder_allowed=0
```

The probe proves the `.hako` source order can affect the fused `ny_main` hot
shape, but it does not authorize keeping the source change. Continue through
the accumulator overflow policy / proof bridge before reordering
`requested_bytes` around primitive hot-state stores.
Do not open duplicate `RecordStateResidencePlanV0` lowering unless a later
representation delta turns positive.

The coverage overlay now buckets those field hints:

```text
page_model_hot_field_traffic_plan_v0=1
page_model_hot_field_traffic_ready=1
page_model_hot_field_top=free_top
page_model_hot_field_top_bucket=primitive_hot_state
page_model_hot_field_buckets=free_top:primitive_hot_state,requested_bytes:public_semantics_proof_evidence,peak_used:primitive_hot_state
page_model_hot_field_counter_deletion_allowed=0
page_model_hot_field_next_bridge=record_state_residence_plan_report
record_state_residence_plan_v0=1
record_state_residence_report_only=1
record_state_residence_ready=1
record_state_residence_static_candidate_fields=used,free_top,local_free_top,retired,decommitted,peak_used
record_state_residence_observed_candidate_fields=free_top,peak_used
record_state_residence_rejected_observed_fields=requested_bytes:public_semantics_proof_evidence
record_state_residence_source_migration_allowed=0
record_state_residence_next_bridge=record_state_residence_metadata_producer
```

Interpretation: the hot field owner is not another counter-elision row. The
next narrow compiler-facing owner is report-only `RecordStateResidencePlanV0`
or equivalent state-representation vocabulary for primitive PageModel fields.
PageModel remains the owner `box`; source migration to a `PageState` record is
not open until a dedicated lowering/source-migration slice accepts it.

The first metadata producer slice now emits the report-only row through MIR JSON
and `hako_check state-explain`:

```text
record_state_residence_plan_count=1
record_state_residence_plan_0_owner_box=HakoAllocPageModel
record_state_residence_plan_0_candidate_record=PageState
record_state_residence_plan_0_residence=box_private_record_state_v0
record_state_residence_plan_0_report_only=1
record_state_residence_plan_0_source_migration_allowed=0
record_state_residence_plan_0_selected_field_count=6
record_state_residence_plan_0_rejected_field_count=20
```

The next report-only slice also emits access-site metadata. When the algorithm
coverage overlay consumes both the fastpath report and this state report:

```text
fastpath_report_consumed=1
state_report_consumed=1
page_model_hot_array_source_route_measured=1
page_model_hot_array_source_route_measurement_blocker=none
record_state_field_access_plan_count=88
record_state_field_access_ready=1
record_state_field_access_lowering_enabled=0
record_state_route_decision_enabled=0
record_state_lowering_owner_selected=typed_object_exact_slot_existing
record_state_access_exact_slot_covered_count=88
record_state_access_exact_slot_missing_count=0
record_state_lowering_owner_next_bridge=measure_representation_delta_before_record_state_lowering
record_state_representation_delta_plan_v0=1
record_state_representation_delta_ready=1
record_state_representation_delta_positive_candidate=0
record_state_representation_delta_blocker=typed_object_exact_slot_already_covers_record_state_access
record_state_representation_delta_next_bridge=design_non_linear_product_pages_bridge
record_state_residence_next_bridge=design_non_linear_product_pages_bridge
```

This is still metadata only. It does not create a runtime `PageState`, direct
record lowering, whole-record ABI, or source migration permission.

Interpretation: the observed record-state access sites are already covered by
the typed-object exact slot storage owner. Enabling a separate record-state
lowering lane now would duplicate the existing slot route unless a
representation delta is measured first. The current representation delta report
has no positive candidate (`record_state_representation_delta_positive_candidate=0`),
so keep record-state RouteDecision rows disabled and hand off to the next
structural owner: a non-linear product-pages bridge design.

RouteDecision stop-line:

```text
record_state_field_access_lowering_enabled=0
record_state_route_decision_enabled=0
```

Do not add `RecordStateFieldAccessPlan` rows to `RouteDecision` while lowering
is disabled. `hako_check fastpath-explain --profile direct-memory` is currently
the DirectArray/direct-memory route truth and must not be polluted with
report-only record-state candidates. The next accepted slice must first choose
the lowering owner and flip a narrow `lowering_enabled` contract, then add
RouteDecision rows for those enabled sites.

The same algorithm coverage overlay now reports product-pages bridge readiness
without opening product replacement:

```text
replacement_front_product_pages_bridge_plan_v0=1
replacement_front_product_pages_bridge_report_only=1
replacement_front_product_pages_consumer_enabled=0
replacement_front_product_pages_route=not_consumed
replacement_front_product_pages_source_ready=1
replacement_front_product_pages_full_source_ready=1
replacement_front_product_pages_bridge_blocker=consumer_not_enabled
replacement_front_product_pages_next_bridge=design_non_linear_product_pages_bridge
replacement_front_product_pages_non_linear_lookup_plan_v0=1
replacement_front_product_pages_linear_probe_closed=1
replacement_front_product_pages_non_linear_lookup_strategy=range_decision_tree_or_indexed_page_table
replacement_front_product_pages_non_linear_next_bridge=replacement_front_product_pages_non_linear_plan
page_map_source_ready=1
page_map_release_source_ready=1
realloc_same_class_source_ready=1
realloc_grow_copy_release_source_ready=1
huge_page_source_ready=1
osvm_page_source_pilot_ready=1
```

Interpretation: `.hako` PageMap, page-map release, same-class realloc,
grow/copy/release, huge model, and OSVM pilot seams are source-present for the
next bridge design. The replacement front still does not consume product pages,
and `replacement_front_is_full_hako_algorithm=0` remains the claim boundary.
The previous linear page-map-backed lookup probe is closed as a nonkeeper; the
next implementation owner is a non-linear ownership lookup plan for a
benchmark-only product-pages bridge, not product activation or a full allocator
claim.

The first benchmark-only non-linear consumer is available through
`--replacement-front-product-pages-nonlinear-mode` on top of page-bins:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --replacement-front-page-bins-mode \
  --replacement-front-hotcore-page-model-mode \
  --replacement-front-size-class-table-mode \
  --replacement-front-eager-init-mode \
  --replacement-front-product-pages-nonlinear-mode \
  --threads 1 \
  --out target/hakozuna-mixed-ws-product-pages-nonlinear/report.out \
  --out-dir target/hakozuna-mixed-ws-product-pages-nonlinear/artifacts \
  --sample-count 3
```

Small smoke evidence:

```text
report=target/hakozuna-product-pages-nonlinear-smoke/report.out
replacement_front_algorithm_shape=page_bin_hotcore_page_model_product_pages_nonlinear_benchmark_front
replacement_front_product_pages_nonlinear_mode=1
replacement_front_product_pages_consumer_enabled=1
replacement_front_benchmark_product_pages_consumer_enabled=1
replacement_front_product_pages_route=benchmark_product_pages_indexed_page_table
replacement_front_benchmark_product_pages_route=benchmark_product_pages_indexed_page_table
replacement_front_product_pages_product_connected=0
replacement_front_page_bins_lookup_route=indexed_page_table
replacement_front_page_index_insert_count_total=10382
replacement_front_page_index_probe_count_total=37
replacement_front_page_index_collision_count_total=0
replacement_front_page_index_overflow_count_total=0
summary=ok
```

Interpretation: this consumes a non-linear page-key ownership lookup in the
benchmark-only replacement front. The subsequent 7-sample measurement above
closed this exact indexed lookup as a nonkeeper; product replacement, hooks,
globals, full `.hako` algorithm claims, and winner claims remain closed.

The same report carries the current PageModel hot-array readiness view:

```text
size_class_policy_product_bins_connected=0
size_class_policy_single_class_benchmark_bridge_supported=1
size_class_policy_single_class_bridge_mode=hako_good_size_request_ceiling
structural_owner_selection_plan_v0=1
structural_owner_refresh_required=1
structural_owner_selected=page_model_hot_array_source_route_measurement
structural_owner_selected_reason=hotcore_measured_and_directarray_source_ready
structural_owner_next_action=measure_page_model_hot_array_perf_delta
structural_owner_candidate_0=page_model_hot_array_source_route_measurement
structural_owner_candidate_0_ready=1
structural_owner_candidate_1=product_pages_bridge_non_linear_owner_lookup
structural_owner_candidate_1_ready=1
fastpath_report_consumed=1
page_model_hot_array_source_route_measurement_plan_v0=1
page_model_hot_array_source_route_measured=1
page_model_hot_array_source_route_measurement_blocker=none
page_model_hot_array_source_route_next_bridge=perf_delta_measurement
page_model_hot_array_fastpath_direct_array_plan_count=24
page_model_hot_array_fastpath_route_decision_count=24
page_model_hot_array_fastpath_fast_selected_count=24
page_model_hot_array_fastpath_slow_selected_count=0
page_model_hot_array_bridge_plan_v0=1
page_model_hot_array_access_plan_v0=1
page_model_hot_array_source_migration_selected=1
page_model_hot_array_source_type_ready=1
page_model_hot_array_birth_contract_ready=1
page_model_hot_array_source_migration_blocker=none
page_model_hot_array_next_bridge=source_migration_measurement
page_model_hot_array_candidate_type=DirectArrayI64
page_model_hot_array_arraybox_fields=none
page_model_hot_array_directarray_fields=free,local_free,block_used
page_model_hot_array_directarray_supported_ops=get,set
page_model_hot_array_directarray_missing_ops=none
page_model_hot_array_seed_push_blocker=0
page_model_hot_array_op_summary=free:get=...:set=...:push=...
hotcore_replacement_bridge_plan_v0=1
hotcore_replacement_consumer_enabled=0
hotcore_replacement_shape_ready=1
hotcore_replacement_bridge_blocker=consumer_not_enabled
hotcore_replacement_next_bridge=replacement_front_consume_hotcore_page_model
hotcore_page_model_source_ready=1
hotcore_small_alloc_calls_acquire_fresh_small=1
hotcore_release_calls_release_local_known_live=1
page_model_hot_methods_ready=1
hotcore_source_methods=objectLifecycleSmallAlloc,objectLifecycleReleaseBlock
```

This block is the source-ready-only boundary before
`--replacement-front-hotcore-page-model-mode`. The current keeper report overlays
the consumed route as `hotcore_replacement_consumer_enabled=1`,
`hotcore_replacement_measurement_reported=1`, and
`hotcore_replacement_next_bridge=select_next_structural_owner`. The structural
owner handoff selects `page_model_hot_array_source_route_measurement` first,
because `free` / `local_free` / `block_used` are already source-level
`DirectArrayI64`. A matching `hako_check fastpath-explain` report now confirms
that this source route has clean DirectArray RouteDecision coverage
(`fast_selected=24`, `slow_selected=0`). Product pages stay parked after both
the linear page-map and indexed page-key ownership probes regressed; reopen
them only if a different structural owner or workload evidence selects them.
Activation, hooks, globals, and winner claims remain closed.

The first perf/asm attribution pass is now an explicit measurement boundary:
`tools/allocator/hako_mimalloc_direct_exact_app_perf_asm.sh` appends
`tools/allocator/hako_mimalloc_perf_attribution.py` output to its report. The
algorithm coverage overlay accepts this report through
`--perf-attribution-report`, so the owner handoff can move past "measure perf
delta" once the blocker is known. The current known artifact shape is:

```text
top_symbol=ny_main
symbol_collapse_detected=0
symbol_attribution_available=0
instruction_attribution_available=1
page_model_hot_array_perf_delta_measurement_plan_v0=1
page_model_hot_array_perf_delta_ready=0
page_model_hot_array_perf_delta_blocker=missing_directarray_or_pagemodel_symbol_attribution
page_model_hot_array_perf_delta_next_bridge=asm_instruction_classifier_or_symbol_split
top_instruction_category=store_like
top_instruction_field_hints=0xa0:free_top
backend_store_shape_classifier_v0=1
backend_store_shape_selected=mixed_primitive_and_public_store_shape
backend_store_shape_next_bridge=split_init_public_stores_from_primitive_hot_state_stores
```

Interpretation: the source route is clean, and perf has enough annotated
instructions to guide instruction-shape cleanup, but symbol ownership is still
too collapsed to prove a DirectArray/PageModel-specific perf delta. Do not read
symbol-based DirectArray owner refresh scripts returning `0%` as negative
evidence while this blocker is present.

With benchmark, fastpath, and perf attribution reports all supplied,
`hako_mimalloc_algorithm_coverage.py` now reports:

```text
perf_attribution_report_consumed=1
structural_owner_next_action=record_state_residence_metadata_producer
perf_top_instruction_category=store_like
perf_top_instruction_field_hints=0xa0:free_top
page_model_hot_field_top=free_top
page_model_hot_field_top_bucket=primitive_hot_state
page_model_hot_field_buckets=free_top:primitive_hot_state,requested_bytes:public_semantics_proof_evidence,peak_used:primitive_hot_state
record_state_residence_observed_candidate_fields=free_top,peak_used
record_state_residence_rejected_observed_fields=requested_bytes:public_semantics_proof_evidence
record_state_residence_next_bridge=record_state_residence_metadata_producer
perf_hot_instruction_0_context_categories=arithmetic_compare,branch,memory,store_like
```

The PageModel hot-array access scan distinguishes hot `get/set` traffic from
seed-time initialization traffic. `seedFreeBlocks` uses append-or-overwrite
`set(i, ...)` shape for all three arrays, so the old `ArrayBox.push` blocker is
closed. The PageModel `free` / `local_free` / `block_used` fields are now
`DirectArrayI64` source fields.

Current implementation note: DirectArrayI64 source receivers are accepted by
the generic `get/set` route producer and by DirectArrayAccessPlan /
DirectArrayExtentFact metadata.

Explicit `new DirectArrayI64()` also lowers to the direct-i64 birth symbol in
the LLVM exact front. The next bridge is source migration measurement, not
another route/birth fixture.

Acceptance for claiming algorithmic completeness stays closed until the report
can show the `.hako` size-class/page-local/HotCore route as the executed
replacement-front path. Until then, replacement-front benchmark results are
thin-front evidence only:

```text
replacement_front_is_full_hako_algorithm=0
size_class_policy_product_bins_connected=0
provider_activation=0
production_replacement_active=0
winner_claim=0
```

## Current Evidence Anchors

Recent completed slices:

```text
MIM-116..MIM-123:
  state bucket / record-state residence / hako_check state-explain /
  source-surface responsibility cleanup

MIM-124..MIM-134:
  owner-first perf refreshes, body-timer control, workload matrix,
  direct-exact app perf/asm tooling, observer-light closeout

MIM-135..MIM-146:
  provider package explicit measurement, hakmem and hakozuna LD_PRELOAD
  bridge, generated global allocator smoke, provider export bundle,
  same-machine Hakozuna mixed-ws C mimalloc comparison
```

Latest local benchmark bridge:

```text
tool:
  tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py

fixture:
  benchmarks/external/hakozuna/mixed-ws/build/bench_mixed_ws_crt

reference_subject:
  c_mimalloc_ldpreload

optional_subject:
  hakorune_provider_ldpreload when --manifest is passed

report_contract:
  hakozuna-mixed-ws-ldpreload-compare-v0

interpretation:
  provider subject is provider ABI wrapper + LD_PRELOAD shim bridge evidence
  provider_ldpreload_is_hako_core_speed_claim=0
  provider_ldpreload_is_product_allocator_claim=0
  provider_manifest_hako_provider_alloc_free_route=host_malloc_free_wrapper
  provider_manifest_hako_provider_alloc_free_uses_hako_object_lifecycle=0

key shim diagnostic:
  shim_init_real_fallback_per_provider_operation

owner hint:
  provider_alloc_free_internal_real_malloc_boundary when init fallback
  dominates provider operations
```

Provider ABI / shim boundary cleanup is now documented in:

```text
docs/development/current/main/design/provider-abi-shim-boundary-ssot.md
```

Current task order:

```text
PROV-ABI-001:
  docs/report-only boundary for provider kinds, claim ops, and future
  HostAllocatorV0

PROV-ABI-002:
  add optional free_claim API tail entry
  LD_PRELOAD shim free path prefers provider.free_claim
  keep alloc/free/owns compatibility surface
  do not add realloc_claim or HostAllocatorV0 in this slice

PROV-ABI-003:
  add optional usable_size_claim API tail entry
  native-slot route owns usable-size truth
  host-backed route was deferred until PROV-ABI-005 HostAllocatorV0

PROV-ABI-004:
  add optional realloc_claim API tail entry
  native-slot route handles provider-owned realloc lifecycle
  host-backed route was deferred until PROV-ABI-005 HostAllocatorV0

PROV-ABI-005:
  add HostAllocatorV0 init tail entry
  host-backed route receives explicit host malloc/free/realloc/usable_size vtable
  provider direct libc/private symbol dependency remains closed
```

PROV-ABI-002 landed as a narrow ABI-boundary cleanup:

```text
provider API:
  appended optional free_claim tail entry
  alloc/free/owns compatibility surface remains

manifest/report:
  provider_allocator_kind=host_backed_adapter|pure_allocator
  provider_abi_claim_ops_v1=1
  provider_free_claim_enabled=1
  provider_realloc_claim_enabled=0
  provider_usable_size_claim_enabled=0
  compat_owns_free_mainline=0

smoke evidence:
  host-backed smoke free_claim_bound=1 free_claim_count=2 summary=ok
  native-slot smoke free_claim_bound=1 free_claim_count=2 summary=ok

gap evidence:
  host-backed report=target/prov-abi/free-claim-host-gap-s3.out
  native-slot single-thread report=target/prov-abi/free-claim-native-gap-t1-s3.out
  shim tracking remains present for realloc/size compatibility
  next boundary=usable_size_claim_or_realloc_claim_before_tracking_removal
```

Provider package route split:

```text
object-lifecycle-small-alloc-release-v0:
  verifies selected .hako object-lifecycle MIR call chain
  provider alloc/free route = host_malloc_free_wrapper

object-lifecycle-native-slot-bridge-v0:
  verifies the same selected .hako object-lifecycle MIR call chain
  provider alloc/free route = native_static_slot_bridge_from_object_lifecycle_shape
  uses_host_malloc = 0
  use only as explicit provider-package / LD_PRELOAD bridge evidence
  product activation / hook / global allocator / winner claim remain closed
```

## Next Task Order

1. Keep algorithm-port coverage visible.
   - Run `tools/allocator/hako_mimalloc_algorithm_coverage.py` before opening a
     new allocator-algorithm claim.
   - If the report still says `replacement_front_is_full_hako_algorithm=0`,
     treat the benchmark-only front as thin-front evidence, not product
     allocator evidence.

2. Bridge `.hako` algorithm pieces only by explicit owner evidence.
   - First candidate: size-class policy to replacement bins/pages.
     `--replacement-front-match-hako-size-class` is the benchmark-only
     single-class bridge and may be used for local evidence. It does not claim
     product bins/pages.
     The compare report now emits product bins/pages fields. Without bins mode:
     `replacement_front_product_bins_consumer_enabled=0` and
     `replacement_front_product_pages_consumer_enabled=0`.
     `--replacement-front-native-bins-mode` is the next benchmark-only v0:
     it consumes workload regular bins but keeps pages/product activation
     closed.
  - Second candidate: `HakoAllocPageModel` hot arrays to DirectArrayI64-backed
    storage. This source migration is complete for `free` / `local_free` /
    `block_used`; do not keep probing local get/set shape unless new owner
    evidence selects it. The remaining bridge is measurement in a product-like
    execution path, not another source rewrite.
  - Third candidate: HotCore/PageModel plan consumption by replacement-front
    lowering. The benchmark-only HotCore/PageModel wrapper mode is connected
    and measured; `hako_mimalloc_algorithm_coverage.py --benchmark-report ...`
    now reports `hotcore_replacement_measurement_reported=1` and
    `hotcore_replacement_next_bridge=select_next_structural_owner`.
    The same overlay reports
    `structural_owner_selected=page_model_hot_array_source_route_measurement`
    as the next owner. With `--fastpath-report`, it also reports
    `page_model_hot_array_source_route_measured=1`; the next action is
    `measure_page_model_hot_array_perf_delta`, not another generated-C local
    probe. The perf/asm attribution helper now classifies the current backend
    store shape as `mixed_primitive_and_public_store_shape`; continue by
    separating initialization/public stores from primitive hot-state stores
    before claiming a PageModel hot-array perf delta.
  - Current stop-line: local generated-C probes around `find_owned`, free-only
    ownership decode, counters, and switch layout have enough negative evidence.
    The next positive-net candidate must change the structural owner family
    before editing generated C again.
  - Do not weaken ProviderFront or add source syntax to force the bridge.

3. Measure the native slot bridge provider route.
   - Build with `--provider-package-hako-semantic-codegen
     object-lifecycle-native-slot-bridge-v0`.
   - Confirm `hako_provider_alloc_free_uses_host_malloc=0`.
   - Compare against the previous host-wrapper provider route and C mimalloc.

4. Run a stable same-machine Hakozuna mixed-ws comparison.
   - Use enough iterations/samples to reduce startup noise.
   - Compare system malloc, C mimalloc LD_PRELOAD, and optional Hakorune
     provider LD_PRELOAD.
   - Treat the result as local evidence only.

5. Classify the remaining gap.
   - If provider shim counters dominate, optimize shim/provider boundary first.
   - If `.hako` allocator core dominates, return to direct-exact app perf/asm.
   - If benchmark setup noise dominates, improve measurement before code edits.
   - Do not call provider LD_PRELOAD evidence a `.hako` core speed result while
     `provider_ldpreload_is_hako_core_speed_claim=0`.
   - Use `HAKORUNE_PROVIDER_LDPRELOAD_USE_USABLE_SIZE=1` only as a
     measurement-only native-slot shim variant. It bypasses shim pointer
     tracking through the private `hakorune_provider_usable_size_v0` export and
     exists to distinguish tracking tax from provider ABI call-boundary tax.
   - Use `HAKORUNE_PROVIDER_LDPRELOAD_ASSUME_PROVIDER_OWNED=1` only with
     usable-size mode in controlled benchmarks. It skips provider `owns` checks
     to isolate the remaining call-boundary tax and is not a replacement
     contract.

6. Continue provider replacement ladder only as smoke/readiness.
   - Keep `provider_activation=0`, `production_replacement_active=0`,
     `hook_installed=0`, `global_allocator_product_claim=0`,
     `winner_claim=0`.

7. Add a benchmark-only `HakoAllocReplacementFront` probe if C-like thinness is
   still required.
   - Do not weaken `HakoAllocProviderFront`.
   - The probe must bypass provider API table dispatch, hot function-pointer
     calls, shim pointer tracking, and hot `owns` checks.
   - Required report fields:

```text
provider_table_dispatch=0
function_pointer_hot_call=0
owns_check_hot_path=0
tracking_hot_path=0
direct_core_call=1
activation=0
benchmark_only=1
summary=ok
```

   - Tool entry:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --replacement-front-native-slot-mode \
  --out-dir /tmp/hakorune_replacement_front_compare \
  --out /tmp/hakorune_replacement_front_compare/report.out
```

   - Locked multithread smoke entry:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --replacement-front-native-slot-mode \
  --replacement-front-lock-mode \
  --threads 4 \
  --out-dir /tmp/hakorune_replacement_front_locked_mt_compare \
  --out /tmp/hakorune_replacement_front_locked_mt_compare/report.out
```

   - Interpret the locked front as the first thread-safety shape only. It is
     allowed to lose throughput to C mimalloc because the point is to prove that
     the benchmark-only replacement front no longer uses an unsynchronized
     global free stack.

   - Thread-local arena probe entry:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --replacement-front-native-slot-mode \
  --replacement-front-thread-local-mode \
  --threads 4 \
  --out-dir /tmp/hakorune_replacement_front_tls_mt_compare \
  --out /tmp/hakorune_replacement_front_tls_mt_compare/report.out
```

   - Interpret the thread-local arena probe as the first scalable shape. It is
     still benchmark-only. Cross-thread free routes through a remote queue;
     cross-thread realloc remains unsupported and must stay visible in counters.
     Remote-free publication is serialized with the arena registry so owner
     thread exit cannot race with a foreign free touching owner arena storage.
     Use `--replacement-front-skip-hot-counters` only as a measurement-only
     counter-tax probe; it is incompatible with the focused counter smokes and
     is not allocator readiness evidence.
     Use `--replacement-front-tls-counter-mode` as the benchmark-only keeper
     direction for preserving counters while avoiding hot atomic increments in
     the thread-local replacement front.
     GNU/Linux thread-local replacement fronts use the initial-exec TLS model
     for the benchmark-only shim. This removes hot `__tls_get_addr` calls from
     the generated replacement front and remains benchmark-only evidence.
     Same-thread free uses a local-only fast path; remote owner publication is
     only consulted after the local slot check fails.

```text
thread_local_replacement_front_smoke=1
thread_local_arena=1
cross_thread_free_policy=remote_queue
replacement_front_arena_registry_overflow_count_total=0
activation=0
benchmark_only=1
winner_claim=0
summary=ok
```

   - The Hakozuna mixed-ws workload may not exercise cross-thread free. Use a
     focused cross-thread free smoke when changing this path:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --replacement-front-native-slot-mode \
  --replacement-front-thread-local-mode \
  --replacement-front-cross-thread-smoke \
  --threads 4 \
  --out-dir /tmp/hakorune_replacement_front_tls_cross_thread_compare \
  --out /tmp/hakorune_replacement_front_tls_cross_thread_compare/report.out
```

   - The focused smoke must require:

```text
replacement_front_cross_thread_free_smoke_ok=1
replacement_front_cross_thread_free_remote_free_push_count>=1
replacement_front_cross_thread_free_remote_free_drain_count>=1
replacement_front_cross_thread_free_arena_registry_overflow_count=0
replacement_front_cross_thread_realloc_smoke_ok=1
replacement_front_cross_thread_realloc_unsupported_count>=1
replacement_front_cross_thread_realloc_host_passthrough_count=0
```

   - If the owner thread has already exited, the benchmark-only front must not
     pass a recognized Hakorune pointer to host `free`. Mark it abandoned,
     count it, and keep product activation closed:

```text
replacement_front_abandoned_owner_smoke_ok=1
replacement_front_abandoned_owner_abandoned_arena_count>=1
replacement_front_abandoned_owner_abandoned_remote_free_count>=1
replacement_front_abandoned_owner_host_passthrough_count=0
activation=0
benchmark_only=1
winner_claim=0
```

6. Finish the Hakorune mimalloc lane before any victory claim.

## Replacement Front Hot-Path Plan

Historical REPL-001..REPL-018 evidence was archived to:

```text
docs/development/current/main/investigations/mimalloc-current-history-2026-06-02.md
```

Current replacement-front truth:

```text
fixed_slot_native_front=available
matched_hako_good_size_slot=available
multi_bin_native_benchmark_front=available_single_thread_v0
page_bin_benchmark_front=available_single_thread_v0
locked_global_multithread_front=positive_local_evidence_v0
thread_local_multithread_front=correctness_smoke_available_not_perf_keeper
product_pages=not_connected
provider_activation=0
production_replacement_active=0
winner_claim=0
```

Next replacement-front order:

1. Keep `--replacement-front-native-bins-mode` benchmark-only and single-thread
   until a thread/page plan is selected.
2. Treat the locked global counterless front as the current local multithread
   performance evidence owner; keep thread-local as correctness/smoke evidence
   until perf/asm selects a concrete thread-local hot cost.
3. Open product pages only after bins evidence says pages are the owner.
4. Reopen `.hako` core or generated-C local optimization only with fresh
   structural owner evidence.
5. Keep detailed evidence in the investigation archive, not this active card.

Product pages v0 boundary:

```text
replacement_front_page_bins_plan_v0=1
replacement_front_page_bins_supported=1
replacement_front_page_bins_consumer_enabled=0 by default; 1 only when
  --replacement-front-page-bins-mode is selected
replacement_front_page_bins_route=not_consumed | benchmark_page_bins
replacement_front_page_bins_owner=benchmark_only
replacement_front_page_bins_threading=single_thread_until_plan_selected
replacement_front_page_bins_product_claim=0
```

The first implementation is a benchmark-only page/bin-backed route that keeps
provider activation, product replacement, hooks, global allocator, and winner
claims closed. It consumes the workload regular bins and adds a page-shaped
owner structure, but it must not claim full `.hako` mimalloc until the coverage
report stops saying `replacement_front_is_full_hako_algorithm=0`.

## Daily Commands

Build the Hakozuna mixed-ws fixture:

```bash
make -C benchmarks/external/hakozuna/mixed-ws
```

Run same-machine C mimalloc comparison:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --out target/hakozuna-mixed-ws-compare/report.out \
  --out-dir target/hakozuna-mixed-ws-compare/artifacts \
  --sample-count 5
```

Add the optional Hakorune provider subject:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --manifest target/.../provider/pkg/hakorune_provider.json \
  --out target/hakozuna-mixed-ws-compare-provider/report.out \
  --out-dir target/hakozuna-mixed-ws-compare-provider/artifacts \
  --sample-count 5
```

Run the benchmark-only replacement-front smoke/evidence subject:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --replacement-front-native-slot-mode \
  --replacement-front-thread-local-mode \
  --replacement-front-tls-counter-mode \
  --replacement-front-cross-thread-smoke \
  --replacement-front-slot-size 1024 \
  --out target/hakozuna-mixed-ws-replacement-smoke/report.out \
  --out-dir target/hakozuna-mixed-ws-replacement-smoke/artifacts \
  --sample-count 5
```

Run the replacement-front performance distribution subject after smoke:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --replacement-front-native-slot-mode \
  --replacement-front-lock-mode \
  --replacement-front-skip-hot-counters \
  --replacement-front-slot-size 1024 \
  --threads 2 \
  --out target/hakozuna-mixed-ws-replacement-perf/report.out \
  --out-dir target/hakozuna-mixed-ws-replacement-perf/artifacts \
  --sample-count 7
```

Run the single-thread benchmark-only multi-bin front:

```bash
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --replacement-front-native-bins-mode \
  --threads 1 \
  --out target/hakozuna-mixed-ws-native-bins/report.out \
  --out-dir target/hakozuna-mixed-ws-native-bins/artifacts \
  --sample-count 5
```

Run pointer guard after docs pointer edits:

```bash
bash tools/checks/current_state_pointer_guard.sh
```

## Parking Lot

- DirectMemory / Span / Bytes / LayoutSpan remain future substrate work.
- `direct {}` remains parked; use RequiredFastPathRegion diagnostics first.
- `DirectArray<T>` generic source form remains parked; v0 source-visible type is
  concrete `DirectArrayI64`.
- `RecordStateResidencePlanV0` stays a narrow box-private primitive residence
  plan, not record-as-box or ordinary-box auto-recordification.
- Mixed-base helper extraction stays parked unless `EffectSummary` /
  `ReceiverSnapshotPublicationPlanV0` evidence selects it again.
- External Ubuntu benchmark numbers remain non-horizontal until CPU and run
  conditions are aligned.
