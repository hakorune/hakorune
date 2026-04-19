---
Status: Locked Contract / Done
Date: 2026-04-19
Card: 289x-1g
Scope: current profile/helper/caller names を explicit demand vocabulary へ写像する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-289x/README.md
  - docs/development/current/main/phases/phase-289x/289x-91-runtime-value-object-task-board.md
  - docs/development/current/main/phases/phase-289x/289x-92-value-boundary-inventory-ledger.md
---

# Phase 289x Demand Vocabulary Ledger

## Decision

Current code may keep its existing names, but future runtime-wide work must not
derive value/object legality from those names.

The next code API must express demand first:

| Demand family | Meaning |
| --- | --- |
| `ValueDemand` | how a caller wants to read or encode a value |
| `StorageDemand` | how a container wants to keep residence |
| `PublishDemand` | why a stable object/handle is required |
| `MutationDemand` | which aliases/caches/read sessions must be invalidated |

This card is docs-only.
It does not rename code and does not authorize storage rewrite.

## Demand Vocabulary

| Demand | Contract |
| --- | --- |
| `ValueDemand::ReadRef` | read without owning or publishing |
| `ValueDemand::EncodeImmediate` | return an unboxed scalar-compatible payload |
| `ValueDemand::EncodeAlias` | return a borrowed/cached alias handle if still valid |
| `ValueDemand::OwnedPayload` | produce unpublished owned bytes/value for a sink |
| `ValueDemand::StableObject` | read as object-capable representation because the caller requires it |
| `StorageDemand::CellResidence` | store value in a container-specific residence without publication |
| `StorageDemand::ImmediateResidence` | store scalar payload inline/unboxed |
| `StorageDemand::GenericResidence` | store object/box representation in generic container storage |
| `StorageDemand::DegradeGeneric` | leave typed residence because value class is heterogeneous or unknown |
| `PublishDemand::ExternalBoundary` | public/host ABI requires handle/object |
| `PublishDemand::GenericFallback` | fallback path only accepts public object/handle |
| `PublishDemand::ExplicitApi` | explicit helper/API asks for publication |
| `PublishDemand::NeedStableObject` | stable object identity is required by a downstream consumer |
| `MutationDemand::InvalidateAliases` | mutation must expire cached alias/read-source state |
| `MutationDemand::DropEpoch` | host handle epoch decides whether old aliases can be reused |

## Runtime Name Mapping

| Current name | Current anchor | Demand reading | Future direction |
| --- | --- | --- | --- |
| `CodecProfile::Generic` | `value_codec/decode.rs` | `ValueDemand::StableObject` or compat fallback | split call sites into explicit stable/object vs immediate demand |
| `CodecProfile::ArrayFastBorrowString` | `value_codec/decode.rs` | `ValueDemand::EncodeImmediate` + `ValueDemand::EncodeAlias` | replace profile-name truth with array read/write demand |
| `CodecProfile::ArrayBorrowStringOnly` | `value_codec/decode.rs` | `ValueDemand::EncodeAlias`, otherwise integer fallback | keep only as compatibility row until caller demand is explicit |
| `CodecProfile::MapKeyBorrowString` | `value_codec/decode.rs`, `map_key_codec.rs` | key decode demand + scalar-prefer alias preservation | move under map key boundary demand |
| `CodecProfile::MapValueBorrowString` | `value_codec/decode.rs`, `map_slot_store.rs` | map value residence may preserve text alias | move under map value storage demand |
| `ArrayFastDecodedValue::ImmediateI64` | `value_codec/decode.rs`, `array_slot_store.rs` | `ValueDemand::EncodeImmediate` / `StorageDemand::ImmediateResidence` | keep as scalar immediate carrier |
| `ArrayFastDecodedValue::ImmediateBool` | `value_codec/decode.rs`, `array_slot_store.rs` | `ValueDemand::EncodeImmediate` / `StorageDemand::ImmediateResidence` | keep as scalar immediate carrier |
| `ArrayFastDecodedValue::ImmediateF64` | `value_codec/decode.rs`, `array_slot_store.rs` | `ValueDemand::EncodeImmediate` / `StorageDemand::ImmediateResidence` | keep as scalar immediate carrier |
| `ArrayFastDecodedValue::Boxed` | `value_codec/decode.rs`, `array_slot_store.rs` | `StorageDemand::GenericResidence` or alias-preserving text fallback | separate generic residence from text alias residence |
| `BorrowedAliasEncodeCaller::Generic` | `value_codec/borrowed_handle.rs` | caller-unclassified `ValueDemand::EncodeAlias` | shrink as call sites get explicit demand |
| `BorrowedAliasEncodeCaller::ArrayGetIndexEncoded` | `value_codec/borrowed_handle.rs`, `array_handle_cache.rs` | array read `ValueDemand::EncodeAlias` | future array get demand row |
| `BorrowedAliasEncodeCaller::MapRuntimeDataGetAnyKey` | `value_codec/borrowed_handle.rs`, `map_runtime_data.rs` | RuntimeData bridge read `ValueDemand::EncodeAlias` | mark bridge-only, not semantic owner |
| `BorrowedAliasEncodePlan::LiveSourceHandle` | `value_codec/borrowed_handle.rs` | alias hit under `MutationDemand::DropEpoch` validity | keep as hot alias outcome |
| `BorrowedAliasEncodePlan::CachedRuntimeHandle` | `value_codec/borrowed_handle.rs` | cached alias hit under current epoch | keep as hot alias outcome |
| `BorrowedAliasEncodePlan::EncodeFallback` | `value_codec/borrowed_handle.rs` | `PublishDemand::NeedStableObject` via fallback | make publication reason explicit at boundary |
| `BorrowedHandleBox` | `value_codec/borrowed_handle.rs` | boundary/cache carrier for borrowed-alias encode and cached stable-handle reuse | do not treat as semantic `TextRef` |
| `PublishReason::ExternalBoundary` | `value_codec/string_materialize.rs` | `PublishDemand::ExternalBoundary` | keep as publish reason |
| `PublishReason::GenericFallback` | `value_codec/string_materialize.rs` | `PublishDemand::GenericFallback` | keep cold and measured |
| `PublishReason::ExplicitApi` | `value_codec/string_materialize.rs` | `PublishDemand::ExplicitApi` | keep as explicit boundary |
| `PublishReason::NeedStableObject` | `value_codec/string_materialize.rs` | `PublishDemand::NeedStableObject` | require upstream demand fact |
| `StringPublishSite::*` | `value_codec/string_materialize.rs` | observability site, not legality | keep as counters only |
| `KernelTextSlotState::OwnedBytes` | `value_codec/string_materialize.rs` | `ValueDemand::OwnedPayload` / `StorageDemand::CellResidence` through transport | keep as text residence seed, not final `TextCell` |
| `KernelTextSlotState::Published` | `value_codec/string_materialize.rs` | boundary already crossed | cold path only; not normal residence target |
| `KernelTextSlotState::DeferredConstSuffix` | `value_codec/string_materialize.rs` | deferred text plan residence | future text plan/cell contract seed |

