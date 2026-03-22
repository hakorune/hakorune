---
Status: SSOT
Decision: provisional
Date: 2026-03-23
Scope: deeper substrate capability workの前提として、runtime value representation と ABI manifest の正本を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-29ct/README.md
  - docs/development/current/main/design/substrate-capability-ladder-ssot.md
  - docs/development/current/main/design/abi-export-inventory.md
  - docs/development/current/main/design/hako-runtime-c-abi-cutover-order-ssot.md
  - crates/nyash_kernel/src/plugin/value_codec/mod.rs
  - crates/nyash_kernel/src/plugin/value_codec/decode.rs
  - crates/nyash_kernel/src/plugin/value_codec/encode.rs
  - lang/src/vm/boxes/abi_adapter_registry.hako
---

# Value Repr And ABI Manifest (SSOT)

## Goal

- current plugin export surface を手書き alias 群のまま増殖させず、manifest-first に寄せる。
- runtime value の classes と ownership を先に固定して、future `hako.abi` / `hako.value_repr` の土台にする。
- old compact call-shape codes を immediate truth にせず、manifest から生成される compatibility artifact に落とす。

## Canonical Runtime Value Classes

current repo の実態に寄せた canonical classes は次。

1. `imm_i64`
   - plain integer immediate
2. `imm_bool`
   - bool semantic value
   - bridge/call surface では `0/1` i64 に lower されても class は区別する
3. `handle_owned`
   - ordinary live handle
   - return ownership を伴う canonical handle
4. `handle_borrowed_string`
   - borrowed string/string-view handle alias
   - fast path のための stable class
5. `boxed_local`
   - codec-local temporary only
   - public ABI manifest には直接出さない

## Current Mapping Rule

current `value_codec` から読むと、次が事実上の substrate rule だよ。

- `imm_i64`
  - `decode_array_fast_value(...)` は non-positive or integer-like route を immediate として扱う
- `imm_bool`
  - `runtime_i64_from_box_ref(...)` は bool を `0/1` へ encode する
- `handle_borrowed_string`
  - string / string-view is allowed to reuse borrowed handle alias route
- `handle_owned`
  - non-scalar / non-borrowed values are returned as owned handles
- `boxed_local`
  - only substrate helpers may use it as a temporary carrier

## ABI Manifest Row

future manifest row の最小 truth は次で固定する。

- `box_type`
- `method`
- `symbol`
- `args`
- `ret`
- `arg_ownership`
- `ret_ownership`
- `failure_contract`
- `compat_status`

example reading:

- `ArrayBox.get`
  - symbol: `nyash.array.slot_load_hi`
  - args: `handle_owned, imm_i64`
  - ret: `imm_i64`
  - arg_ownership: borrowed
  - ret_ownership: none
  - compat_status: active mainline

legacy `h` / `hh` / `hi` / `hii` 表記は compatibility artifact として残してよいが、SSOT の第一表現にはしない。

## Ownership Rule

- args are borrowed by default
- return is owned by default unless explicitly `none`
- borrowed string handle is a first-class manifest class
- ownership mismatch must fail-fast; no silent fallback

## Fixed Order

### V0. Inventory current symbols

- `nyash.array.*`
- `nyash.map.*`
- `nyash.runtime_data.*`
- `nyash.string.*`
- docs-side inventory truth is [`abi-export-inventory.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/abi-export-inventory.md)
- [`AbiAdapterRegistryBox`](/home/tomoaki/git/hakorune-selfhost/lang/src/vm/boxes/abi_adapter_registry.hako) is a runtime consumer of that inventory, not the manifest truth

current hand-written exports are allowed only as inventory input.

### V1. Freeze value classes

- `imm_i64`
- `imm_bool`
- `handle_owned`
- `handle_borrowed_string`
- `boxed_local`

### V2. Freeze manifest schema

- row shape above becomes the only truth
- `AbiAdapterRegistryBox` defaults become generated from the manifest later

### V3. Generate compatibility shims

- `array.rs`
- `map.rs`
- `runtime_data.rs`

these should shrink to thin exports over substrate helpers and generated alias data.

### V4. Shrink manual codec glue

- `value_codec/*` remains substrate owner
- but public ABI semantics must be readable from the manifest, not inferred from scattered exports

## Immediate Task Pack

1. manifest inventory doc for current array/map/runtime_data/string symbols
   - [`abi-export-inventory.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/abi-export-inventory.md)
2. `AbiAdapterRegistryBox` default rows mapped into manifest vocabulary
3. `array.rs` / `map.rs` / `runtime_data.rs` export groups tagged as:
   - mainline substrate
   - compat-only
   - pure/historical compat
4. `value_codec/*` contract notes updated to use canonical value-class names

## Non-Goals

- broad ABI redesign in one wave
- changing current borrowed/owned return contract
- removing all compact export names immediately
- turning `boxed_local` into a public ABI type
