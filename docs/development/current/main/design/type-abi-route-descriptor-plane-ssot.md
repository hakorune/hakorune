---
Status: Active
Date: 2026-06-04
Scope: Type ABI route descriptor / control-plane boundary for allocator and mimalloc route evidence.
Related:
  - docs/development/current/main/design/mimalloc-benchmark-route-taxonomy-ssot.md
  - docs/development/current/main/design/provider-abi-v1-ssot.md
  - docs/development/current/main/design/provider-abi-shim-boundary-ssot.md
  - docs/development/current/main/workstreams/mimalloc-current.md
---

# Type ABI Route Descriptor Plane

## Decision

Type ABI is the descriptor/control plane for route identity, type/layout
metadata, capability metadata, and report interpretation. It is not the
allocator execution plane.

Use this split:

```text
Descriptor Plane:
  Type ABI

Execution Plane:
  Provider ABI

Hot Replacement Plane:
  Replacement front

Host Escape Plane:
  HostAllocator vtable
```

Do not describe this as "unifying execution on Type ABI". The intended design
is:

```text
Type ABI:
  what is this?
  what route/capability/layout can it claim?
  what should reports and tools call it?

Provider ABI:
  do alloc/free/realloc/usable_size/free_claim now

Replacement front:
  run the shortest malloc/free hot path
```

## Allowed Type ABI Descriptor Responsibilities

Type ABI may own:

```text
route identity
benchmark taxonomy
contract metadata
type/layout metadata
ownership capability metadata
provider kind metadata
claim operation availability metadata
Python/introspection surface
manifest / report / hako_check interpretation
```

Type ABI may be read by:

```text
init / registration
manifest verification
benchmark report generation
hako_check
Python read-only introspection
provider capability negotiation
```

## Forbidden Hot-Path Responsibilities

Type ABI must not own or dispatch:

```text
alloc/free/realloc/usable_size execution
free_claim / realloc_claim execution
provider table hot dispatch
replacement front hot path
host malloc/free calls
LD_PRELOAD reentry handling
per-pointer ownership tracking
remote free queue operations
arena/page mutable state
```

Required invariant:

```text
type_abi_hot_path_lookup_count=0
```

for provider-backed LD_PRELOAD and replacement-front hot-path reports unless a
future explicit diagnostic row intentionally measures Type ABI lookup overhead.

## Descriptor Vs Execution Evidence

Keep package declaration separate from measured execution:

```text
descriptor / declared:
  .hako-derived provider package exists
  object-lifecycle entrypoint exists
  provider kind and capability bits are declared

execution evidence:
  benchmark subject ran through provider-host adapter
  or provider object-lifecycle bridge
  or replacement-front C shim
  hako_hot_path_claim is proven or closed
```

Example:

```text
declared_package_origin=hako_derived
declared_route=provider_hako_object_lifecycle_ldpreload

execution_route=provider_host_adapter_ldpreload
object_lifecycle_entrypoint_usage=metadata_verification_only
hako_hot_path_claim=0
```

This is valid and must not be summarized as `.hako` mimalloc hot-path timing.

## Descriptor Names

Preferred names:

```text
HakoTypeAbiDescriptorV0:
  type/layout/capability descriptor

HakoRouteDescriptorV0:
  route_id / route_kind / front_class / declared capability descriptor

HakoProviderDescriptorV0:
  provider kind / package origin / claim-op availability descriptor

HakoBenchmarkSubjectDescriptorV0:
  report-side subject identity and claim boundary descriptor
```

Provider execution remains:

```text
HakoProviderOpsV1
HakoProviderClaimResult
```

Host escape remains:

```text
HakoHostAllocatorV0
```

Replacement-front planning remains:

```text
HakoReplacementFrontPlanV0
```

## Required Report Fields

Route descriptor reports should expose:

```text
type_abi_route_descriptor_present=1
type_abi_descriptor_plane=route_descriptor_control_plane
type_abi_hot_path_lookup_count=0

declared_route=...
execution_route=...
benchmark_front_class=...
hako_hot_path_claim=0|1
object_lifecycle_entrypoint_usage=metadata_verification_only|hot_path|none

provider_ops_version=1
provider_kind=pure_allocator|host_backed_adapter|object_lifecycle_bridge|unknown
provider_claim_ops_enabled=0|1

host_allocator_vtable_init=0|1
provider_direct_libc_symbol_dependency=0
ldpreload_reentry_for_host_alloc=0

replacement_front_bypasses_type_abi=0|1
replacement_front_bypasses_provider_dispatch=0|1
```

Provider-host reports must keep:

```text
provider_kind=host_backed_adapter
hako_hot_path_claim=0
host_allocator_vtable_init=1
```

Replacement-front reports must keep:

```text
benchmark_front_class=replacement_front_c_shim
type_abi_hot_path_lookup_count=0
provider_dispatch_hot_path=0
```

## Task Order

```text
TYPEROUTE-001:
  docs/report-only descriptor plane
  define HakoRouteDescriptorV0 / descriptor-vs-execution vocabulary
  behavior change: none
  status: landed 2026-06-04

TYPEROUTE-002:
  add type_abi_route_descriptor_present and
  type_abi_hot_path_lookup_count to hakozuna compare/gap reports
  behavior change: none
  status: landed 2026-06-05

TYPEROUTE-003:
  add declared_route vs execution_route fields
  fail if host_backed_adapter claims .hako hot path
  status: landed 2026-06-05

TYPEROUTE-004:
  hako_check / Python introspection consumes route descriptor as read-only
  descriptor data
  provider/replacement execution change: none
  status: landed 2026-06-05

TYPEROUTE-005:
  ProviderRegistrationV1 report pairs descriptor + ops, but hot path uses ops
  only
```

## Declared Vs Execution Route Fields

Hakozuna compare and gap ladder reports expose:

```text
provider_ldpreload_declared_package_origin=hako_derived|unknown
provider_ldpreload_declared_route=...
provider_ldpreload_execution_route=...

subject_N_declared_route=...
subject_N_execution_route=...

provider_declared_route=...
provider_execution_route=...
```

`declared_route` comes from package/descriptor metadata. `execution_route`
comes from the measured hot route. A host-backed adapter must not claim the
`.hako` hot path.

## Read-Only Descriptor Consumption

`tools/allocator/type_abi_route_descriptor_readonly.py` is the first Python
introspection adapter for route descriptors. It reads an existing key-value
report, validates the Type ABI descriptor/control-plane fields, and re-emits
route identity evidence.

Required output:

```text
output_contract=type-abi-route-descriptor-readonly-v0
readonly_descriptor_consumption=1
python_introspection_adapter=1
hako_check_core_change=0
provider_abi_execution_change=0
replacement_front_hot_path_change=0
type_abi_hot_path_lookup_count=0
```

The adapter must not run a benchmark, call Provider ABI operations, activate a
provider, install hooks, replace the process allocator, or read Type ABI from
allocator hot paths.

## Current Next Action

The next active work is `TYPEROUTE-005`: add ProviderRegistrationV1-style
report pairing for descriptor + ops while keeping allocator hot paths on ops
only. Do not route malloc/free/realloc/usable_size execution through Type ABI.
