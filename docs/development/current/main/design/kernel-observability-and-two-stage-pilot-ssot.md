---
Status: SSOT
Date: 2026-04-18
Scope: kernel perf-observe vocabulary and `.hako` pilot comparison order
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-137x/README.md
  - docs/development/current/main/design/semantic-optimization-authority-ssot.md
---

# Kernel Observability And Two-Stage Pilot SSOT

## Goal

Keep `Rust kernel vs .hako kernel` measurable without mixing two different causes:

1. implementation-language cost
2. protocol / seam cost

Do not compare a Rust kernel and a `.hako` kernel after changing both the implementation language and the publication protocol at the same time.

## Decision

- reject:
  - direct `Rust kernel vs .hako kernel` winner-takes-all comparison
  - full `.hako` migration before protocol-normalized A/B
- adopt:
  - common perf-observe vocabulary first
  - `Stage A: same protocol`
  - `Stage B: same public ABI, different internal seam`

## Common Observability Vocabulary

The kernel-common phase vocabulary is:

- `capture_source`
- `session_enter`
- `materialize_owned`
- `objectize_box`
- `publish_handle`
- `slot_boundary`

The kernel-common count vocabulary is:

- `bytes_copied`
- `handles_issued`
- `registry_read_locks`
- `registry_write_locks`
- `session_count`
- `publish_count`

The kernel-common classification vocabulary is:

- `carrier_kind`
  - `stable_box`
  - `source_keep`
  - `owned_bytes`
  - `handle`
- `publish_reason`
  - `external_boundary`
  - `need_stable_object`
  - `generic_fallback`
  - `explicit_api`

The current `perf-observe` lane may realize these as runtime-private counters and noinline seams. The release lane must keep meaning unchanged.

## Pilot Order

### Stage A: Same Protocol

Write one narrow `.hako` pilot while keeping the current cold tail unchanged:

- `materialize_owned_string`
- `objectize_stable_string_box`
- `issue_fresh_handle`
- `to_handle_arc`

Interpretation:

- if Stage A barely moves, the current owner is not implementation language
- if Stage A moves materially, language/runtime implementation overhead is live

### Stage B: Same Public ABI, Different Internal Seam

Keep the public handle ABI stable, but allow same-corridor unpublished carrier flow until the first true external boundary.

Interpretation:

- if Stage B wins on both Rust and `.hako`, the main owner is protocol / seam
- if Stage B only wins on `.hako`, `.hako` may be the cleaner semantic owner for this protocol

## First Pilot Placement

The first pilot stays narrow and uses `store.array.str`.

Reason:

- canonical contract is already stable as `store.array.str`
- backend-private split is already visible:
  - `SourceKindCheck`
  - `SourceLifetimeKeep`
  - `AliasUpdate`
  - `NeedStableObject`
- `NeedStableObject` is already the explicit stop-line for object-world demand

Do not start with:

- `host_handles`
- `StringBox` / `Arc` objectize
- fresh handle issue
- generic registry/session

Those would turn Stage A into a protocol redesign instead of an implementation-language comparison.

## Guardrails

- keep public ABI stable
- keep legality owned by MIR/lowering, not runtime re-recognition
- do not widen this lane into generic slot API design
- use exact-front comparison as the primary gate
- use whole-kilo only as a regression guard

## Current Exact Front

- primary front:
  - `kilo_micro_array_string_store`
- guard:
  - `kilo_kernel_small_hk`

## Current Reading

Current exact samples point first at protocol-heavy responsibilities:

- `freeze_owned_bytes`
- `issue_fresh_handle`
- `capture_store_array_str_source`
- `StringBox::perf_observe_from_owned`
- `execute_store_array_str_slot_boundary`

This is sufficient to justify the two-stage pilot order. It is not sufficient to justify a full `.hako` rewrite.
