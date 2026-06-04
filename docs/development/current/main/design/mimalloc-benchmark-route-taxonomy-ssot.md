---
Status: Active
Date: 2026-06-04
Scope: Mimalloc benchmark route taxonomy and report keys.
Related:
  - docs/development/current/main/workstreams/mimalloc-current.md
  - docs/development/current/main/design/type-abi-route-descriptor-plane-ssot.md
  - docs/development/current/main/design/mimalloc-benchmark-dll-roadmap-ssot.md
  - docs/development/current/main/design/provider-abi-shim-boundary-ssot.md
  - tools/allocator/hako_mimalloc_direct_exact_pair.sh
  - tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py
  - tools/allocator/hakozuna_mixed_ws_gap_ladder.py
---

# Mimalloc Benchmark Route Taxonomy

## Decision

Mimalloc evidence must name the measured hot route explicitly. A report must
not be interpreted as `.hako` mimalloc hot-path evidence just because it loaded
a `.hako`-derived provider package.

Long-term owner:

```text
docs/development/current/main/design/type-abi-route-descriptor-plane-ssot.md
```

This taxonomy is the current report vocabulary. Type ABI is the descriptor
plane that should eventually publish the same route identity and capability
metadata for manifest/report/hako_check/Python introspection. Provider ABI and
replacement-front execution must remain separate.

## Routes

```text
hako_direct_exact_body:
  .hako object-lifecycle body timing against the paired C mimalloc body
  no provider ABI
  no LD_PRELOAD replacement claim

provider_host_adapter_ldpreload:
  LD_PRELOAD malloc-family shim calls a provider ABI table
  provider allocator kind is host_backed_adapter
  alloc/free storage comes from the HostAllocator vtable / host malloc
  .hako object-lifecycle entrypoint may be generated and verified as metadata
  but it is not the alloc/free hot path

provider_hako_object_lifecycle_ldpreload:
  LD_PRELOAD malloc-family shim calls a provider ABI table
  provider alloc/free route consumes .hako object-lifecycle storage/lifecycle
  this is the first provider route allowed to make a `.hako hot path` claim

provider_pure_allocator_ldpreload:
  LD_PRELOAD malloc-family shim calls a provider ABI table
  provider owns allocation storage and pointer lifecycle without host malloc

replacement_front_benchmark:
  benchmark-only C replacement front
  may mirror selected .hako shapes or size-class policy
  `replacement_front_is_full_hako_algorithm=0` means it is not the full
  product `.hako` mimalloc algorithm

replacement_front_product_ldpreload:
  future ordinary-application malloc/free replacement route
  LD_PRELOAD or equivalent process allocator replacement enters the
  replacement front directly, without Type ABI lookup or Provider ABI dispatch
  this route may claim product replacement only after a dedicated activation
  row accepts hooks/global allocator/process replacement risk

c_mimalloc_ldpreload:
  same-machine C mimalloc LD_PRELOAD baseline
```

## Required Provider Route Keys

Provider-backed LD_PRELOAD reports must expose these keys:

```text
provider_benchmark_front_class=
  provider_host_adapter
  | provider_pure_object_lifecycle_bridge
  | provider_pure_allocator
  | provider_unknown

provider_ldpreload_measurement_route=
  provider_host_adapter_ldpreload
  | provider_hako_object_lifecycle_ldpreload
  | provider_pure_allocator_ldpreload
  | provider_ldpreload_unknown

provider_ldpreload_provider_allocator_kind=
  host_backed_adapter|pure_allocator|unknown

provider_ldpreload_alloc_free_route=
  host_malloc_free_wrapper|...|unknown

provider_ldpreload_uses_host_malloc=0|1|unknown
provider_ldpreload_uses_hako_object_lifecycle=0|1|unknown
provider_ldpreload_object_lifecycle_entrypoint_usage=
  metadata_verification_only|hot_path|unknown

provider_ldpreload_hako_hot_path_claim=0|1
provider_ldpreload_hako_object_lifecycle_hot_path=0|1
provider_ldpreload_hako_object_lifecycle_metadata_only=0|1
```

Every subject row in a Hakozuna compare report should also expose:

