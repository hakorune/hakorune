---
Status: Active
Date: 2026-06-09
Scope: split typed-object exact slot ABI from compat field access before the next C-speed user-box optimization.
Blocker: HAKO-MIMALLOC-TYPED-OBJECT-EXACT-SLOT-ABI-SPLIT-296X-001
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/typed-object-exact-slot-abi-ssot.md
  - docs/development/current/main/design/representation-direct-lowering-ssot.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
  - crates/nyash_kernel/src/exports/typed_object.rs
  - crates/nyash_kernel/src/exports/typed_object_pinned_arena.rs
  - tools/perf/bench_micro_aot_asm.sh
  - tools/perf/bench_micro_c_vs_aot_stat.sh
---

# 296x-651 Hako Mimalloc Typed Object Exact Slot ABI Split

## Purpose

The current user-box counter-heavy optimization moved the hot owner from broad
compat extraction to the exact-lane field ABI boundary. The next work must not
continue shaving `field_get_hii` as a unified helper. It must split exact slot
routes from the public/compat field route.

## Decision

```text
exact_lane_abi_separate_from_compat=1
field_get_hii_exact_ssot=0
field_get_hii_compat_legacy_adapter=1
i64_field_benchmark_primary_route=hako.typed_object.slot_load_i64
i64_field_helper_bridge=hako.object.exact_slot_get_i64_hii
handle_field_helper_bridge=hako.object.exact_slot_get_handle_hii
helper_internal_dispatch_keeper=0
native_direct_final_target=1
```

## Required Output

```text
output_contract=hako-mimalloc-typed-object-exact-slot-abi-split-v0
typed_object_exact_slot_abi_split=1
typed_object_field_get_hii_compat_only=1
typed_object_get_compat_i64_count=0
typed_object_exact_internal_dispatch_count=0
typed_object_exact_silent_fallback_count=0
typed_object_exact_name_lookup_count=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Task Ladder

```text
TYPEDOBJ-ABI-000:
  Land the typed-object exact slot ABI SSOT.

TYPEDOBJ-ABI-001:
  Add report/check vocabulary for typed_object.slot_load/store_* routes,
  compat field-get counts, exact helper counts, internal dispatch counts, and
  silent fallback counts.

TYPEDOBJ-ABI-002:
  Route the i64 user-box benchmark through hako.typed_object.slot_load_i64
  and hako.object.exact_slot_get_i64_hii when proof is selected.

TYPEDOBJ-ABI-003:
  Keep field_get_hii on compat/legacy only. It may exist, but it must not be
  accepted as exact-lane keeper evidence.

TYPEDOBJ-ABI-004:
  Select the first NativeDirect typed-object slot load/store pilot after route
  evidence proves the helper boundary is the remaining owner.
```

## First Implementation Slice

```text
target=TYPEDOBJ-ABI-001
behavior_change=report/check vocabulary only
must_not_change=runtime helper semantics
must_not_add=benchmark-name special cases
```

## First Commands

```bash
bash tools/checks/current_state_pointer_guard.sh
cargo fmt --check
git diff --check
```

## Stop Line

- do not make `field_get_hii` the exact slot SSOT
- do not route i64 field benchmarks through `hako.object.exact_slot_get_handle_hii`
- do not hide selected exact routes inside helper-internal dispatch
- do not silently fall back after an exact route is selected
- do not reopen provider activation, hooks, global allocator claims, or winner
  claims

## Next

```text
HAKO-MIMALLOC-TYPED-OBJECT-EXACT-SLOT-ABI-SPLIT-296X-001:
  add report/check vocabulary for exact slot versus compat field routes

After green:
  route the i64 user-box benchmark through selected
  hako.typed_object.slot_load_i64 and hako.object.exact_slot_get_i64_hii
  evidence, then measure whether helper call or NativeDirect inline lowering
  is the next owner.
```
