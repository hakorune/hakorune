---
Status: Locked Contract / Done
Date: 2026-04-19
Card: 289x-2d
Scope: Array/Map の read/write demand table と lane-host boundary を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-289x/README.md
  - docs/development/current/main/phases/phase-289x/289x-91-runtime-value-object-task-board.md
  - docs/development/current/main/phases/phase-289x/289x-92-value-boundary-inventory-ledger.md
  - docs/development/current/main/phases/phase-289x/289x-93-demand-vocabulary-ledger.md
---

# Phase 289x Container Demand Table

## Decision

Array and Map remain identity containers.

Lane-host work may specialize only internal element/key/value residence.
It must not redefine Array/Map as immutable values, and it must not widen public ABI.

## Container Boundary Rule

| Container | Public semantic truth | Internal specialization allowed |
| --- | --- | --- |
| `Array` | identity container with handle/object surface | element residence may be immediate, text cell, or generic box |
| `Map` | identity container with handle/object surface | key decode and value residence may specialize separately |
| `RuntimeData` | facade/bridge only | may route to Array/Map demand rows, but owns no semantics |

## Array Demand Table

| Current route | Current anchors | Demand rows | Boundary rule |
| --- | --- | --- | --- |
| index get encoded | `array_slot_load.rs`, `array_handle_cache.rs` | `ValueDemand::EncodeImmediate`, `ValueDemand::EncodeAlias`, fallback `ValueDemand::StableObject` | encoded get must say which read shape it needs; stable fallback is boundary, not normal read |
| index has/len/cap | `array_slot_load.rs`, `array_runtime_substrate.rs` | read-only observer | no publish or alias materialization |
| string len by index | `array_string_slot.rs::array_string_len_by_index` | `ValueDemand::ReadRef` | must not publish |
| string indexOf by index | `array_string_slot.rs::array_string_indexof_by_index` | `ValueDemand::ReadRef` | must not publish; needle cache is observer-local |
| const suffix from slot | `array_string_slot.rs::array_string_concat_const_suffix_by_index_into_slot` | `ValueDemand::ReadRef` + `ValueDemand::OwnedPayload` | reads source as text and writes unpublished output into `KernelTextSlot` |
| set any | `array_slot_store.rs::array_slot_store_any` | scalar `StorageDemand::ImmediateResidence`, text/generic `StorageDemand::GenericResidence` today | future table must split text cell residence from generic box fallback |
| set i64/bool/f64 | `array_slot_store.rs` raw scalar stores | `StorageDemand::ImmediateResidence` + `MutationDemand::InvalidateAliases` | scalar residence can stay unboxed; mutation invalidates dependent read state |
| set string handle | `array_slot_store.rs::array_slot_store_string_handle`, `array_string_slot.rs::array_string_store_handle_at` | `StorageDemand::CellResidence` or `PublishDemand::NeedStableObject` when required | source preservation and stable-object need must be named demand |
| set kernel text slot | `array_slot_store.rs::array_slot_store_kernel_text_slot`, `array_string_slot.rs::array_string_store_kernel_text_slot_at` | `StorageDemand::CellResidence` + `ValueDemand::OwnedPayload` through transport | first eligible Array text-residence pilot sink seed; not final `TextCell` |
| append any | `array_slot_append.rs::array_slot_append_any` | scalar immediate residence or generic/text fallback | append is mutation; it must not hide publication demand inside `CodecProfile` |
| RuntimeData array get/set/has | `array_runtime_any.rs`, `runtime_data.rs` | bridge to Array rows after key/index decode | facade only; not semantic owner |

## Map Demand Table

