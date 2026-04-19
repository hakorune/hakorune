---
Status: Active Gate
Date: 2026-04-19
Card: 289x-3b
Scope: demand-backed cutover の残 cluster と最適化レーン復帰条件を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-289x/README.md
  - docs/development/current/main/phases/phase-289x/289x-91-runtime-value-object-task-board.md
  - docs/development/current/main/phases/phase-289x/289x-95-array-text-residence-pilot.md
---

# Phase 289x Demand-Backed Cutover Inventory

## Decision

Do not return to optimization work until the remaining value-boundary cutover
clusters are closed or explicitly rejected with evidence.

This is the anti-halfway gate:

```text
finish demand-backed value/object boundary cutover
then return to optimization lane
```

No cluster may be silently skipped.
High-risk clusters stay planned, but they are not touched until earlier
metadata-only cuts have landed.

## Completion Gate

Optimization may resume only when this table is all `done` or `rejected`.

| Gate | Required state |
| --- | --- |
| Rust runtime clusters | 8 clusters closed or rejected |
| C shim / MIR clusters | 8 clusters closed or rejected |
| high-risk deferrals | scheduled phase/card exists for each deferral |
| behavior checks | exact same-slot suffix path stays closed; live-after-get fallback remains valid |
| docs pointers | `CURRENT_TASK.md` and `10-Now.md` point back to optimization only after this gate closes |

## Rust Runtime Clusters

Estimate: 8 clusters, about 24-30 production decision/call sites when wired by cluster.

| Order | Cluster | Files / functions | Sites | Risk | Phase/card | State |
| --- | --- | --- | ---: | --- | --- | --- |
| R1 | `CodecProfile -> DemandSet` bridge | `value_codec/decode.rs`, `any_arg_to_box_with_profile`, `decode_array_fast_value` | 5 variants, 2 helpers | medium | `289x-3c` | done |
| R2 | `BorrowedAliasEncodeCaller -> DemandSet` | `value_codec/borrowed_handle.rs`, `array_handle_cache.rs`, `map_runtime_data.rs` | 3 variants, 2 non-generic users | medium | `289x-3d` | done |
| R3 | `PublishReason -> PublishDemand` bridge | `value_codec/string_materialize.rs` | about 10 sites | medium | `289x-3e` | next |
| R4 | Array generic load/encode | `array_handle_cache.rs`, `array_slot_load.rs` | 2 sites | medium | `289x-3f` | pending |
| R5 | Array store/append decode | `array_slot_store.rs`, `array_slot_append.rs` | 2 sites | medium-high | `289x-3g` | pending |
| R6 | Map key/value codec | `map_key_codec.rs`, `map_slot_store.rs` | 2 sites | medium | `289x-6d` | pending |
| R7 | Map load encoding split | `map_slot_load.rs` | 2 sites | medium-high | `289x-6e` | pending |
| R8 | `KernelTextSlotState` + array text slot | `value_codec/string_materialize.rs`, `array_string_slot.rs` | about 20 state refs, 3 demand touchpoints | high | `289x-3h` | pending-high-risk |

Already landed before this inventory:

| Landed | Files | Reading |
| --- | --- | --- |
| runtime-private demand vocabulary | `crates/nyash_kernel/src/plugin/value_demand.rs` | behavior unchanged |
| Array text read/owned-cell demand tags | `crates/nyash_kernel/src/plugin/array_string_slot.rs` | behavior unchanged |

## C Shim / MIR Clusters

Estimate: 8 clusters. The main blocker is C shim helper-name routing; MIR already has some inspection-only scaffolding.

