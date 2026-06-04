---
Status: Active
Date: 2026-06-04
Scope: Provider ABI / LD_PRELOAD shim ownership boundary for allocator-provider measurements.
Related:
  - docs/development/current/main/workstreams/mimalloc-current.md
  - docs/reference/runtime/provider-package-v0.md
  - tools/allocator/hakozuna_mixed_ws_gap_ladder.py
  - tools/allocator/provider_package_ldpreload_replacement_smoke.py
---

# Provider ABI / Shim Boundary SSOT

## Decision

Provider-backed LD_PRELOAD evidence must keep these boundaries separate:

```text
shim:
  C malloc-family symbol adapter

provider:
  owned pointer lifecycle owner

host allocator:
  explicit dependency passed through a HostAllocator vtable in a later row

replacement front:
  benchmark/direct hot front that does not imply provider API activation
```

`alloc/free/owns` remains compatibility surface, but it is not the mainline
replacement-shim ownership model. The mainline model moves ownership decisions
into claiming provider operations.

## Provider Kinds

```text
pure_allocator:
  provider owns allocation storage and pointer lifecycle
  provider does not call host malloc/free

host_backed_adapter:
  provider adapts host allocator storage
  provider must use an explicit HostAllocator vtable once that row opens
  provider must not depend on nonportable libc-private symbols
```

The existing manifest field `provider_kind=allocator` is the package kind. The
allocator-provider lifecycle kind is reported separately as:

```text
provider_allocator_kind=pure_allocator|host_backed_adapter
```

## Claim Operations

The provider is the truth source for provider-owned pointers. The shim should
not maintain a second provider-owned pointer table for the mainline path.

First claim operation:

```text
free_claim(ptr) -> handled | not_owned
```

Future claim operations:

```text
realloc_claim(ptr, new_size) -> handled(ptr) | not_owned | failed
usable_size_claim(ptr) -> owned(size) | not_owned
```

`owns(ptr)` stays diagnostic / compatibility / cold query only.

`usable_size_claim` is enabled only when the provider route owns usable-size
truth. Current route status:

```text
host_backed_adapter:
  provider_usable_size_claim_enabled=0 until HostAllocatorV0 supplies host
  usable-size truth

pure_allocator / native-slot:
  provider_usable_size_claim_enabled=1
```

## Host Allocator Vtable

Future host-backed adapters use:

```text
HostAllocatorV0:
  ctx
  malloc
  calloc
  realloc
  free
  usable_size_optional
```

This prevents provider-internal host allocation from re-entering LD_PRELOAD
symbols and avoids direct dependencies on symbols such as `__libc_malloc`.

## Acceptance Fields

Docs/report-only rows should expose:

```text
provider_abi_claim_ops_v1=1
provider_free_claim_enabled=1
provider_realloc_claim_enabled=0
provider_usable_size_claim_enabled=0|1
compat_alloc_free_owns_still_supported=1
compat_owns_free_mainline=0
shim_provider_owned_truth=0
shim_owns_precheck_hot_path=0
host_allocator_vtable_init=0
provider_direct_libc_symbol_dependency=0
ld_preload_reentry_for_host_alloc=0
product_activation=0
global_allocator_claim=0
hook_installed=0
winner_claim=0
```

Counter rows should expose:

```text
shim_provider_free_claim_count
shim_provider_free_not_owned_count
shim_provider_usable_size_claim_count
shim_provider_usable_size_not_owned_count
shim_tracking_lookup_count
shim_tracking_insert_count
shim_tracking_remove_count
```

`shim_tracking_*` may be nonzero while compatibility tracking remains present,
but the next keeper must drive the mainline free path through
`provider.free_claim` instead of `owns + shim table`.

## Implementation Order

```text
PROV-ABI-001:
  docs/report only
  ProviderKind / claim ops / HostAllocatorV0 boundary

PROV-ABI-002:
  free_claim optional API tail entry
  shim free path prefers free_claim and reports counters
  landed as the first implementation slice

PROV-ABI-003:
  usable_size_claim optional API tail entry
  native-slot route reports provider_usable_size_claim_enabled=1
  host-backed route stays provider_usable_size_claim_enabled=0 until HostAllocatorV0

PROV-ABI-004:
  realloc_claim provider-owned realloc lifecycle

PROV-ABI-005:
  HostAllocatorV0 init for host-backed adapters
```