## Lowering Name Mapping

| Current route/name | Anchor | Demand reading | Future direction |
| --- | --- | --- | --- |
| `runtime_array_get` | `hako_llvmc_ffi_mir_call_route_policy.inc` | array get with currently implicit `ValueDemand` | split into read-ref, encoded alias, or stable object row |
| `runtime_array_string` | `hako_llvmc_ffi_mir_call_route_policy.inc` | array string read corridor | keep as proof seed, but replace with explicit read demand |
| `runtime_array_push` | `hako_llvmc_ffi_mir_call_route_policy.inc` | array mutation + storage demand | add `MutationDemand::InvalidateAliases` row |
| `runtime_map_get` | `hako_llvmc_ffi_mir_call_route_policy.inc` | map key decode + value read demand | split key boundary from value publication |
| `runtime_map_has` | `hako_llvmc_ffi_mir_call_route_policy.inc` | map key probe, read-only | keep object publication out of probe |
| `runtime_string` | `hako_llvmc_ffi_mir_call_route_policy.inc` | string value method route | distinguish read-only op from publish boundary |
| `array_store_string_source_preserve` | `hako_llvmc_ffi_mir_call_route_policy.inc` | `ValueDemand::ReadRef` source preservation | keep as source-lifetime demand until MIR fact exists |
| `array_store_string_identity_demand_stable_object` | `hako_llvmc_ffi_mir_call_route_policy.inc` | `PublishDemand::NeedStableObject` | rename only after demand fact exists |
| `array_store_string_publication_demand_publish_handle` | `hako_llvmc_ffi_mir_call_route_policy.inc` | `PublishDemand::ExternalBoundary` or explicit publish | keep boundary-specific |
| `nyash.array.slot_load_hi` | C shim declarations/lowering | encoded array get, may publish through fallback | replace route decision with demand row |
| `nyash.array.string_len_hi` | array string observer | `ValueDemand::ReadRef` | no publish allowed |
| `nyash.array.string_indexof_hih` | array string observer | `ValueDemand::ReadRef` | no publish allowed |
| `nyash.array.kernel_slot_store_hi` | array text sink | `StorageDemand::CellResidence` | first Array text-residence pilot sink |
| `nyash.array.kernel_slot_concat_his` | array text producer into slot | `ValueDemand::ReadRef` + `ValueDemand::OwnedPayload` | keep as exact proof, not general TextLane |
| `nyash.map.slot_load_hi/hh` | map load lowering | key decode + value read/publication | split in map demand table |
| `nyash.map.slot_store_hih/hhh` | map store lowering | key decode + value storage + invalidation | split in map demand table |
| `nyash.runtime_data.get_hh/set_hhh` | runtime-data facade | bridge demand only | do not treat as semantic owner |

## Acceptance

- every current profile/helper/caller row has a demand-family reading
- future code APIs can depend on demand names rather than helper/class names
- publication reasons remain boundary facts, not storage states
- observer helpers such as `string_len_hi` and `string_indexof_hih` are read-only demand rows
- RuntimeData stays a bridge/facade row, not the source of language semantics
- borrowed-alias plans and `BorrowedHandleBox` remain boundary/cache behavior, not semantic `TextRef`

## No-Go

- do not rename current code as part of this card
- do not add a new enum to runtime before `289x-2d` closes container demand rows
- do not make `StringPublishSite` decide legality
- do not treat `KernelTextSlotState::Published` as normal hot residence
- do not treat `KernelTextSlot` as the final `TextCell` abstraction
- do not infer stable object demand from `ArrayBox` / `MapBox` / `RuntimeDataBox` names