| Order | Cluster | Files / functions | Sites | Risk | Phase/card | State |
| --- | --- | --- | ---: | --- | --- | --- |
| C1 | Generic method emit kind + set route metadata | `hako_llvmc_ffi_generic_method_policy.inc`, `hako_llvmc_ffi_generic_method_match.inc`, `hako_llvmc_ffi_generic_method_lowering.inc` | about 11 checks, 1 switch | high, metadata-only safe | `289x-7a` | pending |
| C2 | MIR demand/placement parallel facts | `thin_entry.rs`, `thin_entry_selection.rs`, `placement_effect.rs`, `semantic_refresh.rs` | about 4 surfaces | low-medium | `289x-7b` | pending |
| C3 | `get/len/has/push` policy split | `hako_llvmc_ffi_generic_method_get_policy.inc`, `*_len_policy.inc`, `*_has_policy.inc`, `*_push_policy.inc` | about 10 checks, 4 switches | medium-high | `289x-7c` | pending |
| C4 | Main `bname/mname` route classifier | `hako_llvmc_ffi_mir_call_route_policy.inc` | about 27 name checks, 2 consumers | high | `289x-7d` | pending-high-risk |
| C5 | concrete `slot_load/store` helper emission | `hako_llvmc_ffi_generic_method_lowering.inc`, `*_get_policy.inc`, `*_get_lowering.inc`, `hako_llvmc_ffi_indexof_observer_lowering.inc` | about 20 emits/declarations | high | `289x-7e` | pending-high-risk |
| C6 | `runtime_array_string` observer/window routes | `hako_llvmc_ffi_generic_method_get_window.inc`, `hako_llvmc_ffi_indexof_observer_*` | about 5 matcher families | high | `289x-7f` | pending-high-risk |
| C7 | MIR string helper-name compat/recovery | `string_corridor_compat.rs`, `string_corridor_recognizer.rs`, `string_corridor_placement/plan_infer.rs`, `passes/string_corridor_sink/mod.rs` | about 15 recognizers/constants | medium | `289x-7g` | pending |
| C8 | Prepass/declaration need classifier | `hako_llvmc_ffi_mir_call_need_policy.inc`, `hako_llvmc_ffi_mir_call_prepass.inc`, `hako_llvmc_ffi_pure_compile.inc` | about 42 checks plus declarations | high | `289x-7h` | pending-high-risk |

## Phase Order

| Phase | Work | Rule |
| --- | --- | --- |
| `289x-3c` | Rust `CodecProfile -> DemandSet` mapping | done; behavior unchanged |
| `289x-3d` | Rust borrowed-alias caller mapping | done; behavior unchanged |
| `289x-3e` | Rust publish reason mapping | next; behavior unchanged |
| `289x-3f` | Rust array generic load/encode demand tags | behavior unchanged |
| `289x-3g` | Rust array store/append demand tags | behavior unchanged |
| `289x-3h` | `KernelTextSlotState` demand bridge | high-risk; no ABI change |
| `289x-7a` | C shim set-route demand metadata | metadata-only; emitted lowering identical |
| `289x-7b` | MIR parallel demand/placement facts | inspection-only |
| `289x-6d` | Map key/value codec demand bridge | no typed map lane |
| `289x-6e` | Map load encoding split | no public ABI change |
| `289x-7c` to `289x-7h` | high-risk C shim/MIR route replacement | one cluster per commit; exact gates required |

## Do Not Touch Yet

These are planned, not skipped.

| Area | Reason not first | Scheduled card |
| --- | --- | --- |
| concrete `slot_load_hi` / `slot_store` emission changes | behavior-producing; easy to break exact/live-after-get split | `289x-7e` |
| `runtime_array_string` observer/window matcher rewrites | tightly coupled to indexOf branch/select windows | `289x-7f` |
| full `ArrayStorage::Text` / full `TextLane` rewrite | too broad before demand-backed pilot closure | after `289x-7h`, separate phase |
| Map typed lane | key/value publication split not fully code-backed yet | after `289x-6e`, separate phase |
| allocator / arena | must wait until objectization frequency is reduced | after value-boundary cutover, perf evidence only |

## Acceptance

- no helper/class name remains as the only source of publication legality in the active cutover path
- risky route/emission rewrites are isolated behind explicit cards
- exact same-slot suffix store remains closed
- live-after-get reuse keeps its current fallback behavior
- optimization lane is not resumed until this inventory is complete
