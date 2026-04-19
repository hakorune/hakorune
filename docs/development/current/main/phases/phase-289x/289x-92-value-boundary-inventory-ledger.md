---
Status: Active Inventory
Date: 2026-04-19
Scope: `value world -> boundary effect -> object world` を runtime-wide に完了させる前の棚卸 ledger。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/lifecycle-typed-value-language-ssot.md
  - docs/development/current/main/design/value-repr-and-abi-manifest-ssot.md
  - docs/development/current/main/phases/phase-137x/README.md
  - docs/development/current/main/phases/phase-289x/README.md
  - docs/development/current/main/phases/phase-289x/289x-91-runtime-value-object-task-board.md
  - docs/development/current/main/phases/phase-289x/289x-93-demand-vocabulary-ledger.md
---

# Phase 289x Value Boundary Inventory Ledger

## Decision

Optimization work is paused until the value-boundary architecture is inventoried and
the next implementation cut is selected as a phase/card, not as a helper-local patch.

The rule remains:

```text
internal execution:
  value / alias / cell / immediate

boundary:
  publish / promote / freeze.str / handle issue

public surface:
  object / handle
```

This ledger is BoxShape only.
It records current mixed-demand seams and does not authorize runtime storage rewrite,
public ABI widening, allocator work, or MIR dialect expansion.

## Completion Bar Before Optimization Resumes

Further optimization resumes only after these are documented:

| Gate | Required outcome |
| --- | --- |
| `289x-1f` post-keeper inventory sync | phase-137x keeper `49c356339` is reflected as the string proof, and older owner snapshots are marked historical |
| `289x-1g` demand ledger | done in `289x-93-demand-vocabulary-ledger.md`; current profile/helper/caller names are mapped to `ValueDemand`, `StorageDemand`, `PublishDemand`, and `MutationDemand` |
| `289x-2d` container demand table | Array/Map read/write paths say whether they need read-ref, encoded alias, cell residence, stable object, degrade, or invalidation |
| `289x-3a` pilot selection | exactly one runtime-private storage pilot is chosen, with reject seams and tests, after Phase 1/2 docs are complete |
| `289x-7a` MIR fact lift plan | MIR/lowering is identified as the owner of boundary demand; runtime stays executor, not legality oracle |

No single gate above changes behavior by itself.

## Rust Runtime / Kernel Inventory

| Slice | Current anchors | Current vocabulary | Gap |
| --- | --- | --- | --- |
| scalar decode and array read/append | `crates/nyash_kernel/src/plugin/value_codec/decode.rs`, `array_slot_append.rs`, `array_handle_cache.rs` | `CodecProfile`, `ArrayFastDecodedValue` | demand is still encoded as profile/helper names; some paths box first then recover immediates |
| borrowed alias encode | `value_codec/borrowed_handle.rs`, `value_codec/encode.rs`, `map_slot_load.rs`, `map_runtime_data.rs` | `BorrowedAliasEncodeCaller`, live-source, cached-handle, cold-fallback | read outcome is caller-scoped; fallback can publish during read encoding |
| string publication | `value_codec/string_materialize.rs` | `PublishReason`, `StringPublishSite`, `KernelTextSlotState` | materialize, publish, objectize, and residence are still close enough to blur responsibility |
| array string residence | `array_string_slot.rs`, `array_slot_store.rs`, `value_codec/string_classify.rs` | `StoreArrayStrPlan`, `StringHandleSourceKind`, `StringLikeProof`, `VerifiedTextSource` | storage demand is planned by source/slot/action names; non-string fallback can still force stable materialization |
| map key/value boundary | `map_key_codec.rs`, `map_slot_load.rs`, `map_slot_store.rs`, `map_runtime_data.rs` | key coercion, boxed value storage, caller-scoped read encode | key decode, value residence, and read publication are split operationally but not yet expressed as demand facts |
| runtime-data facade | `runtime_data.rs`, `map_runtime_data.rs` | mixed `i64` / handle surface | facade route names can leak handle semantics into generic data access |

## Lowering / MIR / C Shim Inventory

| Slice | Current anchors | Current behavior | Gap |
| --- | --- | --- | --- |
| generic route classification | `lang/c-abi/shims/hako_llvmc_ffi_generic_method_policy.inc`, `hako_llvmc_ffi_generic_method_lowering.inc` | `bname` / `mname` select `slot_load`, `slot_store`, `slot_append`, `entry_count`, `kernel_slot_publish` helpers | object demand is inferred from class/method/helper names, not explicit facts |
| MIR call route and need prepass | `hako_llvmc_ffi_mir_call_route_policy.inc`, `hako_llvmc_ffi_mir_call_need_policy.inc` | receiver family and method names set route/need flags | stable object need, publish need, and invalidation are reconstructed from scanner state |
| get/len/push/string chain lowering | `hako_llvmc_ffi_generic_method_get_policy.inc`, `*_len_policy.inc`, `*_push_policy.inc`, `hako_llvmc_ffi_string_chain_policy.inc` | helper family decides read, alias, stable, concat, or fallback behavior | read-vs-alias-vs-stable boundary is implicit |
| MIR storage inventory | `src/mir/storage_class.rs` | `MirType` maps to storage class such as borrowed text, primitive, box ref | shape inventory exists, but demand to keep/ref/publish is not first-class |
| string kernel plan | `src/mir/string_kernel_plan.rs` | plan derives publication boundary, contract, carrier, and legality from proofs | existing plan is a good seed, but runtime-wide demand vocabulary is not yet the selecting truth |

## Docs Inventory

| State | Reading |
| --- | --- |
| locked | `String` is a value; `publish` is a boundary effect; `freeze.str` is the only string birth sink |
| locked | carrier vocabulary is `Ref / Owned / Cell / Immediate / Stable`; `get / set / call` are demand verbs |
| locked | Array/Map are identity containers; lane-hosting applies only to internal residence |
| stale/historical | pre-keeper `739 ms` / `856 ms` whole-owner snapshots must be read as history after keeper `49c356339` |
| stale/historical | older “next card is read-side alias lane split” text is history after the branch-target-aware same-slot suffix keeper |

## Next Card Order

| Card | Type | Output |
| --- | --- | --- |
| `289x-1f` | BoxShape docs-only | sync post-keeper inventory and current pointers |
| `289x-1g` | BoxShape docs-only | done: exact demand ledger for current profile/helper/caller names |
| `289x-2d` | BoxShape docs-only | Array/Map demand table and identity-container lane-host contract |
| `289x-6d` | BoxShape docs-only | map key/value boundary map with compat-retirement criteria |
| `289x-3a` | BoxCount implementation planning only | one runtime-private Array text-residence pilot proposal |
| `289x-7a` | later BoxShape | MIR/lowering demand fact lift plan |

Do not mix `289x-3a` implementation with the docs-only inventory cards.