| Current route | Current anchors | Demand rows | Boundary rule |
| --- | --- | --- | --- |
| i64 key decode | `map_key_codec.rs::map_key_string_from_i64` | key decode, read-only | no publication |
| any key decode | `map_key_codec.rs::map_key_string_from_any` | key decode + `ValueDemand::StableObject` today | future map key demand must isolate key coercion from value publication |
| RuntimeData key decode | `map_key_codec.rs::map_runtime_data_key_string_from_any` | bridge key decode | facade-specific fast path only |
| load i64/any key | `map_slot_load.rs::map_slot_load_i64`, `map_slot_load_any` | key decode + value read | must split key decode from value read/publish |
| load string key materializing | `map_slot_load.rs::map_slot_load_str` | `PublishDemand::NeedStableObject` for current materializing return | materializing load is boundary path |
| load string key with caller | `map_slot_load.rs::map_slot_load_str_with_caller` | `ValueDemand::EncodeAlias` with caller scope | caller must be demand row, not semantic owner |
| store i64/any key | `map_slot_store.rs::map_slot_store_i64_any`, `map_slot_store_any` | key decode + value storage + mutation | mutation invalidates value/read aliases |
| store string key any value | `map_slot_store.rs::map_slot_store_str_any` | `StorageDemand::GenericResidence` today, possible text alias preservation | future map value lane must not couple to key decode |
| has/probe | `map_probe.rs` | key decode + read-only probe | no value publication |
| RuntimeData map get/set/has | `map_runtime_data.rs`, `runtime_data.rs` | bridge to Map rows | bridge only; not semantic owner |

## Map Boundary Lock for 137x-B

Current landed truth:

- Map key decode demand metadata is landed for i64 / any / RuntimeData bridge keys.
- Map value store demand metadata is landed for value residence plus alias invalidation.
- Map value load demand metadata is landed for materializing return vs caller-scoped encode.
- These are metadata / boundary naming cuts; behavior is unchanged.

Current stop-line:

- `289x-6c` typed map lane is still unopened.
- key decode policy is not value residence truth.
- value residence / read publication must not be inferred from key coercion.
- RuntimeData may dispatch to Map rows, but it owns no Map semantics.

## RuntimeData Facade Table

| Facade route | Delegates to | Demand reading |
| --- | --- | --- |
| `nyash.runtime_data.get_hh` | Array get-any-key or Map get-any-key | bridge read demand after receiver classification |
| `nyash.runtime_data.set_hhh` | Array set-any-key or Map set-any-key | bridge mutation/storage demand |
| `nyash.runtime_data.has_hh` | Array has-any-key or Map has-any-key | bridge read-only probe |
| `nyash.runtime_data.push_hh` | Array push only | bridge mutation/storage demand |

RuntimeData may classify the receiver, but it must delegate demand ownership to
Array or Map rows.

## First Storage Pilot Eligibility

Only this pilot is eligible after this table:

| Pilot | Why eligible | Required reject seam |
| --- | --- | --- |
| Array text residence through `KernelTextSlot` store | exact proof already exists, and it stays runtime-private | reject if read-side stable publication moves elsewhere or public ABI widens |

Reading:

- this pilot proves transport through `KernelTextSlot`
- it does not prove the final `TextCell` abstraction
- full `ArrayStorage::Text` / `TextCell` design remains deferred to `289x-8a`

Not eligible yet:

| Candidate | Reason |
| --- | --- |
| full `ArrayStorage::Text` rewrite | too broad before one pilot proves demand rows |
| Map typed lane | key/value boundary is not yet separated enough |
| scalar-wide lane rewrite | needs boxed scalar transition audit |
| allocator/arena | must wait until objectization frequency is reduced |

## Acceptance

- Array/Map identity semantics stay unchanged
- every Array/Map read/write route has a demand row
- RuntimeData is documented as facade-only
- first storage pilot is narrowed to Array text residence, not full typed containers
- runtime must execute named demand and must not infer legality from helper/class names

## No-Go

- do not make Array/Map semantic values
- do not implement storage rewrite in this card
- do not merge map key decode with map value publication
- do not let RuntimeData become semantic owner
- do not add allocator work before a storage pilot changes the owner