```text
subject_N_benchmark_front_class=
  system_malloc
  | c_mimalloc_ldpreload
  | provider_host_adapter
  | provider_pure_object_lifecycle_bridge
  | provider_pure_allocator
  | replacement_front_c_shim
  | unknown

subject_N_hako_hot_path_claim=0|1
```

Replacement-front reports must expose both the executed benchmark route and the
ordinary-application candidate route:

```text
replacement_front_execution_route=replacement_front_benchmark
replacement_front_ordinary_app_route_candidate=replacement_front_product_ldpreload
replacement_front_product_gate=closed
replacement_front_product_activation_ready=0
replacement_front_benchmark_only=1
replacement_front_product_claim=0
replacement_front_product_activation_contract_v0=1
replacement_front_product_activation_requires_quality_ok=1
replacement_front_product_activation_requires_provider_dispatch_bypass=1
replacement_front_product_activation_requires_type_abi_hot_lookup_zero=1
replacement_front_product_activation_requires_cross_thread_policy=1
replacement_front_product_activation_requires_remote_abandoned_counters=1
replacement_front_product_activation_requires_rollback_optout_plan=1
replacement_front_rollback_optout_plan_v0=1
replacement_front_rollback_optout_env=HAKORUNE_REPLACEMENT_FRONT_DISABLE
replacement_front_rollback_optout_env_value=1
replacement_front_per_process_disable=1
replacement_front_activation_mode=explicit_only
replacement_front_activation_default=off
replacement_front_activation_report_required=1
replacement_front_rollback_report_path_required=1
replacement_front_product_activation_blockers=...
```

The current replacement-front benchmark may prove that the hot boundary can be
thin. It must not be interpreted as product allocator replacement until:

```text
production_replacement_active=1
hook_installed=1 or global_allocator_product_claim=1
replacement_front_benchmark_only=0
replacement_front_product_claim=1
```

The activation contract is intentionally stricter than the benchmark route. A
future activation row must prove, in the same report family, all of:

```text
measurement_quality=ok
replacement_front_bypasses_provider_dispatch=1
type_abi_hot_path_lookup_count=0
subject_N_cross_thread_free_policy=...
replacement_front_remote_free_push_count / drain_count are reported
replacement_front_abandoned_arena_count is reported
rollback / opt-out plan is documented
```

Until then, the product route remains a descriptor candidate only.

Rollback / opt-out plan v0 is a control-plane contract, not product activation.
It reserves the default-off and per-process-disable controls that a future
activation row must implement before flipping any hook/global allocator claim.
After this plan is present, `no_rollback_optout_plan` must no longer be listed
as an activation blocker; remaining blockers are product-gate and activation-row
decisions.

Interpretation rule:

```text
provider_ldpreload_hako_hot_path_claim=1
```

is allowed only when alloc/free storage or lifecycle is actually routed through
the `.hako` object-lifecycle provider path. A generated or verified `.hako`
entrypoint with `metadata_verification_only` is proof of package/codegen
connectivity, not proof of `.hako` mimalloc hot-path speed.

## Current Thread Evidence

The current provider-host claim-mainline thread reports are route evidence for:

```text
provider_ldpreload_measurement_route=provider_host_adapter_ldpreload
provider_ldpreload_hako_hot_path_claim=0
provider_manifest_hako_provider_alloc_free_route=host_malloc_free_wrapper
provider_manifest_hako_provider_alloc_free_uses_host_malloc=1
provider_manifest_hako_provider_alloc_free_uses_hako_object_lifecycle=0
provider_manifest_hako_provider_object_lifecycle_entrypoint_usage=metadata_verification_only
```

Therefore they must not be summarized as `.hako mimalloc thread hot-path`
benchmarks. They are valid provider ABI / shim / host-backed adapter benchmarks.

## Next Clean Route

To measure `.hako` mimalloc under threaded allocation pressure, open one of:

```text
provider_hako_object_lifecycle_ldpreload
provider_pure_allocator_ldpreload
thread_local_replacement_front_then_promote_to_provider_route
```

Do not continue optimizing `provider_host_adapter_ldpreload` as if it were the
`.hako` object-lifecycle allocator body.
